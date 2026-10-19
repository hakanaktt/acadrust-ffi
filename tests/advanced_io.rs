use acadrust_ffi::*;
use std::ffi::{c_void, CStr, CString};
use std::ptr;
use std::sync::atomic::{AtomicU16, AtomicUsize, Ordering};

struct Drawing(*mut Document);

impl Drawing {
    fn new() -> Self {
        let mut doc = ptr::null_mut();
        unsafe { ok(ar_document_new(1032, &mut doc)) };
        Self(doc)
    }

    fn line(&self) {
        let line = Line {
            start: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            end: Vector3 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
        };
        let mut handle = 0;
        unsafe { ok(ar_line_add(self.0, &line, &mut handle)) };
    }

    fn count(&self) -> usize {
        let mut count = 0;
        unsafe { ok(ar_entity_count(self.0, &mut count)) };
        count
    }

    fn write(&self, format: i32, options: &WriteOptions) -> Vec<u8> {
        unsafe {
            let mut buffer = Buffer::default();
            ok(ar_document_write_buffer_ex(
                self.0,
                format,
                options,
                &mut buffer,
            ));
            let bytes = std::slice::from_raw_parts(buffer.data, buffer.len).to_vec();
            ar_buffer_free(&mut buffer);
            bytes
        }
    }
}

impl Drop for Drawing {
    fn drop(&mut self) {
        unsafe { ar_document_free(self.0) }
    }
}

fn ok(status: Status) {
    assert_eq!(status, Status::Ok, "{}", unsafe {
        CStr::from_ptr(ar_last_error()).to_string_lossy()
    });
}

struct Report(*mut ReadReport);

impl Report {
    fn view(&self) -> ValueView {
        let mut value = ValueView::default();
        unsafe { ok(ar_read_report_view(self.0, &mut value)) };
        value
    }

    fn stats(&self) -> ValueView {
        field(self.view(), c"stats")
    }
}

impl Drop for Report {
    fn drop(&mut self) {
        unsafe { ar_read_report_free(self.0) }
    }
}

fn field(value: ValueView, name: &CStr) -> ValueView {
    let mut out = ValueView::default();
    unsafe { ok(ar_value_field(value, name.as_ptr(), &mut out)) };
    out
}

fn child(value: ValueView, index: usize) -> ValueView {
    let mut out = ValueView::default();
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    unsafe { ok(ar_value_child(value, index, &mut name, &mut out)) };
    out
}

fn len(value: ValueView) -> usize {
    let mut out = 0;
    unsafe { ok(ar_value_len(value, &mut out)) };
    out
}

fn scalar<T: Copy>(value: ValueView) -> T {
    unsafe {
        let mut data = ptr::null();
        let mut len = 0;
        let mut stride = 0;
        ok(ar_value_data(value, &mut data, &mut len, &mut stride));
        assert_eq!(len, 1);
        assert_eq!(stride, std::mem::size_of::<T>());
        *data.cast::<T>()
    }
}

fn string(value: ValueView) -> String {
    unsafe {
        let mut data = ptr::null();
        let mut len = 0;
        let mut stride = 0;
        ok(ar_value_data(value, &mut data, &mut len, &mut stride));
        assert_eq!(stride, 1);
        std::str::from_utf8(std::slice::from_raw_parts(data.cast::<u8>(), len))
            .unwrap()
            .to_owned()
    }
}

fn variant(value: ValueView) -> String {
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    unsafe {
        ok(ar_value_variant(value, &mut name));
        std::str::from_utf8(std::slice::from_raw_parts(name.data.cast::<u8>(), name.len))
            .unwrap()
            .to_owned()
    }
}

fn read(bytes: &[u8], options: &ReadOptions) -> (Drawing, Report) {
    unsafe {
        let mut doc = ptr::null_mut();
        let mut report = ptr::null_mut();
        ok(ar_document_read_buffer_ex(
            bytes.as_ptr(),
            bytes.len(),
            AR_FORMAT_AUTO,
            options,
            &mut doc,
            &mut report,
        ));
        (Drawing(doc), Report(report))
    }
}

