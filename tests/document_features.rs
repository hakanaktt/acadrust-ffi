use acadrust_ffi::*;
use std::ffi::{c_char, CStr, CString};
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
unsafe fn number(value: ValueView) -> u64 {
    let mut result = 0;
    ok(ar_value_read_integer(value, &mut result));
    result as u64
}
unsafe fn payload(value: ValueView) -> ValueView {
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    let mut result = ValueView::default();
    ok(ar_value_child(value, 0, &mut name, &mut result));
    result
}

#[test]
fn moving_entities_updates_native_block_membership_and_roundtrips() {
    unsafe {
        let doc = Drawing::new();
        let mut block = 0;
        let mut handle = 0;
        ok(ar_table_add(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"Parts".as_ptr(),
            &mut block,
        ));
        ok(ar_line_add(
            doc.0,
            &Line {
                end: Vector3 {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..Line::default()
            },
            &mut handle,
        ));
        ok(ar_entity_move_to_block_handle(doc.0, handle, block));
        let mut entity = ValueView::default();
        ok(ar_entity_view(doc.0, handle, &mut entity));
        assert_eq!(
            number(field(field(payload(entity), c"common"), c"owner_handle")),
            block
        );
        let mut table = ValueView::default();
        ok(ar_table_find_view(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"Parts".as_ptr(),
            &mut table,
        ));
        let mut len = 0;
        ok(ar_value_len(field(table, c"entity_handles"), &mut len));
        assert_eq!(len, 1);
        let mut buffer = Buffer::default();
        ok(ar_document_write_buffer(doc.0, AR_FORMAT_DWG, &mut buffer));
        let mut restored = ptr::null_mut();
        ok(ar_document_read_buffer(
            buffer.data,
            buffer.len,
            AR_FORMAT_DWG,
            &mut restored,
        ));
        ar_buffer_free(&mut buffer);
        let restored = Drawing(restored);
        ok(ar_entity_view(restored.0, handle, &mut entity));
        assert_eq!(
            number(field(field(payload(entity), c"common"), c"owner_handle")),
            block
        );
        assert_eq!(
            ar_entity_move_to_block_handle(doc.0, handle, u64::MAX),
            Status::NotFound
        );
    }
}

#[test]
fn layer_states_variables_and_xrecords_use_native_results_and_borrowed_strings() {
    unsafe {
        let doc = Drawing::new();
        ok(ar_layer_add(doc.0, c"Fixtures".as_ptr(), 2));
        let mut handle = 0;
        ok(ar_layer_state_capture(
            doc.0,
            c"Working".as_ptr(),
            c"".as_ptr(),
            &mut handle,
        ));
        ok(ar_layer_set_flags(
            doc.0,
            c"Fixtures".as_ptr(),
            AR_LAYER_FROZEN,
        ));
        let mut count = 0;
        ok(ar_layer_state_restore(
            doc.0,
            c"Working".as_ptr(),
            &mut count,
        ));
        assert!(count > 0);
        let mut flags = 0;
        ok(ar_layer_get_flags(doc.0, c"Fixtures".as_ptr(), &mut flags));
        assert_eq!(flags, 0);
        let mut states = ptr::null_mut();
        ok(ar_layer_states(doc.0, &mut states));
        let mut value = ValueView::default();
        ok(ar_result_view(states, &mut value));
        ok(ar_value_len(value, &mut count));
        assert_eq!(count, 1);
        ar_result_free(states);
        ok(ar_layer_state_rename(
            doc.0,
            c"Working".as_ptr(),
            c"Saved".as_ptr(),
        ));
        ok(ar_layer_state_remove(doc.0, c"Saved".as_ptr()));

        ok(ar_document_set_drawing_variable(
            doc.0,
            c"HPORIGIN".as_ptr(),
            c"1.5;2.5".as_ptr(),
        ));
        let mut text = StringView {
            data: ptr::null(),
            len: 0,
        };
        ok(ar_document_drawing_variable(
            doc.0,
            c"HPORIGIN".as_ptr(),
            &mut text,
        ));
        assert_eq!(
            std::slice::from_raw_parts(text.data.cast::<u8>(), text.len),
            b"1.5;2.5"
        );

        ok(ar_line_add(doc.0, &Line::default(), &mut handle));
        let mut dictionary = 0;
        ok(ar_document_extension_dictionary(
            doc.0,
            handle,
            1,
            &mut dictionary,
        ));
        assert_ne!(dictionary, 0);
        let mut record = 0;
        ok(ar_document_ensure_xrecord(
            doc.0,
            handle,
            c"Custom".as_ptr(),
            &mut record,
        ));
        ok(ar_document_xrecord_view(
            doc.0,
            handle,
            c"Custom".as_ptr(),
            &mut value,
        ));
        assert_eq!(number(field(value, c"handle")), record);
        let mut again = 0;
        ok(ar_document_ensure_xrecord(
            doc.0,
            handle,
            c"Custom".as_ptr(),
            &mut again,
        ));
        assert_eq!(again, record);
    }
}

#[test]
fn native_audit_results_outlive_documents_and_layer_rename_updates_references() {
    unsafe {
        let doc = Drawing::new();
        ok(ar_layer_add(doc.0, c"Old".as_ptr(), 3));
        let mut handle = 0;
        ok(ar_point_add(doc.0, &Point::default(), &mut handle));
        ok(ar_entity_set_layer(doc.0, handle, c"Old".as_ptr()));
        ok(ar_layer_rename(doc.0, c"Old".as_ptr(), c"New".as_ptr()));
        let mut entity = ValueView::default();
        ok(ar_entity_view(doc.0, handle, &mut entity));
        let name = field(field(payload(entity), c"common"), c"layer");
        let mut data = ptr::null();
        let mut len = 0;
        let mut stride = 0;
        ok(ar_value_data(name, &mut data, &mut len, &mut stride));
        assert_eq!(std::slice::from_raw_parts(data.cast::<u8>(), len), b"New");
        let mut report = ptr::null_mut();
        ok(ar_document_audit(doc.0, &mut report));
        drop(doc);
        let mut value = ValueView::default();
        ok(ar_result_view(report, &mut value));
        ok(ar_value_len(field(value, c"issues"), &mut len));
        ar_result_free(report);
        assert_eq!(
            ar_document_audit(ptr::null(), ptr::null_mut()),
            Status::InvalidArgument
        );
    }
}

#[test]
fn document_resolve_rejects_null_and_owned_outputs_are_cleared_on_failure() {
    unsafe {
        assert_eq!(
            ar_document_resolve_references(ptr::null_mut()),
            Status::InvalidArgument
        );
        let doc = Drawing::new();
        let mut out = ptr::dangling_mut::<NativeResult>();
        assert_eq!(
            ar_document_export_block(doc.0, u64::MAX, &mut out),
            Status::NotFound
        );
        assert!(out.is_null());
        let name = CString::new("missing").unwrap();
        let mut text = StringView {
            data: ptr::null::<c_char>(),
            len: 0,
        };
        assert_eq!(
            ar_document_drawing_variable(doc.0, name.as_ptr(), &mut text),
            Status::NotFound
        );
    }
}
