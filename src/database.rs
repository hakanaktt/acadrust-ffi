//! Symbol-table and object access without copying native records.

use std::ffi::c_char;

use acadrust::tables::{
    AppId, BlockRecord, DimStyle, Layer, LineType, TableEntry, TextStyle, Ucs, VPort,
    View as NamedView, VxTableRecord,
};
use acadrust::Handle;

use crate::views::{view, ValueView};
use crate::{
    document, document_mut, ffi_call, input_str, invalid, not_found, output, Document, Status,
};

pub const AR_TABLE_LAYER: i32 = 1;
pub const AR_TABLE_LINETYPE: i32 = 2;
pub const AR_TABLE_TEXT_STYLE: i32 = 3;
pub const AR_TABLE_BLOCK_RECORD: i32 = 4;
pub const AR_TABLE_DIM_STYLE: i32 = 5;
pub const AR_TABLE_APP_ID: i32 = 6;
pub const AR_TABLE_VIEW: i32 = 7;
pub const AR_TABLE_VPORT: i32 = 8;
pub const AR_TABLE_UCS: i32 = 9;
pub const AR_TABLE_VX: i32 = 10;

macro_rules! table_match {
    ($kind:expr, $doc:expr, $table:ident, $body:expr) => {
        match $kind {
            AR_TABLE_LAYER => {
                let $table = &$doc.layers;
                $body
            }
            AR_TABLE_LINETYPE => {
                let $table = &$doc.line_types;
                $body
            }
            AR_TABLE_TEXT_STYLE => {
                let $table = &$doc.text_styles;
                $body
            }
            AR_TABLE_BLOCK_RECORD => {
                let $table = &$doc.block_records;
                $body
            }
            AR_TABLE_DIM_STYLE => {
                let $table = &$doc.dim_styles;
                $body
            }
            AR_TABLE_APP_ID => {
                let $table = &$doc.app_ids;
                $body
            }
            AR_TABLE_VIEW => {
                let $table = &$doc.views;
                $body
            }
            AR_TABLE_VPORT => {
                let $table = &$doc.vports;
                $body
            }
            AR_TABLE_UCS => {
                let $table = &$doc.ucss;
                $body
            }
            AR_TABLE_VX => {
                let $table = &$doc.vx_table;
                $body
            }
            _ => Err(invalid("unknown symbol table kind")),
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_count(
    doc: *const Document,
    kind: i32,
    out_count: *mut usize,
) -> Status {
    ffi_call(|| {
        let out = output(out_count)?;
        let doc = document(doc)?;
        *out = table_match!(kind, doc, table, Ok(table.len()))?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_view(
    doc: *const Document,
    kind: i32,
    out_view: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out_view)?;
        *out = ValueView::default();
        let doc = document(doc)?;
        *out = table_match!(kind, doc, table, Ok(view(table)))?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_entry_view(
    doc: *const Document,
    kind: i32,
    index: usize,
    out_view: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out_view)?;
        *out = ValueView::default();
        let doc = document(doc)?;
        *out = table_match!(
            kind,
            doc,
            table,
            table
                .iter()
                .nth(index)
                .map(view)
                .ok_or_else(|| not_found("table entry index was not found"))
        )?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_find_view(
    doc: *const Document,
    kind: i32,
    name: *const c_char,
    out_view: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out_view)?;
        *out = ValueView::default();
        let name = input_str(name)?;
        let doc = document(doc)?;
        *out = table_match!(
            kind,
            doc,
            table,
            table
                .get(name)
                .map(view)
                .ok_or_else(|| not_found("table entry name was not found"))
        )?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_add(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        *out = 0;
        let name = input_str(name)?;
        if name.is_empty() {
            return Err(invalid("table entry name must not be empty"));
        }
        let doc = document_mut(doc)?;
        if table_match!(kind, doc, table, Ok(table.contains(name)))? && kind != AR_TABLE_VPORT {
            return Err(invalid("table entry name already exists"));
        }
        let handle = doc.allocate_handle();
        macro_rules! add {
            ($field:ident, $ty:ty) => {{
                let mut entry = <$ty>::new(name);
                entry.set_handle(handle);
                doc.$field.add(entry).map_err(invalid)?;
            }};
        }
        match kind {
            AR_TABLE_LAYER => add!(layers, Layer),
            AR_TABLE_LINETYPE => add!(line_types, LineType),
            AR_TABLE_TEXT_STYLE => add!(text_styles, TextStyle),
            AR_TABLE_BLOCK_RECORD => {
                let mut block = BlockRecord::new(name);
                block.handle = handle;
                block.block_entity_handle = doc.allocate_handle();
                block.block_end_handle = doc.allocate_handle();
                doc.block_records.add(block).map_err(invalid)?;
            }
            AR_TABLE_DIM_STYLE => add!(dim_styles, DimStyle),
            AR_TABLE_APP_ID => add!(app_ids, AppId),
            AR_TABLE_VIEW => add!(views, NamedView),
            AR_TABLE_VPORT => {
                let mut entry = VPort::new(name);
                entry.handle = handle;
                doc.vports.add_allow_duplicate(entry);
            }
            AR_TABLE_UCS => add!(ucss, Ucs),
            AR_TABLE_VX => add!(vx_table, VxTableRecord),
            _ => unreachable!("kind was validated before allocating"),
        }
        *out = handle.value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layout_add(
    doc: *mut Document,
    name: *const c_char,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        *out = 0;
        let name = input_str(name)?;
        if name.is_empty() {
            return Err(invalid("layout name must not be empty"));
        }
        let doc = document_mut(doc)?;
        // The upstream helper can allocate before detecting a block-name collision.
        // Apply it to a snapshot so every error leaves the caller's drawing intact.
        let mut updated = doc.clone();
        let handle = updated.add_layout(name)?;
        *doc = updated;
        *out = handle.value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_count(doc: *const Document, out_count: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_count)? = document(doc)?.objects.len();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_handle(
    doc: *const Document,
    index: usize,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        *out = 0;
        *out = document(doc)?
            .objects
            .keys()
            .nth(index)
            .ok_or_else(|| not_found("object index was not found"))?
            .value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_view(
    doc: *const Document,
    handle: u64,
    out_view: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out_view)?;
        *out = ValueView::default();
        let object = document(doc)?
            .objects
            .get(&Handle::new(handle))
            .ok_or_else(|| not_found("object handle was not found"))?;
        *out = view(object);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_class_count(doc: *const Document, out_count: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_count)? = document(doc)?.classes.len();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_class_view(
    doc: *const Document,
    index: usize,
    out_view: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out_view)?;
        *out = ValueView::default();
        let class = document(doc)?
            .classes
            .iter()
            .nth(index)
            .ok_or_else(|| not_found("class index was not found"))?;
        *out = view(class);
        Ok(())
    })
}