#[test]
fn reports_and_export_versions_roundtrip_without_mutating_source() {
    let original = Drawing::new();
    original.line();
    for format in [AR_FORMAT_DXF_ASCII, AR_FORMAT_DXF_BINARY, AR_FORMAT_DWG] {
        let bytes = original.write(
            format,
            &WriteOptions {
                version: 1015,
                dwg_no_lz77: 0,
            },
        );
        let (copy, report) = read(&bytes, &ReadOptions::default());
        assert_eq!(copy.count(), 1);
        assert_eq!(
            scalar::<usize>(field(report.stats(), c"output_entities")),
            1
        );
        assert!(scalar::<bool>(field(report.stats(), c"stream_completed")));
        assert!(!scalar::<bool>(field(report.stats(), c"recovery_mode")));
        assert_eq!(len(field(report.view(), c"repair")), 0);
        let mut version = 0;
        unsafe { ok(ar_document_get_version(copy.0, &mut version)) };
        assert_eq!(
            version,
            if format == AR_FORMAT_DXF_BINARY {
                1024
            } else {
                1015
            }
        );
    }
    let mut version = 0;
    unsafe { ok(ar_document_get_version(original.0, &mut version)) };
    assert_eq!(version, 1032);
}

#[test]
fn report_views_reuse_native_storage_after_document_is_freed() {
    let original = Drawing::new();
    original.line();
    let bytes = original.write(AR_FORMAT_DXF_ASCII, &WriteOptions::default());
    let (doc, report) = read(&bytes, &ReadOptions::default());
    let version = field(report.stats(), c"source_version");
    let mut before = ptr::null();
    let mut count = 0;
    let mut stride = 0;
    unsafe { ok(ar_value_data(version, &mut before, &mut count, &mut stride)) };
    assert!(count > 0);
    assert_eq!(stride, 1);
    drop(doc);
    let mut after = ptr::null();
    unsafe {
        ok(ar_value_data(
            field(report.stats(), c"source_version"),
            &mut after,
            &mut count,
            &mut stride,
        ))
    };
    assert_eq!(before, after);
    assert!(!string(version).is_empty());
}

#[test]
fn failsafe_reports_truncation_and_recovery_repairs_usable_records() {
    let original = Drawing::new();
    original.line();
    let bytes = original.write(AR_FORMAT_DXF_ASCII, &WriteOptions::default());
    let text = std::str::from_utf8(&bytes).unwrap();
    let eof = text.rfind("EOF").unwrap();
    let truncated = &bytes[..eof];
    let (_, report) = read(
        truncated,
        &ReadOptions {
            failsafe: 1,
            ..ReadOptions::default()
        },
    );
    assert!(scalar::<bool>(field(report.stats(), c"recovery_mode")));
    assert!(!scalar::<bool>(field(report.stats(), c"stream_completed")));
    let diagnostics = field(report.stats(), c"diagnostics");
    assert!(len(diagnostics) > 0);
    assert!(!string(field(child(diagnostics, 0), c"message")).is_empty());
    assert_eq!(len(field(report.view(), c"repair")), 0);
    let (recovered, report) = read(
        truncated,
        &ReadOptions {
            recover: 1,
            ..ReadOptions::default()
        },
    );
    assert_eq!(recovered.count(), 1);
    let repair = field(report.view(), c"repair");
    assert_eq!(len(repair), 1);
    assert!(
        scalar::<usize>(field(
            field(child(repair, 0), c"after"),
            c"scanned_entities"
        )) > 0
    );
    let empty = b"0\nSECTION\n2\nENTITIES\n0\nENDSEC\n0\nEOF\n";
    unsafe {
        let mut doc = ptr::null_mut();
        let mut report = ptr::null_mut();
        assert_eq!(
            ar_document_read_buffer_ex(
                empty.as_ptr(),
                empty.len(),
                AR_FORMAT_AUTO,
                &ReadOptions {
                    recover: 1,
                    ..ReadOptions::default()
                },
                &mut doc,
                &mut report,
            ),
            Status::ParseError
        );
        assert!(doc.is_null());
        assert!(report.is_null());
    }
}

