use acadrust_ffi::*;
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

    fn count(&self) -> usize {
        unsafe {
            let mut count = 0;
            ok(ar_entity_count(self.0, &mut count));
            count
        }
    }

    fn roundtrip(&self) -> Self {
        unsafe {
            let mut data = Buffer::default();
            ok(ar_document_write_buffer(
                self.0,
                AR_FORMAT_DXF_ASCII,
                &mut data,
            ));
            let mut doc = ptr::null_mut();
            let status = ar_document_read_buffer(data.data, data.len, AR_FORMAT_AUTO, &mut doc);
            ar_buffer_free(&mut data);
            ok(status);
            Self(doc)
        }
    }

    unsafe fn payload(&self, handle: u64) -> ValueView {
        let mut root = ValueView::default();
        ok(ar_entity_view(self.0, handle, &mut root));
        let mut payload = ValueView::default();
        let mut name = StringView {
            data: ptr::null(),
            len: 0,
        };
        ok(ar_value_child(root, 0, &mut name, &mut payload));
        payload
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
    let mut child = ValueView::default();
    ok(ar_value_field(value, name.as_ptr(), &mut child));
    child
}

unsafe fn string(value: ValueView) -> String {
    let mut data = ptr::null();
    let (mut len, mut stride) = (0, 0);
    ok(ar_value_data(value, &mut data, &mut len, &mut stride));
    assert_eq!(stride, 1);
    String::from_utf8(std::slice::from_raw_parts(data.cast::<u8>(), len).to_vec()).unwrap()
}

unsafe fn number(value: ValueView) -> f64 {
    let mut data = ptr::null();
    let (mut len, mut stride) = (0, 0);
    ok(ar_value_data(value, &mut data, &mut len, &mut stride));
    assert_eq!((len, stride), (1, std::mem::size_of::<f64>()));
    *data.cast::<f64>()
}

unsafe fn length(value: ValueView) -> usize {
    let mut len = 0;
    ok(ar_value_len(value, &mut len));
    len
}

fn point(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 { x, y, z }
}

#[test]
fn direct_creation_preserves_owned_text_and_geometry_through_dxf() {
    unsafe {
        let doc = Drawing::new();
        let mut handles = [0; 9];
        let text = CString::new("native text").unwrap();
        let mtext = CString::new("first\\Psecond").unwrap();
        let mut t = TextOptions::default();
        ok(ar_text_options_default(&mut t));
        t.text = text.as_ptr();
        t.insertion_point = point(1.0, 2.0, 3.0);
        t.height = 2.5;
        t.rotation = 0.25;
        t.width_factor = 1.2;
        ok(ar_text_add(doc.0, &t, &mut handles[0]));
        let mut m = MTextOptions::default();
        ok(ar_mtext_options_default(&mut m));
        m.text = mtext.as_ptr();
        m.height = 3.0;
        m.rectangle_width = 14.0;
        m.attachment_point = 5;
        ok(ar_mtext_add(doc.0, &m, &mut handles[1]));
        drop(text);
        drop(mtext);
        let ellipse = EllipseOptions {
            center: point(4.0, 5.0, 0.0),
            major_axis: point(6.0, 0.0, 0.0),
            minor_axis_ratio: 0.75,
            ..EllipseOptions::default()
        };
        ok(ar_ellipse_add(doc.0, &ellipse, &mut handles[2]));
        let control_points = [
            point(0.0, 0.0, 0.0),
            point(1.0, 2.0, 0.0),
            point(3.0, 2.0, 0.0),
            point(4.0, 0.0, 0.0),
        ];
        let spline = SplineOptions {
            control_points: control_points.as_ptr(),
            control_point_count: 4,
            ..SplineOptions::default()
        };
        ok(ar_spline_add(doc.0, &spline, &mut handles[3]));
        let mut block_handle = 0;
        ok(ar_table_add(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"SYMBOL".as_ptr(),
            &mut block_handle,
        ));
        let insert = InsertOptions {
            block_name: c"SYMBOL".as_ptr(),
            insertion_point: point(5.0, 6.0, 0.0),
            scale: point(2.0, 3.0, 1.0),
            rotation: 0.5,
            column_count: 2,
            column_spacing: 10.0,
            ..InsertOptions::default()
        };
        ok(ar_insert_add(doc.0, &insert, &mut handles[4]));
        let corners = [
            point(0.0, 0.0, 0.0),
            point(2.0, 0.0, 0.0),
            point(2.0, 2.0, 1.0),
            point(0.0, 2.0, 1.0),
        ];
        let face = Face3DOptions {
            corners,
            invisible_edges: 5,
            ..Face3DOptions::default()
        };
        ok(ar_face3d_add(doc.0, &face, &mut handles[5]));
        let solid = SolidOptions {
            corners: [corners[0], corners[1], corners[3], corners[2]],
            thickness: 2.0,
            ..SolidOptions::default()
        };
        ok(ar_solid_add(doc.0, &solid, &mut handles[6]));
        let directed = RayOptions {
            base_point: point(7.0, 8.0, 9.0),
            direction: point(3.0, 4.0, 0.0),
            ..RayOptions::default()
        };
        ok(ar_ray_add(doc.0, &directed, &mut handles[7]));
        ok(ar_xline_add(doc.0, &directed, &mut handles[8]));
        assert_eq!(doc.count(), 9);

        for doc in [&doc, &doc.roundtrip()] {
            assert_eq!(
                string(field(doc.payload(handles[0]), c"value")),
                "native text"
            );
            assert_eq!(number(field(doc.payload(handles[0]), c"height")), 2.5);
            assert_eq!(
                string(field(doc.payload(handles[1]), c"value")),
                "first\\Psecond"
            );
            assert_eq!(
                number(field(doc.payload(handles[2]), c"minor_axis_ratio")),
                0.75
            );
            assert_eq!(length(field(doc.payload(handles[3]), c"control_points")), 4);
            assert_eq!(length(field(doc.payload(handles[3]), c"knots")), 8);
            assert_eq!(
                string(field(doc.payload(handles[4]), c"block_name")),
                "SYMBOL"
            );
            assert_eq!(number(field(doc.payload(handles[6]), c"thickness")), 2.0);
            assert!(
                (number(field(field(doc.payload(handles[7]), c"direction"), c"x")) - 0.6).abs()
                    < 1e-12
            );
            assert!(
                (number(field(field(doc.payload(handles[8]), c"direction"), c"y")) - 0.8).abs()
                    < 1e-12
            );
        }
    }
}

