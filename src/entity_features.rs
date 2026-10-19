//! Complete entity construction and shared entity operations.

use std::ffi::c_char;

use acadrust::entities::*;
use acadrust::types::{Matrix4 as NativeMatrix4, Transform};
use acadrust::{CadDocument, EntityType, Handle};

use crate::{
    document, document_mut, ffi_call, input_str, invalid, not_found, output, type_mismatch,
    Document, FfiError, FfiResult, Status, StringView, Vector3,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

/// Row-major affine matrix acting on column vectors.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    pub values: [f64; 16],
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self {
            values: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

macro_rules! entity_catalog {
    ($($name:literal => $make:expr),+ $(,)?) => {
        pub(crate) const ENTITY_NAMES: &[&str] = &[$($name),+];

        pub(crate) fn make_entity(name: &str) -> FfiResult<EntityType> {
            let normalized = name.to_ascii_uppercase();
            match normalized.as_str() {
                $($name => Ok($make),)+
                "3DFACE" => make_entity("FACE3D"),
                "3DSOLID" => make_entity("SOLID3D"),
                "ATTDEF" => make_entity("ATTRIBUTEDEFINITION"),
                "ATTRIB" => make_entity("ATTRIBUTEENTITY"),
                "ENDBLK" => make_entity("BLOCKEND"),
                "IMAGE" => make_entity("RASTERIMAGE"),
                "ACAD_TABLE" => make_entity("TABLE"),
                "EXTENDED" | "EXTENDED_ENTITY" => make_entity("CAMERA"),
                _ => Err(invalid(format!("unknown entity kind {name:?}; see ar_entity_kind_name"))),
            }
        }
    };
}

fn extended(data: ExtendedEntityData) -> EntityType {
    EntityType::Extended(Box::new(ExtendedEntity {
        common: EntityCommon::default(),
        data,
    }))
}

fn dynamic(name: &str) -> EntityType {
    extended(ExtendedEntityData::DynamicBlock(
        acadrust::objects::DynamicBlockData::empty_entity_from_dxf_name(name)
            .expect("catalog dynamic entity name"),
    ))
}

fn dimension(value: Dimension) -> EntityType {
    EntityType::Dimension(Box::new(value))
}

fn surface(kind: SurfaceKind) -> EntityType {
    EntityType::Surface(Box::new(Surface::new(kind)))
}

entity_catalog! {
    "POINT" => EntityType::Point(Point::default()),
    "LINE" => EntityType::Line(Line::default()),
    "CIRCLE" => EntityType::Circle(Circle::default()),
    "ARC" => EntityType::Arc(Arc::default()),
    "ELLIPSE" => EntityType::Ellipse(Ellipse::default()),
    "POLYLINE" => EntityType::Polyline(Polyline::default()),
    "POLYLINE2D" => EntityType::Polyline2D(Polyline2D::default()),
    "POLYLINE3D" => EntityType::Polyline3D(Polyline3D::default()),
    "LWPOLYLINE" => EntityType::LwPolyline(LwPolyline::default()),
    "TEXT" => EntityType::Text(Text::default()),
    "MTEXT" => EntityType::MText(Box::default()),
    "SPLINE" => EntityType::Spline(Spline::default()),
    "HELIX" => EntityType::Helix(Box::default()),
    "DIMENSION" => dimension(Dimension::Aligned(DimensionAligned::default())),
    "DIMENSION_ALIGNED" => dimension(Dimension::Aligned(DimensionAligned::default())),
    "DIMENSION_LINEAR" => dimension(Dimension::Linear(DimensionLinear::default())),
    "DIMENSION_RADIUS" => dimension(Dimension::Radius(DimensionRadius::default())),
    "DIMENSION_DIAMETER" => dimension(Dimension::Diameter(DimensionDiameter::default())),
    "DIMENSION_ANGULAR2LN" => dimension(Dimension::Angular2Ln(DimensionAngular2Ln::default())),
    "DIMENSION_ANGULAR3PT" => dimension(Dimension::Angular3Pt(DimensionAngular3Pt::default())),
    "DIMENSION_ORDINATE" => dimension(Dimension::Ordinate(DimensionOrdinate::default())),
    "DIMENSION_ARC" => dimension(Dimension::Arc(DimensionArc::default())),
    "DIMENSION_LARGERADIAL" => dimension(Dimension::LargeRadial(DimensionLargeRadial::default())),
    "HATCH" => EntityType::Hatch(Box::default()),
    "SOLID" => EntityType::Solid(Solid::new(acadrust::Vector3::ZERO, acadrust::Vector3::ZERO, acadrust::Vector3::ZERO, acadrust::Vector3::ZERO)),
    "FACE3D" => EntityType::Face3D(Face3D::new(acadrust::Vector3::ZERO, acadrust::Vector3::ZERO, acadrust::Vector3::ZERO, acadrust::Vector3::ZERO)),
    "INSERT" => EntityType::Insert(Insert::new("", acadrust::Vector3::ZERO)),
    "BLOCK" => EntityType::Block(Block::new("", acadrust::Vector3::ZERO)),
    "BLOCKEND" => EntityType::BlockEnd(BlockEnd::default()),
    "RAY" => EntityType::Ray(Ray::default()),
    "XLINE" => EntityType::XLine(XLine::default()),
    "VIEWPORT" => EntityType::Viewport(Box::default()),
    "ATTRIBUTEDEFINITION" => EntityType::AttributeDefinition(AttributeDefinition::default()),
    "ATTRIBUTEENTITY" => EntityType::AttributeEntity(AttributeEntity::default()),
    "LEADER" => EntityType::Leader(Leader::default()),
    "MULTILEADER" => EntityType::MultiLeader(Box::default()),
    "MLINE" => EntityType::MLine(MLine::default()),
    "MESH" => EntityType::Mesh(Mesh::default()),
    "RASTERIMAGE" => EntityType::RasterImage(RasterImage::default()),
    "SOLID3D" => EntityType::Solid3D(Solid3D::default()),
    "REGION" => EntityType::Region(Region::default()),
    "BODY" => EntityType::Body(Body::default()),
    "SURFACE" => surface(SurfaceKind::Generic),
    "PLANESURFACE" => surface(SurfaceKind::Plane),
    "EXTRUDEDSURFACE" => surface(SurfaceKind::Extruded),
    "LOFTEDSURFACE" => surface(SurfaceKind::Lofted),
    "REVOLVEDSURFACE" => surface(SurfaceKind::Revolved),
    "SWEPTSURFACE" => surface(SurfaceKind::Swept),
    "NURBSURFACE" => surface(SurfaceKind::Nurb),
    "TABLE" => EntityType::Table(Box::default()),
    "TOLERANCE" => EntityType::Tolerance(Tolerance::default()),
    "POLYFACEMESH" => EntityType::PolyfaceMesh(PolyfaceMesh::default()),
    "WIPEOUT" => EntityType::Wipeout(Wipeout::default()),
    "SHAPE" => EntityType::Shape(Shape::default()),
    "UNDERLAY" => EntityType::Underlay(Underlay::default()),
    "PDFUNDERLAY" => EntityType::Underlay(Underlay::new(UnderlayType::Pdf)),
    "DWFUNDERLAY" => EntityType::Underlay(Underlay::new(UnderlayType::Dwf)),
    "DGNUNDERLAY" => EntityType::Underlay(Underlay::new(UnderlayType::Dgn)),
    "SEQEND" => EntityType::Seqend(Seqend::default()),
    "OLE2FRAME" => EntityType::Ole2Frame(Box::default()),
    "POLYGONMESH" => EntityType::PolygonMesh(PolygonMeshEntity::default()),
    "LIGHT" => EntityType::Light(Box::default()),
    "SECTIONSYMBOL" => EntityType::SectionSymbol(SectionSymbol::default()),
    "VIEWBORDER" => EntityType::ViewBorder(ViewBorder::default()),
    "UNKNOWN" => EntityType::Unknown(UnknownEntity::new("UNKNOWN")),
    "CAMERA" => extended(ExtendedEntityData::Camera { view_handle: Handle::NULL }),
    "SECTIONOBJECT" => extended(ExtendedEntityData::SectionObject(SectionObjectData {
        state: 0, flags: 0, name: String::new(), vertical_direction: acadrust::Vector3::UNIT_Z,
        top_height: 0.0, bottom_height: 0.0, indicator_alpha: 0, indicator_color: acadrust::Color::ByLayer,
        vertices: Vec::new(), back_line_vertices: Vec::new(), settings_handle: Handle::NULL,
    })),
    "ARCALIGNEDTEXT" => extended(ExtendedEntityData::ArcAlignedText(ArcAlignedTextData {
        text: String::new(), font_name: String::new(), big_font_name: String::new(), style_name: "Standard".into(),
        center: acadrust::Vector3::ZERO, radius: 1.0, x_scale: 1.0, text_size: 1.0, character_spacing: 0.0,
        offset_from_arc: 0.0, right_offset: 0.0, left_offset: 0.0, start_angle: 0.0,
        end_angle: std::f64::consts::PI, reverse: false, text_direction: 0, alignment: 0,
        text_position: 0, bold: false, italic: false, underlined: false, character_set: 0,
        pitch_and_family: 0, is_shx: false, text_color: 256, normal: acadrust::Vector3::UNIT_Z,
        wizard_flag: false, arc_handle: Handle::NULL,
    })),
    "RTEXT" => extended(ExtendedEntityData::RemoteText(RemoteTextData {
        position: acadrust::Vector3::ZERO, normal: acadrust::Vector3::UNIT_Z, rotation: 0.0,
        height: 1.0, style_handle: Handle::NULL, style_name: "Standard".into(), flags: 0, text: String::new(),
    })),
    "POSITIONMARKER" => extended(ExtendedEntityData::GeoPositionMarker(GeoPositionMarkerData {
        class_version: 0, position: acadrust::Vector3::ZERO, radius: 1.0, notes: String::new(),
        landing_gap: 0.0, mtext_visible: false, text_alignment: 0, enable_frame_text: false, embedded_mtext: None,
    })),
    "COORDINATION_MODEL" => extended(ExtendedEntityData::CoordinationModel(CoordinationModelData {
        flags: 0, definition_handle: Handle::NULL, transform: Matrix4::default().values, unit_factor: 1.0,
    })),
    "ACDBPOINTCLOUD" => extended(ExtendedEntityData::PointCloud(PointCloudData {
        class_version: 0, origin: acadrust::Vector3::ZERO, saved_filename: String::new(), source_files: Vec::new(),
        extents_min: acadrust::Vector3::ZERO, extents_max: acadrust::Vector3::ZERO, point_count: 0,
        ucs_name: String::new(), ucs_origin: acadrust::Vector3::ZERO, ucs_x_direction: acadrust::Vector3::UNIT_X,
        ucs_y_direction: acadrust::Vector3::UNIT_Y, ucs_z_direction: acadrust::Vector3::UNIT_Z,
        definition_handle: Handle::NULL, reactor_handle: Handle::NULL, show_intensity: false, intensity_scheme: 0,
        minimum_intensity: 0.0, maximum_intensity: 1.0, low_intensity_threshold: 0.0, high_intensity_threshold: 1.0,
        show_clipping: false, clippings: Vec::new(),
    })),
    "ACDBPOINTCLOUDEX" => extended(ExtendedEntityData::PointCloudEx(PointCloudExData {
        class_version: 0, extents_min: acadrust::Vector3::ZERO, extents_max: acadrust::Vector3::ZERO,
        ucs_origin: acadrust::Vector3::ZERO, ucs_x_direction: acadrust::Vector3::UNIT_X,
        ucs_y_direction: acadrust::Vector3::UNIT_Y, ucs_z_direction: acadrust::Vector3::UNIT_Z,
        locked: false, definition_handle: Handle::NULL, reactor_handle: Handle::NULL, name: String::new(),
        show_intensity: false, show_cropping: false, hidden_scans: Vec::new(), hidden_regions: Vec::new(),
        stylization_type: 0, intensity_color_scheme: String::new(), current_color_scheme: String::new(),
        classification_color_scheme: String::new(), elevation_min: 0.0, elevation_max: 1.0,
        intensity_min: 0, intensity_max: 255, intensity_out_of_range_behavior: 0,
        elevation_out_of_range_behavior: 0, elevation_apply_to_fixed_range: false,
        intensity_as_gradient: false, elevation_as_gradient: false, croppings: Vec::new(),
    })),
    "ACAD_PROXY_ENTITY" => extended(ExtendedEntityData::Proxy(ProxyEntityData {
        proxy_id: 0, class_id: 0, dxf_subclass: String::new(), version: 0, dwg_version: 0,
        maintenance_version: 0, from_dxf: false, graphics: Default::default(), payload: Default::default(),
        text_payload: Default::default(), object_ids: Vec::new(),
    })),
    "OLEFRAME" => extended(ExtendedEntityData::OleFrame(OleFrameData { flag: 0, mode: 0, storage: Default::default() })),
    "LAYOUTPRINTCONFIG" => extended(ExtendedEntityData::LayoutPrintConfig(LayoutPrintConfigData::default())),
    "FORMAT" => extended(ExtendedEntityData::Format(FormatData::default())),
    "REPEAT" => extended(ExtendedEntityData::Legacy(LegacyEntityData::Repeat)),
    "ENDREP" => extended(ExtendedEntityData::Legacy(LegacyEntityData::EndRepeat {
        columns: 1, rows: 1, column_spacing: 0.0, row_spacing: 0.0,
    })),
    "LOAD" => extended(ExtendedEntityData::Legacy(LegacyEntityData::Load { filename: String::new() })),
    "JUMP" => extended(ExtendedEntityData::Legacy(LegacyEntityData::Jump { address: 0 })),
    "REGISTEREDCLASS" => extended(ExtendedEntityData::RegisteredClass(RegisteredClassEntityData {
        dxf_name: "CUSTOM_ENTITY".into(), ..Default::default()
    })),
    "ALIGNMENTPARAMETERENTITY" => dynamic("ALIGNMENTPARAMETERENTITY"),
    "BASEPOINTPARAMETERENTITY" => dynamic("BASEPOINTPARAMETERENTITY"),
    "FLIPPARAMETERENTITY" => dynamic("FLIPPARAMETERENTITY"),
    "LINEARPARAMETERENTITY" => dynamic("LINEARPARAMETERENTITY"),
    "POINTPARAMETERENTITY" => dynamic("POINTPARAMETERENTITY"),
    "ROTATIONPARAMETERENTITY" => dynamic("ROTATIONPARAMETERENTITY"),
    "VISIBILITYPARAMETERENTITY" => dynamic("VISIBILITYPARAMETERENTITY"),
    "XYPARAMETERENTITY" => dynamic("XYPARAMETERENTITY"),
    "BLOCKANGULARCONSTRAINTPARAMETERENTITY" => extended(ExtendedEntityData::DynamicBlock(
        acadrust::objects::DynamicBlockData::AngularConstraintParameterEntity(Default::default())
    )),
    "FLIPGRIPENTITY" => dynamic("FLIPGRIPENTITY"),
    "LINEARGRIPENTITY" => dynamic("LINEARGRIPENTITY"),
    "POLARGRIPENTITY" => dynamic("POLARGRIPENTITY"),
    "ROTATIONGRIPENTITY" => dynamic("ROTATIONGRIPENTITY"),
    "VISIBILITYGRIPENTITY" => dynamic("VISIBILITYGRIPENTITY"),
    "XYGRIPENTITY" => dynamic("XYGRIPENTITY"),
}

fn get_entity(doc: &CadDocument, handle: u64) -> FfiResult<&EntityType> {
    doc.get_entity(Handle::new(handle))
        .ok_or_else(|| not_found(format!("entity {handle} was not found")))
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_kind_count(out_count: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_count)? = ENTITY_NAMES.len();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_kind_name(index: usize, out_name: *mut StringView) -> Status {
    ffi_call(|| {
        let name = ENTITY_NAMES
            .get(index)
            .ok_or_else(|| not_found("entity kind index is out of range"))?;
        *output(out_name)? = StringView::new(name);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_kind_variant(index: usize, out_name: *mut StringView) -> Status {
    use crate::views::NativeView;
    ffi_call(|| {
        let name = ENTITY_NAMES
            .get(index)
            .ok_or_else(|| not_found("entity kind index is out of range"))?;
        let value = make_entity(name)?;
        *output(out_name)? = StringView::new(value.variant().expect("entity variant"));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_add_default(
    doc: *mut Document,
    kind: *const c_char,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let value = make_entity(input_str(kind)?)?;
        let out_handle = output(out_handle)?;
        *out_handle = document_mut(doc)?.add_entity(value)?.value();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_class_name(
    doc: *const Document,
    handle: u64,
    out_name: *mut StringView,
) -> Status {
    ffi_call(|| {
        let value = get_entity(document(doc)?, handle)?;
        let name = match value {
            EntityType::Extended(value) => value.class_name(),
            EntityType::Unknown(value) => &value.dxf_name,
            _ => value.as_entity().entity_type(),
        };
        *output(out_name)? = StringView::new(name);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_bounding_box(
    doc: *const Document,
    handle: u64,
    out_bounds: *mut BoundingBox,
) -> Status {
    ffi_call(|| {
        let bounds = get_entity(document(doc)?, handle)?
            .as_entity()
            .bounding_box();
        *output(out_bounds)? = BoundingBox {
            min: bounds.min.into(),
            max: bounds.max.into(),
        };
        Ok(())
    })
}

fn finite_vector(value: Vector3) -> FfiResult<()> {
    if [value.x, value.y, value.z]
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err(invalid("vector coordinates must be finite"));
    }
    Ok(())
}

fn transform_entity(doc: &mut CadDocument, handle: u64, transform: &Transform) -> FfiResult<()> {
    let previous = get_entity(doc, handle)?;
    if matches!(
        previous,
        EntityType::Extended(_)
            | EntityType::Unknown(_)
            | EntityType::Light(_)
            | EntityType::SectionSymbol(_)
            | EntityType::ViewBorder(_)
    ) {
        return Err(type_mismatch(
            "upstream does not support general transforms for this entity class",
        ));
    }
    let mut value = previous.clone();
    value.apply_transform(transform);
    crate::entities::validate_entity(doc, &value)?;
    value.common_mut().raw_record = None;
    *doc.get_entity_mut(Handle::new(handle))
        .expect("validated entity handle") = value;
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_translate(
    doc: *mut Document,
    handle: u64,
    offset: Vector3,
) -> Status {
    ffi_call(|| {
        finite_vector(offset)?;
        let doc = document_mut(doc)?;
        let previous = get_entity(doc, handle)?;
        if matches!(
            previous,
            EntityType::SectionSymbol(_) | EntityType::ViewBorder(_) | EntityType::Unknown(_)
        ) {
            return Err(type_mismatch(
                "upstream does not support translation for this entity class",
            ));
        }
        if let EntityType::Extended(value) = previous {
            if !matches!(
                value.data,
                ExtendedEntityData::SectionObject(_)
                    | ExtendedEntityData::ArcAlignedText(_)
                    | ExtendedEntityData::RemoteText(_)
                    | ExtendedEntityData::GeoPositionMarker(_)
                    | ExtendedEntityData::PointCloud(_)
                    | ExtendedEntityData::PointCloudEx(_)
            ) {
                return Err(type_mismatch(
                    "upstream does not support translation for this extended entity class",
                ));
            }
        }
        let mut value = previous.clone();
        value.translate(offset.into());
        crate::entities::validate_entity(doc, &value)?;
        value.common_mut().raw_record = None;
        *doc.get_entity_mut(Handle::new(handle))
            .expect("validated entity handle") = value;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_transform(
    doc: *mut Document,
    handle: u64,
    matrix: *const Matrix4,
) -> Status {
    ffi_call(|| {
        let matrix = matrix.as_ref().ok_or_else(|| invalid("matrix is NULL"))?;
        if matrix.values.iter().any(|value| !value.is_finite()) {
            return Err(invalid("matrix entries must be finite"));
        }
        if matrix.values[12..] != [0.0, 0.0, 0.0, 1.0] {
            return Err(invalid("matrix must be affine with last row [0, 0, 0, 1]"));
        }
        let matrix = NativeMatrix4 {
            m: std::array::from_fn(|row| {
                std::array::from_fn(|column| matrix.values[row * 4 + column])
            }),
        };
        transform_entity(document_mut(doc)?, handle, &Transform::from_matrix(matrix))
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_rotate(
    doc: *mut Document,
    handle: u64,
    axis: Vector3,
    angle: f64,
) -> Status {
    ffi_call(|| {
        finite_vector(axis)?;
        if !angle.is_finite() || (axis.x == 0.0 && axis.y == 0.0 && axis.z == 0.0) {
            return Err(invalid("rotation requires a finite angle and nonzero axis"));
        }
        transform_entity(
            document_mut(doc)?,
            handle,
            &Transform::from_rotation(axis.into(), angle),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_scale(
    doc: *mut Document,
    handle: u64,
    scale: Vector3,
    origin: Vector3,
) -> Status {
    ffi_call(|| {
        finite_vector(scale)?;
        finite_vector(origin)?;
        if scale.x == 0.0 || scale.y == 0.0 || scale.z == 0.0 {
            return Err(invalid("scale factors must be nonzero"));
        }
        transform_entity(
            document_mut(doc)?,
            handle,
            &Transform::from_scaling_with_origin(scale.into(), origin.into()),
        )
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_mirror(
    doc: *mut Document,
    handle: u64,
    point: Vector3,
    normal: Vector3,
) -> Status {
    ffi_call(|| {
        finite_vector(point)?;
        finite_vector(normal)?;
        if normal.x == 0.0 && normal.y == 0.0 && normal.z == 0.0 {
            return Err(invalid("mirror plane normal must be nonzero"));
        }
        let doc = document_mut(doc)?;
        let previous = get_entity(doc, handle)?;
        if matches!(
            previous,
            EntityType::Extended(_)
                | EntityType::Unknown(_)
                | EntityType::Light(_)
                | EntityType::SectionSymbol(_)
                | EntityType::ViewBorder(_)
        ) {
            return Err(type_mismatch(
                "upstream does not support mirrors for this entity class",
            ));
        }
        let mut value = previous.clone();
        value.apply_mirror(&Transform::from_mirror_plane(point.into(), normal.into()));
        crate::entities::validate_entity(doc, &value)?;
        value.common_mut().raw_record = None;
        *doc.get_entity_mut(Handle::new(handle))
            .expect("validated entity handle") = value;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_explode(
    doc: *mut Document,
    handle: u64,
    handles: *mut u64,
    capacity: usize,
    required: *mut usize,
) -> Status {
    ffi_call(|| {
        let required = output(required)?;
        let doc = document_mut(doc)?;
        let source = get_entity(doc, handle)?;
        let owner = source.common().owner_handle;
        let mut parts = source.explode();
        *required = parts.len();
        if handles.is_null() && capacity == 0 {
            return Ok(());
        }
        if handles.is_null() {
            return Err(invalid("handles pointer is NULL"));
        }
        if capacity < parts.len() {
            return Err(FfiError {
                status: Status::BufferTooSmall,
                message: format!("buffer requires {} handles", parts.len()),
            });
        }
        for part in &mut parts {
            crate::entities::clear_identity(part);
            part.common_mut().owner_handle = owner;
            crate::entities::validate_entity(doc, part)?;
        }
        for (index, part) in parts.into_iter().enumerate() {
            handles.add(index).write(doc.add_entity(part)?.value());
        }
        Ok(())
    })
}
