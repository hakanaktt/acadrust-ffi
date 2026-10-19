use acadrust_ffi::*;

#[test]
fn class_metadata_writes_rebuild_native_lookup_without_changing_identity() {
    unsafe {
        let mut doc = std::ptr::null_mut();
        assert_eq!(ar_document_new(1032, &mut doc), Status::Ok);
        let mut index = 0;
        assert_eq!(
            ar_class_add(
                doc,
                c"CUSTOM_CLASS".as_ptr(),
                c"AcDbCustom".as_ptr(),
                c"Host".as_ptr(),
                0,
                &mut index
            ),
            Status::Ok
        );
        let bits = 31u16;
        let input = NativeWrite {
            kind: AR_VALUE_U16,
            data: std::ptr::from_ref(&bits).cast(),
            len: 1,
        };
        assert_eq!(
            ar_class_write(doc, index, c"proxy_flags/0".as_ptr(), &input),
            Status::Ok
        );
        let mut value = ValueView::default();
        assert_eq!(ar_class_view(doc, index, &mut value), Status::Ok);
        let mut flags = ValueView::default();
        assert_eq!(
            ar_value_field(value, c"proxy_flags".as_ptr(), &mut flags),
            Status::Ok
        );
        let mut field = ValueView::default();
        assert_eq!(ar_value_field(flags, c"0".as_ptr(), &mut field), Status::Ok);
        let mut data = std::ptr::null();
        let mut len = 0;
        let mut stride = 0;
        assert_eq!(
            ar_value_data(field, &mut data, &mut len, &mut stride),
            Status::Ok
        );
        assert_eq!(*data.cast::<u16>(), bits);
        assert_eq!(
            ar_class_write(doc, index, c"class_number".as_ptr(), &input),
            Status::InvalidArgument
        );
        ar_document_free(doc);
    }
}
use std::ffi::{c_void, CStr, CString};
use std::ptr;

