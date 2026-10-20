//! Detached native entity construction for ergonomic C callers.
use crate::*;
use acadrust::EntityType;
use std::ffi::c_char;

pub struct EntityBuilder {
    pub(crate) kind: String,
    pub(crate) value: EntityType,
}

pub const AR_ENTITY_EDIT_WRITE: i32 = 0;
pub const AR_ENTITY_EDIT_RESIZE: i32 = 1;
pub const AR_ENTITY_EDIT_SELECT: i32 = 2;
pub const AR_ENTITY_EDIT_OPTION: i32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EntityEdit {
    pub kind: i32,
    pub path: *const c_char,
    pub value: NativeWrite,
    pub length: usize,
    pub variant: *const c_char,
    pub enabled: u8,
}

unsafe fn apply_edits(
    value: &mut EntityType,
    edits: *const EntityEdit,
    count: usize,
) -> FfiResult<()> {
    if count > isize::MAX as usize / std::mem::size_of::<EntityEdit>()
        || (count > 0 && edits.is_null())
    {
        return Err(invalid("invalid entity edit array"));
    }
    let edits = if count == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(edits, count)
    };
    for edit in edits {
        let path = input_str(edit.path)?;
        match edit.kind {
            AR_ENTITY_EDIT_WRITE => {
                let staged = crate::mutations::stage_write(&edit.value)?;
                crate::mutations::edit_native_value(
                    value,
                    path,
                    crate::mutations::Edit::Write(staged.input()),
                )?;
            }
            AR_ENTITY_EDIT_RESIZE => crate::mutations::edit_native_value(
                value,
                path,
                crate::mutations::Edit::Resize(edit.length),
            )?,
            AR_ENTITY_EDIT_SELECT => crate::mutations::edit_native_value(
                value,
                path,
                crate::mutations::Edit::Select(input_str(edit.variant)?),
            )?,
            AR_ENTITY_EDIT_OPTION => {
                if edit.enabled > 1 {
                    return Err(invalid("optional enabled flag must be 0 or 1"));
                }
                crate::mutations::edit_native_value(
                    value,
                    path,
                    crate::mutations::Edit::Option(edit.enabled == 1),
                )?;
            }
            _ => return Err(invalid("unknown entity edit operation")),
        }
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_apply(
    value: *mut EntityBuilder,
    edits: *const EntityEdit,
    count: usize,
) -> Status {
    ffi_call(|| {
        let value = builder(value)?;
        let mut candidate = value.value.clone();
        apply_edits(&mut candidate, edits, count)?;
        value.value = candidate;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_create(
    doc: *mut Document,
    kind: *const c_char,
    edits: *const EntityEdit,
    count: usize,
    out: *mut u64,
) -> Status {
    ffi_call(|| {
        *output(out)? = 0;
        let kind = input_str(kind)?.to_ascii_uppercase();
        let mut value = crate::entity_features::make_entity(&kind)?;
        apply_edits(&mut value, edits, count)?;
        let mut value = EntityBuilder { kind, value };
        let doc = document_mut(doc)?;
        crate::entities::validate_entity(doc, &value.value)?;
        crate::entities::clear_identity(&mut value.value);
        *output(out)? = doc.add_entity(value.value)?.value();
        Ok(())
    })
}

unsafe fn builder<'a>(value: *mut EntityBuilder) -> FfiResult<&'a mut EntityBuilder> {
    value
        .as_mut()
        .ok_or_else(|| invalid("entity builder is NULL"))
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_new(
    kind: *const c_char,
    out: *mut *mut EntityBuilder,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        let kind = input_str(kind)?.to_ascii_uppercase();
        let value = crate::entity_features::make_entity(&kind)?;
        *out = Box::into_raw(Box::new(EntityBuilder { kind, value }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_free(value: *mut EntityBuilder) {
    if !value.is_null() {
        drop(Box::from_raw(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_view(
    value: *const EntityBuilder,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let value = value
            .as_ref()
            .ok_or_else(|| invalid("entity builder is NULL"))?;
        *output(out)? = crate::views::view(&value.value);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_kind(
    value: *const EntityBuilder,
    out: *mut StringView,
) -> Status {
    ffi_call(|| {
        let value = value
            .as_ref()
            .ok_or_else(|| invalid("entity builder is NULL"))?;
        *output(out)? = StringView::new(&value.kind);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_write(
    value: *mut EntityBuilder,
    path: *const c_char,
    input: *const NativeWrite,
) -> Status {
    ffi_call(|| {
        let staged = crate::mutations::stage_write(input)?;
        let path = input_str(path)?.to_owned();
        let value = builder(value)?;
        crate::mutations::edit_native_value(
            &mut value.value,
            &path,
            crate::mutations::Edit::Write(staged.input()),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_resize(
    value: *mut EntityBuilder,
    path: *const c_char,
    len: usize,
) -> Status {
    ffi_call(|| {
        let value = builder(value)?;
        crate::mutations::edit_native_value(
            &mut value.value,
            input_str(path)?,
            crate::mutations::Edit::Resize(len),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_select(
    value: *mut EntityBuilder,
    path: *const c_char,
    variant: *const c_char,
) -> Status {
    ffi_call(|| {
        let value = builder(value)?;
        crate::mutations::edit_native_value(
            &mut value.value,
            input_str(path)?,
            crate::mutations::Edit::Select(input_str(variant)?),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_option(
    value: *mut EntityBuilder,
    path: *const c_char,
    enabled: u8,
) -> Status {
    ffi_call(|| {
        if enabled > 1 {
            return Err(invalid("optional enabled flag must be 0 or 1"));
        }
        let value = builder(value)?;
        crate::mutations::edit_native_value(
            &mut value.value,
            input_str(path)?,
            crate::mutations::Edit::Option(enabled == 1),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_f64(
    value: *mut EntityBuilder,
    path: *const c_char,
    number: f64,
) -> Status {
    ar_entity_builder_write(
        value,
        path,
        &NativeWrite {
            kind: AR_VALUE_F64,
            data: std::ptr::from_ref(&number).cast(),
            len: 1,
        },
    )
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_i64(
    value: *mut EntityBuilder,
    path: *const c_char,
    number: i64,
) -> Status {
    ffi_call(|| {
        crate::mutations::edit_native_value(
            &mut builder(value)?.value,
            input_str(path)?,
            crate::mutations::Edit::Integer(number),
        )
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_u64(
    value: *mut EntityBuilder,
    path: *const c_char,
    number: u64,
) -> Status {
    ffi_call(|| {
        crate::mutations::edit_native_value(
            &mut builder(value)?.value,
            input_str(path)?,
            crate::mutations::Edit::Unsigned(number),
        )
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_bool(
    value: *mut EntityBuilder,
    path: *const c_char,
    enabled: u8,
) -> Status {
    ar_entity_builder_write(
        value,
        path,
        &NativeWrite {
            kind: AR_VALUE_BOOL,
            data: std::ptr::from_ref(&enabled).cast(),
            len: 1,
        },
    )
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_text(
    value: *mut EntityBuilder,
    path: *const c_char,
    text: *const c_char,
) -> Status {
    ffi_call(|| {
        let text = input_str(text)?.to_owned();
        let input = NativeWrite {
            kind: AR_VALUE_STRING,
            data: text.as_ptr().cast(),
            len: text.len(),
        };
        let staged = crate::mutations::stage_write(&input)?;
        crate::mutations::edit_native_value(
            &mut builder(value)?.value,
            input_str(path)?,
            crate::mutations::Edit::Write(staged.input()),
        )
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_vec3(
    value: *mut EntityBuilder,
    path: *const c_char,
    vector: Vector3,
) -> Status {
    ffi_call(|| {
        crate::mutations::edit_native_value(
            &mut builder(value)?.value,
            input_str(path)?,
            crate::mutations::Edit::Vector(vector),
        )
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_set_vec3s(
    value: *mut EntityBuilder,
    path: *const c_char,
    vectors: *const Vector3,
    count: usize,
) -> Status {
    ffi_call(|| {
        if count > isize::MAX as usize / std::mem::size_of::<Vector3>()
            || (count > 0 && vectors.is_null())
        {
            return Err(invalid("invalid vector array"));
        }
        let values = if count == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(vectors, count).to_vec()
        };
        crate::mutations::edit_native_value(
            &mut builder(value)?.value,
            input_str(path)?,
            crate::mutations::Edit::Vectors(&values),
        )
    })
}

enum Placement<'a> {
    Model,
    Block(&'a str),
    Layout(&'a str),
}

unsafe fn add_builder(
    doc: *mut Document,
    value: *mut *mut EntityBuilder,
    placement: Placement<'_>,
    out: *mut u64,
) -> FfiResult<()> {
    let out = output(out)?;
    *out = 0;
    let slot = output(value)?;
    let builder = builder(*slot)?;
    let doc = document_mut(doc)?;
    crate::entities::validate_entity(doc, &builder.value)?;
    let target = match placement {
        Placement::Model => None,
        Placement::Block(name) => Some(
            doc.block_records
                .get(name)
                .ok_or_else(|| not_found("destination block record was not found"))?
                .handle,
        ),
        Placement::Layout(name) => {
            let owner = doc
                .objects
                .values()
                .find_map(|v| match v {
                    acadrust::objects::ObjectType::Layout(v)
                        if v.name.eq_ignore_ascii_case(name) =>
                    {
                        Some(v.block_record)
                    }
                    _ => None,
                })
                .ok_or_else(|| not_found("destination layout was not found"))?;
            if !doc.block_records.iter().any(|v| v.handle == owner) {
                return Err(not_found("layout block record was not found"));
            }
            Some(owner)
        }
    };
    if target.is_some()
        && matches!(
            builder.value,
            EntityType::AttributeEntity(_)
                | EntityType::Block(_)
                | EntityType::BlockEnd(_)
                | EntityType::Seqend(_)
        )
    {
        return Err(type_mismatch(
            "structural entities cannot be placed independently",
        ));
    }
    let reset = crate::entity_features::make_entity(&builder.kind)?;
    let mut entity = std::mem::replace(&mut builder.value, reset);
    crate::entities::clear_identity(&mut entity);
    if let Some(owner) = target {
        entity.common_mut().owner_handle = owner;
    }
    let handle = doc.add_entity(entity)?;
    if let Some(owner) = target {
        crate::database_mutations::move_entity(doc, handle, owner)?;
    }
    *out = handle.value();
    drop(Box::from_raw(*slot));
    *slot = ptr::null_mut();
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_add(
    doc: *mut Document,
    value: *mut *mut EntityBuilder,
    out: *mut u64,
) -> Status {
    ffi_call(|| add_builder(doc, value, Placement::Model, out))
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_add_to_block(
    doc: *mut Document,
    value: *mut *mut EntityBuilder,
    block: *const c_char,
    out: *mut u64,
) -> Status {
    ffi_call(|| add_builder(doc, value, Placement::Block(input_str(block)?), out))
}
#[no_mangle]
pub unsafe extern "C" fn ar_entity_builder_add_to_layout(
    doc: *mut Document,
    value: *mut *mut EntityBuilder,
    layout: *const c_char,
    out: *mut u64,
) -> Status {
    ffi_call(|| add_builder(doc, value, Placement::Layout(input_str(layout)?), out))
}
