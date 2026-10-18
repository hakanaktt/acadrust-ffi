use crate::*;
use acadrust::{DwgReader, DwgWriter, DxfReader, DxfWriter};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub const AR_FORMAT_AUTO: i32 = 0;
pub const AR_FORMAT_DXF_ASCII: i32 = 1;
pub const AR_FORMAT_DXF_BINARY: i32 = 2;
pub const AR_FORMAT_DWG: i32 = 3;

pub(crate) fn read_document<R: Read + Seek + 'static>(
    mut reader: R,
    format: i32,
) -> FfiResult<CadDocument> {
    let format = if format == AR_FORMAT_AUTO {
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
        if &signature[..4] == b"AC10" {
            AR_FORMAT_DWG
        } else {
            AR_FORMAT_DXF_ASCII
        }
    } else {
        format
    };
    match format {
        AR_FORMAT_DXF_ASCII | AR_FORMAT_DXF_BINARY => Ok(DxfReader::from_reader(reader)?.read()?),
        AR_FORMAT_DWG => Ok(DwgReader::from_stream(reader).read()?),
        _ => Err(invalid("unknown input format")),
    }
}

pub(crate) fn write_document(doc: &CadDocument, format: i32) -> FfiResult<Vec<u8>> {
    match format {
        AR_FORMAT_DXF_ASCII => Ok(DxfWriter::new(doc)
            .with_version(doc.version)
            .write_to_vec()?),
        AR_FORMAT_DXF_BINARY => Ok(DxfWriter::new_binary(doc)
            .with_version(doc.version)
            .write_to_vec()?),
        AR_FORMAT_DWG => Ok(DwgWriter::write_to_vec(doc)?),
        _ => Err(invalid(
            "output format must be DXF ASCII, DXF binary, or DWG",
        )),
    }
}

pub(crate) fn read_file(path: &str) -> FfiResult<CadDocument> {
    read_document(File::open(path)?, AR_FORMAT_AUTO)
}

pub(crate) fn write_file(doc: &CadDocument, path: &str, format: i32) -> FfiResult<()> {
    match format {
        AR_FORMAT_DXF_ASCII => DxfWriter::new(doc)
            .with_version(doc.version)
            .write_to_file(path)?,
        AR_FORMAT_DXF_BINARY => DxfWriter::new_binary(doc)
            .with_version(doc.version)
            .write_to_file(path)?,
        AR_FORMAT_DWG => DwgWriter::write_to_file(path, doc)?,
        _ => {
            return Err(invalid(
                "output format must be DXF ASCII, DXF binary, or DWG",
            ))
        }
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_read_file(
    path: *const c_char,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        let path = input_str(path)?.to_owned();
        let inner = install(|| read_file(&path))??;
        *out = Box::into_raw(Box::new(Document { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_read_buffer(
    data: *const u8,
    len: usize,
    format: i32,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        if len == 0 || data.is_null() || len > isize::MAX as usize {
            return Err(invalid(
                "input buffer must be non-NULL with a valid, nonzero length",
            ));
        }
        // Readers own their input; never keep a caller's memory after returning.
        let bytes = std::slice::from_raw_parts(data, len).to_vec();
        let inner = install(|| read_document(Cursor::new(bytes), format))??;
        *out = Box::into_raw(Box::new(Document { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_write_file(
    doc: *const Document,
    path: *const c_char,
    format: i32,
) -> Status {
    ffi_call(|| {
        let doc = document(doc)?;
        let path = input_str(path)?;
        install(|| write_file(doc, path, format))??;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_write_buffer(
    doc: *const Document,
    format: i32,
    out: *mut Buffer,
) -> Status {
    ffi_call(|| {
        *output(out)? = Buffer::default();
        let document = document(doc)?;
        let bytes = install(|| write_document(document, format))??;
        store_buffer(bytes, out)
    })
}
