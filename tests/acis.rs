use acadrust_ffi::*;
use std::ffi::CStr;
use std::ptr;

fn ok(status: Status) {
    assert_eq!(status, Status::Ok, "{}", unsafe {
        CStr::from_ptr(ar_last_error()).to_string_lossy()
    });
}

#[test]
fn all_upstream_primitives_have_native_views_and_sat_sab_roundtrips() {
    unsafe {
        for kind in 0..=6 {
            let mut model = ptr::null_mut();
            ok(ar_acis_primitive(
                kind,
                Vector3::default(),
                Vector3 {
                    x: 3.0,
                    y: 1.0,
                    z: 2.0,
                },
                &mut model,
            ));
            let mut value = ValueView::default();
            ok(ar_acis_view(model, &mut value));
            let mut records = ValueView::default();
            ok(ar_value_field(value, c"records".as_ptr(), &mut records));
            let mut count = 0;
            ok(ar_value_len(records, &mut count));
            assert!(count > 0);
            let mut sat = ptr::null_mut();
            ok(ar_acis_write_sat(model, &mut sat));
            let mut parsed = ptr::null_mut();
            ok(ar_acis_parse_sat(sat, &mut parsed));
            ar_string_free(sat);
            ar_acis_free(parsed);
            let mut sab = Buffer::default();
            ok(ar_acis_write_sab(model, &mut sab));
            ok(ar_acis_parse_sab(sab.data, sab.len, &mut parsed));
            ar_buffer_free(&mut sab);
            ar_acis_free(parsed);
            ar_acis_free(model);
        }
    }
}

#[test]
fn acis_models_attach_to_all_four_native_modeler_entity_kinds() {
    unsafe {
        let mut doc = ptr::null_mut();
        ok(ar_document_new(1032, &mut doc));
        let mut model = ptr::null_mut();
        ok(ar_acis_primitive(
            5,
            Vector3::default(),
            Vector3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            &mut model,
        ));
        for kind in [c"SOLID3D", c"REGION", c"BODY", c"SURFACE"] {
            let mut handle = 0;
            ok(ar_entity_add_default(doc, kind.as_ptr(), &mut handle));
            ok(ar_entity_set_acis(doc, handle, model));
            let mut parsed = ptr::null_mut();
            ok(ar_entity_parse_acis(doc, handle, &mut parsed));
            ar_acis_free(parsed);
        }
        ar_acis_free(model);
        ar_document_free(doc);
        assert_eq!(
            ar_acis_view(ptr::null(), ptr::null_mut()),
            Status::InvalidArgument
        );
        let mut out = ptr::dangling_mut::<AcisDocument>();
        assert_eq!(
            ar_acis_primitive(5, Vector3::default(), Vector3::default(), &mut out),
            Status::InvalidArgument
        );
        assert!(out.is_null());
    }
}
