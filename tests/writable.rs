use acadrust_ffi::*;
use std::ffi::CStr;
use std::ptr;

struct Drawing(*mut Document);
impl Drawing {
    fn new() -> Self {
        let mut doc = ptr::null_mut();
        unsafe {
            ok(ar_document_new(1032, &mut doc));
        }
        Self(doc)
    }
}
impl Drop for Drawing {
    fn drop(&mut self) {
        unsafe {
            ar_document_free(self.0);
        }
    }
}
fn ok(status: Status) {
    assert_eq!(status, Status::Ok, "{}", unsafe {
        CStr::from_ptr(ar_last_error()).to_string_lossy()
    });
}
unsafe fn field(value: ValueView, name: &CStr) -> ValueView {
    let mut result = ValueView::default();
    ok(ar_value_field(value, name.as_ptr(), &mut result));
    result
}
unsafe fn payload(doc: &Drawing, handle: u64) -> ValueView {
    let mut value = ValueView::default();
    ok(ar_entity_view(doc.0, handle, &mut value));
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    let mut result = ValueView::default();
    ok(ar_value_child(value, 0, &mut name, &mut result));
    result
}
unsafe fn data(value: ValueView) -> *const std::ffi::c_void {
    let mut data = ptr::null();
    let mut len = 0;
    let mut stride = 0;
    ok(ar_value_data(value, &mut data, &mut len, &mut stride));
    data
}

#[test]
fn numeric_pointer_is_native_and_excludes_all_other_document_access() {
    unsafe {
        let doc = Drawing::new();
        let mut handle = 0;
        ok(ar_line_add(doc.0, &Line::default(), &mut handle));
        let address = data(field(field(payload(&doc, handle), c"start"), c"x"));
        let mut span = WritableSpan::default();
        ok(ar_entity_borrow_mut(
            doc.0,
            handle,
            c"Line/start/x".as_ptr(),
            &mut span,
        ));
        assert_eq!(span.data.cast_const(), address);
        assert_eq!((span.kind, span.len, span.stride), (AR_VALUE_F64, 1, 8));
        *span.data.cast::<f64>() = 42.0;
        let mut count = 0;
        assert_eq!(ar_entity_count(doc.0, &mut count), Status::InvalidArgument);
        let mut clone = ptr::null_mut();
        assert_eq!(
            ar_document_clone(doc.0, &mut clone),
            Status::InvalidArgument
        );
        assert!(clone.is_null());
        ar_document_free(doc.0);
        ok(ar_writable_release(&mut span));
        assert!(span.token.is_null());
        assert!(span.data.is_null());
        let mut line = Line::default();
        ok(ar_line_get(doc.0, handle, &mut line));
        assert_eq!(line.start.x, 42.0);
        assert_eq!(ar_writable_release(&mut span), Status::InvalidArgument);
        assert_eq!(
            ar_entity_borrow_mut(doc.0, handle, c"common/handle".as_ptr(), &mut span),
            Status::InvalidArgument
        );
        ok(ar_entity_count(doc.0, &mut count));
    }
}

#[test]
fn invalid_utf8_keeps_the_same_writable_allocation_until_corrected() {
    unsafe {
        let doc = Drawing::new();
        let mut handle = 0;
        ok(ar_entity_add_default(doc.0, c"TEXT".as_ptr(), &mut handle));
        let input = NativeWrite {
            kind: AR_VALUE_STRING,
            data: b"hello".as_ptr().cast(),
            len: 5,
        };
        ok(ar_entity_write(
            doc.0,
            handle,
            c"value/value".as_ptr(),
            &input,
        ));
        let original = data(field(payload(&doc, handle), c"value"));
        let mut span = WritableSpan::default();
        ok(ar_entity_borrow_mut(
            doc.0,
            handle,
            c"Text/value/value".as_ptr(),
            &mut span,
        ));
        assert_eq!(span.data.cast_const(), original);
        let bytes = std::slice::from_raw_parts_mut(span.data.cast::<u8>(), span.len);
        bytes.copy_from_slice(b"world");
        bytes[0] = 255;
        assert_eq!(ar_writable_release(&mut span), Status::InvalidArgument);
        assert!(!span.token.is_null());
        let mut value = ValueView::default();
        assert_eq!(
            ar_entity_view(doc.0, handle, &mut value),
            Status::InvalidArgument
        );
        *span.data.cast::<u8>() = b'w';
        ok(ar_writable_release(&mut span));
        let current = data(field(payload(&doc, handle), c"value"));
        assert_eq!(current, original);
        assert_eq!(
            std::slice::from_raw_parts(current.cast::<u8>(), 5),
            b"world"
        );
    }
}

