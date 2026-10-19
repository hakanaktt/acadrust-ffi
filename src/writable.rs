//! Exclusive writable spans into native document allocations.
use crate::mutations::NativeMut;
use crate::*;
use std::ffi::c_void;
use std::sync::atomic::Ordering;

#[repr(C)]
pub struct WritableSpan {
    pub data: *mut c_void,
    pub len: usize,
    pub stride: usize,
    pub kind: i32,
    pub token: *mut c_void,
}
impl Default for WritableSpan {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
            len: 0,
            stride: 0,
            kind: 0,
            token: ptr::null_mut(),
        }
    }
}

pub(crate) struct RawStorage {
    data: *mut c_void,
    len: usize,
    stride: usize,
    kind: i32,
    string: Option<(*mut String, Option<Vec<u8>>)>,
    record: Option<(
        *mut acadrust::xdata::ExtendedData,
        usize,
        Box<acadrust::xdata::ExtendedDataRecord>,
    )>,
}

pub(crate) fn restore_record(
    owner: &mut acadrust::xdata::ExtendedData,
    index: usize,
    record: acadrust::xdata::ExtendedDataRecord,
) {
    let mut tail = Vec::new();
    while let Some(record) = owner.records().get(index) {
        let name = record.application_name.clone();
        tail.push(owner.remove_record(&name).expect("existing XDATA record"));
    }
    owner.add_record(record);
    for record in tail {
        owner.add_record(record);
    }
}
impl RawStorage {
    pub(crate) fn primitive<T>(data: *mut T, len: usize, kind: i32) -> Self {
        Self {
            data: data.cast(),
            len,
            stride: std::mem::size_of::<T>(),
            kind,
            string: None,
            record: None,
        }
    }
    pub(crate) fn string(value: &mut String) -> Self {
        let target = std::ptr::from_mut(value);
        let mut bytes = std::mem::take(value).into_bytes();
        Self {
            data: bytes.as_mut_ptr().cast(),
            len: bytes.len(),
            stride: 1,
            kind: AR_VALUE_STRING,
            string: Some((target, Some(bytes))),
            record: None,
        }
    }
    pub(crate) fn with_record(
        mut self,
        owner: &mut acadrust::xdata::ExtendedData,
        index: usize,
        record: Box<acadrust::xdata::ExtendedDataRecord>,
    ) -> Self {
        self.record = Some((std::ptr::from_mut(owner), index, record));
        self
    }
    unsafe fn attach_record(&mut self) {
        if let Some((owner, index, record)) = self.record.take() {
            let owner = &mut *owner;
            restore_record(owner, index, *record);
        }
    }
    unsafe fn validate(&self) -> FfiResult<()> {
        let valid = match self.kind {
            AR_VALUE_F32 => std::slice::from_raw_parts(self.data.cast::<f32>(), self.len)
                .iter()
                .all(|v| v.is_finite()),
            AR_VALUE_F64 => std::slice::from_raw_parts(self.data.cast::<f64>(), self.len)
                .iter()
                .all(|v| v.is_finite()),
            AR_VALUE_STRING => {
                std::str::from_utf8(std::slice::from_raw_parts(self.data.cast::<u8>(), self.len))
                    .is_ok()
            }
            _ => true,
        };
        if valid {
            Ok(())
        } else {
            Err(invalid("writable data must contain finite numbers or valid UTF-8; fix the span and release it again"))
        }
    }
    unsafe fn attach_string(&mut self) {
        if let Some((target, bytes)) = &mut self.string {
            if let Some(bytes) = bytes.take() {
                **target = String::from_utf8_unchecked(bytes);
            }
        }
    }
    unsafe fn detach_string(&mut self) {
        if let Some((target, bytes)) = &mut self.string {
            if bytes.is_none() {
                *bytes = Some(std::mem::take(&mut **target).into_bytes());
            }
        }
    }
}
impl Drop for RawStorage {
    fn drop(&mut self) {
        if let Some((target, bytes)) = &mut self.string {
            if let Some(bytes) = bytes.take() {
                // Also restores storage if projection fails before a span is published.
                unsafe {
                    **target = String::from_utf8(bytes).unwrap_or_default();
                }
            }
        }
        unsafe {
            self.attach_record();
        }
    }
}

pub(crate) enum WriteTarget {
    Document,
    Entity(u64),
    Object(u64),
    Table(i32, String),
    TableIndex(i32, usize),
}
struct WriteLease {
    doc: *mut Document,
    target: WriteTarget,
    path: String,
    storage: RawStorage,
}

pub(crate) fn storage_at(value: &mut dyn NativeMut, parts: &[&str]) -> FfiResult<RawStorage> {
    if let Some(result) = value.writable_nested(parts) {
        return result;
    }
    if let Some((segment, rest)) = parts.split_first() {
        if *segment == "name" || *segment == "application_name" {
            return Err(invalid(
                "indexed names require the native setter or rename API",
            ));
        }
        if value.variant_name() == Some(*segment) {
            return storage_at(value, rest);
        }
        return storage_at(value.child_mut(segment)?, rest);
    }
    value.writable_storage()
}