struct Progress {
    calls: AtomicUsize,
    value: AtomicU16,
}

unsafe extern "C" fn progress(value: u16, user_data: *mut c_void) {
    let state = &*(user_data as *const Progress);
    state.calls.fetch_add(1, Ordering::Relaxed);
    state.value.fetch_max(value, Ordering::Relaxed);
}

#[test]
fn dwg_progress_no_compression_and_metadata_inspection() {
    let original = Drawing::new();
    original.line();
    let bytes = original.write(
        AR_FORMAT_DWG,
        &WriteOptions {
            version: 1021,
            dwg_no_lz77: 1,
        },
    );
    let state = Progress {
        calls: AtomicUsize::new(0),
        value: AtomicU16::new(0),
    };
    let (copy, _) = read(
        &bytes,
        &ReadOptions {
            progress: Some(progress),
            user_data: &state as *const Progress as *mut c_void,
            ..ReadOptions::default()
        },
    );
    assert_eq!(copy.count(), 1);
    assert!(state.calls.load(Ordering::Relaxed) > 0);
    assert_eq!(state.value.load(Ordering::Relaxed), 1000);
    unsafe {
        let mut inspection = ptr::null_mut();
        ok(ar_dwg_inspect_buffer(
            bytes.as_ptr(),
            bytes.len(),
            AR_DWG_INSPECT_HEADER,
            &mut inspection,
        ));
        let mut header = ValueView::default();
        ok(ar_dwg_inspection_view(inspection, &mut header));
        assert_eq!(variant(header), "Header");
        let header = child(header, 0);
        assert_eq!(string(field(header, c"version_string")), "AC1021");
        assert!(len(field(header, c"section_descriptors")) > 0);
        let metadata = child(field(header, c"ac21_metadata"), 0);
        assert!(scalar::<u64>(field(metadata, c"file_size")) > 0);
        ar_dwg_inspection_free(inspection);
        ok(ar_dwg_inspect_buffer(
            bytes.as_ptr(),
            bytes.len(),
            AR_DWG_INSPECT_CRCS,
            &mut inspection,
        ));
        let mut crcs = ValueView::default();
        ok(ar_dwg_inspection_view(inspection, &mut crcs));
        assert_eq!(variant(crcs), "Crcs");
        let crcs = child(crcs, 0);
        assert_eq!(string(field(crcs, c"version")), "AC1021");
        assert!(len(field(crcs, c"page_crcs")) > 0);
        ar_dwg_inspection_free(inspection);
        let mut section = Buffer::default();
        ok(ar_dwg_section_buffer(
            bytes.as_ptr(),
            bytes.len(),
            c"AcDb:Header".as_ptr(),
            &mut section,
        ));
        assert!(section.len > 0);
        ar_buffer_free(&mut section);
    }
}

unsafe extern "C" fn keep_lines(entity: ValueView, user_data: *mut c_void) -> u8 {
    let calls = &*(user_data as *const AtomicUsize);
    calls.fetch_add(1, Ordering::Relaxed);
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    if ar_value_variant(entity, &mut name) != Status::Ok {
        return 2;
    }
    u8::from(std::slice::from_raw_parts(name.data.cast::<u8>(), name.len) == b"Line")
}

#[test]
fn dwg_entity_visitors_filter_native_borrowed_entities() {
    let original = Drawing::new();
    original.line();
    let mut handle = 0;
    unsafe {
        ok(ar_point_add(
            original.0,
            &Point {
                position: Vector3 {
                    x: 5.0,
                    y: 6.0,
                    z: 7.0,
                },
            },
            &mut handle,
        ));
    }
    let bytes = original.write(AR_FORMAT_DWG, &WriteOptions::default());
    let calls = AtomicUsize::new(0);
    let options = ReadOptions {
        user_data: &calls as *const AtomicUsize as *mut c_void,
        ..ReadOptions::default()
    };
    let mut doc = ptr::null_mut();
    unsafe {
        ok(ar_dwg_read_buffer_visiting(
            bytes.as_ptr(),
            bytes.len(),
            &options,
            Some(keep_lines),
            &mut doc,
        ));
    }
    let filtered = Drawing(doc);
    assert!(calls.load(Ordering::Relaxed) >= 2);
    assert_eq!(filtered.count(), 1);
}

