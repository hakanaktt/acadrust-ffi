//! C ABI for acadrust. See `include/acadrust.h` for ownership and calling rules.

#![allow(clippy::missing_safety_doc)]

use acadrust::{CadDocument, DxfError, DxfVersion};
use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

mod drawing_io;
mod entities;
mod layers;
mod parallel_io;

pub use drawing_io::*;
pub use entities::*;
pub use layers::*;
pub use parallel_io::*;

/// Status values are fixed-width integers at the ABI boundary.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok = 0,
    InvalidArgument = 1,
    NotFound = 2,
    TypeMismatch = 3,
    IoError = 4,
    ParseError = 5,
    BufferTooSmall = 6,
    Panic = 7,
}

/// Opaque to C callers. Allocate and release only using this library.
pub struct Document {
    pub(crate) inner: CadDocument,
}

/// Owned bytes. Initialize to zero and release with `ar_buffer_free`.
#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
            len: 0,
        }
    }
}

#[derive(Debug)]
pub(crate) struct FfiError {
    pub status: Status,
    pub message: String,
}

pub(crate) type FfiResult<T> = Result<T, FfiError>;

impl From<DxfError> for FfiError {
    fn from(error: DxfError) -> Self {
        let status = match &error {
            DxfError::Io(_) => Status::IoError,
            DxfError::ObjectNotFound(_) | DxfError::InvalidHandle(_) => Status::NotFound,
            DxfError::InvalidEntityType(_) => Status::TypeMismatch,
            _ => Status::ParseError,
        };
        Self {
            status,
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for FfiError {
    fn from(error: std::io::Error) -> Self {
        Self {
            status: Status::IoError,
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for FfiError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            status: Status::ParseError,
            message: error.to_string(),
        }
    }
}

pub(crate) fn invalid(message: impl Into<String>) -> FfiError {
    FfiError {
        status: Status::InvalidArgument,
        message: message.into(),
    }
}

pub(crate) fn not_found(message: impl Into<String>) -> FfiError {
    FfiError {
        status: Status::NotFound,
        message: message.into(),
    }
}

pub(crate) fn type_mismatch(message: impl Into<String>) -> FfiError {
    FfiError {
        status: Status::TypeMismatch,
        message: message.into(),
    }
}

thread_local! {
    static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

fn set_error(message: &str) {
    LAST_ERROR.with(|last| {
        // Error messages may contain file data, including embedded NUL bytes.
        *last.borrow_mut() = CString::new(message.replace('\0', "\\0")).unwrap_or_default();
    });
}

pub(crate) fn catch_ffi<T>(f: impl FnOnce() -> FfiResult<T>) -> FfiResult<T> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                .unwrap_or("unknown Rust panic");
            let error = FfiError {
                status: Status::Panic,
                message: format!("Rust panic: {message}"),
            };
            // A custom panic payload can itself panic when dropped.
            std::mem::forget(payload);
            Err(error)
        }
    }
}

pub(crate) fn ffi_call(f: impl FnOnce() -> FfiResult<()>) -> Status {
    match catch_ffi(f) {
        Ok(()) => {
            set_error("");
            Status::Ok
        }
        Err(error) => {
            set_error(&error.message);
            error.status
        }
    }
}

pub(crate) unsafe fn document<'a>(doc: *const Document) -> FfiResult<&'a CadDocument> {
    doc.as_ref()
        .map(|doc| &doc.inner)
        .ok_or_else(|| invalid("document is NULL"))
}

pub(crate) unsafe fn document_mut<'a>(doc: *mut Document) -> FfiResult<&'a mut CadDocument> {
    doc.as_mut()
        .map(|doc| &mut doc.inner)
        .ok_or_else(|| invalid("document is NULL"))
}

pub(crate) unsafe fn output<'a, T>(out: *mut T) -> FfiResult<&'a mut T> {
    out.as_mut()
        .ok_or_else(|| invalid("output pointer is NULL"))
}

pub(crate) unsafe fn input_str<'a>(text: *const c_char) -> FfiResult<&'a str> {
    if text.is_null() {
        return Err(invalid("string is NULL"));
    }
    CStr::from_ptr(text)
        .to_str()
        .map_err(|_| invalid("string is not valid UTF-8"))
}

pub(crate) unsafe fn store_string(text: &str, out: *mut *mut c_char) -> FfiResult<()> {
    let out = output(out)?;
    *out = ptr::null_mut();
    let text = CString::new(text).map_err(|_| invalid("string contains an embedded NUL"))?;
    *out = text.into_raw();
    Ok(())
}

pub(crate) unsafe fn store_buffer(bytes: Vec<u8>, out: *mut Buffer) -> FfiResult<()> {
    let out = output(out)?;
    *out = Buffer::default();
    if !bytes.is_empty() {
        let bytes = bytes.into_boxed_slice();
        out.len = bytes.len();
        out.data = Box::into_raw(bytes) as *mut u8;
    }
    Ok(())
}

pub(crate) fn version(code: u32) -> FfiResult<DxfVersion> {
    let result = u16::try_from(code).ok().map(DxfVersion::from_version_code);
    match result {
        Some(version) if version != DxfVersion::Unknown => Ok(version),
        _ => Err(invalid(
            "unsupported version; use 1009, 1012, 1014, 1015, 1018, 1021, 1024, 1027, or 1032",
        )),
    }
}

#[no_mangle]
pub extern "C" fn ar_abi_version() -> u32 {
    1
}

#[no_mangle]
pub extern "C" fn ar_version() -> *const c_char {
    c"0.1.0".as_ptr()
}

#[no_mangle]
pub extern "C" fn ar_upstream_version() -> *const c_char {
    c"0.6.5".as_ptr()
}

#[no_mangle]
pub extern "C" fn ar_last_error() -> *const c_char {
    LAST_ERROR.with(|error| error.borrow().as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn ar_string_free(text: *mut c_char) {
    if !text.is_null() {
        drop(CString::from_raw(text));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_buffer_free(buffer: *mut Buffer) {
    if let Some(buffer) = buffer.as_mut() {
        let owned = std::mem::take(buffer);
        if !owned.data.is_null() {
            drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
                owned.data, owned.len,
            )));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_new(code: u32, out: *mut *mut Document) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        let inner = CadDocument::with_version(version(code)?);
        *out = Box::into_raw(Box::new(Document { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_clone(
    doc: *const Document,
    out: *mut *mut Document,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        let inner = document(doc)?.clone();
        *out = Box::into_raw(Box::new(Document { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_free(doc: *mut Document) {
    if !doc.is_null() {
        drop(Box::from_raw(doc));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_get_version(doc: *const Document, out: *mut u32) -> Status {
    ffi_call(|| {
        *output(out)? = u32::from(document(doc)?.version.version_code());
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_set_version(doc: *mut Document, code: u32) -> Status {
    ffi_call(|| {
        let version = version(code)?;
        document_mut(doc)?.version = version;
        Ok(())
    })
}

#[cfg(test)]
mod tests;
