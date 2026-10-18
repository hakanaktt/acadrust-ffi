//! Layer-table access. Returned names must be freed with `ar_string_free`.

use std::ffi::c_char;
use std::ptr;

use acadrust::tables::Layer;
use acadrust::{CadDocument, Color};

use crate::{
    document, document_mut, ffi_call, input_str, invalid, not_found, output, store_string,
    type_mismatch, Document, FfiResult, Status,
};

pub const AR_LAYER_FROZEN: u32 = 1;
pub const AR_LAYER_LOCKED: u32 = 2;
pub const AR_LAYER_OFF: u32 = 4;
const LAYER_FLAGS: u32 = AR_LAYER_FROZEN | AR_LAYER_LOCKED | AR_LAYER_OFF;

fn color(index: i16) -> FfiResult<Color> {
    if !(1..=255).contains(&index) {
        return Err(invalid("layer color index must be in 1..255"));
    }
    Ok(Color::Index(index as u8))
}

fn layer<'a>(doc: &'a CadDocument, name: &str) -> FfiResult<&'a Layer> {
    doc.layers
        .get(name)
        .ok_or_else(|| not_found(format!("layer {name:?} was not found")))
}

fn layer_mut<'a>(doc: &'a mut CadDocument, name: &str) -> FfiResult<&'a mut Layer> {
    doc.layers
        .get_mut(name)
        .ok_or_else(|| not_found(format!("layer {name:?} was not found")))
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_count(doc: *const Document, out_count: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_count)? = document(doc)?.layers.len();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_name(
    doc: *const Document,
    index: usize,
    out_name: *mut *mut c_char,
) -> Status {
    ffi_call(|| {
        *output(out_name)? = ptr::null_mut();
        let layer = document(doc)?
            .layers
            .iter()
            .nth(index)
            .ok_or_else(|| not_found(format!("layer index {index} was not found")))?;
        store_string(&layer.name, out_name)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_add(
    doc: *mut Document,
    name: *const c_char,
    color_index: i16,
) -> Status {
    ffi_call(|| {
        let name = input_str(name)?;
        if name.is_empty() {
            return Err(invalid("layer name must not be empty"));
        }
        let color = color(color_index)?;
        let doc = document_mut(doc)?;
        if doc.layers.contains(name) {
            return Err(invalid(format!("layer {name:?} already exists")));
        }
        let mut layer = Layer::with_color(name, color);
        layer.handle = doc.allocate_handle();
        doc.layers.add(layer).map_err(invalid)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_get_color_index(
    doc: *const Document,
    name: *const c_char,
    out_index: *mut i16,
) -> Status {
    ffi_call(|| {
        let out_index = output(out_index)?;
        let layer = layer(document(doc)?, input_str(name)?)?;
        let index = layer
            .color
            .index()
            .filter(|index| (1..=255).contains(index))
            .ok_or_else(|| type_mismatch("layer color is not an ACI color in 1..255"))?;
        *out_index = index as i16;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_set_color_index(
    doc: *mut Document,
    name: *const c_char,
    color_index: i16,
) -> Status {
    ffi_call(|| {
        let color = color(color_index)?;
        let layer = layer_mut(document_mut(doc)?, input_str(name)?)?;
        layer.color = color;
        layer.color_name = None;
        layer.book_name = None;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_get_flags(
    doc: *const Document,
    name: *const c_char,
    out_flags: *mut u32,
) -> Status {
    ffi_call(|| {
        let out_flags = output(out_flags)?;
        let flags = &layer(document(doc)?, input_str(name)?)?.flags;
        *out_flags = (u32::from(flags.frozen) * AR_LAYER_FROZEN)
            | (u32::from(flags.locked) * AR_LAYER_LOCKED)
            | (u32::from(flags.off) * AR_LAYER_OFF);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_layer_set_flags(
    doc: *mut Document,
    name: *const c_char,
    flags: u32,
) -> Status {
    ffi_call(|| {
        if flags & !LAYER_FLAGS != 0 {
            return Err(invalid("unknown layer flag bits"));
        }
        let layer = layer_mut(document_mut(doc)?, input_str(name)?)?;
        layer.flags.frozen = flags & AR_LAYER_FROZEN != 0;
        layer.flags.locked = flags & AR_LAYER_LOCKED != 0;
        layer.flags.off = flags & AR_LAYER_OFF != 0;
        Ok(())
    })
}
