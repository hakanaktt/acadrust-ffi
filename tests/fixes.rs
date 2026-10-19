use acadrust_ffi::*;
use std::ffi::CStr;
use std::ptr;

fn ok(value: Status) {
    assert_eq!(value, Status::Ok, "{}", unsafe {
        CStr::from_ptr(ar_last_error()).to_string_lossy()
    });
}
unsafe fn child(value: ValueView, index: usize) -> ValueView {
    let mut out = ValueView::default();
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    ok(ar_value_child(value, index, &mut name, &mut out));
    out
}
unsafe fn field(value: ValueView, name: &CStr) -> ValueView {
    let mut out = ValueView::default();
    ok(ar_value_field(value, name.as_ptr(), &mut out));
    out
}
unsafe fn text(value: ValueView) -> String {
    let (mut data, mut len, mut stride) = (ptr::null(), 0, 0);
    ok(ar_value_data(value, &mut data, &mut len, &mut stride));
    String::from_utf8(std::slice::from_raw_parts(data.cast(), len).to_vec()).unwrap()
}

#[test]
fn repeated_root_variant_is_rejected_and_field_side_view_stays_synced() {
    unsafe {
        let mut doc = ptr::null_mut();
        ok(ar_document_new(1032, &mut doc));
        let mut object = 0;
        ok(ar_object_add_default(
            doc,
            c"FIELD".as_ptr(),
            0,
            &mut object,
        ));
        assert_eq!(
            ar_object_select(doc, object, c"Field/Field".as_ptr(), c"Dictionary".as_ptr()),
            Status::InvalidArgument
        );
        let input = NativeWrite {
            kind: AR_VALUE_STRING,
            data: c"updated".as_ptr().cast(),
            len: 7,
        };
        ok(ar_object_write(doc, object, c"code".as_ptr(), &input));
        let mut root = ValueView::default();
        ok(ar_document_view(doc, &mut root));
        let fields = field(root, c"fields");
        let mut count = 0;
        ok(ar_value_len(fields, &mut count));
        assert!(count > 0);
        assert_eq!(text(field(child(fields, 1), c"code")), "updated");
        ar_document_free(doc);
    }
}

#[test]
fn writable_borrow_rejects_preexisting_invalid_entities_before_locking() {
    unsafe {
        let mut doc = ptr::null_mut();
        ok(ar_document_new(1032, &mut doc));
        let mut handle = 0;
        ok(ar_entity_add_default(
            doc,
            c"LWPOLYLINE".as_ptr(),
            &mut handle,
        ));
        ok(ar_entity_resize(doc, handle, c"vertices".as_ptr(), 1));
        let width = -1.0f64;
        let input = NativeWrite {
            kind: AR_VALUE_F64,
            data: (&width as *const f64).cast(),
            len: 1,
        };
        assert_eq!(
            ar_entity_write(doc, handle, c"vertices/0/start_width".as_ptr(), &input),
            Status::InvalidArgument
        );
        let mut span = WritableSpan::default();
        ok(ar_entity_borrow_mut(
            doc,
            handle,
            c"vertices/0/location/x".as_ptr(),
            &mut span,
        ));
        ok(ar_writable_release(&mut span));
        let mut count = 0;
        ok(ar_entity_count(doc, &mut count));
        ar_document_free(doc);
    }
}

#[test]
fn deleting_viewport_cleans_all_layout_membership() {
    unsafe {
        let mut doc = ptr::null_mut();
        ok(ar_document_new(1032, &mut doc));
        let mut layout = 0;
        ok(ar_layout_add(doc, c"FixLayout".as_ptr(), &mut layout));
        let mut viewport = 0;
        ok(ar_entity_add_default(
            doc,
            c"VIEWPORT".as_ptr(),
            &mut viewport,
        ));
        ok(ar_entity_move_to_layout(
            doc,
            viewport,
            c"FixLayout".as_ptr(),
        ));
        ok(ar_entity_remove(doc, viewport));
        let mut value = ValueView::default();
        ok(ar_object_view(doc, layout, &mut value));
        let layout_value = child(value, 0);
        let viewports = field(layout_value, c"viewports");
        let mut count = 0;
        ok(ar_value_len(viewports, &mut count));
        for index in 0..count {
            let mut handle = 0;
            ok(ar_value_read_integer(child(viewports, index), &mut handle));
            assert_ne!(handle as u64, viewport);
        }
        let mut current = 0;
        ok(ar_value_read_integer(
            field(layout_value, c"viewport"),
            &mut current,
        ));
        assert_ne!(current as u64, viewport);
        ar_document_free(doc);
    }
}