unsafe extern "C" fn legacy_font(
    _font: *const std::ffi::c_char,
    user_data: *mut c_void,
) -> *const std::ffi::c_char {
    (&*(user_data as *const AtomicUsize)).fetch_add(1, Ordering::Relaxed);
    c"fallback-font".as_ptr()
}

#[test]
fn r12_encoding_and_font_resolution_reach_upstream_reader() {
    let mut native = acadrust::CadDocument::with_version(acadrust::DxfVersion::AC1009);
    native.header.code_page = "ANSI_1252".into();
    native.text_styles.get_mut("Standard").unwrap().font_file = "missing.shx".into();
    native
        .add_entity(acadrust::EntityType::Text(acadrust::entities::Text {
            value: "caf\u{e9}".into(),
            ..acadrust::entities::Text::default()
        }))
        .unwrap();
    let bytes = acadrust::DwgWriter::write_to_vec(&native).unwrap();
    let calls = AtomicUsize::new(0);
    let (copy, _) = read(
        &bytes,
        &ReadOptions {
            dwg_r12_code_page: c"ANSI_1252".as_ptr(),
            legacy_font_resolver: Some(legacy_font),
            user_data: &calls as *const AtomicUsize as *mut c_void,
            ..ReadOptions::default()
        },
    );
    assert!(calls.load(Ordering::Relaxed) > 0);
    unsafe {
        let mut entity = ValueView::default();
        ok(ar_document_entity_at_view(copy.0, 0, &mut entity));
        assert_eq!(variant(entity), "Text");
        assert_eq!(string(field(child(entity, 0), c"value")), "caf\u{e9}");
    }
}

