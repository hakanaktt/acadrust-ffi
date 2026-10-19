//! Configurable readers, recovery reports, DWG inspection, and 3D import.

use crate::views::{view, view_enum, view_struct};
use crate::*;
use acadrust::io::dwg::dwg_reader::{CrcExtractionReport, DwgFileHeaderInfo, DwgReadOptions};
use acadrust::io::dxf::DxfReaderConfiguration;
use acadrust::io::import::{self, ImportConfig};
use acadrust::io::{DwgReader, DwgWriter, DxfReader, DxfWriter, ReadOutcome, ReadStats};
use std::borrow::Cow;
use std::ffi::c_void;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::Arc;

pub type ReadProgressCallback = unsafe extern "C" fn(u16, *mut c_void);
pub type LegacyFontCallback = unsafe extern "C" fn(*const c_char, *mut c_void) -> *const c_char;
pub type DwgEntityVisitCallback = unsafe extern "C" fn(ValueView, *mut c_void) -> u8;

/// Initialize with `ar_read_options_default`; NULL options use these defaults.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ReadOptions {
    pub failsafe: u8,
    pub recover: u8,
    pub dxf_default_encoding: *const c_char,
    pub dwg_r12_code_page: *const c_char,
    pub progress: Option<ReadProgressCallback>,
    pub legacy_font_resolver: Option<LegacyFontCallback>,
    pub user_data: *mut c_void,
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self {
            failsafe: 0,
            recover: 0,
            dxf_default_encoding: ptr::null(),
            dwg_r12_code_page: ptr::null(),
            progress: None,
            legacy_font_resolver: None,
            user_data: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WriteOptions {
    pub version: u32,
    pub dwg_no_lz77: u8,
}

pub struct ReadReport {
    stats: ReadStats,
    repair: Option<acadrust::RepairReport>,
}

pub struct DwgInspection {
    inner: DwgInspectionData,
}

enum DwgInspectionData {
    Header(Box<DwgFileHeaderInfo>),
    Crcs(Box<CrcExtractionReport>),
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ImportColor {
    pub mode: i32,
    pub index: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const AR_IMPORT_COLOR_BY_LAYER: i32 = 0;
pub const AR_IMPORT_COLOR_BY_BLOCK: i32 = 1;
pub const AR_IMPORT_COLOR_NONE: i32 = 2;
pub const AR_IMPORT_COLOR_INDEX: i32 = 3;
pub const AR_IMPORT_COLOR_RGB: i32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImportOptions {
    pub layer_prefix: *const c_char,
    pub default_color: ImportColor,
    pub merge_vertices: u8,
    pub merge_tolerance: f64,
    pub scale_factor: f64,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            layer_prefix: ptr::null(),
            default_color: ImportColor::default(),
            merge_vertices: 1,
            merge_tolerance: 1e-9,
            scale_factor: 1.0,
        }
    }
}

pub const AR_IMPORT_AUTO: i32 = 0;
pub const AR_IMPORT_STL: i32 = 1;
pub const AR_IMPORT_COLLADA: i32 = 2;
pub const AR_IMPORT_OBJ: i32 = 3;
pub const AR_IMPORT_GLTF: i32 = 4;
pub const AR_IMPORT_GLB: i32 = 5;
pub const AR_IMPORT_FBX: i32 = 6;

struct OwnedReadOptions {
    failsafe: bool,
    recover: bool,
    default_encoding: Option<String>,
    r12_code_page: Option<String>,
    progress: Option<ReadProgressCallback>,
    legacy_font_resolver: Option<LegacyFontCallback>,
    user_data: usize,
}

fn flag(value: u8, name: &str) -> FfiResult<bool> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(invalid(format!("{name} must be 0 or 1"))),
    }
}

unsafe fn optional_string(value: *const c_char) -> FfiResult<Option<String>> {
    if value.is_null() {
        Ok(None)
    } else {
        input_str(value).map(|value| Some(value.to_owned()))
    }
}