#[test]
fn failed_constructor_validation_does_not_change_document_or_output() {
    unsafe {
        let doc = Drawing::new();
        let mut handle = 123456;
        let bad_text = TextOptions {
            height: 0.0,
            ..TextOptions::default()
        };
        assert_eq!(
            ar_text_add(doc.0, &bad_text, &mut handle),
            Status::InvalidArgument
        );
        let bad_mtext = MTextOptions {
            drawing_direction: 2,
            ..MTextOptions::default()
        };
        assert_eq!(
            ar_mtext_add(doc.0, &bad_mtext, &mut handle),
            Status::InvalidArgument
        );
        let bad_ellipse = EllipseOptions {
            minor_axis_ratio: 2.0,
            ..EllipseOptions::default()
        };
        assert_eq!(
            ar_ellipse_add(doc.0, &bad_ellipse, &mut handle),
            Status::InvalidArgument
        );
        let bad_spline = SplineOptions {
            control_point_count: 4,
            ..SplineOptions::default()
        };
        assert_eq!(
            ar_spline_add(doc.0, &bad_spline, &mut handle),
            Status::InvalidArgument
        );
        let bad_insert = InsertOptions {
            block_name: c"MISSING".as_ptr(),
            ..InsertOptions::default()
        };
        assert_eq!(
            ar_insert_add(doc.0, &bad_insert, &mut handle),
            Status::NotFound
        );
        let bad_face = Face3DOptions {
            invisible_edges: 16,
            ..Face3DOptions::default()
        };
        assert_eq!(
            ar_face3d_add(doc.0, &bad_face, &mut handle),
            Status::InvalidArgument
        );
        let bad_solid = SolidOptions {
            thickness: f64::INFINITY,
            ..SolidOptions::default()
        };
        assert_eq!(
            ar_solid_add(doc.0, &bad_solid, &mut handle),
            Status::InvalidArgument
        );
        let bad_ray = RayOptions {
            direction: Vector3::default(),
            ..RayOptions::default()
        };
        assert_eq!(
            ar_ray_add(doc.0, &bad_ray, &mut handle),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_xline_add(doc.0, &bad_ray, &mut handle),
            Status::InvalidArgument
        );
        let invalid_utf8 = [0xff_u8, 0];
        let utf8 = TextOptions {
            text: invalid_utf8.as_ptr().cast(),
            ..TextOptions::default()
        };
        assert_eq!(
            ar_text_add(doc.0, &utf8, &mut handle),
            Status::InvalidArgument
        );
        assert_eq!(handle, 123456);
        assert_eq!(doc.count(), 0);
    }
}

#[test]
fn spline_custom_knots_weights_and_array_guards() {
    unsafe {
        let doc = Drawing::new();
        let points = [
            point(0.0, 0.0, 0.0),
            point(1.0, 2.0, 0.0),
            point(2.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let weights = [1.0, 2.0, 1.0];
        let mut spline = SplineOptions {
            degree: 2,
            control_points: points.as_ptr(),
            control_point_count: 3,
            knots: knots.as_ptr(),
            knot_count: 6,
            weights: weights.as_ptr(),
            weight_count: 3,
            ..SplineOptions::default()
        };
        let mut handle = 0;
        ok(ar_spline_add(doc.0, &spline, &mut handle));
        let copy = doc.roundtrip();
        assert_eq!(length(field(copy.payload(handle), c"weights")), 3);
        spline.weight_count = 2;
        assert_eq!(
            ar_spline_add(doc.0, &spline, &mut handle),
            Status::InvalidArgument
        );
        spline.weight_count = 3;
        spline.control_point_count = usize::MAX;
        assert_eq!(
            ar_spline_add(doc.0, &spline, &mut handle),
            Status::InvalidArgument
        );
        assert_eq!(doc.count(), 1);
    }
}

#[test]
fn directed_line_normalization_handles_extreme_finite_values() {
    unsafe {
        let doc = Drawing::new();
        let ray = RayOptions {
            direction: point(f64::MAX, f64::MAX, 0.0),
            ..RayOptions::default()
        };
        let mut handle = 0;
        ok(ar_ray_add(doc.0, &ray, &mut handle));
        let x = number(field(field(doc.payload(handle), c"direction"), c"x"));
        assert!((x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12);
    }
}
