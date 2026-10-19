use acadrust_ffi::*;
use std::collections::BTreeSet;
use std::ffi::{CStr, CString};
use std::ptr;

struct Drawing(*mut Document);
impl Drawing {
    fn new() -> Self {
        unsafe {
            let mut value = ptr::null_mut();
            ok(ar_document_new(1032, &mut value));
            Self(value)
        }
    }
}
impl Drop for Drawing {
    fn drop(&mut self) {
        unsafe { ar_document_free(self.0) }
    }
}
fn string(value: &str) -> CString {
    CString::new(value).unwrap()
}
fn empty_string() -> StringView {
    StringView {
        data: ptr::null(),
        len: 0,
    }
}
unsafe fn text(value: StringView) -> String {
    String::from_utf8_lossy(std::slice::from_raw_parts(value.data.cast(), value.len)).into_owned()
}
fn ok(value: Status) {
    assert_eq!(value, Status::Ok, "{}", unsafe {
        CStr::from_ptr(ar_last_error()).to_string_lossy()
    });
}

unsafe fn intrinsic_handle(mut value: ValueView) -> u64 {
    let mut variant = empty_string();
    ok(ar_value_variant(value, &mut variant));
    let variant = text(variant);
    let mut name = empty_string();
    if variant != "Unknown" {
        ok(ar_value_child(value, 0, &mut name, &mut value));
    }
    if variant == "TableContent" {
        ok(ar_value_field(value, c"common".as_ptr(), &mut value));
    }
    let mut field = ValueView::default();
    ok(ar_value_field(value, c"handle".as_ptr(), &mut field));
    let mut handle = 0;
    ok(ar_value_read_integer(field, &mut handle));
    handle as u64
}

#[test]
fn every_native_object_kind_constructs_clones_and_removes() {
    unsafe {
        let mut count = 0;
        ok(ar_object_kind_count(&mut count));
        assert!(count > 200);
        let mut variants = BTreeSet::new();
        for index in 0..count {
            let doc = Drawing::new();
            let mut name = empty_string();
            let mut variant = empty_string();
            ok(ar_object_kind_name(index, &mut name));
            ok(ar_object_kind_variant(index, &mut variant));
            let name = text(name);
            let variant = text(variant);
            variants.insert(variant.clone());
            let mut source = 0;
            ok(ar_object_add_default(
                doc.0,
                string(&name).as_ptr(),
                0,
                &mut source,
            ));
            let mut view = ValueView::default();
            ok(ar_object_view(doc.0, source, &mut view));
            assert_eq!(intrinsic_handle(view), source, "{name}");
            let mut actual = empty_string();
            ok(ar_value_variant(view, &mut actual));
            assert_eq!(text(actual), variant, "{name}");
            let mut cloned = 0;
            ok(ar_object_clone(doc.0, source, 0, &mut cloned));
            assert_ne!(source, cloned);
            ok(ar_object_view(doc.0, cloned, &mut view));
            assert_eq!(intrinsic_handle(view), cloned, "{name}");
            ok(ar_object_remove(doc.0, cloned));
            assert_eq!(ar_object_view(doc.0, cloned, &mut view), Status::NotFound);
            ok(ar_object_remove(doc.0, source));
        }
        assert_eq!(variants.len(), 36);
        assert_eq!(
            ar_object_kind_name(count, &mut empty_string()),
            Status::NotFound
        );
    }
}

#[test]
fn dependent_objects_and_default_tables_are_protected() {
    unsafe {
        let doc = Drawing::new();
        let (mut owner, mut child) = (0, 0);
        ok(ar_object_add_default(
            doc.0,
            c"DICTIONARY".as_ptr(),
            0,
            &mut owner,
        ));
        ok(ar_object_add_default(
            doc.0,
            c"PLACEHOLDER".as_ptr(),
            owner,
            &mut child,
        ));
        assert_eq!(ar_object_remove(doc.0, owner), Status::InvalidArgument);
        ok(ar_object_remove(doc.0, child));
        ok(ar_object_remove(doc.0, owner));
        let mut table = 0;
        for (kind, name) in [
            (AR_TABLE_LAYER, "UnusedLayer"),
            (AR_TABLE_LINETYPE, "UnusedType"),
            (AR_TABLE_TEXT_STYLE, "UnusedStyle"),
            (AR_TABLE_BLOCK_RECORD, "UnusedBlock"),
            (AR_TABLE_DIM_STYLE, "UnusedDim"),
            (AR_TABLE_APP_ID, "UnusedApp"),
            (AR_TABLE_VIEW, "UnusedView"),
            (AR_TABLE_VPORT, "UnusedPort"),
            (AR_TABLE_UCS, "UnusedUcs"),
            (AR_TABLE_VX, "UnusedVx"),
        ] {
            ok(ar_table_add(doc.0, kind, string(name).as_ptr(), &mut table));
            ok(ar_table_remove(doc.0, kind, string(name).as_ptr()));
            assert_eq!(
                ar_table_remove(doc.0, kind, string(name).as_ptr()),
                Status::NotFound
            );
        }
        assert_eq!(
            ar_table_remove(doc.0, AR_TABLE_LAYER, c"0".as_ptr()),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_table_remove(doc.0, AR_TABLE_BLOCK_RECORD, c"*Model_Space".as_ptr()),
            Status::InvalidArgument
        );
    }
}