unsafe fn read_options(options: *const ReadOptions) -> FfiResult<OwnedReadOptions> {
    let options = options.as_ref().copied().unwrap_or_default();
    let default_encoding = optional_string(options.dxf_default_encoding)?;
    let r12_code_page = optional_string(options.dwg_r12_code_page)?;
    for encoding in default_encoding.iter().chain(r12_code_page.iter()) {
        if acadrust::io::dxf::code_page::try_encoding_from_code_page(encoding).is_none() {
            return Err(invalid(format!("unsupported code page: {encoding}")));
        }
    }
    Ok(OwnedReadOptions {
        failsafe: flag(options.failsafe, "failsafe")?,
        recover: flag(options.recover, "recover")?,
        default_encoding,
        r12_code_page,
        progress: options.progress,
        legacy_font_resolver: options.legacy_font_resolver,
        user_data: options.user_data as usize,
    })
}

unsafe fn bytes(data: *const u8, len: usize) -> FfiResult<Vec<u8>> {
    if len == 0 || data.is_null() || len > isize::MAX as usize {
        return Err(invalid(
            "input buffer must be non-NULL with a valid, nonzero length",
        ));
    }
    Ok(std::slice::from_raw_parts(data, len).to_vec())
}

fn input_format<R: Read + Seek>(reader: &mut R, format: i32) -> FfiResult<i32> {
    match format {
        AR_FORMAT_AUTO => {
            let mut signature = [0; 6];
            let mut count = 0;
            while count < signature.len() {
                let read = reader.read(&mut signature[count..])?;
                if read == 0 {
                    break;
                }
                count += read;
            }
            reader.seek(SeekFrom::Start(0))?;
            Ok(if &signature[..4] == b"AC10" {
                AR_FORMAT_DWG
            } else {
                AR_FORMAT_DXF_ASCII
            })
        }
        AR_FORMAT_DXF_ASCII | AR_FORMAT_DXF_BINARY | AR_FORMAT_DWG => Ok(format),
        _ => Err(invalid("unknown input format")),
    }
}

fn dwg_reader<R: Read + Seek>(reader: R, options: &OwnedReadOptions) -> FfiResult<DwgReader<R>> {
    configure_dwg_reader(DwgReader::from_stream(reader), options)
}

fn configure_dwg_reader<R: Read + Seek>(
    mut reader: DwgReader<R>,
    options: &OwnedReadOptions,
) -> FfiResult<DwgReader<R>> {
    reader.options = DwgReadOptions {
        failsafe: options.failsafe,
    };
    if let Some(code_page) = &options.r12_code_page {
        reader = reader.with_r12_code_page(code_page)?;
    }
    let user_data = options.user_data;
    if let Some(callback) = options.progress {
        reader.set_progress_callback(Arc::new(move |value| unsafe {
            callback(value, user_data as *mut c_void);
        }));
    }
    if let Some(callback) = options.legacy_font_resolver {
        reader = reader.with_legacy_font_resolver(move |font| {
            let Ok(font_string) = CString::new(font) else {
                return font.to_owned();
            };
            let result = unsafe { callback(font_string.as_ptr(), user_data as *mut c_void) };
            if result.is_null() {
                font.to_owned()
            } else {
                unsafe { CStr::from_ptr(result) }
                    .to_str()
                    .unwrap_or(font)
                    .to_owned()
            }
        });
    }
    Ok(reader)
}

fn finish_dwg<R: Read + Seek>(
    mut reader: DwgReader<R>,
    options: &OwnedReadOptions,
) -> FfiResult<(CadDocument, ReadReport)> {
    if options.recover {
        let outcome = reader.recover()?;
        Ok((
            outcome.document,
            ReadReport {
                stats: outcome.stats,
                repair: Some(outcome.repair),
            },
        ))
    } else {
        let ReadOutcome { document, stats } = reader.read_with_stats()?;
        Ok((
            document,
            ReadReport {
                stats,
                repair: None,
            },
        ))
    }
}

fn read_configured<R: Read + Seek + 'static>(
    mut source: R,
    format: i32,
    options: &OwnedReadOptions,
) -> FfiResult<(CadDocument, ReadReport)> {
    let format = input_format(&mut source, format)?;
    if format == AR_FORMAT_DWG {
        return finish_dwg(dwg_reader(source, options)?, options);
    }
    let (document, stats, repair) = {
        let reader = DxfReader::from_reader(source)?.with_configuration(DxfReaderConfiguration {
            failsafe: options.failsafe,
            default_encoding: options.default_encoding.clone(),
        });
        if options.recover {
            let outcome = reader.recover()?;
            (outcome.document, outcome.stats, Some(outcome.repair))
        } else {
            let ReadOutcome { document, stats } = reader.read_with_stats()?;
            (document, stats, None)
        }
    };
    Ok((document, ReadReport { stats, repair }))
}