unsafe fn acquire(
    doc: *mut Document,
    target: WriteTarget,
    path: *const c_char,
    out: *mut WritableSpan,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = WritableSpan::default();
        let path = input_str(path)?.to_owned();
        if let WriteTarget::Entity(handle) = target {
            let entity = document(doc)?
                .get_entity(acadrust::Handle::new(handle))
                .ok_or_else(|| not_found("entity was not found"))?;
            crate::entities::validate_entity(document(doc)?, entity)?;
        }
        let storage = crate::mutations::borrow_target(document_mut(doc)?, &target, &path)?;
        let lease = Box::new(WriteLease {
            doc,
            target,
            path,
            storage,
        });
        (*doc).writable.store(true, Ordering::Release);
        *out = WritableSpan {
            data: lease.storage.data,
            len: lease.storage.len,
            stride: lease.storage.stride,
            kind: lease.storage.kind,
            token: Box::into_raw(lease).cast(),
        };
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_borrow_mut(
    doc: *mut Document,
    handle: u64,
    path: *const c_char,
    out: *mut WritableSpan,
) -> Status {
    acquire(doc, WriteTarget::Entity(handle), path, out)
}
#[no_mangle]
pub unsafe extern "C" fn ar_object_borrow_mut(
    doc: *mut Document,
    handle: u64,
    path: *const c_char,
    out: *mut WritableSpan,
) -> Status {
    acquire(doc, WriteTarget::Object(handle), path, out)
}
#[no_mangle]
pub unsafe extern "C" fn ar_document_borrow_mut(
    doc: *mut Document,
    path: *const c_char,
    out: *mut WritableSpan,
) -> Status {
    acquire(doc, WriteTarget::Document, path, out)
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_entry_borrow_mut(
    doc: *mut Document,
    kind: i32,
    index: usize,
    path: *const c_char,
    out: *mut WritableSpan,
) -> Status {
    acquire(doc, WriteTarget::TableIndex(kind, index), path, out)
}
#[no_mangle]
pub unsafe extern "C" fn ar_table_borrow_mut(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    path: *const c_char,
    out: *mut WritableSpan,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = WritableSpan::default();
        let name = input_str(name)?.to_owned();
        let path = input_str(path)?.to_owned();
        let target = WriteTarget::Table(kind, name);
        let storage = crate::mutations::borrow_target(document_mut(doc)?, &target, &path)?;
        let lease = Box::new(WriteLease {
            doc,
            target,
            path,
            storage,
        });
        (*doc).writable.store(true, Ordering::Release);
        *out = WritableSpan {
            data: lease.storage.data,
            len: lease.storage.len,
            stride: lease.storage.stride,
            kind: lease.storage.kind,
            token: Box::into_raw(lease).cast(),
        };
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_writable_release(span: *mut WritableSpan) -> Status {
    ffi_call(|| {
        let span = output(span)?;
        let lease = span
            .token
            .cast::<WriteLease>()
            .as_mut()
            .ok_or_else(|| invalid("writable span is empty or already released"))?;
        lease.storage.validate()?;
        lease.storage.attach_string();
        let doc = &mut *lease.doc;
        let valid = if let WriteTarget::Entity(handle) = lease.target {
            let entity = doc
                .inner
                .get_entity(acadrust::Handle::new(handle))
                .ok_or_else(|| not_found("writable entity was not found"))?;
            crate::entities::validate_entity(&doc.inner, entity)
        } else {
            Ok(())
        };
        if let Err(error) = valid {
            lease.storage.detach_string();
            return Err(error);
        }
        lease.storage.attach_record();
        if let Err(error) =
            crate::mutations::finish_target(&mut doc.inner, &lease.target, &lease.path)
        {
            lease.storage.detach_string();
            return Err(error);
        }
        doc.writable.store(false, Ordering::Release);
        let token = span.token;
        *span = WritableSpan::default();
        drop(Box::from_raw(token.cast::<WriteLease>()));
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xdata_writes_preserve_native_payload_allocation_and_record_order() {
        use acadrust::xdata::{ExtendedDataRecord, XDataValue};
        let mut doc = Document::new(CadDocument::new());
        let mut line = acadrust::Line::default();
        for app in ["FIRST", "SECOND", "THIRD"] {
            let mut record = ExtendedDataRecord::new(app);
            record.values.push(XDataValue::String("hello".into()));
            line.common.extended_data.add_record(record);
        }
        let handle = doc
            .inner
            .add_entity(acadrust::EntityType::Line(line))
            .unwrap();
        let record = doc
            .inner
            .get_entity(handle)
            .unwrap()
            .common()
            .extended_data
            .get_record("SECOND")
            .unwrap();
        let XDataValue::String(text) = &record.values[0] else {
            panic!()
        };
        let address = text.as_ptr();
        let mut span = WritableSpan::default();
        unsafe {
            assert_eq!(
                ar_entity_borrow_mut(
                    &mut doc,
                    handle.value(),
                    c"common/extended_data/SECOND/values/0/value".as_ptr(),
                    &mut span
                ),
                Status::Ok
            );
            assert_eq!(span.data.cast::<u8>(), address.cast_mut());
            std::slice::from_raw_parts_mut(span.data.cast::<u8>(), span.len)
                .copy_from_slice(b"world");
            assert_eq!(ar_writable_release(&mut span), Status::Ok);
        }
        let data = &doc.inner.get_entity(handle).unwrap().common().extended_data;
        assert_eq!(
            data.records()
                .iter()
                .map(|r| r.application_name.as_str())
                .collect::<Vec<_>>(),
            ["FIRST", "SECOND", "THIRD"]
        );
        let XDataValue::String(text) = &data.get_record("SECOND").unwrap().values[0] else {
            panic!()
        };
        assert_eq!(text, "world");
        assert_eq!(text.as_ptr(), address);
    }
}