#[test]
fn extension_dictionary_cannot_be_removed_from_non_entity_owner() {
    unsafe {
        let doc = Drawing::new();
        let mut owner = 0;
        ok(ar_object_add_default(
            doc.0,
            c"PLACEHOLDER".as_ptr(),
            0,
            &mut owner,
        ));
        let mut dictionary = 0;
        ok(ar_document_extension_dictionary(
            doc.0,
            owner,
            1,
            &mut dictionary,
        ));
        assert_ne!(dictionary, 0);
        assert_eq!(ar_object_remove(doc.0, dictionary), Status::InvalidArgument);

        // The private non-entity ownership cache must still resolve after the
        // failed removal.  Creating an XRecord through the owner exercises the
        // same resolver used by downstream callers.
        let mut record = 0;
        ok(ar_document_ensure_xrecord(
            doc.0,
            owner,
            c"KEEP".as_ptr(),
            &mut record,
        ));
        assert_ne!(record, 0);
    }
}

#[test]
fn header_symbol_names_protect_block_and_ucs_removal() {
    unsafe {
        let doc = Drawing::new();
        let mut block = 0;
        ok(ar_table_add(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"DimArrowHeader".as_ptr(),
            &mut block,
        ));
        let block_name = c"DimArrowHeader";
        let block_path = c"header/dim_arrow_block";
        let write = NativeWrite {
            kind: AR_VALUE_STRING,
            data: block_name.as_ptr().cast(),
            len: block_name.to_bytes().len(),
        };
        ok(ar_document_write(doc.0, block_path.as_ptr(), &write));
        assert_eq!(
            ar_table_remove(doc.0, AR_TABLE_BLOCK_RECORD, block_name.as_ptr()),
            Status::InvalidArgument
        );

        let mut ucs = 0;
        ok(ar_table_add(
            doc.0,
            AR_TABLE_UCS,
            c"UcsHeader".as_ptr(),
            &mut ucs,
        ));
        let ucs_name = c"UcsHeader";
        let ucs_path = c"header/model_space_ucs_name";
        let write = NativeWrite {
            kind: AR_VALUE_STRING,
            data: ucs_name.as_ptr().cast(),
            len: ucs_name.to_bytes().len(),
        };
        ok(ar_document_write(doc.0, ucs_path.as_ptr(), &write));
        assert_eq!(
            ar_table_remove(doc.0, AR_TABLE_UCS, ucs_name.as_ptr()),
            Status::InvalidArgument
        );
    }
}

#[test]
fn entity_clone_and_moves_preserve_native_block_membership() {
    unsafe {
        let doc = Drawing::new();
        let (mut entity, mut block, mut layout, mut cloned) = (0, 0, 0, 0);
        ok(ar_entity_add_default(doc.0, c"LINE".as_ptr(), &mut entity));
        ok(ar_table_add(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"Target".as_ptr(),
            &mut block,
        ));
        ok(ar_entity_move_to_block_name(
            doc.0,
            entity,
            c"Target".as_ptr(),
        ));
        assert_eq!(
            ar_table_remove(doc.0, AR_TABLE_BLOCK_RECORD, c"Target".as_ptr()),
            Status::InvalidArgument
        );
        ok(ar_entity_clone(doc.0, entity, &mut cloned));
        assert_ne!(entity, cloned);
        ok(ar_layout_add(doc.0, c"Sheet2".as_ptr(), &mut layout));
        ok(ar_entity_move_to_layout(doc.0, entity, c"Sheet2".as_ptr()));
        ok(ar_entity_move_to_layout(doc.0, cloned, c"Sheet2".as_ptr()));
        ok(ar_table_remove(
            doc.0,
            AR_TABLE_BLOCK_RECORD,
            c"Target".as_ptr(),
        ));
        let mut count = 0;
        ok(ar_entity_count(doc.0, &mut count));
        // A newly connected paper-space layout also creates its viewport.
        assert_eq!(count, 3);
        assert_eq!(
            ar_entity_move_to_block_name(doc.0, entity, c"Missing".as_ptr()),
            Status::NotFound
        );
        assert_eq!(
            ar_entity_move_to_layout(doc.0, entity, c"Missing".as_ptr()),
            Status::NotFound
        );
        let mut buffer = Buffer::default();
        ok(ar_document_write_buffer(
            doc.0,
            AR_FORMAT_DXF_ASCII,
            &mut buffer,
        ));
        let mut roundtrip = ptr::null_mut();
        let status =
            ar_document_read_buffer(buffer.data, buffer.len, AR_FORMAT_AUTO, &mut roundtrip);
        ar_buffer_free(&mut buffer);
        ok(status);
        let roundtrip = Drawing(roundtrip);
        ok(ar_entity_count(roundtrip.0, &mut count));
        assert_eq!(count, 3);
    }
}