struct Drawing(*mut Document);
impl Drawing {
    fn new() -> Self {
        unsafe {
            let mut doc = ptr::null_mut();
            ok(ar_document_new(1032, &mut doc));
            Self(doc)
        }
    }
    fn add(&self, kind: &str) -> u64 {
        unsafe {
            let mut handle = 0;
            ok(ar_entity_add_default(
                self.0,
                CString::new(kind).unwrap().as_ptr(),
                &mut handle,
            ));
            handle
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
fn write<T>(kind: i32, values: &[T]) -> NativeWrite {
    NativeWrite {
        kind,
        data: values.as_ptr().cast(),
        len: values.len(),
    }
}
fn text(value: &str) -> NativeWrite {
    NativeWrite {
        kind: AR_VALUE_STRING,
        data: value.as_ptr().cast(),
        len: value.len(),
    }
}

unsafe fn field(value: ValueView, name: &str) -> ValueView {
    let mut result = ValueView::default();
    ok(ar_value_field(
        value,
        CString::new(name).unwrap().as_ptr(),
        &mut result,
    ));
    result
}
unsafe fn child(value: ValueView, index: usize) -> ValueView {
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    let mut result = ValueView::default();
    ok(ar_value_child(value, index, &mut name, &mut result));
    result
}
unsafe fn payload(doc: *const Document, handle: u64) -> ValueView {
    let mut root = ValueView::default();
    ok(ar_entity_view(doc, handle, &mut root));
    child(root, 0)
}
unsafe fn data<T: Copy>(value: ValueView) -> Vec<T> {
    let mut data = ptr::null();
    let mut len = 0;
    let mut stride = 0;
    ok(ar_value_data(value, &mut data, &mut len, &mut stride));
    assert_eq!(stride, std::mem::size_of::<T>());
    if len == 0 {
        vec![]
    } else {
        std::slice::from_raw_parts(data.cast::<T>(), len).to_vec()
    }
}
unsafe fn len(value: ValueView) -> usize {
    let mut len = 0;
    ok(ar_value_len(value, &mut len));
    len
}

#[test]
fn scalar_and_color_changes_use_exclusive_native_projections() {
    let drawing = Drawing::new();
    let handle = drawing.add("LINE");
    unsafe {
        let before = payload(drawing.0, handle);
        ok(ar_entity_write(
            drawing.0,
            handle,
            c"start/x".as_ptr(),
            &write(AR_VALUE_F64, &[12.5f64]),
        ));
        let after = payload(drawing.0, handle);
        assert_eq!(before.data, after.data);
        let mut line = Line::default();
        ok(ar_line_get(drawing.0, handle, &mut line));
        assert_eq!(line.start.x, 12.5);
        ok(ar_entity_select(
            drawing.0,
            handle,
            c"common/color".as_ptr(),
            c"Rgb".as_ptr(),
        ));
        ok(ar_entity_write(
            drawing.0,
            handle,
            c"Line/common/color/r".as_ptr(),
            &write(AR_VALUE_U8, &[193u8]),
        ));
        assert_eq!(
            data::<u8>(field(
                field(field(payload(drawing.0, handle), "common"), "color"),
                "r"
            )),
            [193]
        );
        let common = field(payload(drawing.0, handle), "common");
        let mut current_handle = 0;
        ok(ar_value_read_integer(
            field(common, "handle"),
            &mut current_handle,
        ));
        assert_eq!(current_handle, handle as i64);
    }
}

#[test]
fn vectors_options_and_enum_payloads_can_be_built_and_edited() {
    let drawing = Drawing::new();
    let handle = drawing.add("SPLINE");
    unsafe {
        ok(ar_entity_write(
            drawing.0,
            handle,
            c"weights".as_ptr(),
            &write(AR_VALUE_F64, &[1.0f64, 2.0, 3.0]),
        ));
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"weights".as_ptr(),
                &write(AR_VALUE_F64, &[4.0f64, f64::NAN])
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"weights".as_ptr(),
                &write::<u8>(AR_VALUE_U8, &[])
            ),
            Status::TypeMismatch
        );
        ok(ar_entity_resize(
            drawing.0,
            handle,
            c"control_points".as_ptr(),
            2,
        ));
        ok(ar_entity_write(
            drawing.0,
            handle,
            c"control_points/1/z".as_ptr(),
            &write(AR_VALUE_F64, &[8.0f64]),
        ));
        ok(ar_entity_option(
            drawing.0,
            handle,
            c"dwg_scenario".as_ptr(),
            1,
        ));
        ok(ar_entity_write(
            drawing.0,
            handle,
            c"dwg_scenario/value".as_ptr(),
            &write(AR_VALUE_I32, &[2i32]),
        ));
        let spline = payload(drawing.0, handle);
        assert_eq!(data::<f64>(field(spline, "weights")), [1.0, 2.0, 3.0]);
        assert_eq!(
            data::<f64>(field(child(field(spline, "control_points"), 1), "z")),
            [8.0]
        );
        assert_eq!(data::<i32>(child(field(spline, "dwg_scenario"), 0)), [2]);
        ok(ar_entity_option(
            drawing.0,
            handle,
            c"dwg_scenario".as_ptr(),
            0,
        ));
        assert_eq!(len(field(payload(drawing.0, handle), "dwg_scenario")), 0);
    }
}

#[test]
fn table_records_without_default_can_be_constructed_recursively() {
    let drawing = Drawing::new();
    unsafe {
        let mut handle = 0;
        ok(ar_table_add(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            &mut handle,
        ));
        ok(ar_table_resize(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            c"elements".as_ptr(),
            1,
        ));
        ok(ar_table_write(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            c"elements/0/length".as_ptr(),
            &write(AR_VALUE_F64, &[2.5f64]),
        ));
        ok(ar_table_option(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            c"elements/0/complex".as_ptr(),
            1,
        ));
        ok(ar_table_select(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            c"elements/0/complex/value/content".as_ptr(),
            c"Text".as_ptr(),
        ));
        ok(ar_table_write(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            c"elements/0/complex/value/content/text".as_ptr(),
            &text("CAD"),
        ));
        let mut record = ValueView::default();
        ok(ar_table_find_view(
            drawing.0,
            AR_TABLE_LINETYPE,
            c"NativeDash".as_ptr(),
            &mut record,
        ));
        let element = child(field(record, "elements"), 0);
        assert_eq!(data::<f64>(field(element, "length")), [2.5]);
        assert_eq!(
            data::<u8>(field(
                field(child(field(element, "complex"), 0), "content"),
                "text"
            )),
            b"CAD"
        );
    }
}

#[test]
fn invalid_input_and_protected_fields_are_atomic() {
    let drawing = Drawing::new();
    let handle = drawing.add("CIRCLE");
    unsafe {
        let mut original = Circle::default();
        ok(ar_circle_get(drawing.0, handle, &mut original));
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"radius".as_ptr(),
                &write(AR_VALUE_F64, &[-2.0f64])
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"radius".as_ptr(),
                &write(AR_VALUE_F64, &[f64::NAN])
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"radius".as_ptr(),
                &write(AR_VALUE_I32, &[3i32])
            ),
            Status::TypeMismatch
        );
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"common/handle".as_ptr(),
                &write(AR_VALUE_U64, &[999u64])
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"common/raw_record".as_ptr(),
                &write(AR_VALUE_U8, &[0u8])
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_write(
                drawing.0,
                handle,
                c"common/invisible".as_ptr(),
                &write(AR_VALUE_BOOL, &[2u8])
            ),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_select(drawing.0, handle, c"Circle".as_ptr(), c"Line".as_ptr()),
            Status::InvalidArgument
        );
        let mut after = Circle::default();
        ok(ar_circle_get(drawing.0, handle, &mut after));
        assert_eq!(after, original);
        assert_eq!(
            ar_table_write(
                drawing.0,
                AR_TABLE_LAYER,
                c"0".as_ptr(),
                c"name".as_ptr(),
                &text("Renamed")
            ),
            Status::InvalidArgument
        );
    }
}