#[test]
fn numeric_array_edits_are_in_place_and_invalid_numbers_are_retryable() {
    unsafe {
        let doc = Drawing::new();
        let mut handle = 0;
        ok(ar_entity_add_default(
            doc.0,
            c"SPLINE".as_ptr(),
            &mut handle,
        ));
        let values = [0.0, 0.0, 1.0, 1.0];
        let input = NativeWrite {
            kind: AR_VALUE_F64,
            data: values.as_ptr().cast(),
            len: values.len(),
        };
        ok(ar_entity_write(doc.0, handle, c"knots".as_ptr(), &input));
        let address = data(field(payload(&doc, handle), c"knots"));
        let mut span = WritableSpan::default();
        ok(ar_entity_borrow_mut(
            doc.0,
            handle,
            c"knots".as_ptr(),
            &mut span,
        ));
        assert_eq!(span.data.cast_const(), address);
        assert_eq!(span.len, values.len());
        *span.data.cast::<f64>().add(2) = f64::NAN;
        assert_eq!(ar_writable_release(&mut span), Status::InvalidArgument);
        *span.data.cast::<f64>().add(2) = 0.5;
        ok(ar_writable_release(&mut span));
        assert_eq!(
            *data(field(payload(&doc, handle), c"knots"))
                .cast::<f64>()
                .add(2),
            0.5
        );
    }
}

#[test]
fn shared_clones_keep_upstream_copy_on_write_isolation() {
    unsafe {
        let doc = Drawing::new();
        let mut handle = 0;
        ok(ar_line_add(doc.0, &Line::default(), &mut handle));
        let mut cloned = ptr::null_mut();
        ok(ar_document_clone(doc.0, &mut cloned));
        let cloned = Drawing(cloned);
        let mut span = WritableSpan::default();
        ok(ar_entity_borrow_mut(
            cloned.0,
            handle,
            c"start/x".as_ptr(),
            &mut span,
        ));
        *span.data.cast::<f64>() = 99.0;
        ok(ar_writable_release(&mut span));
        let mut original = Line::default();
        ok(ar_line_get(doc.0, handle, &mut original));
        assert_eq!(original.start.x, 0.0);
    }
}

#[test]
fn indexed_writable_spans_reach_duplicate_viewports_individually() {
    unsafe {
        let doc = Drawing::new();
        let mut count = 0;
        ok(ar_table_count(doc.0, AR_TABLE_VPORT, &mut count));
        let mut handle = 0;
        for _ in 0..2 {
            ok(ar_table_add(
                doc.0,
                AR_TABLE_VPORT,
                c"Shared".as_ptr(),
                &mut handle,
            ));
        }
        let mut first = ValueView::default();
        ok(ar_table_entry_view(
            doc.0,
            AR_TABLE_VPORT,
            count,
            &mut first,
        ));
        let original = *data(field(first, c"view_height")).cast::<f64>();
        let mut span = WritableSpan::default();
        ok(ar_table_entry_borrow_mut(
            doc.0,
            AR_TABLE_VPORT,
            count + 1,
            c"view_height".as_ptr(),
            &mut span,
        ));
        *span.data.cast::<f64>() = original + 10.0;
        ok(ar_writable_release(&mut span));
        ok(ar_table_entry_view(
            doc.0,
            AR_TABLE_VPORT,
            count,
            &mut first,
        ));
        assert_eq!(*data(field(first, c"view_height")).cast::<f64>(), original);
        ok(ar_table_entry_view(
            doc.0,
            AR_TABLE_VPORT,
            count + 1,
            &mut first,
        ));
        assert_eq!(
            *data(field(first, c"view_height")).cast::<f64>(),
            original + 10.0
        );
    }
}