#[test]
fn invalid_options_and_outputs_are_rejected_before_writing_files() {
    unsafe {
        let mut doc = ptr::null_mut();
        let mut report = ptr::null_mut();
        assert_eq!(
            ar_document_read_buffer_ex(
                b"x".as_ptr(),
                1,
                AR_FORMAT_AUTO,
                &ReadOptions {
                    failsafe: 2,
                    ..ReadOptions::default()
                },
                &mut doc,
                &mut report,
            ),
            Status::InvalidArgument
        );
        assert!(doc.is_null());
        assert!(report.is_null());
        assert_eq!(
            ar_document_read_buffer_ex(
                b"x".as_ptr(),
                1,
                AR_FORMAT_AUTO,
                &ReadOptions {
                    dxf_default_encoding: c"invalid-code-page".as_ptr(),
                    ..ReadOptions::default()
                },
                &mut doc,
                &mut report,
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_document_read_buffer_ex(
                ptr::null(),
                10,
                AR_FORMAT_AUTO,
                ptr::null(),
                &mut doc,
                &mut report,
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_read_options_default(ptr::null_mut()),
            Status::InvalidArgument
        );
        let drawing = Drawing::new();
        let mut buffer = Buffer::default();
        assert_eq!(
            ar_document_write_buffer_ex(
                drawing.0,
                AR_FORMAT_DWG,
                &WriteOptions {
                    version: 1032,
                    dwg_no_lz77: 1
                },
                &mut buffer,
            ),
            Status::InvalidArgument
        );
        assert!(buffer.data.is_null());
        let path =
            std::env::temp_dir().join(format!("acadrust-ffi-invalid-{}.dwg", std::process::id()));
        std::fs::write(&path, b"keep existing data").unwrap();
        let path_c = CString::new(path.to_str().unwrap()).unwrap();
        assert_eq!(
            ar_document_write_file_ex(drawing.0, path_c.as_ptr(), AR_FORMAT_AUTO, ptr::null(),),
            Status::InvalidArgument
        );
        assert_eq!(std::fs::read(&path).unwrap(), b"keep existing data");
        std::fs::remove_file(path).unwrap();
    }
}

const STL: &[u8] = b"solid triangle\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\nendsolid triangle\n";
const OBJ: &[u8] = b"o triangle\nv 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
const COLLADA: &[u8] = br##"<COLLADA version="1.4.1"><library_geometries>
<geometry id="g0" name="Triangle"><mesh>
<source id="p0"><float_array id="p0a" count="9">0 0 0 1 0 0 0 1 0</float_array>
<technique_common><accessor source="#p0a" count="3" stride="3"/></technique_common></source>
<vertices id="v0"><input semantic="POSITION" source="#p0"/></vertices>
<triangles count="1"><input semantic="VERTEX" source="#v0" offset="0"/><p>0 1 2</p></triangles>
</mesh></geometry></library_geometries></COLLADA>"##;
const FBX: &[u8] = br#"Objects: {
    Geometry: 100, "Geometry::Triangle", "Mesh" {
        Vertices: *9 {
            a: 0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0
        }
        PolygonVertexIndex: *3 {
            a: 0,1,-3
        }
    }
}
"#;

fn gltf(uri: Option<&str>) -> serde_json::Value {
    let mut value = serde_json::json!({
        "asset": {"version": "2.0"},
        "buffers": [{"byteLength": 42}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": 0, "byteLength": 36},
            {"buffer": 0, "byteOffset": 36, "byteLength": 6}
        ],
        "accessors": [
            {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3"},
            {"bufferView": 1, "componentType": 5123, "count": 3, "type": "SCALAR"}
        ],
        "meshes": [{"name": "Triangle", "primitives": [{"attributes": {"POSITION": 0}, "indices": 1}]}]
    });
    if let Some(uri) = uri {
        value["buffers"][0]["uri"] = uri.into();
    }
    value
}

fn glb() -> Vec<u8> {
    let mut json = serde_json::to_vec(&gltf(None)).unwrap();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let mut binary = Vec::new();
    for value in [0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
        binary.extend_from_slice(&value.to_le_bytes());
    }
    for index in [0_u16, 1, 2] {
        binary.extend_from_slice(&index.to_le_bytes());
    }
    while !binary.len().is_multiple_of(4) {
        binary.push(0);
    }
    let mut bytes = b"glTF".to_vec();
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    bytes.extend_from_slice(&((12 + 8 + json.len() + 8 + binary.len()) as u32).to_le_bytes());
    bytes.extend_from_slice(&(json.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&0x4e4f534a_u32.to_le_bytes());
    bytes.extend(json);
    bytes.extend_from_slice(&(binary.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&0x004e4942_u32.to_le_bytes());
    bytes.extend(binary);
    bytes
}

#[test]
fn all_six_import_formats_produce_meshes_with_configured_scale_and_color() {
    let gltf = serde_json::to_vec(&gltf(Some("data:application/octet-stream;base64,AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAAAAABAAIA"))).unwrap();
    let glb = glb();
    for (format, bytes) in [
        (AR_IMPORT_STL, STL),
        (AR_IMPORT_COLLADA, COLLADA),
        (AR_IMPORT_OBJ, OBJ),
        (AR_IMPORT_GLTF, gltf.as_slice()),
        (AR_IMPORT_GLB, glb.as_slice()),
        (AR_IMPORT_FBX, FBX),
    ] {
        unsafe {
            let mut doc = ptr::null_mut();
            ok(ar_document_import_buffer(
                bytes.as_ptr(),
                bytes.len(),
                format,
                &ImportOptions {
                    layer_prefix: c"test".as_ptr(),
                    default_color: ImportColor {
                        mode: AR_IMPORT_COLOR_RGB,
                        index: 0,
                        r: 12,
                        g: 34,
                        b: 56,
                    },
                    scale_factor: 3.0,
                    ..ImportOptions::default()
                },
                &mut doc,
            ));
            let doc = Drawing(doc);
            assert_eq!(doc.count(), 1, "import format {format}");
            let mut handle = 0;
            let mut required = 0;
            ok(ar_entity_handles(doc.0, &mut handle, 1, &mut required));
            let mut entity = ValueView::default();
            ok(ar_entity_view(doc.0, handle, &mut entity));
            assert_eq!(variant(entity), "Mesh");
            let mesh = child(entity, 0);
            let vertices = field(mesh, c"vertices");
            assert_eq!(len(vertices), 3);
            assert_eq!(len(field(mesh, c"faces")), 1);
            let common = field(mesh, c"common");
            assert!(string(field(common, c"layer")).starts_with("test"));
            let mut color = field(common, c"color");
            if variant(color) == "ByLayer" {
                let layer_name = CString::new(string(field(common, c"layer"))).unwrap();
                let mut layer = ValueView::default();
                ok(ar_table_find_view(
                    doc.0,
                    AR_TABLE_LAYER,
                    layer_name.as_ptr(),
                    &mut layer,
                ));
                color = field(layer, c"color");
            }
            assert_eq!(variant(color), "Rgb");
            assert_eq!(scalar::<u8>(field(color, c"r")), 12);
            assert_eq!(scalar::<u8>(field(color, c"g")), 34);
            assert_eq!(scalar::<u8>(field(color, c"b")), 56);
            assert!((0..len(vertices))
                .any(|index| scalar::<f64>(field(child(vertices, index), c"x")) == 3.0));
        }
    }
}

#[test]
fn file_import_and_extended_read_set_source_path() {
    unsafe {
        let path =
            std::env::temp_dir().join(format!("acadrust-ffi-import-{}.obj", std::process::id()));
        std::fs::write(&path, OBJ).unwrap();
        let path_c = CString::new(path.to_str().unwrap()).unwrap();
        let mut doc = ptr::null_mut();
        ok(ar_document_import_file(
            path_c.as_ptr(),
            ptr::null(),
            &mut doc,
        ));
        let drawing = Drawing(doc);
        assert_eq!(drawing.count(), 1);
        let dxf_path = path.with_extension("dxf");
        let dxf_path_c = CString::new(dxf_path.to_str().unwrap()).unwrap();
        ok(ar_document_write_file_ex(
            drawing.0,
            dxf_path_c.as_ptr(),
            AR_FORMAT_DXF_ASCII,
            ptr::null(),
        ));
        let mut report = ptr::null_mut();
        ok(ar_document_read_file_ex(
            dxf_path_c.as_ptr(),
            AR_FORMAT_AUTO,
            ptr::null(),
            &mut doc,
            &mut report,
        ));
        let copy = Drawing(doc);
        assert_eq!(copy.count(), 1);
        let mut document_view = ValueView::default();
        ok(ar_document_view(copy.0, &mut document_view));
        assert_eq!(
            string(child(field(document_view, c"source_path"), 0)),
            dxf_path.to_str().unwrap()
        );
        let report = Report(report);
        assert_eq!(
            variant(child(field(report.stats(), c"source_format"), 0)),
            "Dxf"
        );
        let dwg_path = path.with_extension("dwg");
        let dwg_path_c = CString::new(dwg_path.to_str().unwrap()).unwrap();
        ok(ar_document_write_file_ex(
            drawing.0,
            dwg_path_c.as_ptr(),
            AR_FORMAT_DWG,
            &WriteOptions {
                version: 1021,
                dwg_no_lz77: 0,
            },
        ));
        let mut mmap_report = ptr::null_mut();
        ok(ar_dwg_read_file_mmap(
            dwg_path_c.as_ptr(),
            ptr::null(),
            &mut doc,
            &mut mmap_report,
        ));
        let mapped = Drawing(doc);
        let mmap_report = Report(mmap_report);
        assert_eq!(mapped.count(), 1);
        assert_eq!(
            variant(child(field(mmap_report.stats(), c"source_format"), 0)),
            "Dwg"
        );
        let mut inspection = ptr::null_mut();
        ok(ar_dwg_inspect_file(
            dwg_path_c.as_ptr(),
            AR_DWG_INSPECT_HEADER,
            &mut inspection,
        ));
        ar_dwg_inspection_free(inspection);
        let mut section = Buffer::default();
        ok(ar_dwg_section_file(
            dwg_path_c.as_ptr(),
            c"AcDb:Header".as_ptr(),
            &mut section,
        ));
        assert!(section.len > 0);
        ar_buffer_free(&mut section);
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(dxf_path).unwrap();
        std::fs::remove_file(dwg_path).unwrap();
    }
}
