//! Entity access through copied C-compatible geometry values.
//!
//! # Safety
//! Every non-null pointer must be aligned, point to initialized memory of the
//! declared type, and remain valid for the call. Output pointers must be
//! writable. Document pointers must come from this library, must not have been
//! freed, and must not be accessed concurrently with a mutating call. Buffers
//! must have the declared capacity. Returned strings belong to the caller and
//! must be released with `ar_string_free`.

use std::ffi::c_char;
use std::mem::discriminant;

use acadrust::entities::LwVertex as NativeLwVertex;
use acadrust::{CadDocument, Color, EntityType, Handle, Vector3 as NativeVector3};

use crate::{
    document, document_mut, ffi_call, input_str, invalid, not_found, output, store_string,
    type_mismatch, Document, FfiError, FfiResult, Status,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Line {
    pub start: Vector3,
    pub end: Vector3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle {
    pub center: Vector3,
    pub radius: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Arc {
    pub center: Vector3,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub position: Vector3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LwVertex {
    pub x: f64,
    pub y: f64,
    pub bulge: f64,
    pub start_width: f64,
    pub end_width: f64,
}

impl From<Vector3> for NativeVector3 {
    fn from(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<NativeVector3> for Vector3 {
    fn from(value: NativeVector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

fn finite(value: f64, name: &str) -> FfiResult<()> {
    if !value.is_finite() {
        return Err(invalid(format!("{name} must be finite")));
    }
    Ok(())
}

fn vector(value: Vector3) -> FfiResult<()> {
    finite(value.x, "x")?;
    finite(value.y, "y")?;
    finite(value.z, "z")
}

fn radius(value: f64) -> FfiResult<()> {
    finite(value, "radius")?;
    if value <= 0.0 {
        return Err(invalid("radius must be positive"));
    }
    Ok(())
}

fn line(value: Line) -> FfiResult<()> {
    vector(value.start)?;
    vector(value.end)
}

fn circle(value: Circle) -> FfiResult<()> {
    vector(value.center)?;
    radius(value.radius)
}

fn arc(value: Arc) -> FfiResult<()> {
    vector(value.center)?;
    radius(value.radius)?;
    finite(value.start_angle, "start angle")?;
    finite(value.end_angle, "end angle")
}

fn point(value: Point) -> FfiResult<()> {
    vector(value.position)
}

unsafe fn input_value<T: Copy>(ptr: *const T) -> FfiResult<T> {
    ptr.as_ref()
        .copied()
        .ok_or_else(|| invalid("input pointer is null"))
}

fn entity(doc: &CadDocument, handle: u64) -> FfiResult<&EntityType> {
    doc.get_entity(Handle::new(handle))
        .ok_or_else(|| not_found(format!("entity {handle} was not found")))
}

fn entity_mut(doc: &mut CadDocument, handle: u64) -> FfiResult<&mut EntityType> {
    doc.get_entity_mut(Handle::new(handle))
        .ok_or_else(|| not_found(format!("entity {handle} was not found")))
}

fn buffer_too_small(required: usize) -> FfiError {
    FfiError {
        status: Status::BufferTooSmall,
        message: format!("buffer requires {required} elements"),
    }
}

unsafe fn add_typed<T: Copy>(
    doc: *mut Document,
    value: *const T,
    out_handle: *mut u64,
    validate: impl FnOnce(T) -> FfiResult<()>,
    make: impl FnOnce(T) -> EntityType,
) -> FfiResult<()> {
    let value = input_value(value)?;
    let out_handle = output(out_handle)?;
    let doc = document_mut(doc)?;
    validate(value)?;
    *out_handle = doc.add_entity(make(value))?.value();
    Ok(())
}

unsafe fn get_typed<T>(
    doc: *const Document,
    handle: u64,
    out_value: *mut T,
    extract: impl FnOnce(&EntityType) -> FfiResult<T>,
) -> FfiResult<()> {
    let out_value = output(out_value)?;
    *out_value = extract(entity(document(doc)?, handle)?)?;
    Ok(())
}

unsafe fn set_typed<T: Copy>(
    doc: *mut Document,
    handle: u64,
    value: *const T,
    validate: impl FnOnce(T) -> FfiResult<()>,
    update: impl FnOnce(&mut EntityType, T) -> FfiResult<()>,
) -> FfiResult<()> {
    let value = input_value(value)?;
    validate(value)?;
    update(entity_mut(document_mut(doc)?, handle)?, value)
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_count(doc: *const Document, out_count: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_count)? = document(doc)?.entity_count();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_handles(
    doc: *const Document,
    handles: *mut u64,
    capacity: usize,
    required: *mut usize,
) -> Status {
    ffi_call(|| {
        let doc = document(doc)?;
        let count = doc.entity_count();
        *output(required)? = count;
        if handles.is_null() && capacity == 0 {
            return Ok(());
        }
        if handles.is_null() {
            return Err(invalid("handles pointer is null"));
        }
        if capacity < count {
            return Err(buffer_too_small(count));
        }
        for (index, value) in doc.entities().enumerate() {
            handles.add(index).write(value.common().handle.value());
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_type(
    doc: *const Document,
    handle: u64,
    out_type: *mut *mut c_char,
) -> Status {
    ffi_call(|| {
        let value = entity(document(doc)?, handle)?.as_entity().entity_type();
        store_string(value, out_type)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_get_json(
    doc: *const Document,
    handle: u64,
    out_json: *mut *mut c_char,
) -> Status {
    ffi_call(|| {
        let value = serde_json::to_string(entity(document(doc)?, handle)?)?;
        store_string(&value, out_json)
    })
}

fn validate_json_entity(doc: &CadDocument, value: &EntityType) -> FfiResult<()> {
    if !doc.layers.contains(&value.common().layer) {
        return Err(not_found(format!(
            "layer {:?} was not found",
            value.common().layer
        )));
    }
    match value {
        EntityType::Line(value) => line(Line {
            start: value.start.into(),
            end: value.end.into(),
        }),
        EntityType::Circle(value) => circle(Circle {
            center: value.center.into(),
            radius: value.radius,
        }),
        EntityType::Arc(value) => arc(Arc {
            center: value.center.into(),
            radius: value.radius,
            start_angle: value.start_angle,
            end_angle: value.end_angle,
        }),
        EntityType::Point(value) => point(Point {
            position: value.location.into(),
        }),
        EntityType::LwPolyline(value) => {
            finite(value.elevation, "elevation")?;
            for vertex in &value.vertices {
                validate_vertex(LwVertex {
                    x: vertex.location.x,
                    y: vertex.location.y,
                    bulge: vertex.bulge,
                    start_width: vertex.start_width,
                    end_width: vertex.end_width,
                })?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn clear_common_identity(value: &mut acadrust::entities::EntityCommon) {
    value.handle = Handle::NULL;
    value.owner_handle = Handle::NULL;
    value.entity_mode = None;
    value.reactors.clear();
    value.xdictionary_handle = None;
}

// Clear actual identities, without changing references to existing document
// objects such as block definitions or handles stored in extended data.
fn clear_identity(value: &mut EntityType) {
    clear_common_identity(value.common_mut());
    match value {
        EntityType::Insert(value) => {
            value.seqend_handle = None;
            for attribute in &mut value.attributes {
                clear_common_identity(&mut attribute.common);
            }
        }
        EntityType::Polyline3D(value) => {
            for vertex in &mut value.vertices {
                vertex.handle = Handle::NULL;
            }
        }
        EntityType::PolyfaceMesh(value) => {
            value.seqend_handle = None;
            for vertex in &mut value.vertices {
                clear_common_identity(&mut vertex.common);
            }
            for face in &mut value.faces {
                clear_common_identity(&mut face.common);
            }
        }
        EntityType::PolygonMesh(value) => {
            for vertex in &mut value.vertices {
                clear_common_identity(&mut vertex.common);
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_add_json(
    doc: *mut Document,
    json: *const c_char,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let mut value: EntityType = serde_json::from_str(input_str(json)?)?;
        clear_identity(&mut value);
        let out_handle = output(out_handle)?;
        let doc = document_mut(doc)?;
        validate_json_entity(doc, &value)?;
        *out_handle = doc.add_entity(value)?.value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_set_json(
    doc: *mut Document,
    handle: u64,
    json: *const c_char,
) -> Status {
    ffi_call(|| {
        let mut value: EntityType = serde_json::from_str(input_str(json)?)?;
        let doc = document_mut(doc)?;
        let previous = entity(doc, handle)?;
        if discriminant(previous) != discriminant(&value) {
            return Err(type_mismatch(
                "JSON replacement must have the same entity variant",
            ));
        }
        validate_json_entity(doc, &value)?;
        value.preserve_storage_data_from(previous);
        value.common_mut().handle = previous.common().handle;
        value.common_mut().owner_handle = previous.common().owner_handle;
        value.common_mut().raw_record = None;
        *entity_mut(doc, handle)? = value;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_remove(doc: *mut Document, handle: u64) -> Status {
    ffi_call(|| {
        let doc = document_mut(doc)?;
        entity(doc, handle)?;
        doc.remove_entity(Handle::new(handle));
        // Upstream removal keeps membership for undo. C callers permanently
        // remove entities, so remove the corresponding block references too.
        for block in doc.block_records.iter_mut() {
            block.entity_handles.retain(|value| value.value() != handle);
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_line_add(
    doc: *mut Document,
    value: *const Line,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        add_typed(doc, value, out_handle, line, |value| {
            EntityType::Line(acadrust::Line::from_points(
                value.start.into(),
                value.end.into(),
            ))
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_line_get(
    doc: *const Document,
    handle: u64,
    out_value: *mut Line,
) -> Status {
    ffi_call(|| {
        get_typed(doc, handle, out_value, |value| match value {
            EntityType::Line(value) => Ok(Line {
                start: value.start.into(),
                end: value.end.into(),
            }),
            _ => Err(type_mismatch("entity is not a LINE")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_line_set(
    doc: *mut Document,
    handle: u64,
    value: *const Line,
) -> Status {
    ffi_call(|| {
        set_typed(doc, handle, value, line, |entity, value| match entity {
            EntityType::Line(entity) => {
                entity.start = value.start.into();
                entity.end = value.end.into();
                Ok(())
            }
            _ => Err(type_mismatch("entity is not a LINE")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_circle_add(
    doc: *mut Document,
    value: *const Circle,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        add_typed(doc, value, out_handle, circle, |value| {
            EntityType::Circle(acadrust::Circle::from_center_radius(
                value.center.into(),
                value.radius,
            ))
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_circle_get(
    doc: *const Document,
    handle: u64,
    out_value: *mut Circle,
) -> Status {
    ffi_call(|| {
        get_typed(doc, handle, out_value, |value| match value {
            EntityType::Circle(value) => Ok(Circle {
                center: value.center.into(),
                radius: value.radius,
            }),
            _ => Err(type_mismatch("entity is not a CIRCLE")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_circle_set(
    doc: *mut Document,
    handle: u64,
    value: *const Circle,
) -> Status {
    ffi_call(|| {
        set_typed(doc, handle, value, circle, |entity, value| match entity {
            EntityType::Circle(entity) => {
                entity.center = value.center.into();
                entity.radius = value.radius;
                Ok(())
            }
            _ => Err(type_mismatch("entity is not a CIRCLE")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_arc_add(
    doc: *mut Document,
    value: *const Arc,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        add_typed(doc, value, out_handle, arc, |value| {
            EntityType::Arc(acadrust::Arc::from_center_radius_angles(
                value.center.into(),
                value.radius,
                value.start_angle,
                value.end_angle,
            ))
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_arc_get(
    doc: *const Document,
    handle: u64,
    out_value: *mut Arc,
) -> Status {
    ffi_call(|| {
        get_typed(doc, handle, out_value, |value| match value {
            EntityType::Arc(value) => Ok(Arc {
                center: value.center.into(),
                radius: value.radius,
                start_angle: value.start_angle,
                end_angle: value.end_angle,
            }),
            _ => Err(type_mismatch("entity is not an ARC")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_arc_set(doc: *mut Document, handle: u64, value: *const Arc) -> Status {
    ffi_call(|| {
        set_typed(doc, handle, value, arc, |entity, value| match entity {
            EntityType::Arc(entity) => {
                entity.center = value.center.into();
                entity.radius = value.radius;
                entity.start_angle = value.start_angle;
                entity.end_angle = value.end_angle;
                Ok(())
            }
            _ => Err(type_mismatch("entity is not an ARC")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_point_add(
    doc: *mut Document,
    value: *const Point,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        add_typed(doc, value, out_handle, point, |value| {
            EntityType::Point(acadrust::Point::at(value.position.into()))
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_point_get(
    doc: *const Document,
    handle: u64,
    out_value: *mut Point,
) -> Status {
    ffi_call(|| {
        get_typed(doc, handle, out_value, |value| match value {
            EntityType::Point(value) => Ok(Point {
                position: value.location.into(),
            }),
            _ => Err(type_mismatch("entity is not a POINT")),
        })
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_point_set(
    doc: *mut Document,
    handle: u64,
    value: *const Point,
) -> Status {
    ffi_call(|| {
        set_typed(doc, handle, value, point, |entity, value| match entity {
            EntityType::Point(entity) => {
                entity.location = value.position.into();
                Ok(())
            }
            _ => Err(type_mismatch("entity is not a POINT")),
        })
    })
}

fn validate_vertex(vertex: LwVertex) -> FfiResult<()> {
    finite(vertex.x, "vertex x")?;
    finite(vertex.y, "vertex y")?;
    finite(vertex.bulge, "vertex bulge")?;
    finite(vertex.start_width, "vertex start width")?;
    finite(vertex.end_width, "vertex end width")?;
    if vertex.start_width < 0.0 || vertex.end_width < 0.0 {
        return Err(invalid("vertex widths must not be negative"));
    }
    Ok(())
}

unsafe fn polyline_vertices(
    vertices: *const LwVertex,
    count: usize,
    closed: u8,
    elevation: f64,
) -> FfiResult<Vec<NativeLwVertex>> {
    if closed > 1 {
        return Err(invalid("closed must be 0 or 1"));
    }
    finite(elevation, "elevation")?;
    if count > isize::MAX as usize / std::mem::size_of::<LwVertex>() {
        return Err(invalid("vertex count is too large"));
    }
    if count == 0 {
        return Ok(Vec::new());
    }
    if vertices.is_null() {
        return Err(invalid("vertices pointer is null"));
    }
    let vertices = std::slice::from_raw_parts(vertices, count);
    vertices
        .iter()
        .map(|value| {
            validate_vertex(*value)?;
            let mut vertex = NativeLwVertex::from_coords(value.x, value.y);
            vertex.bulge = value.bulge;
            vertex.start_width = value.start_width;
            vertex.end_width = value.end_width;
            Ok(vertex)
        })
        .collect()
}

#[no_mangle]
pub unsafe extern "C" fn ar_lwpolyline_add(
    doc: *mut Document,
    vertices: *const LwVertex,
    count: usize,
    closed: u8,
    elevation: f64,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let vertices = polyline_vertices(vertices, count, closed, elevation)?;
        let out_handle = output(out_handle)?;
        let doc = document_mut(doc)?;
        let mut value = acadrust::LwPolyline::new();
        value.vertices = vertices;
        value.is_closed = closed != 0;
        value.elevation = elevation;
        *out_handle = doc.add_entity(EntityType::LwPolyline(value))?.value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_lwpolyline_get(
    doc: *const Document,
    handle: u64,
    vertices: *mut LwVertex,
    capacity: usize,
    required: *mut usize,
    closed: *mut u8,
    elevation: *mut f64,
) -> Status {
    ffi_call(|| {
        let EntityType::LwPolyline(value) = entity(document(doc)?, handle)? else {
            return Err(type_mismatch("entity is not an LWPOLYLINE"));
        };
        let required = output(required)?;
        let closed = output(closed)?;
        let elevation = output(elevation)?;
        *required = value.vertices.len();
        *closed = u8::from(value.is_closed);
        *elevation = value.elevation;
        if vertices.is_null() && capacity == 0 {
            return Ok(());
        }
        if vertices.is_null() {
            return Err(invalid("vertices pointer is null"));
        }
        if capacity < value.vertices.len() {
            return Err(buffer_too_small(value.vertices.len()));
        }
        for (index, vertex) in value.vertices.iter().enumerate() {
            vertices.add(index).write(LwVertex {
                x: vertex.location.x,
                y: vertex.location.y,
                bulge: vertex.bulge,
                start_width: vertex.start_width,
                end_width: vertex.end_width,
            });
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_lwpolyline_set(
    doc: *mut Document,
    handle: u64,
    vertices: *const LwVertex,
    count: usize,
    closed: u8,
    elevation: f64,
) -> Status {
    ffi_call(|| {
        let mut vertices = polyline_vertices(vertices, count, closed, elevation)?;
        let EntityType::LwPolyline(value) = entity_mut(document_mut(doc)?, handle)? else {
            return Err(type_mismatch("entity is not an LWPOLYLINE"));
        };
        // Vertex identifiers belong to the drawing, not the copied C geometry.
        for (new, previous) in vertices.iter_mut().zip(&value.vertices) {
            new.vertex_id = previous.vertex_id;
        }
        value.vertices = vertices;
        value.is_closed = closed != 0;
        value.elevation = elevation;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_get_layer(
    doc: *const Document,
    handle: u64,
    out_layer: *mut *mut c_char,
) -> Status {
    ffi_call(|| store_string(&entity(document(doc)?, handle)?.common().layer, out_layer))
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_set_layer(
    doc: *mut Document,
    handle: u64,
    layer: *const c_char,
) -> Status {
    ffi_call(|| {
        let layer = input_str(layer)?;
        let doc = document_mut(doc)?;
        if !doc.layers.contains(layer) {
            return Err(not_found(format!("layer {layer:?} was not found")));
        }
        let value = entity_mut(doc, handle)?.common_mut();
        value.layer = layer.to_owned();
        value.layer_handle = None;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_get_color_index(
    doc: *const Document,
    handle: u64,
    out_index: *mut i16,
) -> Status {
    ffi_call(|| {
        let out_index = output(out_index)?;
        let color = entity(document(doc)?, handle)?.common().color;
        let index = color
            .index()
            .filter(|index| *index <= 256)
            .ok_or_else(|| type_mismatch("entity color is not an ACI color in 0..256"))?;
        *out_index = index as i16;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_set_color_index(
    doc: *mut Document,
    handle: u64,
    index: i16,
) -> Status {
    ffi_call(|| {
        if !(0..=256).contains(&index) {
            return Err(invalid("color index must be in 0..256"));
        }
        let value = entity_mut(document_mut(doc)?, handle)?.common_mut();
        value.color = Color::from_index(index);
        value.color_name = None;
        value.color_book_handle = None;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::ptr;

    fn drawing() -> Document {
        Document {
            inner: CadDocument::new(),
        }
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
    fn typed_geometry_edits_preserve_entity_metadata() {
        unsafe {
            let mut doc = drawing();
            let mut handle = 0;
            let initial = sample_line();
            assert_eq!(ar_line_add(&mut doc, &initial, &mut handle), Status::Ok);
            let common = doc
                .inner
                .get_entity_mut(Handle::new(handle))
                .unwrap()
                .common_mut();
            common.color = Color::GREEN;
            common.linetype_scale = 2.5;
            let owner = common.owner_handle;
            let replacement = Line {
                end: Vector3 {
                    x: 10.0,
                    ..initial.end
                },
                ..initial
            };
            assert_eq!(ar_line_set(&mut doc, handle, &replacement), Status::Ok);
            let mut result = Line::default();
            assert_eq!(ar_line_get(&doc, handle, &mut result), Status::Ok);
            assert_eq!(result, replacement);
            let common = doc.inner.get_entity(Handle::new(handle)).unwrap().common();
            assert_eq!(common.color, Color::GREEN);
            assert_eq!(common.linetype_scale, 2.5);
            assert_eq!(common.handle.value(), handle);
            assert_eq!(common.owner_handle, owner);
        }
    }

    #[test]
    fn handle_buffers_support_queries_and_leave_small_buffers_intact() {
        unsafe {
            let mut doc = drawing();
            let mut first = 0;
            let mut second = 0;
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), &mut first),
                Status::Ok
            );
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), &mut second),
                Status::Ok
            );
            let mut required = 0;
            assert_eq!(
                ar_entity_handles(&doc, ptr::null_mut(), 0, &mut required),
                Status::Ok
            );
            assert_eq!(required, 2);
            let mut handles = [999_u64; 2];
            assert_eq!(
                ar_entity_handles(&doc, handles.as_mut_ptr(), 1, &mut required),
                Status::BufferTooSmall
            );
            assert_eq!(handles, [999, 999]);
            assert_eq!(
                ar_entity_handles(&doc, handles.as_mut_ptr(), 2, &mut required),
                Status::Ok
            );
            assert_eq!(handles, [first, second]);
        }
    }

    #[test]
    fn invalid_geometry_and_null_outputs_do_not_add_entities() {
        unsafe {
            let mut doc = drawing();
            let mut handle = 77;
            let invalid_circle = Circle {
                center: Vector3::default(),
                radius: -1.0,
            };
            assert_eq!(
                ar_circle_add(&mut doc, &invalid_circle, &mut handle),
                Status::InvalidArgument
            );
            assert_eq!(handle, 77);
            assert_eq!(doc.inner.entity_count(), 0);
            let invalid_line = Line {
                start: Vector3 {
                    x: f64::NAN,
                    ..Vector3::default()
                },
                ..sample_line()
            };
            assert_eq!(
                ar_line_add(&mut doc, &invalid_line, &mut handle),
                Status::InvalidArgument
            );
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), ptr::null_mut()),
                Status::InvalidArgument
            );
            assert_eq!(doc.inner.entity_count(), 0);
            let mut count = 0;
            assert_eq!(
                ar_entity_count(ptr::null(), &mut count),
                Status::InvalidArgument
            );
        }
    }

    #[test]
    fn typed_getters_distinguish_wrong_types_and_missing_handles() {
        unsafe {
            let mut doc = drawing();
            let mut handle = 0;
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), &mut handle),
                Status::Ok
            );
            let mut circle = Circle::default();
            assert_eq!(
                ar_circle_get(&doc, handle, &mut circle),
                Status::TypeMismatch
            );
            assert_eq!(ar_circle_get(&doc, u64::MAX, &mut circle), Status::NotFound);
            assert_eq!(
                ar_line_get(&doc, handle, ptr::null_mut()),
                Status::InvalidArgument
            );
        }
    }

    #[test]
    fn json_add_allocates_a_new_identity_and_json_set_preserves_ownership() {
        unsafe {
            let mut doc = drawing();
            let mut first = 0;
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), &mut first),
                Status::Ok
            );
            let owner = doc
                .inner
                .get_entity(Handle::new(first))
                .unwrap()
                .common()
                .owner_handle;
            let mut json = ptr::null_mut();
            assert_eq!(ar_entity_get_json(&doc, first, &mut json), Status::Ok);
            let mut second = 0;
            assert_eq!(ar_entity_add_json(&mut doc, json, &mut second), Status::Ok);
            assert_ne!(first, second);
            let mut value: serde_json::Value =
                serde_json::from_slice(CStr::from_ptr(json).to_bytes()).unwrap();
            crate::ar_string_free(json);
            value["Line"]["common"]["handle"] = serde_json::json!(999);
            value["Line"]["common"]["owner_handle"] = serde_json::json!(888);
            value["Line"]["end"]["x"] = serde_json::json!(25.0);
            let json = CString::new(value.to_string()).unwrap();
            assert_eq!(
                ar_entity_set_json(&mut doc, first, json.as_ptr()),
                Status::Ok
            );
            let result = doc.inner.get_entity(Handle::new(first)).unwrap();
            assert_eq!(result.common().handle.value(), first);
            assert_eq!(result.common().owner_handle, owner);
            let EntityType::Line(result) = result else {
                panic!("expected line")
            };
            assert_eq!(result.end.x, 25.0);
            assert_eq!(doc.inner.entity_count(), 2);
        }
    }

    #[test]
    fn json_replacements_reject_variant_changes_and_missing_layers() {
        unsafe {
            let mut doc = drawing();
            let mut handle = 0;
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), &mut handle),
                Status::Ok
            );
            let circle = EntityType::Circle(acadrust::Circle::new());
            let json = CString::new(serde_json::to_string(&circle).unwrap()).unwrap();
            assert_eq!(
                ar_entity_set_json(&mut doc, handle, json.as_ptr()),
                Status::TypeMismatch
            );
            let mut line = EntityType::Line(acadrust::Line::new());
            line.common_mut().layer = "missing".to_owned();
            let json = CString::new(serde_json::to_string(&line).unwrap()).unwrap();
            assert_eq!(
                ar_entity_set_json(&mut doc, handle, json.as_ptr()),
                Status::NotFound
            );
            assert_eq!(doc.inner.entity_count(), 1);
        }
    }

    #[test]
    fn polyline_buffers_and_edits_preserve_unexposed_fields() {
        unsafe {
            let mut doc = drawing();
            let vertices = [
                LwVertex {
                    x: 1.0,
                    y: 2.0,
                    bulge: 0.5,
                    start_width: 0.2,
                    end_width: 0.4,
                },
                LwVertex {
                    x: 3.0,
                    y: 4.0,
                    ..LwVertex::default()
                },
            ];
            let mut handle = 0;
            assert_eq!(
                ar_lwpolyline_add(
                    &mut doc,
                    vertices.as_ptr(),
                    vertices.len(),
                    1,
                    7.0,
                    &mut handle
                ),
                Status::Ok
            );
            let mut required = 0;
            let mut closed = 0;
            let mut elevation = 0.0;
            assert_eq!(
                ar_lwpolyline_get(
                    &doc,
                    handle,
                    ptr::null_mut(),
                    0,
                    &mut required,
                    &mut closed,
                    &mut elevation
                ),
                Status::Ok
            );
            assert_eq!((required, closed, elevation), (2, 1, 7.0));
            let mut result = [LwVertex::default(); 2];
            assert_eq!(
                ar_lwpolyline_get(
                    &doc,
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
            assert_eq!(
                ar_lwpolyline_get(
                    &doc,
                    handle,
                    result.as_mut_ptr(),
                    2,
                    &mut required,
                    &mut closed,
                    &mut elevation
                ),
                Status::Ok
            );
            assert_eq!(result, vertices);
            let EntityType::LwPolyline(value) =
                doc.inner.get_entity_mut(Handle::new(handle)).unwrap()
            else {
                panic!("expected polyline")
            };
            value.vertices[0].vertex_id = 42;
            value.thickness = 5.0;
            assert_eq!(
                ar_lwpolyline_set(&mut doc, handle, vertices.as_ptr(), 2, 0, 8.0),
                Status::Ok
            );
            let EntityType::LwPolyline(value) = doc.inner.get_entity(Handle::new(handle)).unwrap()
            else {
                panic!("expected polyline")
            };
            assert_eq!(value.vertices[0].vertex_id, 42);
            assert_eq!(value.thickness, 5.0);
            assert!(!value.is_closed);
            assert_eq!(value.elevation, 8.0);
            assert_eq!(
                ar_lwpolyline_set(&mut doc, handle, vertices.as_ptr(), 2, 2, 8.0),
                Status::InvalidArgument
            );
        }
    }

    #[test]
    fn removal_clears_block_membership() {
        unsafe {
            let mut doc = drawing();
            let mut handle = 0;
            assert_eq!(
                ar_line_add(&mut doc, &sample_line(), &mut handle),
                Status::Ok
            );
            assert!(doc
                .inner
                .block_records
                .iter()
                .any(|block| block.entity_handles.contains(&Handle::new(handle))));
            assert_eq!(ar_entity_remove(&mut doc, handle), Status::Ok);
            assert_eq!(doc.inner.entity_count(), 0);
            assert!(!doc
                .inner
                .block_records
                .iter()
                .any(|block| block.entity_handles.contains(&Handle::new(handle))));
            assert_eq!(ar_entity_remove(&mut doc, handle), Status::NotFound);
        }
    }
}
