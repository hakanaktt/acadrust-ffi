//! Native ACIS model ownership and upstream primitive builders.
use crate::*;
use acadrust::entities::AcisData;
use acadrust::{primitives, SabReader, SabWriter, SatDocument, SatParser, SatWriter};

pub struct AcisDocument {
    inner: SatDocument,
}

fn parse_error(error: impl std::fmt::Display) -> FfiError {
    FfiError {
        status: Status::ParseError,
        message: error.to_string(),
    }
}

unsafe fn acis<'a>(value: *const AcisDocument) -> FfiResult<&'a SatDocument> {
    value
        .as_ref()
        .map(|v| &v.inner)
        .ok_or_else(|| invalid("ACIS document is NULL"))
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_free(value: *mut AcisDocument) {
    if !value.is_null() {
        drop(Box::from_raw(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_view(value: *const AcisDocument, out: *mut ValueView) -> Status {
    ffi_call(|| {
        *output(out)? = crate::views::view(acis(value)?);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_parse_sat(
    text: *const c_char,
    out: *mut *mut AcisDocument,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        let inner = SatParser::parse(input_str(text)?).map_err(parse_error)?;
        *out = Box::into_raw(Box::new(AcisDocument { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_parse_sab(
    bytes: *const u8,
    len: usize,
    out: *mut *mut AcisDocument,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        if bytes.is_null() || len == 0 || len > isize::MAX as usize {
            return Err(invalid("invalid SAB bytes"));
        }
        let inner = SabReader::read(std::slice::from_raw_parts(bytes, len)).map_err(parse_error)?;
        *out = Box::into_raw(Box::new(AcisDocument { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_write_sat(
    value: *const AcisDocument,
    out: *mut *mut c_char,
) -> Status {
    ffi_call(|| {
        *output(out)? = ptr::null_mut();
        store_string(&SatWriter::write(acis(value)?), out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_write_sab(value: *const AcisDocument, out: *mut Buffer) -> Status {
    ffi_call(|| {
        *output(out)? = Buffer::default();
        store_buffer(SabWriter::write(acis(value)?), out)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_acis_primitive(
    kind: i32,
    origin: Vector3,
    size: Vector3,
    out: *mut *mut AcisDocument,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        if [origin.x, origin.y, origin.z, size.x, size.y, size.z]
            .iter()
            .any(|v| !v.is_finite())
        {
            return Err(invalid("primitive coordinates and sizes must be finite"));
        }
        let valid = match kind {
            0 | 1 => size.x > 0.0 && size.y > 0.0 && size.z > 0.0,
            2..=4 => size.x > 0.0 && size.z > 0.0,
            5 => size.x > 0.0,
            6 => size.x > size.y && size.y > 0.0,
            _ => return Err(invalid("unknown ACIS primitive kind")),
        };
        if !valid {
            return Err(invalid(
                "primitive sizes must be positive; torus major radius must exceed minor radius",
            ));
        }
        let p = [origin.x, origin.y, origin.z];
        let inner = match kind {
            0 => primitives::build_box(p, size.x, size.y, size.z),
            1 => primitives::build_wedge(p, size.x, size.y, size.z),
            2 => primitives::build_pyramid(p, size.x, size.z),
            3 => primitives::build_cylinder(p, size.x, size.z),
            4 => primitives::build_cone(p, size.x, size.z),
            5 => primitives::build_sphere(p, size.x),
            6 => primitives::build_torus(p, size.x, size.y),
            _ => unreachable!(),
        };
        *out = Box::into_raw(Box::new(AcisDocument { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_parse_acis(
    doc: *const Document,
    handle: u64,
    out: *mut *mut AcisDocument,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        *out = ptr::null_mut();
        let entity = document(doc)?
            .get_entity(acadrust::Handle::new(handle))
            .ok_or_else(|| not_found("entity was not found"))?;
        let data = match entity {
            acadrust::EntityType::Solid3D(v) => &v.acis_data,
            acadrust::EntityType::Region(v) => &v.acis_data,
            acadrust::EntityType::Body(v) => &v.acis_data,
            acadrust::EntityType::Surface(v) => &v.acis_data,
            _ => return Err(type_mismatch("entity has no ACIS model")),
        };
        let inner = data
            .parse()
            .ok_or_else(|| parse_error("ACIS model could not be parsed"))?;
        *out = Box::into_raw(Box::new(AcisDocument { inner }));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_set_acis(
    doc: *mut Document,
    handle: u64,
    value: *const AcisDocument,
) -> Status {
    ffi_call(|| {
        let value = acis(value)?;
        let entity = document_mut(doc)?
            .get_entity_mut(acadrust::Handle::new(handle))
            .ok_or_else(|| not_found("entity was not found"))?;
        let data = match entity {
            acadrust::EntityType::Solid3D(v) => &mut v.acis_data,
            acadrust::EntityType::Region(v) => &mut v.acis_data,
            acadrust::EntityType::Body(v) => &mut v.acis_data,
            acadrust::EntityType::Surface(v) => &mut v.acis_data,
            _ => return Err(type_mismatch("entity has no ACIS model")),
        };
        *data = AcisData::from_sat_document(value);
        entity.common_mut().raw_record = None;
        Ok(())
    })
}