#[test]
fn document_strings_and_owned_arrays_accept_staged_borrowed_input() {
    let drawing = Drawing::new();
    unsafe {
        ok(ar_document_write(
            drawing.0,
            c"summary_info/title".as_ptr(),
            &text("Native title"),
        ));
        let mut doc = ValueView::default();
        ok(ar_document_view(drawing.0, &mut doc));
        let title = field(field(doc, "summary_info"), "title");
        let mut pointer = ptr::null::<c_void>();
        let mut count = 0;
        let mut stride = 0;
        ok(ar_value_data(title, &mut pointer, &mut count, &mut stride));
        let input = NativeWrite {
            kind: AR_VALUE_STRING,
            data: pointer,
            len: count,
        };
        ok(ar_document_write(
            drawing.0,
            c"summary_info/subject".as_ptr(),
            &input,
        ));
        ok(ar_document_view(drawing.0, &mut doc));
        assert_eq!(
            data::<u8>(field(field(doc, "summary_info"), "subject")),
            b"Native title"
        );
    }
}

#[test]
fn layout_name_changes_keep_dictionary_registration_current() {
    let drawing = Drawing::new();
    unsafe {
        let mut handle = 0;
        ok(ar_layout_add(
            drawing.0,
            c"OriginalLayout".as_ptr(),
            &mut handle,
        ));
        ok(ar_object_write(
            drawing.0,
            handle,
            c"name".as_ptr(),
            &text("EditedLayout"),
        ));
        let mut object = ValueView::default();
        ok(ar_object_view(drawing.0, handle, &mut object));
        let layout = child(object, 0);
        assert_eq!(data::<u8>(field(layout, "name")), b"EditedLayout");
        let mut owner = 0;
        ok(ar_value_read_integer(field(layout, "owner"), &mut owner));
        if owner != 0 {
            ok(ar_object_view(drawing.0, owner as u64, &mut object));
            let entries = field(child(object, 0), "entries");
            let mut found = false;
            for i in 0..len(entries) {
                let pair = child(entries, i);
                let mut target = 0;
                ok(ar_value_read_integer(field(pair, "1"), &mut target));
                if target == handle as i64 {
                    assert_eq!(data::<u8>(field(pair, "0")), b"EditedLayout");
                    found = true;
                }
            }
            assert!(found);
        }
    }
}

#[test]
fn indexed_mutations_reach_duplicate_viewport_names() {
    let drawing = Drawing::new();
    unsafe {
        let mut initial = 0;
        ok(ar_table_count(drawing.0, AR_TABLE_VPORT, &mut initial));
        let mut handle = 0;
        for _ in 0..2 {
            ok(ar_table_add(
                drawing.0,
                AR_TABLE_VPORT,
                c"*Duplicate".as_ptr(),
                &mut handle,
            ));
        }
        ok(ar_table_entry_write(
            drawing.0,
            AR_TABLE_VPORT,
            initial + 1,
            c"view_height".as_ptr(),
            &write(AR_VALUE_F64, &[120.0f64]),
        ));
        let mut entry = ValueView::default();
        ok(ar_table_entry_view(
            drawing.0,
            AR_TABLE_VPORT,
            initial + 1,
            &mut entry,
        ));
        assert_eq!(data::<f64>(field(entry, "view_height")), [120.0]);
        ok(ar_table_entry_view(
            drawing.0,
            AR_TABLE_VPORT,
            initial,
            &mut entry,
        ));
        assert_ne!(data::<f64>(field(entry, "view_height")), [120.0]);
    }
}
