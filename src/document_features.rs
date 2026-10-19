//! Document-wide native access and computed reports.
use crate::views::{view, NativeView, StringView, ValueView};
use crate::*;
use acadrust::{Handle, LayerState, LayerStateMask};

pub struct NativeResult {
    value: ResultValue,
}

enum ResultValue {
    Audit(acadrust::AuditReport),
    Repair(acadrust::RepairReport),
    LayerStates(Vec<LayerState>),
    BlockGraph(acadrust::BlockGraph),
    SolidHistory(acadrust::SolidHistoryGraph),
    SolidHistoryTree(Box<acadrust::objects::SolidHistoryTree>),
    SolidHistoryOperations(Vec<acadrust::objects::SolidHistoryOperation>),
    DynamicProperties(Vec<acadrust::document::DynamicBlockProperty>),
}

crate::views::view_enum!(ResultValue { Audit(v0), Repair(v0), LayerStates(v0), BlockGraph(v0), SolidHistory(v0), SolidHistoryTree(v0), SolidHistoryOperations(v0), DynamicProperties(v0) });

unsafe fn result_out(value: ResultValue, out: *mut *mut NativeResult) -> FfiResult<()> {
    *output(out)? = Box::into_raw(Box::new(NativeResult { value }));
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_result_free(result: *mut NativeResult) {
    if !result.is_null() {
        drop(Box::from_raw(result));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_result_view(
    result: *const NativeResult,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        let result = result.as_ref().ok_or_else(|| invalid("result is NULL"))?;
        *out = result
            .value
            .child(0)
            .ok_or_else(|| invalid("result is empty"))?
            .1;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_view(doc: *const Document, out: *mut ValueView) -> Status {
    ffi_call(|| {
        *output(out)? = view(document(doc)?);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_entity_at_view(
    doc: *const Document,
    index: usize,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        let value = document(doc)?
            .entities()
            .nth(index)
            .ok_or_else(|| not_found("entity index is out of range"))?;
        *out = view(value);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_object_at_view(
    doc: *const Document,
    index: usize,
    out_handle: *mut u64,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        let handle = output(out_handle)?;
        let (h, value) = document(doc)?
            .objects
            .iter()
            .nth(index)
            .ok_or_else(|| not_found("object index is out of range"))?;
        *handle = h.value();
        *out = view(value);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_audit(
    doc: *const Document,
    out: *mut *mut NativeResult,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        result_out(ResultValue::Audit(document(doc)?.audit()), out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_repair(
    doc: *mut Document,
    out: *mut *mut NativeResult,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        result_out(ResultValue::Repair(document_mut(doc)?.repair()), out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_states(
    doc: *const Document,
    out: *mut *mut NativeResult,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        result_out(ResultValue::LayerStates(document(doc)?.layer_states()), out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_state_capture(
    doc: *mut Document,
    name: *const c_char,
    description: *const c_char,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        let name = input_str(name)?;
        let description = input_str(description)?;
        if name.is_empty() {
            return Err(invalid("layer state name is empty"));
        }
        *out = document_mut(doc)?
            .capture_layer_state(name, description)
            .value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_state_restore(
    doc: *mut Document,
    name: *const c_char,
    out_count: *mut usize,
) -> Status {
    ffi_call(|| {
        *output(out_count)? = document_mut(doc)?
            .restore_layer_state(input_str(name)?)
            .ok_or_else(|| not_found("layer state was not found"))?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_state_rename(
    doc: *mut Document,
    old_name: *const c_char,
    new_name: *const c_char,
) -> Status {
    ffi_call(|| {
        let old_name = input_str(old_name)?;
        let new_name = input_str(new_name)?;
        if new_name.is_empty() {
            return Err(invalid("layer state name is empty"));
        }
        let doc = document_mut(doc)?;
        if doc.layer_state(old_name).is_none() {
            return Err(not_found("layer state was not found"));
        }
        if !doc.rename_layer_state(old_name, new_name) {
            return Err(invalid("layer state rename failed"));
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_state_remove(doc: *mut Document, name: *const c_char) -> Status {
    ffi_call(|| {
        if !document_mut(doc)?.delete_layer_state(input_str(name)?) {
            return Err(not_found("layer state was not found"));
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_export_block(
    doc: *const Document,
    handle: u64,
    out: *mut *mut NativeResult,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let graph = document(doc)?
            .export_block_graph(Handle::new(handle))
            .ok_or_else(|| not_found("block record was not found"))?;
        result_out(ResultValue::BlockGraph(graph), out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_solid_history(
    doc: *const Document,
    handle: u64,
    mode: i32,
    out: *mut *mut NativeResult,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let doc = document(doc)?;
        let handle = Handle::new(handle);
        let result = match mode {
            0 => doc
                .solid_history_graph(handle)
                .map(ResultValue::SolidHistory),
            1 => doc
                .solid_history_tree(handle)
                .map(|v| ResultValue::SolidHistoryTree(Box::new(v))),
            2 => doc
                .solid_history_operations(handle)
                .map(ResultValue::SolidHistoryOperations),
            _ => return Err(invalid("unknown solid history mode")),
        }
        .ok_or_else(|| not_found("solid history was not found"))?;
        result_out(result, out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_dynamic_block_properties(
    doc: *const Document,
    definition: u64,
    insert: u64,
    out: *mut *mut NativeResult,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        let values = document(doc)?
            .dynamic_block_properties(
                Handle::new(definition),
                (insert != 0).then(|| Handle::new(insert)),
            )
            .map_err(invalid)?;
        result_out(ResultValue::DynamicProperties(values), out)
    })
}

impl<T: NativeView + acadrust::tables::TableEntry> NativeView for &T {
    fn kind(&self) -> i32 {
        (**self).kind()
    }
    fn len(&self) -> usize {
        (**self).len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        (**self).child(index)
    }
}

impl NativeView for acadrust::notification::NotificationCollection {
    fn kind(&self) -> i32 {
        AR_VALUE_SEQUENCE
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        self.iter()
            .nth(index)
            .map(|v| (StringView::new(""), view(v)))
    }
    fn number(&self) -> Option<i64> {
        Some(self.omitted_count() as i64)
    }
}

impl NativeView for LayerStateMask {
    fn kind(&self) -> i32 {
        AR_VALUE_ENUM
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}

impl NativeView for acadrust::BlockGraph {
    fn kind(&self) -> i32 {
        AR_VALUE_OPAQUE
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HandleMapping {
    pub source: u64,
    pub target: u64,
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_import_block_graph(
    doc: *mut Document,
    graph: *const NativeResult,
    record: u64,
    mappings: *const HandleMapping,
    count: usize,
) -> Status {
    ffi_call(|| {
        let graph = graph
            .as_ref()
            .ok_or_else(|| invalid("block graph is NULL"))?;
        let ResultValue::BlockGraph(graph) = &graph.value else {
            return Err(type_mismatch("result is not a block graph"));
        };
        if count > isize::MAX as usize / std::mem::size_of::<HandleMapping>()
            || (count != 0 && mappings.is_null())
        {
            return Err(invalid("invalid handle mappings"));
        }
        let mappings = if count == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(mappings, count)
        };
        let map = mappings
            .iter()
            .map(|v| (Handle::new(v.source), Handle::new(v.target)))
            .collect();
        let doc = document_mut(doc)?;
        if !doc.block_records.iter().any(|b| b.handle.value() == record) {
            return Err(not_found("destination block record was not found"));
        }
        doc.import_block_graph(graph, Handle::new(record), &map);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_move_to_block_handle(
    doc: *mut Document,
    entity: u64,
    block: u64,
) -> Status {
    ffi_call(|| {
        crate::database_mutations::move_entity(
            document_mut(doc)?,
            Handle::new(entity),
            Handle::new(block),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_rename(
    doc: *mut Document,
    old_name: *const c_char,
    new_name: *const c_char,
) -> Status {
    ffi_call(|| {
        document_mut(doc)?
            .rename_layer(input_str(old_name)?, input_str(new_name)?)
            .map_err(invalid)?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_allocate_handle(doc: *mut Document, out: *mut u64) -> Status {
    ffi_call(|| {
        *output(out)? = document_mut(doc)?.allocate_handle().value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_resolve_references(doc: *mut Document) -> Status {
    ffi_call(|| {
        let doc = document_mut(doc)?;
        doc.resync_table_keys();
        doc.assign_table_entry_handles();
        doc.resolve_references();
        doc.resolve_xrecord_names();
        doc.resolve_xrecord_backed_properties();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_has_version_locked_data(
    doc: *const Document,
    code: u32,
    out: *mut u8,
) -> Status {
    ffi_call(|| {
        *output(out)? = u8::from(document(doc)?.has_version_locked_data(version(code)?));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_drawing_variable(
    doc: *const Document,
    name: *const c_char,
    out: *mut StringView,
) -> Status {
    ffi_call(|| {
        *output(out)? = StringView::new(
            document(doc)?
                .drawing_variable(input_str(name)?)
                .ok_or_else(|| not_found("drawing string variable was not found"))?,
        );
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_set_drawing_variable(
    doc: *mut Document,
    name: *const c_char,
    value: *const c_char,
) -> Status {
    ffi_call(|| {
        if !document_mut(doc)?.set_drawing_variable(input_str(name)?, input_str(value)?) {
            return Err(invalid(
                "drawing variable name is empty or its dictionary has conflicting object types",
            ));
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_extension_dictionary(
    doc: *mut Document,
    owner: u64,
    create: u8,
    out: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        if create > 1 {
            return Err(invalid("create must be 0 or 1"));
        }
        let doc = document_mut(doc)?;
        let owner = Handle::new(owner);
        if doc.semantic_inventory_v1().resolve(owner).is_none() {
            return Err(not_found("dictionary owner was not found"));
        }
        *out = match doc.extension_dictionary_handle(owner) {
            Some(h) => h,
            None if create == 1 => doc.ensure_extension_dictionary(owner),
            None => return Err(not_found("extension dictionary was not found")),
        }
        .value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_xrecord_view(
    doc: *const Document,
    owner: u64,
    key: *const c_char,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        *output(out)? = view(
            document(doc)?
                .xrecord(Handle::new(owner), input_str(key)?)
                .ok_or_else(|| not_found("XRecord was not found"))?,
        );
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_document_ensure_xrecord(
    doc: *mut Document,
    owner: u64,
    key: *const c_char,
    out: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        let key = input_str(key)?;
        if key.is_empty() {
            return Err(invalid("XRecord key is empty"));
        }
        let doc = document_mut(doc)?;
        let owner = Handle::new(owner);
        if doc.semantic_inventory_v1().resolve(owner).is_none() {
            return Err(not_found("XRecord owner was not found"));
        }
        *out = doc.ensure_xrecord(owner, key).value();
        Ok(())
    })
}
