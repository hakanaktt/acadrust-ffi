use acadrust_ffi::*;
use std::ffi::{CStr, CString};
use std::ptr;

fn ok(status: Status) {
    assert_eq!(status, Status::Ok, "{}", unsafe {
        CStr::from_ptr(ar_last_error()).to_string_lossy()
    });
}
struct Drawing(*mut Document);
impl Drawing {
    fn new() -> Self {
        let mut out = ptr::null_mut();
        unsafe {
            ok(ar_document_new(1032, &mut out));
        }
        Self(out)
    }
    fn count(&self) -> usize {
        let mut count = 0;
        unsafe {
            ok(ar_entity_count(self.0, &mut count));
        }
        count
    }
}
impl Drop for Drawing {
    fn drop(&mut self) {
        unsafe {
            ar_document_free(self.0);
        }
    }
}

unsafe fn field(value: ValueView, name: &CStr) -> ValueView {
    let mut out = ValueView::default();
    ok(ar_value_field(value, name.as_ptr(), &mut out));
    out
}
unsafe fn payload(value: ValueView) -> ValueView {
    let mut out = ValueView::default();
    let mut name = StringView {
        data: ptr::null(),
        len: 0,
    };
    ok(ar_value_child(value, 0, &mut name, &mut out));
    out
}
unsafe fn storage(value: ValueView) -> (*const std::ffi::c_void, usize) {
    let mut data = ptr::null();
    let mut count = 0;
    let mut stride = 0;
    ok(ar_value_data(value, &mut data, &mut count, &mut stride));
    (data, count)
}

macro_rules! entity_factories {
    ($($kind:literal => ($new:ident, $create:ident)),* $(,)?) => {
        #[test]
        fn every_named_factory_matches_the_complete_native_catalog() {
            let factories = [$(( $kind, $new as unsafe extern "C" fn(*mut *mut EntityBuilder)->Status,
                $create as unsafe extern "C" fn(*mut Document,*const EntityEdit,usize,*mut u64)->Status)),*];
            unsafe {
                let mut count = 0; ok(ar_entity_kind_count(&mut count)); assert_eq!(factories.len(), count);
                let doc = Drawing::new();
                for (index, (kind, new, create)) in factories.into_iter().enumerate() {
                    let mut catalog = StringView { data: ptr::null(), len: 0 }; ok(ar_entity_kind_name(index, &mut catalog));
                    assert_eq!(std::slice::from_raw_parts(catalog.data.cast::<u8>(), catalog.len), kind.as_bytes());
                    let mut builder = ptr::null_mut(); ok(new(&mut builder));
                    let mut view = ValueView::default(); ok(ar_entity_builder_view(builder, &mut view));
                    let mut handle = 0; ok(ar_entity_builder_add(doc.0, &mut builder, &mut handle));
                    assert!(builder.is_null(), "{kind}"); assert_ne!(handle, 0);
                    ok(create(doc.0, ptr::null(), 0, &mut handle)); assert_ne!(handle, 0);
                }
            }
        }
    };
}
include!("../src/entity_factories_catalog.rs");

#[test]
fn text_input_is_payload_relative_and_allocations_move_into_the_document() {
    unsafe {
        let doc = Drawing::new();
        let mut builder = ptr::null_mut();
        ok(ar_text_builder_new(&mut builder));
        ok(ar_entity_builder_set_text(
            builder,
            c"value".as_ptr(),
            c"native text".as_ptr(),
        ));
        ok(ar_entity_builder_set_vec3(
            builder,
            c"insertion_point".as_ptr(),
            Vector3 {
                x: 3.0,
                y: 4.0,
                z: 5.0,
            },
        ));
        let mut view = ValueView::default();
        ok(ar_entity_builder_view(builder, &mut view));
        let original = storage(field(payload(view), c"value"));
        let mut handle = 0;
        ok(ar_entity_builder_add(doc.0, &mut builder, &mut handle));
        assert!(builder.is_null());
        ok(ar_entity_view(doc.0, handle, &mut view));
        assert_eq!(storage(field(payload(view), c"value")), original);
        let x = field(field(payload(view), c"insertion_point"), c"x");
        assert_eq!(*storage(x).0.cast::<f64>(), 3.0);
    }
}

