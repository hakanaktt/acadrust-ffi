use acadrust_ffi::*;
use std::ffi::{c_void, CString};
use std::ptr;

struct Drawing(*mut Document);

impl Drawing {
    fn new() -> Self {
        let mut doc = ptr::null_mut();
        unsafe { assert_eq!(ar_document_new(1032, &mut doc), Status::Ok) };
        Self(doc)
    }
}

impl Drop for Drawing {
    fn drop(&mut self) {
        unsafe { ar_document_free(self.0) }
    }
}

unsafe fn field(value: ValueView, name: &str) -> ValueView {
    let name = CString::new(name).unwrap();
    let mut result = ValueView::default();
    assert_eq!(
        ar_value_field(value, name.as_ptr(), &mut result),
        Status::Ok
    );
    result
}

unsafe fn integer(value: ValueView) -> i64 {
    let mut result = 0;
    assert_eq!(ar_value_read_integer(value, &mut result), Status::Ok);
    result
}

unsafe fn string(value: ValueView) -> (*const c_void, String) {
    let mut data = ptr::null();
    let mut len = 0;
    let mut stride = 0;
    assert_eq!(
        ar_value_data(value, &mut data, &mut len, &mut stride),
        Status::Ok
    );
    assert_eq!(stride, 1);
    let result = std::str::from_utf8(std::slice::from_raw_parts(data.cast::<u8>(), len))
        .unwrap()
        .to_string();
    (data, result)
}

#[test]
fn every_symbol_table_has_native_creation_and_borrowed_lookup() {
    let drawing = Drawing::new();
    unsafe {
        for kind in AR_TABLE_LAYER..=AR_TABLE_VX {
            let mut count = 0;
            assert_eq!(ar_table_count(drawing.0, kind, &mut count), Status::Ok);
            let name = CString::new(format!("NativeTable{kind}")).unwrap();
            let mut handle = 0;
            assert_eq!(
                ar_table_add(drawing.0, kind, name.as_ptr(), &mut handle),
                Status::Ok
            );
            assert_ne!(handle, 0);
            let mut by_name = ValueView::default();
            let mut by_index = ValueView::default();
            assert_eq!(
                ar_table_find_view(drawing.0, kind, name.as_ptr(), &mut by_name),
                Status::Ok
            );
            assert_eq!(
                ar_table_entry_view(drawing.0, kind, count, &mut by_index),
                Status::Ok
            );
            assert_eq!(by_name.data, by_index.data);
            assert_eq!(integer(field(by_name, "handle")), handle as i64);
            let (first_pointer, first_name) = string(field(by_name, "name"));
            let (second_pointer, _) = string(field(by_index, "name"));
            assert_eq!(first_pointer, second_pointer);
            assert_eq!(first_name.as_bytes(), name.as_bytes());

            if kind == AR_TABLE_BLOCK_RECORD {
                let begin = integer(field(by_name, "block_entity_handle"));
                let end = integer(field(by_name, "block_end_handle"));
                assert_ne!(begin, 0);
                assert_ne!(end, 0);
                assert_ne!(begin, end);
                assert_ne!(begin, handle as i64);
            }
        }
    }
}

#[test]
fn layout_creation_links_native_object_and_block_record() {
    let drawing = Drawing::new();
    unsafe {
        let name = CString::new("NativeLayout").unwrap();
        let mut handle = 0;
        assert_eq!(
            ar_layout_add(drawing.0, name.as_ptr(), &mut handle),
            Status::Ok
        );
        let mut object = ValueView::default();
        assert_eq!(ar_object_view(drawing.0, handle, &mut object), Status::Ok);
        let mut variant = StringView {
            data: ptr::null(),
            len: 0,
        };
        assert_eq!(ar_value_variant(object, &mut variant), Status::Ok);
        assert_eq!(
            std::slice::from_raw_parts(variant.data.cast::<u8>(), variant.len),
            b"Layout"
        );
        let layout = field(object, "value");
        assert_eq!(string(field(layout, "name")).1, "NativeLayout");
        let block = integer(field(layout, "block_record"));
        let mut count = 0;
        assert_eq!(
            ar_table_count(drawing.0, AR_TABLE_BLOCK_RECORD, &mut count),
            Status::Ok
        );
        let mut found = false;
        for index in 0..count {
            let mut record = ValueView::default();
            assert_eq!(
                ar_table_entry_view(drawing.0, AR_TABLE_BLOCK_RECORD, index, &mut record),
                Status::Ok
            );
            if integer(field(record, "handle")) == block {
                assert_eq!(integer(field(record, "layout")), handle as i64);
                found = true;
            }
        }
        assert!(found);
    }
}

#[test]
fn invalid_table_requests_leave_counts_and_outputs_consistent() {
    let drawing = Drawing::new();
    unsafe {
        let name = CString::new("UniqueName").unwrap();
        let mut handle = 0;
        assert_eq!(
            ar_table_add(drawing.0, AR_TABLE_LAYER, name.as_ptr(), &mut handle),
            Status::Ok
        );
        let mut before = 0;
        assert_eq!(
            ar_table_count(drawing.0, AR_TABLE_LAYER, &mut before),
            Status::Ok
        );
        assert_eq!(
            ar_table_add(drawing.0, AR_TABLE_LAYER, name.as_ptr(), &mut handle),
            Status::InvalidArgument
        );
        assert_eq!(handle, 0);
        assert_eq!(
            ar_table_add(drawing.0, -1, name.as_ptr(), &mut handle),
            Status::InvalidArgument
        );
        let mut after = 0;
        assert_eq!(
            ar_table_count(drawing.0, AR_TABLE_LAYER, &mut after),
            Status::Ok
        );
        assert_eq!(before, after);
        let mut view = ValueView::default();
        assert_eq!(
            ar_table_entry_view(drawing.0, AR_TABLE_LAYER, usize::MAX, &mut view),
            Status::NotFound
        );
        assert!(view.data.is_null());
        assert_eq!(
            ar_object_view(drawing.0, u64::MAX, &mut view),
            Status::NotFound
        );
        assert!(view.data.is_null());
    }
}

#[test]
fn tiled_viewports_with_the_same_name_remain_individually_reachable() {
    let drawing = Drawing::new();
    unsafe {
        let name = CString::new("*Active").unwrap();
        let mut initial_count = 0;
        assert_eq!(
            ar_table_count(drawing.0, AR_TABLE_VPORT, &mut initial_count),
            Status::Ok
        );
        let mut handles = [0; 2];
        for handle in &mut handles {
            assert_eq!(
                ar_table_add(drawing.0, AR_TABLE_VPORT, name.as_ptr(), handle),
                Status::Ok
            );
        }
        assert_ne!(handles[0], handles[1]);
        for (index, handle) in handles.into_iter().enumerate() {
            let mut entry = ValueView::default();
            assert_eq!(
                ar_table_entry_view(drawing.0, AR_TABLE_VPORT, initial_count + index, &mut entry),
                Status::Ok
            );
            assert_eq!(integer(field(entry, "handle")), handle as i64);
            assert_eq!(string(field(entry, "name")).1, "*Active");
        }
    }
}