#[test]
fn class_collection_mutations_preserve_order_and_lookup() {
    unsafe {
        let doc = Drawing::new();
        let mut start = 0;
        ok(ar_class_count(doc.0, &mut start));
        for index in 0..3 {
            let mut actual = 0;
            ok(ar_class_add(
                doc.0,
                string(&format!("CUSTOM{index}")).as_ptr(),
                c"CustomCpp".as_ptr(),
                c"TestApp".as_ptr(),
                0,
                &mut actual,
            ));
            assert_eq!(actual, start + index);
        }
        assert_eq!(
            ar_class_add(
                doc.0,
                c"CUSTOM1".as_ptr(),
                c"CustomCpp".as_ptr(),
                c"TestApp".as_ptr(),
                0,
                &mut 0
            ),
            Status::InvalidArgument
        );
        ok(ar_class_remove(doc.0, start + 1));
        let mut view = ValueView::default();
        ok(ar_class_view(doc.0, start + 1, &mut view));
        ok(ar_value_field(view, c"dxf_name".as_ptr(), &mut view));
        let (mut data, mut len, mut stride) = (ptr::null(), 0, 0);
        ok(ar_value_data(view, &mut data, &mut len, &mut stride));
        assert_eq!(
            std::slice::from_raw_parts(data.cast::<u8>(), len),
            b"CUSTOM2"
        );
        assert_eq!(ar_class_remove(doc.0, start + 2), Status::NotFound);
    }
}

#[test]
fn invalid_inputs_leave_counts_unchanged() {
    unsafe {
        let doc = Drawing::new();
        let mut before = 0;
        ok(ar_object_count(doc.0, &mut before));
        assert_eq!(
            ar_object_add_default(doc.0, c"PLACEHOLDER".as_ptr(), u64::MAX, &mut 0),
            Status::NotFound
        );
        assert_eq!(
            ar_object_add_default(doc.0, c"BAD_KIND".as_ptr(), 0, &mut 0),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_object_add_default(doc.0, c"PLACEHOLDER".as_ptr(), 0, ptr::null_mut()),
            Status::InvalidArgument
        );
        assert_eq!(ar_object_remove(doc.0, u64::MAX), Status::NotFound);
        let mut after = 0;
        ok(ar_object_count(doc.0, &mut after));
        assert_eq!(before, after);
    }
}

#[test]
fn table_removal_by_index_preserves_duplicate_viewports() {
    unsafe {
        let doc = Drawing::new();
        let mut start = 0;
        ok(ar_table_count(doc.0, AR_TABLE_VPORT, &mut start));
        let (mut first, mut second) = (0, 0);
        ok(ar_table_add(
            doc.0,
            AR_TABLE_VPORT,
            c"Duplicate".as_ptr(),
            &mut first,
        ));
        ok(ar_table_add(
            doc.0,
            AR_TABLE_VPORT,
            c"Duplicate".as_ptr(),
            &mut second,
        ));
        ok(ar_table_remove_at(doc.0, AR_TABLE_VPORT, start));
        let mut view = ValueView::default();
        ok(ar_table_find_view(
            doc.0,
            AR_TABLE_VPORT,
            c"Duplicate".as_ptr(),
            &mut view,
        ));
        let mut field = ValueView::default();
        ok(ar_value_field(view, c"handle".as_ptr(), &mut field));
        let mut handle = 0;
        ok(ar_value_read_integer(field, &mut handle));
        assert_eq!(handle as u64, second);
        assert_ne!(first, second);
    }
}