#[test]
fn failed_add_retains_configuration_and_does_not_insert_into_a_missing_target() {
    unsafe {
        let doc = Drawing::new();
        let mut builder = ptr::null_mut();
        ok(ar_line_builder_new(&mut builder));
        ok(ar_entity_builder_set_vec3(
            builder,
            c"end".as_ptr(),
            Vector3 {
                x: 2.0,
                y: 3.0,
                z: 0.0,
            },
        ));
        let mut handle = 999;
        assert_eq!(
            ar_entity_builder_add_to_block(doc.0, &mut builder, c"Missing".as_ptr(), &mut handle),
            Status::NotFound
        );
        assert!(!builder.is_null());
        assert_eq!(handle, 0);
        assert_eq!(doc.count(), 0);
        let mut block = 0;
        ok(ar_table_add(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"Missing".as_ptr(),
            &mut block,
        ));
        ok(ar_entity_builder_add_to_block(
            doc.0,
            &mut builder,
            c"Missing".as_ptr(),
            &mut handle,
        ));
        assert_eq!(doc.count(), 1);
        let mut line = Line::default();
        ok(ar_line_get(doc.0, handle, &mut line));
        assert_eq!(line.end.y, 3.0);
    }
}

#[test]
fn construction_edits_are_ordered_atomic_and_keep_identities_protected() {
    unsafe {
        let doc = Drawing::new();
        let mut builder = ptr::null_mut();
        ok(ar_line_builder_new(&mut builder));
        let number = 10.0f64;
        let edits = [
            EntityEdit {
                kind: AR_ENTITY_EDIT_WRITE,
                path: c"start/x".as_ptr(),
                value: NativeWrite {
                    kind: AR_VALUE_F64,
                    data: std::ptr::from_ref(&number).cast(),
                    len: 1,
                },
                length: 0,
                variant: ptr::null(),
                enabled: 0,
            },
            EntityEdit {
                kind: AR_ENTITY_EDIT_WRITE,
                path: c"common/handle".as_ptr(),
                value: NativeWrite {
                    kind: AR_VALUE_F64,
                    data: std::ptr::from_ref(&number).cast(),
                    len: 1,
                },
                length: 0,
                variant: ptr::null(),
                enabled: 0,
            },
        ];
        assert_eq!(
            ar_entity_builder_apply(builder, edits.as_ptr(), edits.len()),
            Status::InvalidArgument
        );
        let mut view = ValueView::default();
        ok(ar_entity_builder_view(builder, &mut view));
        assert_eq!(
            *storage(field(field(payload(view), c"start"), c"x"))
                .0
                .cast::<f64>(),
            0.0
        );
        assert_ne!(
            ar_entity_builder_select(builder, c"Line/Line".as_ptr(), c"Circle".as_ptr()),
            Status::Ok
        );
        let mut handle = 9;
        assert_eq!(
            ar_line_create(doc.0, edits.as_ptr(), edits.len(), &mut handle),
            Status::InvalidArgument
        );
        assert_eq!(handle, 0);
        assert_eq!(doc.count(), 0);
        ok(ar_line_create(doc.0, edits.as_ptr(), 1, &mut handle));
        let mut line = Line::default();
        ok(ar_line_get(doc.0, handle, &mut line));
        assert_eq!(line.start.x, 10.0);
        ar_entity_builder_free(builder);
    }
}

#[test]
fn typed_builder_setters_check_integer_ranges_and_accept_vector_arrays() {
    unsafe {
        let mut builder = ptr::null_mut();
        ok(ar_spline_builder_new(&mut builder));
        ok(ar_entity_builder_set_i64(builder, c"degree".as_ptr(), 2));
        assert_eq!(
            ar_entity_builder_set_i64(builder, c"degree".as_ptr(), i64::MAX),
            Status::InvalidArgument
        );
        let points = [
            Vector3::default(),
            Vector3 {
                x: 2.0,
                y: 3.0,
                z: 0.0,
            },
            Vector3 {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
        ];
        ok(ar_entity_builder_set_vec3s(
            builder,
            c"control_points".as_ptr(),
            points.as_ptr(),
            points.len(),
        ));
        let mut view = ValueView::default();
        ok(ar_entity_builder_view(builder, &mut view));
        let mut count = 0;
        ok(ar_value_len(
            field(payload(view), c"control_points"),
            &mut count,
        ));
        assert_eq!(count, 3);
        let path = CString::new("degree").unwrap();
        assert_eq!(
            ar_entity_builder_set_u64(builder, path.as_ptr(), u64::MAX),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_builder_set_vec3s(builder, c"control_points".as_ptr(), ptr::null(), 1),
            Status::InvalidArgument
        );
        let knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let input = NativeWrite {
            kind: AR_VALUE_F64,
            data: knots.as_ptr().cast(),
            len: knots.len(),
        };
        ok(ar_entity_builder_write(builder, c"knots".as_ptr(), &input));
        assert_eq!(
            ar_entity_builder_set_vec3s(builder, c"knots".as_ptr(), ptr::null(), 0),
            Status::TypeMismatch
        );
        ok(ar_entity_builder_view(builder, &mut view));
        ok(ar_value_len(field(payload(view), c"knots"), &mut count));
        assert_eq!(count, knots.len());
        ok(ar_entity_builder_set_vec3s(
            builder,
            c"control_points".as_ptr(),
            ptr::null(),
            0,
        ));
        ar_entity_builder_free(builder);
    }
}