unsafe fn publish_read(
    inner: CadDocument,
    report: ReadReport,
    out: *mut *mut Document,
    out_report: *mut *mut ReadReport,
) -> FfiResult<()> {
    let report = Box::new(report);
    *out = Box::into_raw(Box::new(Document::new(inner)));
    *out_report = Box::into_raw(report);
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_read_options_default(out: *mut ReadOptions) -> Status {
    ffi_call(|| {
        *output(out)? = ReadOptions::default();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_write_options_default(out: *mut WriteOptions) -> Status {
    ffi_call(|| {
        *output(out)? = WriteOptions::default();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_import_options_default(out: *mut ImportOptions) -> Status {
    ffi_call(|| {
        *output(out)? = ImportOptions::default();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_read_file_ex(
    path: *const c_char,
    format: i32,
    options: *const ReadOptions,
    out: *mut *mut Document,
    out_report: *mut *mut ReadReport,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        *output(out_report)? = ptr::null_mut();
        let path = input_str(path)?.to_owned();
        let options = read_options(options)?;
        let (mut inner, report) =
            install(|| read_configured(File::open(&path)?, format, &options))??;
        inner.source_path = Some(path);
        publish_read(inner, report, out, out_report)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_read_buffer_ex(
    data: *const u8,
    len: usize,
    format: i32,
    options: *const ReadOptions,
    out: *mut *mut Document,
    out_report: *mut *mut ReadReport,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        *output(out_report)? = ptr::null_mut();
        let options = read_options(options)?;
        let data = bytes(data, len)?;
        let (inner, report) = install(|| read_configured(Cursor::new(data), format, &options))??;
        publish_read(inner, report, out, out_report)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_read_file_mmap(
    path: *const c_char,
    options: *const ReadOptions,
    out: *mut *mut Document,
    out_report: *mut *mut ReadReport,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        *output(out_report)? = ptr::null_mut();
        let path = input_str(path)?.to_owned();
        let options = read_options(options)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (inner, report) = install(|| {
                let reader = configure_dwg_reader(DwgReader::from_mmap(&path)?, &options)?;
                finish_dwg(reader, &options)
            })??;
            publish_read(inner, report, out, out_report)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (path, options);
            Err(invalid(
                "memory-mapped DWG input is unavailable on WebAssembly",
            ))
        }
    })
}

fn read_visiting<R: Read + Seek>(
    source: R,
    options: &OwnedReadOptions,
    callback: DwgEntityVisitCallback,
) -> FfiResult<CadDocument> {
    if options.recover {
        return Err(invalid(
            "visiting reads require recover=0; repair the returned document separately",
        ));
    }
    let mut reader = dwg_reader(source, options)?;
    let mut error = None;
    let doc = reader.read_visiting(|_, entity| {
        if error.is_some() {
            return None;
        }
        let result = unsafe { callback(view(&entity), options.user_data as *mut c_void) };
        match result {
            0 => None,
            1 => Some(entity),
            _ => {
                error = Some(invalid("entity visitor must return 0 (skip) or 1 (keep)"));
                None
            }
        }
    })?;
    match error {
        Some(error) => Err(error),
        None => Ok(doc),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_read_file_visiting(
    path: *const c_char,
    options: *const ReadOptions,
    visitor: Option<DwgEntityVisitCallback>,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let path = input_str(path)?.to_owned();
        let options = read_options(options)?;
        let visitor = visitor.ok_or_else(|| invalid("entity visitor is NULL"))?;
        let mut inner = install(|| read_visiting(File::open(&path)?, &options, visitor))??;
        inner.source_path = Some(path);
        *out = Box::into_raw(Box::new(Document::new(inner)));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_read_buffer_visiting(
    data: *const u8,
    len: usize,
    options: *const ReadOptions,
    visitor: Option<DwgEntityVisitCallback>,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let options = read_options(options)?;
        let visitor = visitor.ok_or_else(|| invalid("entity visitor is NULL"))?;
        let data = bytes(data, len)?;
        let inner = install(|| read_visiting(Cursor::new(data), &options, visitor))??;
        *out = Box::into_raw(Box::new(Document::new(inner)));
        Ok(())
    })
}

fn write_configured(doc: &CadDocument, format: i32, options: WriteOptions) -> FfiResult<Vec<u8>> {
    let no_lz77 = flag(options.dwg_no_lz77, "dwg_no_lz77")?;
    let version = if options.version == 0 {
        doc.version
    } else {
        version(options.version)?
    };
    if no_lz77 && (format != AR_FORMAT_DWG || version != DxfVersion::AC1021) {
        return Err(invalid("dwg_no_lz77 requires DWG output version 1021"));
    }
    match format {
        AR_FORMAT_DXF_ASCII => Ok(DxfWriter::new(doc).with_version(version).write_to_vec()?),
        AR_FORMAT_DXF_BINARY => Ok(DxfWriter::new_binary(doc)
            .with_version(version)
            .write_to_vec()?),
        AR_FORMAT_DWG => {
            let mut prepared = Cow::Borrowed(doc);
            if version != doc.version {
                prepared.to_mut().version = version;
            }
            if no_lz77 {
                let mut buffer = Cursor::new(Vec::new());
                DwgWriter::write_to_writer_no_lz77(&mut buffer, prepared.as_ref())?;
                Ok(buffer.into_inner())
            } else {
                Ok(DwgWriter::write_to_vec(prepared.as_ref())?)
            }
        }
        _ => Err(invalid(
            "output format must be DXF ASCII, DXF binary, or DWG",
        )),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_write_buffer_ex(
    doc: *const Document,
    format: i32,
    options: *const WriteOptions,
    out: *mut Buffer,
) -> Status {
    ffi_call(|| {
        *output(out)? = Buffer::default();
        let doc = document(doc)?;
        let options = options.as_ref().copied().unwrap_or_default();
        let bytes = install(|| write_configured(doc, format, options))??;
        store_buffer(bytes, out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_write_file_ex(
    doc: *const Document,
    path: *const c_char,
    format: i32,
    options: *const WriteOptions,
) -> Status {
    ffi_call(|| {
        let doc = document(doc)?;
        let path = input_str(path)?;
        let options = options.as_ref().copied().unwrap_or_default();
        let bytes = install(|| write_configured(doc, format, options))??;
        std::fs::write(path, bytes)?;
        Ok(())
    })
}

unsafe fn import_config(options: *const ImportOptions) -> FfiResult<ImportConfig> {
    let options = options.as_ref().copied().unwrap_or_default();
    if !options.merge_tolerance.is_finite() || options.merge_tolerance < 0.0 {
        return Err(invalid("merge_tolerance must be finite and nonnegative"));
    }
    if !options.scale_factor.is_finite() || options.scale_factor == 0.0 {
        return Err(invalid("scale_factor must be finite and nonzero"));
    }
    let mut config = ImportConfig {
        merge_vertices: flag(options.merge_vertices, "merge_vertices")?,
        merge_tolerance: options.merge_tolerance,
        scale_factor: options.scale_factor,
        ..ImportConfig::default()
    };
    if let Some(prefix) = optional_string(options.layer_prefix)? {
        config.layer_prefix = prefix;
    }
    config.default_color = match options.default_color {
        ImportColor {
            mode: AR_IMPORT_COLOR_BY_LAYER,
            ..
        } => acadrust::Color::ByLayer,
        ImportColor {
            mode: AR_IMPORT_COLOR_BY_BLOCK,
            ..
        } => acadrust::Color::ByBlock,
        ImportColor {
            mode: AR_IMPORT_COLOR_NONE,
            ..
        } => acadrust::Color::None,
        ImportColor {
            mode: AR_IMPORT_COLOR_INDEX,
            index: 1..=255,
            ..
        } => acadrust::Color::Index(options.default_color.index),
        ImportColor {
            mode: AR_IMPORT_COLOR_RGB,
            r,
            g,
            b,
            ..
        } => acadrust::Color::Rgb { r, g, b },
        _ => return Err(invalid("invalid import color mode or index")),
    };
    Ok(config)
}

fn import_bytes(data: Vec<u8>, format: i32, config: ImportConfig) -> FfiResult<CadDocument> {
    match format {
        AR_IMPORT_STL => Ok(import::StlImporter::from_bytes(data)
            .with_config(config)
            .import()?),
        AR_IMPORT_COLLADA => Ok(import::ColladaImporter::from_bytes(data)
            .with_config(config)
            .import()?),
        AR_IMPORT_OBJ => Ok(import::ObjImporter::from_bytes(data)
            .with_config(config)
            .import()?),
        AR_IMPORT_GLTF | AR_IMPORT_GLB => Ok(import::GltfImporter::from_bytes(data)
            .with_config(config)
            .import()?),
        AR_IMPORT_FBX => Ok(import::FbxImporter::from_bytes(data)
            .with_config(config)
            .import()?),
        _ => Err(invalid(
            "buffer import requires an explicit 3D import format",
        )),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_import_file(
    path: *const c_char,
    options: *const ImportOptions,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let path = input_str(path)?.to_owned();
        let config = import_config(options)?;
        let inner = install(|| import::import_file(&path, &config).map_err(FfiError::from))??;
        *out = Box::into_raw(Box::new(Document::new(inner)));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_import_buffer(
    data: *const u8,
    len: usize,
    format: i32,
    options: *const ImportOptions,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let config = import_config(options)?;
        let data = bytes(data, len)?;
        let inner = install(|| import_bytes(data, format, config))??;
        *out = Box::into_raw(Box::new(Document::new(inner)));
        Ok(())
    })
}

fn dwg_inspect<R: Read + Seek>(source: R, kind: i32) -> FfiResult<DwgInspection> {
    let mut reader = DwgReader::from_stream(source);
    let inner = match kind {
        AR_DWG_INSPECT_HEADER => DwgInspectionData::Header(Box::new(reader.read_file_header()?)),
        AR_DWG_INSPECT_CRCS => DwgInspectionData::Crcs(Box::new(reader.extract_all_crcs()?)),
        _ => return Err(invalid("DWG inspection kind must be HEADER or CRCS")),
    };
    Ok(DwgInspection { inner })
}

pub const AR_DWG_INSPECT_HEADER: i32 = 0;
pub const AR_DWG_INSPECT_CRCS: i32 = 1;

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_inspect_file(
    path: *const c_char,
    kind: i32,
    out: *mut *mut DwgInspection,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let path = input_str(path)?.to_owned();
        let inspection = install(|| dwg_inspect(File::open(path)?, kind))??;
        *out = Box::into_raw(Box::new(inspection));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_inspect_buffer(
    data: *const u8,
    len: usize,
    kind: i32,
    out: *mut *mut DwgInspection,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let data = bytes(data, len)?;
        let inspection = install(|| dwg_inspect(Cursor::new(data), kind))??;
        *out = Box::into_raw(Box::new(inspection));
        Ok(())
    })
}

fn dwg_section<R: Read + Seek>(source: R, name: &str) -> FfiResult<Vec<u8>> {
    let mut reader = DwgReader::from_stream(source);
    let header = reader.read_file_header()?;
    Ok(reader.get_section_buffer(name, &header)?)
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_section_file(
    path: *const c_char,
    section_name: *const c_char,
    out: *mut Buffer,
) -> Status {
    ffi_call(|| {
        *output(out)? = Buffer::default();
        let path = input_str(path)?.to_owned();
        let section_name = input_str(section_name)?.to_owned();
        let bytes = install(|| dwg_section(File::open(path)?, &section_name))??;
        store_buffer(bytes, out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_section_buffer(
    data: *const u8,
    len: usize,
    section_name: *const c_char,
    out: *mut Buffer,
) -> Status {
    ffi_call(|| {
        *output(out)? = Buffer::default();
        let data = bytes(data, len)?;
        let section_name = input_str(section_name)?.to_owned();
        let bytes = install(|| dwg_section(Cursor::new(data), &section_name))??;
        store_buffer(bytes, out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_read_report_view(
    report: *const ReadReport,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        *output(out)? = ValueView::default();
        let report = report
            .as_ref()
            .ok_or_else(|| invalid("read report is NULL"))?;
        *out = view(report);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_read_report_free(report: *mut ReadReport) {
    if !report.is_null() {
        drop(Box::from_raw(report));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_inspection_view(
    inspection: *const DwgInspection,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        *output(out)? = ValueView::default();
        let inspection = inspection
            .as_ref()
            .ok_or_else(|| invalid("DWG inspection is NULL"))?;
        *out = view(&inspection.inner);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_dwg_inspection_free(inspection: *mut DwgInspection) {
    if !inspection.is_null() {
        drop(Box::from_raw(inspection));
    }
}

view_struct!(ReadReport { stats, repair });
view_enum!(DwgInspectionData { Header(info), Crcs(report) });
view_enum!(acadrust::io::SourceFormat { Dwg, Dxf });
view_enum!(acadrust::io::ReadStage {
    FileHeader,
    Classes,
    Header,
    Handles,
    Objects,
    RecordStream,
    Section
});
view_struct!(acadrust::io::ReadDiagnostic {
    code,
    stage,
    section,
    source_offset,
    source_offset_basis,
    source_line,
    record_handle,
    record_type,
    message
});
view_struct!(ReadStats {
    source_format,
    source_version,
    maintenance_version,
    recovery_mode,
    stream_completed,
    source_sections,
    source_records,
    decoded_source_records,
    skipped_source_records,
    output_entities,
    output_objects,
    output_table_records,
    recovered_errors,
    record_stream_read,
    diagnostics
});
view_enum!(acadrust::io::dwg::dwg_version::DwgVersion {
    AC9,
    AC12,
    AC15,
    AC18,
    AC21,
    AC24
});
view_struct!(DwgFileHeaderInfo {
    version_string,
    version,
    acad_maintenance_version,
    preview_address,
    dwg_version,
    app_release_version,
    code_page,
    security_type,
    summary_info_addr,
    vba_project_addr,
    ac21_metadata,
    ac21_header_crc,
    ac21_unknown_key,
    ac21_compressed_data_crc,
    page_records,
    section_descriptors,
    section_locators,
    objects_base_offset,
    is_ac18_format
});
view_struct!(acadrust::io::dwg::dwg_reader::DwgSectionInfo {
    name,
    compressed_size,
    decompressed_size,
    encrypted,
    hash_code,
    encoding,
    page_count,
    pages
});
view_struct!(acadrust::io::dwg::dwg_reader::DwgPageCrcInfo {
    page_number,
    offset,
    size,
    decompressed_size,
    compressed_size,
    checksum,
    crc
});
view_struct!(acadrust::io::dwg::dwg_reader::PageCrcEntry {
    section_name,
    page_number,
    crc,
    checksum,
    compressed_size,
    decompressed_size
});
view_struct!(CrcExtractionReport {
    version,
    header_crc64,
    header_crc,
    compressed_data_crc,
    pages_map_crc_compressed,
    pages_map_crc_uncompressed,
    pages_map_crc_seed,
    sections_map_crc_compressed,
    sections_map_crc_uncompressed,
    sections_map_crc_seed,
    crc_seed,
    crc_seed_encoded,
    random_seed,
    page_crcs,
    notifications
});
view_struct!(acadrust::io::dwg::dwg21_metadata::Dwg21CompressedMetadata {
    header_size,
    file_size,
    pages_map_crc_compressed,
    pages_map_correction_factor,
    pages_map_crc_seed,
    map2_offset,
    map2_id,
    pages_map_offset,
    pages_map_id,
    header2_offset,
    pages_map_size_compressed,
    pages_map_size_uncompressed,
    pages_amount,
    pages_max_id,
    unknown_0x20,
    unknown_0x40,
    pages_map_crc_uncompressed,
    unknown_0xf800,
    unknown_4,
    unknown_1,
    sections_amount,
    sections_map_crc_uncompressed,
    sections_map_size_compressed,
    sections_map2_id,
    sections_map_id,
    sections_map_size_uncompressed,
    sections_map_crc_compressed,
    sections_map_correction_factor,
    sections_map_crc_seed,
    stream_version,
    crc_seed,
    crc_seed_encoded,
    random_seed,
    header_crc64
});

const _: fn(&ReadReport) = |value| {
    let _ = view(&value.stats);
    let _ = view(&value.repair);
};
const _: fn(&CrcExtractionReport) = |value| {
    let _ = view(&value.notifications);
};
