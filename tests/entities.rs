use acadrust_ffi::*;
use std::ffi::{c_char, CStr, CString};
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

    fn count(&self) -> usize {
        unsafe {
            let mut count = 0;
            ok(ar_entity_count(self.0, &mut count));
            count
        }
    }

    fn handles(&self) -> Vec<u64> {
        unsafe {
            let mut count = 0;
            ok(ar_entity_handles(self.0, ptr::null_mut(), 0, &mut count));
            let mut handles = vec![0; count];
            ok(ar_entity_handles(
                self.0,
                handles.as_mut_ptr(),
                count,
                &mut count,
            ));
            handles
        }
    }

    fn json(&self, handle: u64) -> serde_json::Value {
        unsafe {
            let mut json = ptr::null_mut();
            ok(ar_entity_get_json(self.0, handle, &mut json));
            serde_json::from_str(&owned_string(json)).unwrap()
        }
    }

    fn roundtrip(&self, format: i32) -> Self {
        unsafe {
            let mut buffer = Buffer::default();
            ok(ar_document_write_buffer(self.0, format, &mut buffer));
            let mut doc = ptr::null_mut();
            let status = ar_document_read_buffer(buffer.data, buffer.len, AR_FORMAT_AUTO, &mut doc);
            ar_buffer_free(&mut buffer);
            ok(status);
            Self(doc)
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

unsafe fn owned_string(text: *mut c_char) -> String {
    let result = CStr::from_ptr(text).to_str().unwrap().to_owned();
    ar_string_free(text);
    result
}

fn sample_line() -> Line {
    Line {
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
    }
}

#[test]
fn typed_geometry_crud_and_enumeration() {
    unsafe {
        let doc = Drawing::new();
        let mut handles = [0; 4];
        let line = sample_line();
        let circle = Circle {
            center: line.start,
            radius: 8.0,
        };
        let arc = Arc {
            center: line.end,
            radius: 2.0,
            start_angle: 0.25,
            end_angle: 1.5,
        };
        let point = Point { position: line.end };
        ok(ar_line_add(doc.0, &line, &mut handles[0]));
        ok(ar_circle_add(doc.0, &circle, &mut handles[1]));
        ok(ar_arc_add(doc.0, &arc, &mut handles[2]));
        ok(ar_point_add(doc.0, &point, &mut handles[3]));
        assert_eq!(doc.count(), 4);
        assert_eq!(doc.handles(), handles);
        let mut got_line = Line::default();
        let mut got_circle = Circle::default();
        let mut got_arc = Arc::default();
        let mut got_point = Point::default();
        ok(ar_line_get(doc.0, handles[0], &mut got_line));
        ok(ar_circle_get(doc.0, handles[1], &mut got_circle));
        ok(ar_arc_get(doc.0, handles[2], &mut got_arc));
        ok(ar_point_get(doc.0, handles[3], &mut got_point));
        assert_eq!(
            (got_line, got_circle, got_arc, got_point),
            (line, circle, arc, point)
        );
        let line = Line {
            start: line.end,
            end: line.start,
        };
        let circle = Circle {
            radius: 12.0,
            ..circle
        };
        let arc = Arc {
            end_angle: 3.0,
            ..arc
        };
        let point = Point { position: line.end };
        ok(ar_line_set(doc.0, handles[0], &line));
        ok(ar_circle_set(doc.0, handles[1], &circle));
        ok(ar_arc_set(doc.0, handles[2], &arc));
        ok(ar_point_set(doc.0, handles[3], &point));
        ok(ar_line_get(doc.0, handles[0], &mut got_line));
        ok(ar_circle_get(doc.0, handles[1], &mut got_circle));
        ok(ar_arc_get(doc.0, handles[2], &mut got_arc));
        ok(ar_point_get(doc.0, handles[3], &mut got_point));
        assert_eq!(
            (got_line, got_circle, got_arc, got_point),
            (line, circle, arc, point)
        );
        let mut entity_type = ptr::null_mut();
        ok(ar_entity_type(doc.0, handles[2], &mut entity_type));
        assert_eq!(owned_string(entity_type), "ARC");
        let mut required = 0;
        let mut short_buffer = [999; 3];
        assert_eq!(
            ar_entity_handles(doc.0, short_buffer.as_mut_ptr(), 3, &mut required),
            Status::BufferTooSmall
        );
        assert_eq!(required, 4);
        assert_eq!(short_buffer, [999; 3]);
        for handle in handles {
            ok(ar_entity_remove(doc.0, handle));
        }
        assert_eq!(doc.count(), 0);
    }
}

#[test]
fn invalid_geometry_types_and_handles_report_errors_without_mutation() {
    unsafe {
        let doc = Drawing::new();
        let mut handle = 0;
        ok(ar_line_add(doc.0, &sample_line(), &mut handle));
        let mut circle = Circle::default();
        assert_eq!(
            ar_circle_get(doc.0, handle, &mut circle),
            Status::TypeMismatch
        );
        assert_eq!(
            ar_circle_get(doc.0, u64::MAX, &mut circle),
            Status::NotFound
        );
        assert_eq!(
            ar_circle_set(
                doc.0,
                handle,
                &Circle {
                    radius: 1.0,
                    ..circle
                }
            ),
            Status::TypeMismatch
        );
        assert_eq!(ar_entity_remove(doc.0, u64::MAX), Status::NotFound);
        assert_eq!(
            ar_line_get(doc.0, handle, ptr::null_mut()),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_line_set(doc.0, handle, ptr::null()),
            Status::InvalidArgument
        );
        let mut untouched = 777;
        for radius in [-1.0, 0.0, f64::NAN, f64::INFINITY] {
            assert_eq!(
                ar_circle_add(doc.0, &Circle { radius, ..circle }, &mut untouched),
                Status::InvalidArgument
            );
            assert_eq!(untouched, 777);
        }
        let line = Line {
            start: Vector3 {
                x: f64::INFINITY,
                ..Vector3::default()
            },
            ..sample_line()
        };
        assert_eq!(ar_line_set(doc.0, handle, &line), Status::InvalidArgument);
        assert_eq!(doc.count(), 1);
        let mut value = Line::default();
        ok(ar_line_get(doc.0, handle, &mut value));
        assert_eq!(value, sample_line());
    }
}

#[test]
fn json_clone_receives_fresh_identity_and_edits_preserve_owner() {
    unsafe {
        let doc = Drawing::new();
        let mut first = 0;
        ok(ar_line_add(doc.0, &sample_line(), &mut first));
        let template = doc.json(first);
        let owner = template["Line"]["common"]["owner_handle"].clone();
        let json = CString::new(template.to_string()).unwrap();
        let mut second = 0;
        ok(ar_entity_add_json(doc.0, json.as_ptr(), &mut second));
        assert_ne!(first, second);
        let mut edited = doc.json(second);
        edited["Line"]["end"]["x"] = serde_json::json!(100.0);
        edited["Line"]["common"]["handle"] = serde_json::json!(u64::MAX);
        edited["Line"]["common"]["owner_handle"] = serde_json::json!(u64::MAX);
        let json = CString::new(edited.to_string()).unwrap();
        ok(ar_entity_set_json(doc.0, second, json.as_ptr()));
        let result = doc.json(second);
        assert_eq!(result["Line"]["common"]["handle"], second);
        assert_eq!(result["Line"]["common"]["owner_handle"], owner);
        assert_eq!(result["Line"]["end"]["x"], 100.0);
        assert_eq!(doc.json(first), template);
        assert_eq!(doc.count(), 2);
        let invalid = CString::new("not JSON").unwrap();
        assert_eq!(
            ar_entity_set_json(doc.0, second, invalid.as_ptr()),
            Status::ParseError
        );
        let circle = CString::new(
            serde_json::to_string(&acadrust::EntityType::Circle(acadrust::Circle::new())).unwrap(),
        )
        .unwrap();
        assert_eq!(
            ar_entity_set_json(doc.0, second, circle.as_ptr()),
            Status::TypeMismatch
        );
    }
}

#[test]
fn removed_entities_stay_removed_in_dxf_and_dwg_roundtrips() {
    unsafe {
        let doc = Drawing::new();
        let mut removed = 0;
        let mut retained = 0;
        ok(ar_line_add(doc.0, &sample_line(), &mut removed));
        let circle = Circle {
            center: Vector3 {
                x: 10.0,
                y: 20.0,
                z: 30.0,
            },
            radius: 4.5,
        };
        ok(ar_circle_add(doc.0, &circle, &mut retained));
        ok(ar_entity_remove(doc.0, removed));
        for format in [AR_FORMAT_DXF_ASCII, AR_FORMAT_DXF_BINARY, AR_FORMAT_DWG] {
            let restored = doc.roundtrip(format);
            assert_eq!(restored.count(), 1, "format {format}");
            let mut actual = Circle::default();
            ok(ar_circle_get(restored.0, retained, &mut actual));
            assert_eq!(actual, circle, "format {format}");
            let mut line = Line::default();
            assert_eq!(
                ar_line_get(restored.0, removed, &mut line),
                Status::NotFound
            );
        }
    }
}

#[test]
fn polyline_buffer_queries_edits_and_validation() {
    unsafe {
        let doc = Drawing::new();
        let vertices = [
            LwVertex {
                x: 1.0,
                y: 2.0,
                bulge: 0.5,
                start_width: 0.25,
                end_width: 0.75,
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
            2,
            1,
            9.0,
            &mut handle,
        ));
        let mut required = 0;
        let mut closed = 0;
        let mut elevation = 0.0;
        ok(ar_lwpolyline_get(
            doc.0,
            handle,
            ptr::null_mut(),
            0,
            &mut required,
            &mut closed,
            &mut elevation,
        ));
        assert_eq!((required, closed, elevation), (2, 1, 9.0));
        let mut result = [LwVertex::default(); 2];
        assert_eq!(
            ar_lwpolyline_get(
                doc.0,
                handle,
                result.as_mut_ptr(),
                1,
                &mut required,
                &mut closed,
                &mut elevation
            ),
            Status::BufferTooSmall
        );
        assert_eq!(result, [LwVertex::default(); 2]);
        ok(ar_lwpolyline_get(
            doc.0,
            handle,
            result.as_mut_ptr(),
            2,
            &mut required,
            &mut closed,
            &mut elevation,
        ));
        assert_eq!(result, vertices);
        ok(ar_lwpolyline_set(
            doc.0,
            handle,
            vertices.as_ptr(),
            2,
            0,
            -4.0,
        ));
        ok(ar_lwpolyline_get(
            doc.0,
            handle,
            result.as_mut_ptr(),
            2,
            &mut required,
            &mut closed,
            &mut elevation,
        ));
        assert_eq!((closed, elevation), (0, -4.0));
        assert_eq!(
            ar_lwpolyline_set(doc.0, handle, vertices.as_ptr(), 2, 2, 0.0),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_lwpolyline_set(doc.0, handle, ptr::null(), 2, 0, 0.0),
            Status::InvalidArgument
        );
        let invalid = LwVertex {
            start_width: -0.1,
            ..vertices[0]
        };
        assert_eq!(
            ar_lwpolyline_set(doc.0, handle, &invalid, 1, 0, 0.0),
            Status::InvalidArgument
        );
        let invalid = LwVertex {
            bulge: f64::NAN,
            ..vertices[0]
        };
        assert_eq!(
            ar_lwpolyline_set(doc.0, handle, &invalid, 1, 0, 0.0),
            Status::InvalidArgument
        );
        ok(ar_lwpolyline_set(doc.0, handle, ptr::null(), 0, 0, 0.0));
        ok(ar_lwpolyline_get(
            doc.0,
            handle,
            ptr::null_mut(),
            0,
            &mut required,
            &mut closed,
            &mut elevation,
        ));
        assert_eq!(required, 0);
    }
}

#[test]
fn layers_and_entity_colors_survive_roundtrips() {
    unsafe {
        let doc = Drawing::new();
        let name = CString::new("Fixtures").unwrap();
        ok(ar_layer_add(doc.0, name.as_ptr(), 2));
        assert_eq!(
            ar_layer_add(doc.0, name.as_ptr(), 2),
            Status::InvalidArgument
        );
        let mut handle = 0;
        ok(ar_line_add(doc.0, &sample_line(), &mut handle));
        ok(ar_entity_set_layer(doc.0, handle, name.as_ptr()));
        ok(ar_entity_set_color_index(doc.0, handle, 256));
        ok(ar_layer_set_color_index(doc.0, name.as_ptr(), 3));
        ok(ar_layer_set_flags(
            doc.0,
            name.as_ptr(),
            AR_LAYER_FROZEN | AR_LAYER_LOCKED | AR_LAYER_OFF,
        ));
        let missing = CString::new("Missing").unwrap();
        assert_eq!(
            ar_entity_set_layer(doc.0, handle, missing.as_ptr()),
            Status::NotFound
        );
        assert_eq!(
            ar_entity_set_color_index(doc.0, handle, 257),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_layer_set_color_index(doc.0, name.as_ptr(), 0),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_layer_set_flags(doc.0, name.as_ptr(), 8),
            Status::InvalidArgument
        );
        for format in [AR_FORMAT_DXF_ASCII, AR_FORMAT_DWG] {
            let restored = doc.roundtrip(format);
            let mut layer = ptr::null_mut();
            ok(ar_entity_get_layer(restored.0, handle, &mut layer));
            assert_eq!(owned_string(layer), "Fixtures");
            let mut color = 0;
            ok(ar_entity_get_color_index(restored.0, handle, &mut color));
            assert_eq!(color, 256);
            ok(ar_layer_get_color_index(
                restored.0,
                name.as_ptr(),
                &mut color,
            ));
            assert_eq!(color, 3);
            let mut flags = 0;
            ok(ar_layer_get_flags(restored.0, name.as_ptr(), &mut flags));
            assert_eq!(flags, AR_LAYER_FROZEN | AR_LAYER_LOCKED | AR_LAYER_OFF);
        }
    }
}
