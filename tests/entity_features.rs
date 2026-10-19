use acadrust_ffi::*;
use std::collections::BTreeSet;
use std::ffi::{CStr, CString};
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
    fn count(&self) -> usize {
        unsafe {
            let mut count = 0;
            ok(ar_entity_count(self.0, &mut count));
            count
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
unsafe fn text(value: StringView) -> String {
    String::from_utf8_lossy(std::slice::from_raw_parts(value.data.cast(), value.len)).into_owned()
}
fn empty_string() -> StringView {
    StringView {
        data: ptr::null(),
        len: 0,
    }
}

#[test]
fn native_catalog_constructs_every_variant_and_subtype() {
    unsafe {
        let mut count = 0;
        ok(ar_entity_kind_count(&mut count));
        assert_eq!(count, 97);
        let mut variants = BTreeSet::new();
        let doc = Drawing::new();
        for index in 0..count {
            let mut name = empty_string();
            let mut variant = empty_string();
            ok(ar_entity_kind_name(index, &mut name));
            ok(ar_entity_kind_variant(index, &mut variant));
            let name = text(name);
            let variant = text(variant);
            variants.insert(variant.clone());
            let handle = doc.add(&name);
            let mut view = ValueView::default();
            ok(ar_entity_view(doc.0, handle, &mut view));
            let mut actual_variant = empty_string();
            ok(ar_value_variant(view, &mut actual_variant));
            assert_eq!(text(actual_variant), variant, "{name}");
            let mut class = empty_string();
            ok(ar_entity_class_name(doc.0, handle, &mut class));
            assert!(!text(class).is_empty(), "{name}");
        }
        assert_eq!(variants.len(), 48);
        // Upstream enumeration excludes the structural BLOCK/ENDBLK markers.
        assert_eq!(doc.count(), count - 2);
        assert_eq!(
            ar_entity_kind_name(count, &mut empty_string()),
            Status::NotFound
        );
        assert_eq!(
            ar_entity_kind_variant(count, &mut empty_string()),
            Status::NotFound
        );
    }
}

#[test]
fn geometry_operations_use_upstream_semantics() {
    unsafe {
        let doc = Drawing::new();
        let line = Line {
            start: Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            end: Vector3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };
        let mut handle = 0;
        ok(ar_line_add(doc.0, &line, &mut handle));
        let mut bounds = BoundingBox::default();
        ok(ar_entity_bounding_box(doc.0, handle, &mut bounds));
        assert_eq!(
            bounds,
            BoundingBox {
                min: line.start,
                max: line.end
            }
        );
        ok(ar_entity_translate(
            doc.0,
            handle,
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
        ));
        let mut matrix = Matrix4::default();
        matrix.values[3] = 3.0;
        ok(ar_entity_transform(doc.0, handle, &matrix));
        let mut got = Line::default();
        ok(ar_line_get(doc.0, handle, &mut got));
        assert_eq!(
            got.start,
            Vector3 {
                x: 5.0,
                y: 2.0,
                z: 2.0
            }
        );
        ok(ar_entity_scale(
            doc.0,
            handle,
            Vector3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            Vector3::default(),
        ));
        ok(ar_entity_mirror(
            doc.0,
            handle,
            Vector3::default(),
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ));
        ok(ar_line_get(doc.0, handle, &mut got));
        assert_eq!(
            got.start,
            Vector3 {
                x: -10.0,
                y: 4.0,
                z: 4.0
            }
        );
        let before = got;
        matrix.values[12] = 1.0;
        assert_eq!(
            ar_entity_transform(doc.0, handle, &matrix),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_rotate(doc.0, handle, Vector3::default(), 1.0),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_translate(
                doc.0,
                handle,
                Vector3 {
                    x: f64::NAN,
                    y: 0.0,
                    z: 0.0
                }
            ),
            Status::InvalidArgument
        );
        ok(ar_line_get(doc.0, handle, &mut got));
        assert_eq!(got, before);
        let light = doc.add("LIGHT");
        assert_eq!(
            ar_entity_transform(doc.0, light, &Matrix4::default()),
            Status::TypeMismatch
        );
        ok(ar_entity_translate(
            doc.0,
            light,
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ));
        let camera = doc.add("CAMERA");
        assert_eq!(
            ar_entity_translate(doc.0, camera, Vector3::default()),
            Status::TypeMismatch
        );
    }
}

#[test]
fn native_explode_queries_before_adding_decomposed_geometry() {
    unsafe {
        let doc = Drawing::new();
        let vertices = [
            LwVertex {
                x: 0.0,
                y: 0.0,
                ..LwVertex::default()
            },
            LwVertex {
                x: 3.0,
                y: 4.0,
                ..LwVertex::default()
            },
        ];
        let mut handle = 0;
        ok(ar_lwpolyline_add(
            doc.0,
            vertices.as_ptr(),
            vertices.len(),
            0,
            0.0,
            &mut handle,
        ));
        let mut required = 0;
        ok(ar_entity_explode(
            doc.0,
            handle,
            ptr::null_mut(),
            0,
            &mut required,
        ));
        assert_eq!(required, 1);
        assert_eq!(doc.count(), 1);
        let mut parts = [0u64; 1];
        assert_eq!(
            ar_entity_explode(doc.0, handle, parts.as_mut_ptr(), 0, &mut required),
            Status::BufferTooSmall
        );
        assert_eq!(doc.count(), 1);
        ok(ar_entity_explode(
            doc.0,
            handle,
            parts.as_mut_ptr(),
            parts.len(),
            &mut required,
        ));
        assert_eq!(doc.count(), 2);
        let mut line = Line::default();
        ok(ar_line_get(doc.0, parts[0], &mut line));
        assert_eq!(
            line.end,
            Vector3 {
                x: 3.0,
                y: 4.0,
                z: 0.0
            }
        );
    }
}

#[test]
fn null_inputs_fail_without_panicking() {
    unsafe {
        let doc = Drawing::new();
        let handle = doc.add("POINT");
        assert_eq!(
            ar_entity_kind_count(ptr::null_mut()),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_kind_name(0, ptr::null_mut()),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_add_default(doc.0, ptr::null(), &mut 0),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_transform(doc.0, handle, ptr::null()),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_bounding_box(doc.0, handle, ptr::null_mut()),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_entity_explode(doc.0, handle, ptr::null_mut(), 0, ptr::null_mut()),
            Status::InvalidArgument
        );
    }
}
