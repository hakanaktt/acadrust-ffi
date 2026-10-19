//! Native borrowed views of upstream entity fields, geometry, and extended data.
use crate::views::{view_enum, view_struct};
#[path = "entity_view_helpers.rs"]
mod helpers;
view_struct!(acadrust::types::bounds::BoundingBox2D { min, max });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_bounds_2(value: &acadrust::types::bounds::BoundingBox2D) {
    let _ = crate::views::view(&value.min);
    let _ = crate::views::view(&value.max);
}
view_struct!(acadrust::types::bounds::BoundingBox3D { min, max });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_bounds_5(value: &acadrust::types::bounds::BoundingBox3D) {
    let _ = crate::views::view(&value.min);
    let _ = crate::views::view(&value.max);
}
view_enum!(acadrust::types::color::Color {
    ByLayer,
    None,
    ByBlock,
    Index(v0),
    Rgb { r, g, b },
});
view_enum!(acadrust::types::line_weight::LineWeight {
    ByLayer,
    ByBlock,
    Default,
    Value(v0),
});
view_enum!(acadrust::types::DxfVersion {
    Unknown,
    AC1009,
    AC1012,
    AC1014,
    AC1015,
    AC1018,
    AC1021,
    AC1024,
    AC1027,
    AC1032,
});
view_struct!(acadrust::types::transform::Matrix3 { m });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_transform_2(value: &acadrust::types::transform::Matrix3) {
    let _ = crate::views::view(&value.m);
}
view_struct!(acadrust::types::transform::Matrix4 { m });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_transform_7(value: &acadrust::types::transform::Matrix4) {
    let _ = crate::views::view(&value.m);
}
view_struct!(acadrust::types::transform::Transform { matrix });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_transform_11(value: &acadrust::types::transform::Transform) {
    let _ = crate::views::view(&value.matrix);
}
view_enum!(acadrust::types::transparency::Transparency {
    ByLayer,
    ByBlock,
    Explicit(v0),
});
view_struct!(acadrust::types::vector::Vector2 { x, y });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_vector_2(value: &acadrust::types::vector::Vector2) {
    let _ = crate::views::view(&value.x);
    let _ = crate::views::view(&value.y);
}
view_struct!(acadrust::types::vector::Vector3 { x, y, z });
#[allow(dead_code, unused_variables)]
fn check_acadrust_types_vector_11(value: &acadrust::types::vector::Vector3) {
    let _ = crate::views::view(&value.x);
    let _ = crate::views::view(&value.y);
    let _ = crate::views::view(&value.z);
}
view_enum!(acadrust::xdata::XDataValue {
    String(v0),
    ControlString(v0),
    LayerName(v0),
    BinaryData(v0),
    Handle(v0),
    Point3D(v0),
    Position3D(v0),
    Displacement3D(v0),
    Direction3D(v0),
    Real(v0),
    Distance(v0),
    ScaleFactor(v0),
    Integer16(v0),
    Integer32(v0),
});
view_struct!(acadrust::xdata::ExtendedDataRecord {
    application_name,
    values
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_xdata_2(value: &acadrust::xdata::ExtendedDataRecord) {
    let _ = crate::views::view(&value.application_name);
    let _ = crate::views::view(&value.values);
}
view_struct!(acadrust::entities::arc::Arc {
    common,
    center,
    radius,
    start_angle,
    end_angle,
    thickness,
    normal
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_arc_2(value: &acadrust::entities::arc::Arc) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.start_angle);
    let _ = crate::views::view(&value.end_angle);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.normal);
}
view_struct!(acadrust::entities::attribute_definition::AttributeFlags {
    invisible,
    constant,
    verify,
    preset,
    locked_position,
    annotative
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_attribute_definition_3(
    value: &acadrust::entities::attribute_definition::AttributeFlags,
) {
    let _ = crate::views::view(&value.invisible);
    let _ = crate::views::view(&value.constant);
    let _ = crate::views::view(&value.verify);
    let _ = crate::views::view(&value.preset);
    let _ = crate::views::view(&value.locked_position);
    let _ = crate::views::view(&value.annotative);
}
view_enum!(
    acadrust::entities::attribute_definition::HorizontalAlignment {
        Left,
        Center,
        Right,
        Aligned,
        Middle,
        Fit,
    }
);
view_enum!(
    acadrust::entities::attribute_definition::VerticalAlignment {
        Baseline,
        Bottom,
        Middle,
        Top,
    }
);
view_enum!(acadrust::entities::attribute_definition::MTextFlag {
    SingleLine,
    MultiLine,
    ConstantMultiLine,
});
view_struct!(
    acadrust::entities::attribute_definition::AttributeDefinition {
        common,
        tag,
        prompt,
        default_value,
        insertion_point,
        alignment_point,
        height,
        rotation,
        width_factor,
        oblique_angle,
        text_style,
        text_generation_flags,
        horizontal_alignment,
        vertical_alignment,
        flags,
        field_length,
        normal,
        mtext_flag,
        is_multiline,
        line_count,
        embedded_mtext,
        lock_position
    }
);
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_attribute_definition_11(
    value: &acadrust::entities::attribute_definition::AttributeDefinition,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.tag);
    let _ = crate::views::view(&value.prompt);
    let _ = crate::views::view(&value.default_value);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.alignment_point);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.width_factor);
    let _ = crate::views::view(&value.oblique_angle);
    let _ = crate::views::view(&value.text_style);
    let _ = crate::views::view(&value.text_generation_flags);
    let _ = crate::views::view(&value.horizontal_alignment);
    let _ = crate::views::view(&value.vertical_alignment);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.field_length);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.mtext_flag);
    let _ = crate::views::view(&value.is_multiline);
    let _ = crate::views::view(&value.line_count);
    let _ = crate::views::view(&value.embedded_mtext);
    let _ = crate::views::view(&value.lock_position);
}
view_struct!(acadrust::entities::attribute_entity::AttributeEntity {
    common,
    tag,
    value,
    insertion_point,
    alignment_point,
    height,
    rotation,
    width_factor,
    oblique_angle,
    text_style,
    text_generation_flags,
    horizontal_alignment,
    vertical_alignment,
    flags,
    field_length,
    normal,
    mtext_flag,
    is_multiline,
    line_count,
    embedded_mtext,
    attdef_handle,
    lock_position
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_attribute_entity_4(
    value: &acadrust::entities::attribute_entity::AttributeEntity,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.tag);
    let _ = crate::views::view(&value.value);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.alignment_point);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.width_factor);
    let _ = crate::views::view(&value.oblique_angle);
    let _ = crate::views::view(&value.text_style);
    let _ = crate::views::view(&value.text_generation_flags);
    let _ = crate::views::view(&value.horizontal_alignment);
    let _ = crate::views::view(&value.vertical_alignment);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.field_length);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.mtext_flag);
    let _ = crate::views::view(&value.is_multiline);
    let _ = crate::views::view(&value.line_count);
    let _ = crate::views::view(&value.embedded_mtext);
    let _ = crate::views::view(&value.attdef_handle);
    let _ = crate::views::view(&value.lock_position);
}
view_struct!(acadrust::entities::block::Block {
    common,
    name,
    base_point,
    description,
    xref_path
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_block_2(value: &acadrust::entities::block::Block) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.base_point);
    let _ = crate::views::view(&value.description);
    let _ = crate::views::view(&value.xref_path);
}
view_struct!(acadrust::entities::block::BlockEnd { common });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_block_4(value: &acadrust::entities::block::BlockEnd) {
    let _ = crate::views::view(&value.common);
}
view_enum!(acadrust::entities::centerline::CenterLineSourceKind {
    Line,
    LwPolylineSegment,
    Polyline2DSegment,
});
view_struct!(acadrust::entities::centerline::CenterLineSource {
    handle,
    kind,
    segment_index,
    pick_point
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_centerline_9(value: &acadrust::entities::centerline::CenterLineSource) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.kind);
    let _ = crate::views::view(&value.segment_index);
    let _ = crate::views::view(&value.pick_point);
}
view_struct!(acadrust::entities::centerline::CenterLineAssociation {
    first,
    second,
    plane_origin,
    plane_x,
    plane_y,
    start_extension,
    end_extension,
    start_length_adjustment,
    end_length_adjustment,
    associated
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_centerline_10(
    value: &acadrust::entities::centerline::CenterLineAssociation,
) {
    let _ = crate::views::view(&value.first);
    let _ = crate::views::view(&value.second);
    let _ = crate::views::view(&value.plane_origin);
    let _ = crate::views::view(&value.plane_x);
    let _ = crate::views::view(&value.plane_y);
    let _ = crate::views::view(&value.start_extension);
    let _ = crate::views::view(&value.end_extension);
    let _ = crate::views::view(&value.start_length_adjustment);
    let _ = crate::views::view(&value.end_length_adjustment);
    let _ = crate::views::view(&value.associated);
}
view_enum!(acadrust::entities::centerline::CenterMarkSourceKind {
    Circle,
    Arc,
    LwPolylineArcSegment,
    Polyline2DArcSegment,
});
view_struct!(acadrust::entities::centerline::CenterMarkSource {
    handle,
    kind,
    segment_index,
    pick_point
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_centerline_14(value: &acadrust::entities::centerline::CenterMarkSource) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.kind);
    let _ = crate::views::view(&value.segment_index);
    let _ = crate::views::view(&value.pick_point);
}
view_struct!(acadrust::entities::centerline::CenterMarkAssociation {
    source,
    plane_origin,
    plane_x,
    plane_y,
    center,
    radius,
    cross_size,
    cross_gap,
    cross_size_relative,
    cross_gap_relative,
    extension_length,
    length_adjustments,
    overshoots,
    show_extensions,
    associated
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_centerline_15(
    value: &acadrust::entities::centerline::CenterMarkAssociation,
) {
    let _ = crate::views::view(&value.source);
    let _ = crate::views::view(&value.plane_origin);
    let _ = crate::views::view(&value.plane_x);
    let _ = crate::views::view(&value.plane_y);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.cross_size);
    let _ = crate::views::view(&value.cross_gap);
    let _ = crate::views::view(&value.cross_size_relative);
    let _ = crate::views::view(&value.cross_gap_relative);
    let _ = crate::views::view(&value.extension_length);
    let _ = crate::views::view(&value.length_adjustments);
    let _ = crate::views::view(&value.overshoots);
    let _ = crate::views::view(&value.show_extensions);
    let _ = crate::views::view(&value.associated);
}
view_struct!(acadrust::entities::circle::Circle {
    common,
    center,
    radius,
    thickness,
    normal
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_circle_2(value: &acadrust::entities::circle::Circle) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.normal);
}
view_enum!(acadrust::entities::dimension::DimensionType {
    Linear,
    Aligned,
    Angular,
    Diameter,
    Radius,
    Angular3Point,
    Ordinate,
    ArcLength,
    LargeRadial,
});
view_enum!(acadrust::entities::dimension::AttachmentPointType {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
});
view_struct!(acadrust::entities::dimension::DimensionBase {
    common,
    definition_point,
    text_middle_point,
    insertion_point,
    dimension_type,
    attachment_point,
    text,
    user_text,
    normal,
    text_rotation,
    horizontal_direction,
    style_name,
    actual_measurement,
    version,
    block_name,
    line_spacing_factor,
    line_spacing_style,
    insertion_scale,
    insertion_rotation,
    dwg_unknown_bit,
    flip_arrow1,
    flip_arrow2,
    dwg_flags_byte,
    text_user_positioned
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_4(value: &acadrust::entities::dimension::DimensionBase) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.text_middle_point);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.dimension_type);
    let _ = crate::views::view(&value.attachment_point);
    let _ = crate::views::view(&value.text);
    let _ = crate::views::view(&value.user_text);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.text_rotation);
    let _ = crate::views::view(&value.horizontal_direction);
    let _ = crate::views::view(&value.style_name);
    let _ = crate::views::view(&value.actual_measurement);
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.block_name);
    let _ = crate::views::view(&value.line_spacing_factor);
    let _ = crate::views::view(&value.line_spacing_style);
    let _ = crate::views::view(&value.insertion_scale);
    let _ = crate::views::view(&value.insertion_rotation);
    let _ = crate::views::view(&value.dwg_unknown_bit);
    let _ = crate::views::view(&value.flip_arrow1);
    let _ = crate::views::view(&value.flip_arrow2);
    let _ = crate::views::view(&value.dwg_flags_byte);
    let _ = crate::views::view(&value.text_user_positioned);
}
view_struct!(acadrust::entities::dimension::DimensionAligned {
    base,
    first_point,
    second_point,
    definition_point,
    ext_line_rotation
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_7(value: &acadrust::entities::dimension::DimensionAligned) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.first_point);
    let _ = crate::views::view(&value.second_point);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.ext_line_rotation);
}
view_struct!(acadrust::entities::dimension::DimensionLinear {
    base,
    first_point,
    second_point,
    definition_point,
    rotation,
    ext_line_rotation
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_10(value: &acadrust::entities::dimension::DimensionLinear) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.first_point);
    let _ = crate::views::view(&value.second_point);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.ext_line_rotation);
}
view_struct!(acadrust::entities::dimension::DimensionRadius {
    base,
    definition_point,
    angle_vertex,
    leader_length
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_13(value: &acadrust::entities::dimension::DimensionRadius) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.angle_vertex);
    let _ = crate::views::view(&value.leader_length);
}
view_struct!(acadrust::entities::dimension::DimensionDiameter {
    base,
    definition_point,
    angle_vertex,
    leader_length
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_16(value: &acadrust::entities::dimension::DimensionDiameter) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.angle_vertex);
    let _ = crate::views::view(&value.leader_length);
}
view_struct!(acadrust::entities::dimension::DimensionAngular2Ln {
    base,
    dimension_arc,
    first_point,
    second_point,
    angle_vertex,
    definition_point
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_19(
    value: &acadrust::entities::dimension::DimensionAngular2Ln,
) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.dimension_arc);
    let _ = crate::views::view(&value.first_point);
    let _ = crate::views::view(&value.second_point);
    let _ = crate::views::view(&value.angle_vertex);
    let _ = crate::views::view(&value.definition_point);
}
view_struct!(acadrust::entities::dimension::DimensionAngular3Pt {
    base,
    definition_point,
    first_point,
    second_point,
    angle_vertex
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_23(
    value: &acadrust::entities::dimension::DimensionAngular3Pt,
) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.first_point);
    let _ = crate::views::view(&value.second_point);
    let _ = crate::views::view(&value.angle_vertex);
}
view_struct!(acadrust::entities::dimension::DimensionOrdinate {
    base,
    definition_point,
    feature_location,
    leader_endpoint,
    is_ordinate_type_x
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_30(value: &acadrust::entities::dimension::DimensionOrdinate) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.feature_location);
    let _ = crate::views::view(&value.leader_endpoint);
    let _ = crate::views::view(&value.is_ordinate_type_x);
}
view_struct!(acadrust::entities::dimension::DimensionArc {
    base,
    definition_point,
    first_extension_point,
    second_extension_point,
    center_point,
    is_partial,
    arc_start_parameter,
    arc_end_parameter,
    has_leader,
    first_leader_point,
    second_leader_point
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_33(value: &acadrust::entities::dimension::DimensionArc) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.first_extension_point);
    let _ = crate::views::view(&value.second_extension_point);
    let _ = crate::views::view(&value.center_point);
    let _ = crate::views::view(&value.is_partial);
    let _ = crate::views::view(&value.arc_start_parameter);
    let _ = crate::views::view(&value.arc_end_parameter);
    let _ = crate::views::view(&value.has_leader);
    let _ = crate::views::view(&value.first_leader_point);
    let _ = crate::views::view(&value.second_leader_point);
}
view_struct!(acadrust::entities::dimension::DimensionLargeRadial {
    base,
    definition_point,
    chord_point,
    jog_angle,
    override_center,
    jog_point
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_dimension_36(
    value: &acadrust::entities::dimension::DimensionLargeRadial,
) {
    let _ = crate::views::view(&value.base);
    let _ = crate::views::view(&value.definition_point);
    let _ = crate::views::view(&value.chord_point);
    let _ = crate::views::view(&value.jog_angle);
    let _ = crate::views::view(&value.override_center);
    let _ = crate::views::view(&value.jog_point);
}
view_enum!(acadrust::entities::dimension::Dimension {
    Aligned(v0),
    Linear(v0),
    Radius(v0),
    Diameter(v0),
    Angular2Ln(v0),
    Angular3Pt(v0),
    Ordinate(v0),
    Arc(v0),
    LargeRadial(v0),
});
view_struct!(acadrust::entities::ellipse::Ellipse {
    common,
    center,
    major_axis,
    minor_axis_ratio,
    start_parameter,
    end_parameter,
    normal
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_ellipse_2(value: &acadrust::entities::ellipse::Ellipse) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.major_axis);
    let _ = crate::views::view(&value.minor_axis_ratio);
    let _ = crate::views::view(&value.start_parameter);
    let _ = crate::views::view(&value.end_parameter);
    let _ = crate::views::view(&value.normal);
}
view_enum!(acadrust::entities::embedded_entity::EmbeddedEntity {
    Point(v0),
    Line(v0),
    Arc(v0),
    Circle(v0),
    Ellipse(v0),
    Spline(v0),
    LwPolyline(v0),
    Region(v0),
    Ray(v0),
    XLine(v0),
    Body { type_code, acis_data },
    Unknown { type_code, bit_count, bytes },
});
view_struct!(acadrust::entities::extended_entity::ExtendedEntity { common, data });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_2(
    value: &acadrust::entities::extended_entity::ExtendedEntity,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.data);
}
view_enum!(acadrust::entities::extended_entity::ExtendedEntityData {
    Camera { view_handle },
    SectionObject(v0),
    ArcAlignedText(v0),
    RemoteText(v0),
    GeoPositionMarker(v0),
    CoordinationModel(v0),
    PointCloud(v0),
    PointCloudEx(v0),
    Proxy(v0),
    OleFrame(v0),
    LayoutPrintConfig(v0),
    Format(v0),
    Legacy(v0),
    DynamicBlock(v0),
    RegisteredClass(v0),
});
view_struct!(acadrust::entities::extended_entity::SectionObjectData {
    state,
    flags,
    name,
    vertical_direction,
    top_height,
    bottom_height,
    indicator_alpha,
    indicator_color,
    vertices,
    back_line_vertices,
    settings_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_5(
    value: &acadrust::entities::extended_entity::SectionObjectData,
) {
    let _ = crate::views::view(&value.state);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.vertical_direction);
    let _ = crate::views::view(&value.top_height);
    let _ = crate::views::view(&value.bottom_height);
    let _ = crate::views::view(&value.indicator_alpha);
    let _ = crate::views::view(&value.indicator_color);
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.back_line_vertices);
    let _ = crate::views::view(&value.settings_handle);
}
view_struct!(acadrust::entities::extended_entity::ArcAlignedTextData {
    text,
    font_name,
    big_font_name,
    style_name,
    center,
    radius,
    x_scale,
    text_size,
    character_spacing,
    offset_from_arc,
    right_offset,
    left_offset,
    start_angle,
    end_angle,
    reverse,
    text_direction,
    alignment,
    text_position,
    bold,
    italic,
    underlined,
    character_set,
    pitch_and_family,
    is_shx,
    text_color,
    normal,
    wizard_flag,
    arc_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_6(
    value: &acadrust::entities::extended_entity::ArcAlignedTextData,
) {
    let _ = crate::views::view(&value.text);
    let _ = crate::views::view(&value.font_name);
    let _ = crate::views::view(&value.big_font_name);
    let _ = crate::views::view(&value.style_name);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.x_scale);
    let _ = crate::views::view(&value.text_size);
    let _ = crate::views::view(&value.character_spacing);
    let _ = crate::views::view(&value.offset_from_arc);
    let _ = crate::views::view(&value.right_offset);
    let _ = crate::views::view(&value.left_offset);
    let _ = crate::views::view(&value.start_angle);
    let _ = crate::views::view(&value.end_angle);
    let _ = crate::views::view(&value.reverse);
    let _ = crate::views::view(&value.text_direction);
    let _ = crate::views::view(&value.alignment);
    let _ = crate::views::view(&value.text_position);
    let _ = crate::views::view(&value.bold);
    let _ = crate::views::view(&value.italic);
    let _ = crate::views::view(&value.underlined);
    let _ = crate::views::view(&value.character_set);
    let _ = crate::views::view(&value.pitch_and_family);
    let _ = crate::views::view(&value.is_shx);
    let _ = crate::views::view(&value.text_color);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.wizard_flag);
    let _ = crate::views::view(&value.arc_handle);
}
view_struct!(acadrust::entities::extended_entity::RemoteTextData {
    position,
    normal,
    rotation,
    height,
    style_handle,
    style_name,
    flags,
    text
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_7(
    value: &acadrust::entities::extended_entity::RemoteTextData,
) {
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.style_handle);
    let _ = crate::views::view(&value.style_name);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.text);
}
view_struct!(acadrust::entities::extended_entity::GeoPositionMarkerData {
    class_version,
    position,
    radius,
    notes,
    landing_gap,
    mtext_visible,
    text_alignment,
    enable_frame_text,
    embedded_mtext
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_8(
    value: &acadrust::entities::extended_entity::GeoPositionMarkerData,
) {
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.notes);
    let _ = crate::views::view(&value.landing_gap);
    let _ = crate::views::view(&value.mtext_visible);
    let _ = crate::views::view(&value.text_alignment);
    let _ = crate::views::view(&value.enable_frame_text);
    let _ = crate::views::view(&value.embedded_mtext);
}
view_struct!(acadrust::entities::extended_entity::CoordinationModelData {
    flags,
    definition_handle,
    transform,
    unit_factor
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_9(
    value: &acadrust::entities::extended_entity::CoordinationModelData,
) {
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.transform);
    let _ = crate::views::view(&value.unit_factor);
}
view_struct!(acadrust::entities::extended_entity::PointCloudClip {
    inverted,
    clip_type,
    vertices,
    z_min,
    z_max
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_10(
    value: &acadrust::entities::extended_entity::PointCloudClip,
) {
    let _ = crate::views::view(&value.inverted);
    let _ = crate::views::view(&value.clip_type);
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.z_min);
    let _ = crate::views::view(&value.z_max);
}
view_struct!(acadrust::entities::extended_entity::PointCloudData {
    class_version,
    origin,
    saved_filename,
    source_files,
    extents_min,
    extents_max,
    point_count,
    ucs_name,
    ucs_origin,
    ucs_x_direction,
    ucs_y_direction,
    ucs_z_direction,
    definition_handle,
    reactor_handle,
    show_intensity,
    intensity_scheme,
    minimum_intensity,
    maximum_intensity,
    low_intensity_threshold,
    high_intensity_threshold,
    show_clipping,
    clippings
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_11(
    value: &acadrust::entities::extended_entity::PointCloudData,
) {
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.origin);
    let _ = crate::views::view(&value.saved_filename);
    let _ = crate::views::view(&value.source_files);
    let _ = crate::views::view(&value.extents_min);
    let _ = crate::views::view(&value.extents_max);
    let _ = crate::views::view(&value.point_count);
    let _ = crate::views::view(&value.ucs_name);
    let _ = crate::views::view(&value.ucs_origin);
    let _ = crate::views::view(&value.ucs_x_direction);
    let _ = crate::views::view(&value.ucs_y_direction);
    let _ = crate::views::view(&value.ucs_z_direction);
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.reactor_handle);
    let _ = crate::views::view(&value.show_intensity);
    let _ = crate::views::view(&value.intensity_scheme);
    let _ = crate::views::view(&value.minimum_intensity);
    let _ = crate::views::view(&value.maximum_intensity);
    let _ = crate::views::view(&value.low_intensity_threshold);
    let _ = crate::views::view(&value.high_intensity_threshold);
    let _ = crate::views::view(&value.show_clipping);
    let _ = crate::views::view(&value.clippings);
}
view_struct!(acadrust::entities::extended_entity::PointCloudExCrop {
    crop_type,
    inside,
    inverted,
    plane,
    x_direction,
    y_direction,
    points
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_12(
    value: &acadrust::entities::extended_entity::PointCloudExCrop,
) {
    let _ = crate::views::view(&value.crop_type);
    let _ = crate::views::view(&value.inside);
    let _ = crate::views::view(&value.inverted);
    let _ = crate::views::view(&value.plane);
    let _ = crate::views::view(&value.x_direction);
    let _ = crate::views::view(&value.y_direction);
    let _ = crate::views::view(&value.points);
}
view_struct!(acadrust::entities::extended_entity::PointCloudExData {
    class_version,
    extents_min,
    extents_max,
    ucs_origin,
    ucs_x_direction,
    ucs_y_direction,
    ucs_z_direction,
    locked,
    definition_handle,
    reactor_handle,
    name,
    show_intensity,
    show_cropping,
    hidden_scans,
    hidden_regions,
    stylization_type,
    intensity_color_scheme,
    current_color_scheme,
    classification_color_scheme,
    elevation_min,
    elevation_max,
    intensity_min,
    intensity_max,
    intensity_out_of_range_behavior,
    elevation_out_of_range_behavior,
    elevation_apply_to_fixed_range,
    intensity_as_gradient,
    elevation_as_gradient,
    croppings
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_13(
    value: &acadrust::entities::extended_entity::PointCloudExData,
) {
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.extents_min);
    let _ = crate::views::view(&value.extents_max);
    let _ = crate::views::view(&value.ucs_origin);
    let _ = crate::views::view(&value.ucs_x_direction);
    let _ = crate::views::view(&value.ucs_y_direction);
    let _ = crate::views::view(&value.ucs_z_direction);
    let _ = crate::views::view(&value.locked);
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.reactor_handle);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.show_intensity);
    let _ = crate::views::view(&value.show_cropping);
    let _ = crate::views::view(&value.hidden_scans);
    let _ = crate::views::view(&value.hidden_regions);
    let _ = crate::views::view(&value.stylization_type);
    let _ = crate::views::view(&value.intensity_color_scheme);
    let _ = crate::views::view(&value.current_color_scheme);
    let _ = crate::views::view(&value.classification_color_scheme);
    let _ = crate::views::view(&value.elevation_min);
    let _ = crate::views::view(&value.elevation_max);
    let _ = crate::views::view(&value.intensity_min);
    let _ = crate::views::view(&value.intensity_max);
    let _ = crate::views::view(&value.intensity_out_of_range_behavior);
    let _ = crate::views::view(&value.elevation_out_of_range_behavior);
    let _ = crate::views::view(&value.elevation_apply_to_fixed_range);
    let _ = crate::views::view(&value.intensity_as_gradient);
    let _ = crate::views::view(&value.elevation_as_gradient);
    let _ = crate::views::view(&value.croppings);
}
view_struct!(acadrust::entities::extended_entity::ProxyEntityData {
    proxy_id,
    class_id,
    dxf_subclass,
    version,
    dwg_version,
    maintenance_version,
    from_dxf,
    graphics,
    payload,
    text_payload,
    object_ids
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_14(
    value: &acadrust::entities::extended_entity::ProxyEntityData,
) {
    let _ = crate::views::view(&value.proxy_id);
    let _ = crate::views::view(&value.class_id);
    let _ = crate::views::view(&value.dxf_subclass);
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.dwg_version);
    let _ = crate::views::view(&value.maintenance_version);
    let _ = crate::views::view(&value.from_dxf);
    let _ = crate::views::view(&value.graphics);
    let _ = crate::views::view(&value.payload);
    let _ = crate::views::view(&value.text_payload);
    let _ = crate::views::view(&value.object_ids);
}
view_struct!(acadrust::entities::extended_entity::OleFrameData {
    flag,
    mode,
    storage
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_15(
    value: &acadrust::entities::extended_entity::OleFrameData,
) {
    let _ = crate::views::view(&value.flag);
    let _ = crate::views::view(&value.mode);
    let _ = crate::views::view(&value.storage);
}
view_struct!(acadrust::entities::extended_entity::LayoutPrintConfigData {
    class_version,
    flag,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_16(
    value: &acadrust::entities::extended_entity::LayoutPrintConfigData,
) {
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.flag);
    let _ = crate::views::view(&value.raw_dwg_data);
    let _ = crate::views::view(&value.raw_dwg_handle_bits);
    let _ = crate::views::view(&value.raw_dwg_version);
}
view_struct!(acadrust::entities::extended_entity::FormatData {
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version,
    raw_dxf_codes
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_17(
    value: &acadrust::entities::extended_entity::FormatData,
) {
    let _ = crate::views::view(&value.raw_dwg_data);
    let _ = crate::views::view(&value.raw_dwg_handle_bits);
    let _ = crate::views::view(&value.raw_dwg_version);
    let _ = crate::views::view(&value.raw_dxf_codes);
}
view_enum!(acadrust::entities::extended_entity::LegacyEntityData {
    Repeat,
    EndRepeat { columns, rows, column_spacing, row_spacing },
    Load { filename },
    Jump { address },
});
view_struct!(
    acadrust::entities::extended_entity::RegisteredClassEntityData {
        dxf_name,
        cpp_class_name,
        properties,
        payload,
        object_ids
    }
);
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_extended_entity_19(
    value: &acadrust::entities::extended_entity::RegisteredClassEntityData,
) {
    let _ = crate::views::view(&value.dxf_name);
    let _ = crate::views::view(&value.cpp_class_name);
    let _ = crate::views::view(&value.properties);
    let _ = crate::views::view(&value.payload);
    let _ = crate::views::view(&value.object_ids);
}
view_struct!(acadrust::entities::face3d::Face3D {
    common,
    first_corner,
    second_corner,
    third_corner,
    fourth_corner,
    invisible_edges
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_face3d_5(value: &acadrust::entities::face3d::Face3D) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.first_corner);
    let _ = crate::views::view(&value.second_corner);
    let _ = crate::views::view(&value.third_corner);
    let _ = crate::views::view(&value.fourth_corner);
    let _ = crate::views::view(&value.invisible_edges);
}
view_enum!(acadrust::entities::hatch::HatchPatternType {
    UserDefined,
    Predefined,
    Custom,
});
view_enum!(acadrust::entities::hatch::HatchStyleType {
    Normal,
    Outer,
    Ignore,
});
view_enum!(acadrust::entities::hatch::EdgeType {
    Polyline,
    Line,
    CircularArc,
    EllipticArc,
    Spline,
});
view_struct!(acadrust::entities::hatch::LineEdge { start, end });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_8(value: &acadrust::entities::hatch::LineEdge) {
    let _ = crate::views::view(&value.start);
    let _ = crate::views::view(&value.end);
}
view_struct!(acadrust::entities::hatch::CircularArcEdge {
    center,
    radius,
    start_angle,
    end_angle,
    counter_clockwise
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_9(value: &acadrust::entities::hatch::CircularArcEdge) {
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.start_angle);
    let _ = crate::views::view(&value.end_angle);
    let _ = crate::views::view(&value.counter_clockwise);
}
view_struct!(acadrust::entities::hatch::EllipticArcEdge {
    center,
    major_axis_endpoint,
    minor_axis_ratio,
    start_angle,
    end_angle,
    counter_clockwise
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_10(value: &acadrust::entities::hatch::EllipticArcEdge) {
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.major_axis_endpoint);
    let _ = crate::views::view(&value.minor_axis_ratio);
    let _ = crate::views::view(&value.start_angle);
    let _ = crate::views::view(&value.end_angle);
    let _ = crate::views::view(&value.counter_clockwise);
}
view_struct!(acadrust::entities::hatch::SplineEdge {
    degree,
    rational,
    periodic,
    knots,
    control_points,
    fit_points,
    start_tangent,
    end_tangent
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_11(value: &acadrust::entities::hatch::SplineEdge) {
    let _ = crate::views::view(&value.degree);
    let _ = crate::views::view(&value.rational);
    let _ = crate::views::view(&value.periodic);
    let _ = crate::views::view(&value.knots);
    let _ = crate::views::view(&value.control_points);
    let _ = crate::views::view(&value.fit_points);
    let _ = crate::views::view(&value.start_tangent);
    let _ = crate::views::view(&value.end_tangent);
}
view_struct!(acadrust::entities::hatch::PolylineEdge {
    vertices,
    is_closed
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_12(value: &acadrust::entities::hatch::PolylineEdge) {
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.is_closed);
}
view_enum!(acadrust::entities::hatch::BoundaryEdge {
    Line(v0),
    CircularArc(v0),
    EllipticArc(v0),
    Spline(v0),
    Polyline(v0),
});
view_struct!(acadrust::entities::hatch::BoundaryPath {
    flags,
    edges,
    boundary_handles
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_15(value: &acadrust::entities::hatch::BoundaryPath) {
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.edges);
    let _ = crate::views::view(&value.boundary_handles);
}
view_struct!(acadrust::entities::hatch::HatchPatternLine {
    angle,
    base_point,
    offset,
    dash_lengths
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_18(value: &acadrust::entities::hatch::HatchPatternLine) {
    let _ = crate::views::view(&value.angle);
    let _ = crate::views::view(&value.base_point);
    let _ = crate::views::view(&value.offset);
    let _ = crate::views::view(&value.dash_lengths);
}
view_struct!(acadrust::entities::hatch::HatchPattern {
    name,
    description,
    lines
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_19(value: &acadrust::entities::hatch::HatchPattern) {
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.description);
    let _ = crate::views::view(&value.lines);
}
view_struct!(acadrust::entities::hatch::GradientColorEntry {
    value,
    color,
    color_name,
    book_name
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_21(value: &acadrust::entities::hatch::GradientColorEntry) {
    let _ = crate::views::view(&value.value);
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.color_name);
    let _ = crate::views::view(&value.book_name);
}
view_struct!(acadrust::entities::hatch::HatchGradientPattern {
    enabled,
    reserved,
    angle,
    shift,
    is_single_color,
    color_tint,
    colors,
    name
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_22(value: &acadrust::entities::hatch::HatchGradientPattern) {
    let _ = crate::views::view(&value.enabled);
    let _ = crate::views::view(&value.reserved);
    let _ = crate::views::view(&value.angle);
    let _ = crate::views::view(&value.shift);
    let _ = crate::views::view(&value.is_single_color);
    let _ = crate::views::view(&value.color_tint);
    let _ = crate::views::view(&value.colors);
    let _ = crate::views::view(&value.name);
}
view_struct!(acadrust::entities::hatch::Hatch {
    common,
    is_mpolygon,
    elevation,
    normal,
    pattern,
    is_solid,
    is_associative,
    pattern_type,
    pattern_angle,
    pattern_scale,
    is_double,
    style,
    paths,
    seed_points,
    pixel_size,
    gradient_color,
    mpolygon_hatch_color,
    mpolygon_x_direction,
    mpolygon_invalid_loops
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_hatch_25(value: &acadrust::entities::hatch::Hatch) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.is_mpolygon);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.pattern);
    let _ = crate::views::view(&value.is_solid);
    let _ = crate::views::view(&value.is_associative);
    let _ = crate::views::view(&value.pattern_type);
    let _ = crate::views::view(&value.pattern_angle);
    let _ = crate::views::view(&value.pattern_scale);
    let _ = crate::views::view(&value.is_double);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.paths);
    let _ = crate::views::view(&value.seed_points);
    let _ = crate::views::view(&value.pixel_size);
    let _ = crate::views::view(&value.gradient_color);
    let _ = crate::views::view(&value.mpolygon_hatch_color);
    let _ = crate::views::view(&value.mpolygon_x_direction);
    let _ = crate::views::view(&value.mpolygon_invalid_loops);
}
view_enum!(acadrust::entities::helix::HelixConstraint {
    TurnHeight,
    Turns,
    Height,
});
view_struct!(acadrust::entities::helix::Helix {
    common,
    spline,
    major_version,
    maintenance_version,
    axis_base_point,
    start_point,
    axis_vector,
    radius,
    turns,
    turn_height,
    handedness,
    constraint
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_helix_6(value: &acadrust::entities::helix::Helix) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.spline);
    let _ = crate::views::view(&value.major_version);
    let _ = crate::views::view(&value.maintenance_version);
    let _ = crate::views::view(&value.axis_base_point);
    let _ = crate::views::view(&value.start_point);
    let _ = crate::views::view(&value.axis_vector);
    let _ = crate::views::view(&value.radius);
    let _ = crate::views::view(&value.turns);
    let _ = crate::views::view(&value.turn_height);
    let _ = crate::views::view(&value.handedness);
    let _ = crate::views::view(&value.constraint);
}
view_struct!(acadrust::entities::insert::Insert {
    common,
    block_name,
    insert_point,
    rotation,
    normal,
    column_count,
    row_count,
    column_spacing,
    row_spacing,
    attributes,
    view_rep_handle,
    seqend_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_insert_4(value: &acadrust::entities::insert::Insert) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.block_name);
    let _ = crate::views::view(&value.insert_point);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.column_count);
    let _ = crate::views::view(&value.row_count);
    let _ = crate::views::view(&value.column_spacing);
    let _ = crate::views::view(&value.row_spacing);
    let _ = crate::views::view(&value.attributes);
    let _ = crate::views::view(&value.view_rep_handle);
    let _ = crate::views::view(&value.seqend_handle);
}
view_enum!(acadrust::entities::leader::LeaderPathType {
    StraightLine,
    Spline,
});
view_enum!(acadrust::entities::leader::LeaderCreationType {
    WithText,
    WithTolerance,
    WithBlock,
    NoAnnotation,
});
view_enum!(acadrust::entities::leader::HooklineDirection { Opposite, Same });
view_struct!(acadrust::entities::leader::Leader {
    common,
    dimension_style,
    arrow_enabled,
    path_type,
    creation_type,
    hookline_direction,
    hookline_enabled,
    text_height,
    text_width,
    vertices,
    override_color,
    annotation_handle,
    normal,
    horizontal_direction,
    block_offset,
    annotation_offset,
    origin,
    dimension_gap,
    arrowhead_type,
    arrow_size,
    byblock_color,
    dwg_unknown_bit1,
    dwg_unknown_bit2,
    dwg_unknown_bit3,
    dwg_unknown_bit4,
    dwg_unknown_bit5,
    dwg_unknown_short1
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_leader_8(value: &acadrust::entities::leader::Leader) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.dimension_style);
    let _ = crate::views::view(&value.arrow_enabled);
    let _ = crate::views::view(&value.path_type);
    let _ = crate::views::view(&value.creation_type);
    let _ = crate::views::view(&value.hookline_direction);
    let _ = crate::views::view(&value.hookline_enabled);
    let _ = crate::views::view(&value.text_height);
    let _ = crate::views::view(&value.text_width);
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.override_color);
    let _ = crate::views::view(&value.annotation_handle);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.horizontal_direction);
    let _ = crate::views::view(&value.block_offset);
    let _ = crate::views::view(&value.annotation_offset);
    let _ = crate::views::view(&value.origin);
    let _ = crate::views::view(&value.dimension_gap);
    let _ = crate::views::view(&value.arrowhead_type);
    let _ = crate::views::view(&value.arrow_size);
    let _ = crate::views::view(&value.byblock_color);
    let _ = crate::views::view(&value.dwg_unknown_bit1);
    let _ = crate::views::view(&value.dwg_unknown_bit2);
    let _ = crate::views::view(&value.dwg_unknown_bit3);
    let _ = crate::views::view(&value.dwg_unknown_bit4);
    let _ = crate::views::view(&value.dwg_unknown_bit5);
    let _ = crate::views::view(&value.dwg_unknown_short1);
}
view_struct!(acadrust::entities::light::LightPhotometricData {
    has_web_file,
    web_file,
    physical_intensity_method,
    physical_intensity,
    illuminance_distance,
    lamp_color_type,
    lamp_color_temperature,
    lamp_color_preset,
    web_rotation,
    extended_light_shape,
    extended_light_length,
    extended_light_width,
    extended_light_radius,
    web_file_type,
    web_symmetry,
    has_target_grip,
    web_flux,
    web_angles,
    glyph_display_type
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_light_2(value: &acadrust::entities::light::LightPhotometricData) {
    let _ = crate::views::view(&value.has_web_file);
    let _ = crate::views::view(&value.web_file);
    let _ = crate::views::view(&value.physical_intensity_method);
    let _ = crate::views::view(&value.physical_intensity);
    let _ = crate::views::view(&value.illuminance_distance);
    let _ = crate::views::view(&value.lamp_color_type);
    let _ = crate::views::view(&value.lamp_color_temperature);
    let _ = crate::views::view(&value.lamp_color_preset);
    let _ = crate::views::view(&value.web_rotation);
    let _ = crate::views::view(&value.extended_light_shape);
    let _ = crate::views::view(&value.extended_light_length);
    let _ = crate::views::view(&value.extended_light_width);
    let _ = crate::views::view(&value.extended_light_radius);
    let _ = crate::views::view(&value.web_file_type);
    let _ = crate::views::view(&value.web_symmetry);
    let _ = crate::views::view(&value.has_target_grip);
    let _ = crate::views::view(&value.web_flux);
    let _ = crate::views::view(&value.web_angles);
    let _ = crate::views::view(&value.glyph_display_type);
}
view_struct!(acadrust::entities::light::Light {
    common,
    name,
    class_version,
    light_type,
    position,
    target,
    status,
    light_color,
    plot_glyph,
    intensity,
    attenuation_type,
    use_attenuation_limits,
    attenuation_start_limit,
    attenuation_end_limit,
    hotspot_angle,
    falloff_angle,
    cast_shadows,
    shadow_type,
    shadow_map_size,
    shadow_map_softness,
    photometric_mode,
    photometric_data
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_light_4(value: &acadrust::entities::light::Light) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.light_type);
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.target);
    let _ = crate::views::view(&value.status);
    let _ = crate::views::view(&value.light_color);
    let _ = crate::views::view(&value.plot_glyph);
    let _ = crate::views::view(&value.intensity);
    let _ = crate::views::view(&value.attenuation_type);
    let _ = crate::views::view(&value.use_attenuation_limits);
    let _ = crate::views::view(&value.attenuation_start_limit);
    let _ = crate::views::view(&value.attenuation_end_limit);
    let _ = crate::views::view(&value.hotspot_angle);
    let _ = crate::views::view(&value.falloff_angle);
    let _ = crate::views::view(&value.cast_shadows);
    let _ = crate::views::view(&value.shadow_type);
    let _ = crate::views::view(&value.shadow_map_size);
    let _ = crate::views::view(&value.shadow_map_softness);
    let _ = crate::views::view(&value.photometric_mode);
    let _ = crate::views::view(&value.photometric_data);
}
view_struct!(acadrust::entities::line::Line {
    common,
    start,
    end,
    thickness,
    normal
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_line_2(value: &acadrust::entities::line::Line) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.start);
    let _ = crate::views::view(&value.end);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.normal);
}
view_struct!(acadrust::entities::lwpolyline::LwVertex {
    location,
    bulge,
    start_width,
    end_width,
    vertex_id
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_lwpolyline_2(value: &acadrust::entities::lwpolyline::LwVertex) {
    let _ = crate::views::view(&value.location);
    let _ = crate::views::view(&value.bulge);
    let _ = crate::views::view(&value.start_width);
    let _ = crate::views::view(&value.end_width);
    let _ = crate::views::view(&value.vertex_id);
}
view_struct!(acadrust::entities::lwpolyline::LwPolyline {
    common,
    vertices,
    is_closed,
    plinegen,
    constant_width,
    elevation,
    thickness,
    normal
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_lwpolyline_4(value: &acadrust::entities::lwpolyline::LwPolyline) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.is_closed);
    let _ = crate::views::view(&value.plinegen);
    let _ = crate::views::view(&value.constant_width);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.normal);
}
view_struct!(acadrust::entities::mesh::MeshEdge { start, end, crease });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mesh_2(value: &acadrust::entities::mesh::MeshEdge) {
    let _ = crate::views::view(&value.start);
    let _ = crate::views::view(&value.end);
    let _ = crate::views::view(&value.crease);
}
view_struct!(acadrust::entities::mesh::MeshFace { vertices });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mesh_5(value: &acadrust::entities::mesh::MeshFace) {
    let _ = crate::views::view(&value.vertices);
}
view_struct!(acadrust::entities::mesh::Mesh {
    common,
    version,
    blend_crease,
    subdivision_level,
    vertices,
    faces,
    edges,
    override_option
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mesh_8(value: &acadrust::entities::mesh::Mesh) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.blend_crease);
    let _ = crate::views::view(&value.subdivision_level);
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.faces);
    let _ = crate::views::view(&value.edges);
    let _ = crate::views::view(&value.override_option);
}
view_struct!(acadrust::entities::mesh::MeshBuilder {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mesh_12(value: &acadrust::entities::mesh::MeshBuilder) {}
view_enum!(acadrust::entities::mline::MLineJustification { Top, Zero, Bottom });
impl crate::views::NativeView for acadrust::entities::mline::MLineFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::mline::MLineStyleFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_struct!(acadrust::entities::mline::MLineStyleElement {
    offset,
    color,
    line_type_handle,
    line_type_name
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mline_8(value: &acadrust::entities::mline::MLineStyleElement) {
    let _ = crate::views::view(&value.offset);
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.line_type_handle);
    let _ = crate::views::view(&value.line_type_name);
}
view_struct!(acadrust::entities::mline::MLineStyle {
    handle,
    name,
    description,
    flags,
    fill_color,
    start_angle,
    end_angle,
    elements
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mline_11(value: &acadrust::entities::mline::MLineStyle) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.description);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.fill_color);
    let _ = crate::views::view(&value.start_angle);
    let _ = crate::views::view(&value.end_angle);
    let _ = crate::views::view(&value.elements);
}
view_struct!(acadrust::entities::mline::MLineSegment {
    parameters,
    area_fill_parameters
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mline_14(value: &acadrust::entities::mline::MLineSegment) {
    let _ = crate::views::view(&value.parameters);
    let _ = crate::views::view(&value.area_fill_parameters);
}
view_struct!(acadrust::entities::mline::MLineVertex {
    position,
    direction,
    miter,
    segments
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mline_16(value: &acadrust::entities::mline::MLineVertex) {
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.direction);
    let _ = crate::views::view(&value.miter);
    let _ = crate::views::view(&value.segments);
}
view_struct!(acadrust::entities::mline::MLine {
    common,
    flags,
    justification,
    normal,
    scale_factor,
    start_point,
    style_handle,
    style_name,
    style_element_count,
    vertices
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mline_19(value: &acadrust::entities::mline::MLine) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.justification);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.scale_factor);
    let _ = crate::views::view(&value.start_point);
    let _ = crate::views::view(&value.style_handle);
    let _ = crate::views::view(&value.style_name);
    let _ = crate::views::view(&value.style_element_count);
    let _ = crate::views::view(&value.vertices);
}
view_struct!(acadrust::entities::mline::MLineBuilder {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mline_23(value: &acadrust::entities::mline::MLineBuilder) {}
view_struct!(acadrust::entities::RawRecord {
    data,
    handle_bits,
    version
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_104(value: &acadrust::entities::RawRecord) {
    let _ = crate::views::view(&value.data);
    let _ = crate::views::view(&value.handle_bits);
    let _ = crate::views::view(&value.version);
}
view_struct!(acadrust::entities::R12EntityMetadata {
    viewport,
    child_viewports
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_105(value: &acadrust::entities::R12EntityMetadata) {
    let _ = crate::views::view(&value.viewport);
    let _ = crate::views::view(&value.child_viewports);
}
view_struct!(acadrust::entities::EntityCommon {
    handle,
    layer,
    layer_handle,
    color,
    line_weight,
    linetype,
    linetype_handle,
    linetype_scale,
    transparency,
    color_name,
    invisible,
    extended_data,
    graphic_data,
    reactors,
    xdictionary_handle,
    owner_handle,
    color_book_handle,
    full_visual_style_handle,
    face_visual_style_handle,
    edge_visual_style_handle,
    material_flags,
    material_handle,
    shadow_flags,
    plotstyle_flags,
    plotstyle_handle,
    entity_mode,
    r12_metadata,
    has_ds_data,
    raw_record
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_106(value: &acadrust::entities::EntityCommon) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.layer);
    let _ = crate::views::view(&value.layer_handle);
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.line_weight);
    let _ = crate::views::view(&value.linetype);
    let _ = crate::views::view(&value.linetype_handle);
    let _ = crate::views::view(&value.linetype_scale);
    let _ = crate::views::view(&value.transparency);
    let _ = crate::views::view(&value.color_name);
    let _ = crate::views::view(&value.invisible);
    let _ = crate::views::view(&value.extended_data);
    let _ = crate::views::view(&value.graphic_data);
    let _ = crate::views::view(&value.reactors);
    let _ = crate::views::view(&value.xdictionary_handle);
    let _ = crate::views::view(&value.owner_handle);
    let _ = crate::views::view(&value.color_book_handle);
    let _ = crate::views::view(&value.full_visual_style_handle);
    let _ = crate::views::view(&value.face_visual_style_handle);
    let _ = crate::views::view(&value.edge_visual_style_handle);
    let _ = crate::views::view(&value.material_flags);
    let _ = crate::views::view(&value.material_handle);
    let _ = crate::views::view(&value.shadow_flags);
    let _ = crate::views::view(&value.plotstyle_flags);
    let _ = crate::views::view(&value.plotstyle_handle);
    let _ = crate::views::view(&value.entity_mode);
    let _ = crate::views::view(&value.r12_metadata);
    let _ = crate::views::view(&value.has_ds_data);
    let _ = crate::views::view(&value.raw_record);
}
view_enum!(acadrust::entities::EntityType {
    Point(v0),
    Line(v0),
    Circle(v0),
    Arc(v0),
    Ellipse(v0),
    Polyline(v0),
    Polyline2D(v0),
    Polyline3D(v0),
    LwPolyline(v0),
    Text(v0),
    MText(v0),
    Spline(v0),
    Helix(v0),
    Dimension(v0),
    Hatch(v0),
    Solid(v0),
    Face3D(v0),
    Insert(v0),
    Block(v0),
    BlockEnd(v0),
    Ray(v0),
    XLine(v0),
    Viewport(v0),
    AttributeDefinition(v0),
    AttributeEntity(v0),
    Leader(v0),
    MultiLeader(v0),
    MLine(v0),
    Mesh(v0),
    RasterImage(v0),
    Solid3D(v0),
    Region(v0),
    Body(v0),
    Surface(v0),
    Table(v0),
    Tolerance(v0),
    PolyfaceMesh(v0),
    Wipeout(v0),
    Shape(v0),
    Underlay(v0),
    Seqend(v0),
    Ole2Frame(v0),
    PolygonMesh(v0),
    Light(v0),
    SectionSymbol(v0),
    ViewBorder(v0),
    Extended(v0),
    Unknown(v0),
});
view_enum!(acadrust::entities::mtext::AttachmentPoint {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
});
view_enum!(acadrust::entities::mtext::DrawingDirection {
    LeftToRight,
    TopToBottom,
    ByStyle,
});
view_struct!(acadrust::entities::mtext::MTextColumnData {
    column_type,
    column_count,
    flow_reversed,
    auto_height,
    width,
    gutter,
    heights
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_4(value: &acadrust::entities::mtext::MTextColumnData) {
    let _ = crate::views::view(&value.column_type);
    let _ = crate::views::view(&value.column_count);
    let _ = crate::views::view(&value.flow_reversed);
    let _ = crate::views::view(&value.auto_height);
    let _ = crate::views::view(&value.width);
    let _ = crate::views::view(&value.gutter);
    let _ = crate::views::view(&value.heights);
}
view_struct!(acadrust::entities::mtext::MText {
    common,
    value,
    insertion_point,
    height,
    rectangle_width,
    rectangle_height,
    rotation,
    dwg_x_direction,
    style,
    attachment_point,
    drawing_direction,
    line_spacing_factor,
    line_spacing_style,
    normal,
    background_fill_flags,
    background_scale,
    background_color,
    background_transparency,
    is_annotative,
    column_data,
    extents_width,
    extents_height
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_7(value: &acadrust::entities::mtext::MText) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.value);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.rectangle_width);
    let _ = crate::views::view(&value.rectangle_height);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.dwg_x_direction);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.attachment_point);
    let _ = crate::views::view(&value.drawing_direction);
    let _ = crate::views::view(&value.line_spacing_factor);
    let _ = crate::views::view(&value.line_spacing_style);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.background_fill_flags);
    let _ = crate::views::view(&value.background_scale);
    let _ = crate::views::view(&value.background_color);
    let _ = crate::views::view(&value.background_transparency);
    let _ = crate::views::view(&value.is_annotative);
    let _ = crate::views::view(&value.column_data);
    let _ = crate::views::view(&value.extents_width);
    let _ = crate::views::view(&value.extents_height);
}
view_enum!(acadrust::entities::multileader::LeaderContentType {
    None,
    Block,
    MText,
    Tolerance,
});
view_enum!(acadrust::entities::multileader::MultiLeaderPathType {
    Invisible,
    StraightLineSegments,
    Spline,
});
view_enum!(acadrust::entities::multileader::TextAttachmentType {
    TopOfTopLine,
    MiddleOfTopLine,
    MiddleOfText,
    MiddleOfBottomLine,
    BottomOfBottomLine,
    BottomLine,
    BottomOfTopLineUnderlineBottomLine,
    BottomOfTopLineUnderlineTopLine,
    BottomOfTopLineUnderlineAll,
    CenterOfText,
    CenterOfTextOverline,
});
view_enum!(acadrust::entities::multileader::TextAngleType {
    ParallelToLastLeaderLine,
    Horizontal,
    Optimized,
});
view_enum!(
    acadrust::entities::multileader::BlockContentConnectionType {
        BlockExtents,
        BasePoint,
    }
);
view_enum!(
    acadrust::entities::multileader::TextAttachmentDirectionType {
        Horizontal,
        Vertical,
    }
);
view_enum!(acadrust::entities::multileader::TextAttachmentPointType {
    Left,
    Center,
    Right,
});
view_enum!(acadrust::entities::multileader::TextAlignmentType {
    Left,
    Center,
    Right,
});
view_enum!(acadrust::entities::multileader::FlowDirectionType {
    Horizontal,
    Vertical,
    ByStyle,
});
view_enum!(acadrust::entities::multileader::LineSpacingStyle { AtLeast, Exactly });
impl crate::views::NativeView
    for acadrust::entities::multileader::MultiLeaderPropertyOverrideFlags
{
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::multileader::LeaderLinePropertyOverrideFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_struct!(acadrust::entities::multileader::StartEndPointPair {
    start_point,
    end_point
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_25(
    value: &acadrust::entities::multileader::StartEndPointPair,
) {
    let _ = crate::views::view(&value.start_point);
    let _ = crate::views::view(&value.end_point);
}
view_struct!(acadrust::entities::multileader::LeaderLineBreakInfo {
    segment_index,
    break_points
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_28(
    value: &acadrust::entities::multileader::LeaderLineBreakInfo,
) {
    let _ = crate::views::view(&value.segment_index);
    let _ = crate::views::view(&value.break_points);
}
view_struct!(acadrust::entities::multileader::LeaderLine {
    index,
    segment_index,
    points,
    break_points,
    break_info_count,
    break_infos,
    path_type,
    line_color,
    line_type_handle,
    line_weight,
    arrowhead_handle,
    arrowhead_size,
    override_flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_29(value: &acadrust::entities::multileader::LeaderLine) {
    let _ = crate::views::view(&value.index);
    let _ = crate::views::view(&value.segment_index);
    let _ = crate::views::view(&value.points);
    let _ = crate::views::view(&value.break_points);
    let _ = crate::views::view(&value.break_info_count);
    let _ = crate::views::view(&value.break_infos);
    let _ = crate::views::view(&value.path_type);
    let _ = crate::views::view(&value.line_color);
    let _ = crate::views::view(&value.line_type_handle);
    let _ = crate::views::view(&value.line_weight);
    let _ = crate::views::view(&value.arrowhead_handle);
    let _ = crate::views::view(&value.arrowhead_size);
    let _ = crate::views::view(&value.override_flags);
}
view_struct!(acadrust::entities::multileader::LeaderRoot {
    leader_index,
    content_valid,
    unknown,
    connection_point,
    direction,
    break_points,
    lines,
    landing_distance,
    text_attachment_direction
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_32(value: &acadrust::entities::multileader::LeaderRoot) {
    let _ = crate::views::view(&value.leader_index);
    let _ = crate::views::view(&value.content_valid);
    let _ = crate::views::view(&value.unknown);
    let _ = crate::views::view(&value.connection_point);
    let _ = crate::views::view(&value.direction);
    let _ = crate::views::view(&value.break_points);
    let _ = crate::views::view(&value.lines);
    let _ = crate::views::view(&value.landing_distance);
    let _ = crate::views::view(&value.text_attachment_direction);
}
view_struct!(acadrust::entities::multileader::BlockAttribute {
    attribute_definition_handle,
    index,
    width,
    text
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_35(value: &acadrust::entities::multileader::BlockAttribute) {
    let _ = crate::views::view(&value.attribute_definition_handle);
    let _ = crate::views::view(&value.index);
    let _ = crate::views::view(&value.width);
    let _ = crate::views::view(&value.text);
}
view_struct!(
    acadrust::entities::multileader::MultiLeaderArrowheadOverride {
        index,
        is_default,
        arrowhead_handle
    }
);
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_38(
    value: &acadrust::entities::multileader::MultiLeaderArrowheadOverride,
) {
    let _ = crate::views::view(&value.index);
    let _ = crate::views::view(&value.is_default);
    let _ = crate::views::view(&value.arrowhead_handle);
}
view_struct!(acadrust::entities::multileader::MultiLeaderAnnotContext {
    leader_roots,
    standalone_flags,
    standalone_uses_root_flags,
    scale_factor,
    content_base_point,
    has_text_contents,
    text_string,
    text_normal,
    text_location,
    text_direction,
    text_rotation,
    text_height,
    text_width,
    text_boundary_height,
    line_spacing_factor,
    line_spacing_style,
    text_color,
    text_attachment_point,
    text_flow_direction,
    text_alignment,
    text_left_attachment,
    text_right_attachment,
    text_top_attachment,
    text_bottom_attachment,
    text_height_automatic,
    word_break,
    dwg_unknown_text_bit,
    text_style_handle,
    has_block_contents,
    block_content_handle,
    block_content_normal,
    block_content_location,
    block_content_scale,
    block_rotation,
    block_content_color,
    block_connection_type,
    column_type,
    column_width,
    column_gutter,
    column_flow_reversed,
    column_sizes,
    background_fill_enabled,
    background_mask_fill_on,
    background_fill_color,
    background_scale_factor,
    background_transparency,
    base_point,
    base_direction,
    base_vertical,
    normal_reversed,
    arrowhead_size,
    landing_gap,
    transform_matrix,
    scale_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_39(
    value: &acadrust::entities::multileader::MultiLeaderAnnotContext,
) {
    let _ = crate::views::view(&value.leader_roots);
    let _ = crate::views::view(&value.standalone_flags);
    let _ = crate::views::view(&value.standalone_uses_root_flags);
    let _ = crate::views::view(&value.scale_factor);
    let _ = crate::views::view(&value.content_base_point);
    let _ = crate::views::view(&value.has_text_contents);
    let _ = crate::views::view(&value.text_string);
    let _ = crate::views::view(&value.text_normal);
    let _ = crate::views::view(&value.text_location);
    let _ = crate::views::view(&value.text_direction);
    let _ = crate::views::view(&value.text_rotation);
    let _ = crate::views::view(&value.text_height);
    let _ = crate::views::view(&value.text_width);
    let _ = crate::views::view(&value.text_boundary_height);
    let _ = crate::views::view(&value.line_spacing_factor);
    let _ = crate::views::view(&value.line_spacing_style);
    let _ = crate::views::view(&value.text_color);
    let _ = crate::views::view(&value.text_attachment_point);
    let _ = crate::views::view(&value.text_flow_direction);
    let _ = crate::views::view(&value.text_alignment);
    let _ = crate::views::view(&value.text_left_attachment);
    let _ = crate::views::view(&value.text_right_attachment);
    let _ = crate::views::view(&value.text_top_attachment);
    let _ = crate::views::view(&value.text_bottom_attachment);
    let _ = crate::views::view(&value.text_height_automatic);
    let _ = crate::views::view(&value.word_break);
    let _ = crate::views::view(&value.dwg_unknown_text_bit);
    let _ = crate::views::view(&value.text_style_handle);
    let _ = crate::views::view(&value.has_block_contents);
    let _ = crate::views::view(&value.block_content_handle);
    let _ = crate::views::view(&value.block_content_normal);
    let _ = crate::views::view(&value.block_content_location);
    let _ = crate::views::view(&value.block_content_scale);
    let _ = crate::views::view(&value.block_rotation);
    let _ = crate::views::view(&value.block_content_color);
    let _ = crate::views::view(&value.block_connection_type);
    let _ = crate::views::view(&value.column_type);
    let _ = crate::views::view(&value.column_width);
    let _ = crate::views::view(&value.column_gutter);
    let _ = crate::views::view(&value.column_flow_reversed);
    let _ = crate::views::view(&value.column_sizes);
    let _ = crate::views::view(&value.background_fill_enabled);
    let _ = crate::views::view(&value.background_mask_fill_on);
    let _ = crate::views::view(&value.background_fill_color);
    let _ = crate::views::view(&value.background_scale_factor);
    let _ = crate::views::view(&value.background_transparency);
    let _ = crate::views::view(&value.base_point);
    let _ = crate::views::view(&value.base_direction);
    let _ = crate::views::view(&value.base_vertical);
    let _ = crate::views::view(&value.normal_reversed);
    let _ = crate::views::view(&value.arrowhead_size);
    let _ = crate::views::view(&value.landing_gap);
    let _ = crate::views::view(&value.transform_matrix);
    let _ = crate::views::view(&value.scale_handle);
}
view_struct!(acadrust::entities::multileader::MultiLeader {
    common,
    dwg_version,
    style_handle,
    content_type,
    context,
    block_attributes,
    arrowhead_overrides,
    path_type,
    line_color,
    line_type_handle,
    line_weight,
    enable_landing,
    enable_dogleg,
    dogleg_length,
    arrowhead_handle,
    arrowhead_size,
    text_style_handle,
    text_color,
    text_frame,
    text_height,
    text_left_attachment,
    text_right_attachment,
    text_top_attachment,
    text_bottom_attachment,
    text_attachment_direction,
    text_attachment_point,
    text_alignment,
    text_angle_type,
    text_direction_negative,
    text_align_in_ipe,
    block_content_handle,
    block_content_color,
    block_connection_type,
    block_rotation,
    block_scale,
    scale_factor,
    property_override_flags,
    enable_annotation_scale,
    extend_leader_to_text
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_42(value: &acadrust::entities::multileader::MultiLeader) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.dwg_version);
    let _ = crate::views::view(&value.style_handle);
    let _ = crate::views::view(&value.content_type);
    let _ = crate::views::view(&value.context);
    let _ = crate::views::view(&value.block_attributes);
    let _ = crate::views::view(&value.arrowhead_overrides);
    let _ = crate::views::view(&value.path_type);
    let _ = crate::views::view(&value.line_color);
    let _ = crate::views::view(&value.line_type_handle);
    let _ = crate::views::view(&value.line_weight);
    let _ = crate::views::view(&value.enable_landing);
    let _ = crate::views::view(&value.enable_dogleg);
    let _ = crate::views::view(&value.dogleg_length);
    let _ = crate::views::view(&value.arrowhead_handle);
    let _ = crate::views::view(&value.arrowhead_size);
    let _ = crate::views::view(&value.text_style_handle);
    let _ = crate::views::view(&value.text_color);
    let _ = crate::views::view(&value.text_frame);
    let _ = crate::views::view(&value.text_height);
    let _ = crate::views::view(&value.text_left_attachment);
    let _ = crate::views::view(&value.text_right_attachment);
    let _ = crate::views::view(&value.text_top_attachment);
    let _ = crate::views::view(&value.text_bottom_attachment);
    let _ = crate::views::view(&value.text_attachment_direction);
    let _ = crate::views::view(&value.text_attachment_point);
    let _ = crate::views::view(&value.text_alignment);
    let _ = crate::views::view(&value.text_angle_type);
    let _ = crate::views::view(&value.text_direction_negative);
    let _ = crate::views::view(&value.text_align_in_ipe);
    let _ = crate::views::view(&value.block_content_handle);
    let _ = crate::views::view(&value.block_content_color);
    let _ = crate::views::view(&value.block_connection_type);
    let _ = crate::views::view(&value.block_rotation);
    let _ = crate::views::view(&value.block_scale);
    let _ = crate::views::view(&value.scale_factor);
    let _ = crate::views::view(&value.property_override_flags);
    let _ = crate::views::view(&value.enable_annotation_scale);
    let _ = crate::views::view(&value.extend_leader_to_text);
}
view_struct!(acadrust::entities::multileader::MultiLeaderBuilder {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_multileader_46(
    value: &acadrust::entities::multileader::MultiLeaderBuilder,
) {
}
view_enum!(acadrust::entities::ole2frame::OleFrameEnvelope {
    None,
    Geometry { marker, extension_records },
    Legacy { records },
});
view_enum!(acadrust::entities::ole2frame::OleObjectType {
    Link,
    Embedded,
    Static,
});
view_struct!(acadrust::entities::ole2frame::Ole2Frame {
    common,
    version,
    source_application,
    upper_left_corner,
    lower_right_corner,
    ole_object_type,
    is_paper_space,
    storage,
    envelope,
    dwg_mode,
    lock_aspect
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_ole2frame_5(value: &acadrust::entities::ole2frame::Ole2Frame) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.source_application);
    let _ = crate::views::view(&value.upper_left_corner);
    let _ = crate::views::view(&value.lower_right_corner);
    let _ = crate::views::view(&value.ole_object_type);
    let _ = crate::views::view(&value.is_paper_space);
    let _ = crate::views::view(&value.storage);
    let _ = crate::views::view(&value.envelope);
    let _ = crate::views::view(&value.dwg_mode);
    let _ = crate::views::view(&value.lock_aspect);
}
view_enum!(acadrust::entities::ole_presentation::OlePresentation {
    Raster(v0),
    Dib(v0),
    Emf(v0),
    Wmf(v0),
});
view_struct!(acadrust::entities::point::Point {
    common,
    location,
    thickness,
    normal,
    x_axis_angle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_point_2(value: &acadrust::entities::point::Point) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.location);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.x_axis_angle);
}
impl crate::views::NativeView for acadrust::entities::polyface_mesh::PolyfaceMeshFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::polyface_mesh::PolyfaceVertexFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_enum!(acadrust::entities::polyface_mesh::PolyfaceSmoothType {
    None,
    Quadratic,
    Cubic,
    Bezier,
});
view_struct!(acadrust::entities::polyface_mesh::PolyfaceVertex {
    common,
    location,
    flags,
    bulge,
    start_width,
    end_width,
    curve_tangent,
    id
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyface_mesh_7(
    value: &acadrust::entities::polyface_mesh::PolyfaceVertex,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.location);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.bulge);
    let _ = crate::views::view(&value.start_width);
    let _ = crate::views::view(&value.end_width);
    let _ = crate::views::view(&value.curve_tangent);
    let _ = crate::views::view(&value.id);
}
view_struct!(acadrust::entities::polyface_mesh::PolyfaceFace {
    common,
    flags,
    index1,
    index2,
    index3,
    index4,
    color
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyface_mesh_10(
    value: &acadrust::entities::polyface_mesh::PolyfaceFace,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.index1);
    let _ = crate::views::view(&value.index2);
    let _ = crate::views::view(&value.index3);
    let _ = crate::views::view(&value.index4);
    let _ = crate::views::view(&value.color);
}
view_struct!(acadrust::entities::polyface_mesh::PolyfaceMesh {
    common,
    elevation,
    flags,
    normal,
    start_width,
    end_width,
    smooth_surface,
    thickness,
    vertices,
    faces,
    seqend_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyface_mesh_13(
    value: &acadrust::entities::polyface_mesh::PolyfaceMesh,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.start_width);
    let _ = crate::views::view(&value.end_width);
    let _ = crate::views::view(&value.smooth_surface);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.vertices);
    let _ = crate::views::view(&value.faces);
    let _ = crate::views::view(&value.seqend_handle);
}
view_enum!(acadrust::entities::polygon_mesh::SurfaceSmoothType {
    NoSmooth,
    Quadratic,
    Cubic,
    Bezier,
});
view_struct!(acadrust::entities::polygon_mesh::PolygonMeshVertex {
    common,
    location,
    flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polygon_mesh_4(
    value: &acadrust::entities::polygon_mesh::PolygonMeshVertex,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.location);
    let _ = crate::views::view(&value.flags);
}
impl crate::views::NativeView for acadrust::entities::polygon_mesh::PolygonMeshFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_struct!(acadrust::entities::polygon_mesh::PolygonMesh {
    common,
    flags,
    m_vertex_count,
    n_vertex_count,
    m_smooth_density,
    n_smooth_density,
    smooth_type,
    elevation,
    normal,
    vertices
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polygon_mesh_8(value: &acadrust::entities::polygon_mesh::PolygonMesh) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.m_vertex_count);
    let _ = crate::views::view(&value.n_vertex_count);
    let _ = crate::views::view(&value.m_smooth_density);
    let _ = crate::views::view(&value.n_smooth_density);
    let _ = crate::views::view(&value.smooth_type);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.vertices);
}
view_enum!(acadrust::entities::polyline::SmoothSurfaceType {
    None,
    QuadraticBSpline,
    CubicBSpline,
    Bezier,
});
view_struct!(acadrust::entities::polyline::Vertex2D {
    location,
    flags,
    start_width,
    end_width,
    bulge,
    curve_tangent,
    id
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline_10(value: &acadrust::entities::polyline::Vertex2D) {
    let _ = crate::views::view(&value.location);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.start_width);
    let _ = crate::views::view(&value.end_width);
    let _ = crate::views::view(&value.bulge);
    let _ = crate::views::view(&value.curve_tangent);
    let _ = crate::views::view(&value.id);
}
view_struct!(acadrust::entities::polyline::Vertex3D { location, flags });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline_12(value: &acadrust::entities::polyline::Vertex3D) {
    let _ = crate::views::view(&value.location);
    let _ = crate::views::view(&value.flags);
}
view_struct!(acadrust::entities::polyline::Polyline2D {
    common,
    flags,
    smooth_surface,
    start_width,
    end_width,
    thickness,
    elevation,
    normal,
    vertices
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline_14(value: &acadrust::entities::polyline::Polyline2D) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.smooth_surface);
    let _ = crate::views::view(&value.start_width);
    let _ = crate::views::view(&value.end_width);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.vertices);
}
view_struct!(acadrust::entities::polyline::Polyline {
    common,
    flags,
    vertices
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline_17(value: &acadrust::entities::polyline::Polyline) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.vertices);
}
view_struct!(acadrust::entities::polyline3d::Polyline3DFlags {
    closed,
    spline_fit,
    is_3d,
    is_3d_mesh,
    mesh_closed_n,
    is_polyface_mesh,
    linetype_continuous
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline3d_2(value: &acadrust::entities::polyline3d::Polyline3DFlags) {
    let _ = crate::views::view(&value.closed);
    let _ = crate::views::view(&value.spline_fit);
    let _ = crate::views::view(&value.is_3d);
    let _ = crate::views::view(&value.is_3d_mesh);
    let _ = crate::views::view(&value.mesh_closed_n);
    let _ = crate::views::view(&value.is_polyface_mesh);
    let _ = crate::views::view(&value.linetype_continuous);
}
view_enum!(acadrust::entities::polyline3d::SmoothSurfaceType {
    None,
    QuadraticBSpline,
    CubicBSpline,
    Bezier,
});
view_struct!(acadrust::entities::polyline3d::Vertex3DPolyline {
    handle,
    layer,
    position,
    flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline3d_6(value: &acadrust::entities::polyline3d::Vertex3DPolyline) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.layer);
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.flags);
}
view_struct!(acadrust::entities::polyline3d::Polyline3D {
    common,
    flags,
    smooth_type,
    default_start_width,
    default_end_width,
    mesh_m_count,
    mesh_n_count,
    smooth_m_density,
    smooth_n_density,
    elevation,
    normal,
    vertices
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_polyline3d_9(value: &acadrust::entities::polyline3d::Polyline3D) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.smooth_type);
    let _ = crate::views::view(&value.default_start_width);
    let _ = crate::views::view(&value.default_end_width);
    let _ = crate::views::view(&value.mesh_m_count);
    let _ = crate::views::view(&value.mesh_n_count);
    let _ = crate::views::view(&value.smooth_m_density);
    let _ = crate::views::view(&value.smooth_n_density);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.vertices);
}
view_struct!(acadrust::entities::proxy_graphics::ProxyGraphics { records });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_proxy_graphics_4(
    value: &acadrust::entities::proxy_graphics::ProxyGraphics,
) {
    let _ = crate::views::view(&value.records);
}
view_enum!(acadrust::entities::proxy_graphics::ProxyGraphicRecord {
    FillOff,
    UnicodeText(v0),
    Unknown { record_type, data },
});
view_struct!(acadrust::entities::proxy_graphics::ProxyUnicodeText {
    position,
    normal,
    direction,
    height,
    width_factor,
    oblique_angle,
    text
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_proxy_graphics_6(
    value: &acadrust::entities::proxy_graphics::ProxyUnicodeText,
) {
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.direction);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.width_factor);
    let _ = crate::views::view(&value.oblique_angle);
    let _ = crate::views::view(&value.text);
}
view_enum!(acadrust::entities::raster_image::ClipMode { Outside, Inside });
view_enum!(acadrust::entities::raster_image::ClipType {
    Rectangular,
    Polygonal,
});
view_enum!(acadrust::entities::raster_image::ResolutionUnit {
    None,
    Centimeters,
    Inches,
});
view_enum!(acadrust::entities::raster_image::ImageDisplayQuality { Draft, High });
impl crate::views::NativeView for acadrust::entities::raster_image::ImageDisplayFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_struct!(acadrust::entities::raster_image::ClipBoundary {
    clip_type,
    clip_mode,
    vertices
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_raster_image_12(value: &acadrust::entities::raster_image::ClipBoundary) {
    let _ = crate::views::view(&value.clip_type);
    let _ = crate::views::view(&value.clip_mode);
    let _ = crate::views::view(&value.vertices);
}
view_struct!(acadrust::entities::raster_image::RasterImage {
    common,
    class_version,
    insertion_point,
    u_vector,
    v_vector,
    size,
    flags,
    clipping_enabled,
    brightness,
    contrast,
    fade,
    clip_boundary,
    definition_handle,
    definition_reactor_handle,
    file_path
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_raster_image_15(value: &acadrust::entities::raster_image::RasterImage) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.u_vector);
    let _ = crate::views::view(&value.v_vector);
    let _ = crate::views::view(&value.size);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.clipping_enabled);
    let _ = crate::views::view(&value.brightness);
    let _ = crate::views::view(&value.contrast);
    let _ = crate::views::view(&value.fade);
    let _ = crate::views::view(&value.clip_boundary);
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.definition_reactor_handle);
    let _ = crate::views::view(&value.file_path);
}
view_struct!(acadrust::entities::raster_image::ImageDefinition {
    handle,
    class_version,
    file_path,
    size,
    pixel_size,
    is_loaded,
    resolution_unit
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_raster_image_19(
    value: &acadrust::entities::raster_image::ImageDefinition,
) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.file_path);
    let _ = crate::views::view(&value.size);
    let _ = crate::views::view(&value.pixel_size);
    let _ = crate::views::view(&value.is_loaded);
    let _ = crate::views::view(&value.resolution_unit);
}
view_struct!(acadrust::entities::raster_image::RasterImageBuilder {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_raster_image_22(
    value: &acadrust::entities::raster_image::RasterImageBuilder,
) {
}
view_struct!(acadrust::entities::ray::Ray {
    common,
    base_point,
    direction
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_ray_2(value: &acadrust::entities::ray::Ray) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.base_point);
    let _ = crate::views::view(&value.direction);
}
view_struct!(acadrust::entities::section_symbol::SectionSymbolPoint {
    point,
    bulge,
    label,
    label_offset,
    raw_flag_280
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_section_symbol_2(
    value: &acadrust::entities::section_symbol::SectionSymbolPoint,
) {
    let _ = crate::views::view(&value.point);
    let _ = crate::views::view(&value.bulge);
    let _ = crate::views::view(&value.label);
    let _ = crate::views::view(&value.label_offset);
    let _ = crate::views::view(&value.raw_flag_280);
}
view_struct!(acadrust::entities::section_symbol::SectionSymbol {
    common,
    view_symbol_version,
    style_handle,
    symbol_scale,
    view_rep_handle,
    raw_view_symbol_70,
    version,
    raw_point_count_90,
    raw_flags_90,
    raw_point_record_count,
    points,
    end_a,
    end_b,
    tick_a,
    tick_b,
    label
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_section_symbol_5(
    value: &acadrust::entities::section_symbol::SectionSymbol,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.view_symbol_version);
    let _ = crate::views::view(&value.style_handle);
    let _ = crate::views::view(&value.symbol_scale);
    let _ = crate::views::view(&value.view_rep_handle);
    let _ = crate::views::view(&value.raw_view_symbol_70);
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.raw_point_count_90);
    let _ = crate::views::view(&value.raw_flags_90);
    let _ = crate::views::view(&value.raw_point_record_count);
    let _ = crate::views::view(&value.points);
    let _ = crate::views::view(&value.end_a);
    let _ = crate::views::view(&value.end_b);
    let _ = crate::views::view(&value.tick_a);
    let _ = crate::views::view(&value.tick_b);
    let _ = crate::views::view(&value.label);
}
view_struct!(acadrust::entities::section_symbol::SectionViewStyle {
    show_arrows,
    show_plane_line,
    show_end_lines,
    arrow_size,
    arrow_extension,
    label_height,
    label_offset,
    label_position,
    arrow_position,
    end_line_length,
    end_line_overshoot,
    arrow_start_handle,
    arrow_end_handle,
    arrow_is_default
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_section_symbol_9(
    value: &acadrust::entities::section_symbol::SectionViewStyle,
) {
    let _ = crate::views::view(&value.show_arrows);
    let _ = crate::views::view(&value.show_plane_line);
    let _ = crate::views::view(&value.show_end_lines);
    let _ = crate::views::view(&value.arrow_size);
    let _ = crate::views::view(&value.arrow_extension);
    let _ = crate::views::view(&value.label_height);
    let _ = crate::views::view(&value.label_offset);
    let _ = crate::views::view(&value.label_position);
    let _ = crate::views::view(&value.arrow_position);
    let _ = crate::views::view(&value.end_line_length);
    let _ = crate::views::view(&value.end_line_overshoot);
    let _ = crate::views::view(&value.arrow_start_handle);
    let _ = crate::views::view(&value.arrow_end_handle);
    let _ = crate::views::view(&value.arrow_is_default);
}
view_struct!(acadrust::entities::seqend::Seqend { common });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_seqend_2(value: &acadrust::entities::seqend::Seqend) {
    let _ = crate::views::view(&value.common);
}
view_struct!(acadrust::entities::shape::Shape {
    common,
    insertion_point,
    size,
    shape_name,
    shape_number,
    rotation,
    relative_x_scale,
    oblique_angle,
    normal,
    thickness,
    style_name,
    style_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_shape_2(value: &acadrust::entities::shape::Shape) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.size);
    let _ = crate::views::view(&value.shape_name);
    let _ = crate::views::view(&value.shape_number);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.relative_x_scale);
    let _ = crate::views::view(&value.oblique_angle);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.style_name);
    let _ = crate::views::view(&value.style_handle);
}
view_struct!(acadrust::entities::solid::Solid {
    common,
    first_corner,
    second_corner,
    third_corner,
    fourth_corner,
    normal,
    thickness,
    is_trace
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid_2(value: &acadrust::entities::solid::Solid) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.first_corner);
    let _ = crate::views::view(&value.second_corner);
    let _ = crate::views::view(&value.third_corner);
    let _ = crate::views::view(&value.fourth_corner);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.is_trace);
}
view_enum!(acadrust::entities::solid3d::WireType {
    Unknown,
    Silhouette,
    VisibleEdge,
    HiddenEdge,
    Isoline,
});
view_struct!(acadrust::entities::solid3d::Wire {
    acis_index,
    wire_type,
    selection_marker,
    color,
    points,
    has_transform,
    has_rotation,
    has_shear,
    has_reflection,
    scale,
    translation,
    x_axis,
    y_axis,
    z_axis
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_4(value: &acadrust::entities::solid3d::Wire) {
    let _ = crate::views::view(&value.acis_index);
    let _ = crate::views::view(&value.wire_type);
    let _ = crate::views::view(&value.selection_marker);
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.points);
    let _ = crate::views::view(&value.has_transform);
    let _ = crate::views::view(&value.has_rotation);
    let _ = crate::views::view(&value.has_shear);
    let _ = crate::views::view(&value.has_reflection);
    let _ = crate::views::view(&value.scale);
    let _ = crate::views::view(&value.translation);
    let _ = crate::views::view(&value.x_axis);
    let _ = crate::views::view(&value.y_axis);
    let _ = crate::views::view(&value.z_axis);
}
view_struct!(acadrust::entities::solid3d::Silhouette {
    viewport_id,
    view_direction,
    up_vector,
    target,
    is_perspective,
    has_wires,
    wires
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_7(value: &acadrust::entities::solid3d::Silhouette) {
    let _ = crate::views::view(&value.viewport_id);
    let _ = crate::views::view(&value.view_direction);
    let _ = crate::views::view(&value.up_vector);
    let _ = crate::views::view(&value.target);
    let _ = crate::views::view(&value.is_perspective);
    let _ = crate::views::view(&value.has_wires);
    let _ = crate::views::view(&value.wires);
}
view_enum!(acadrust::entities::solid3d::AcisVersion { Version1, Version2 });
view_struct!(acadrust::entities::solid3d::AcisRevision {
    has_guid,
    major,
    minor1,
    minor2,
    bytes,
    end_marker
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_12(value: &acadrust::entities::solid3d::AcisRevision) {
    let _ = crate::views::view(&value.has_guid);
    let _ = crate::views::view(&value.major);
    let _ = crate::views::view(&value.minor1);
    let _ = crate::views::view(&value.minor2);
    let _ = crate::views::view(&value.bytes);
    let _ = crate::views::view(&value.end_marker);
}
view_struct!(acadrust::entities::solid3d::AcisMaterial {
    array_index,
    absolute_reference,
    material_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_13(value: &acadrust::entities::solid3d::AcisMaterial) {
    let _ = crate::views::view(&value.array_index);
    let _ = crate::views::view(&value.absolute_reference);
    let _ = crate::views::view(&value.material_handle);
}
view_struct!(acadrust::entities::solid3d::AcisData {
    version,
    sat_data,
    sab_data,
    is_binary,
    revision,
    materials,
    wireframe_data_present,
    wireframe_point_present,
    wireframe_isoline_present,
    acis_empty_bit,
    extra_acis_data,
    wireframe_isolines
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_14(value: &acadrust::entities::solid3d::AcisData) {
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.sat_data);
    let _ = crate::views::view(&value.sab_data);
    let _ = crate::views::view(&value.is_binary);
    let _ = crate::views::view(&value.revision);
    let _ = crate::views::view(&value.materials);
    let _ = crate::views::view(&value.wireframe_data_present);
    let _ = crate::views::view(&value.wireframe_point_present);
    let _ = crate::views::view(&value.wireframe_isoline_present);
    let _ = crate::views::view(&value.acis_empty_bit);
    let _ = crate::views::view(&value.extra_acis_data);
    let _ = crate::views::view(&value.wireframe_isolines);
}
view_struct!(acadrust::entities::solid3d::Solid3D {
    common,
    uid,
    point_of_reference,
    acis_data,
    wires,
    silhouettes,
    history_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_18(value: &acadrust::entities::solid3d::Solid3D) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.uid);
    let _ = crate::views::view(&value.point_of_reference);
    let _ = crate::views::view(&value.acis_data);
    let _ = crate::views::view(&value.wires);
    let _ = crate::views::view(&value.silhouettes);
    let _ = crate::views::view(&value.history_handle);
}
view_struct!(acadrust::entities::solid3d::Region {
    common,
    uid,
    point_of_reference,
    acis_data,
    wires,
    silhouettes,
    history_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_22(value: &acadrust::entities::solid3d::Region) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.uid);
    let _ = crate::views::view(&value.point_of_reference);
    let _ = crate::views::view(&value.acis_data);
    let _ = crate::views::view(&value.wires);
    let _ = crate::views::view(&value.silhouettes);
    let _ = crate::views::view(&value.history_handle);
}
view_struct!(acadrust::entities::solid3d::Body {
    common,
    uid,
    point_of_reference,
    acis_data,
    wires,
    silhouettes,
    history_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_solid3d_26(value: &acadrust::entities::solid3d::Body) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.uid);
    let _ = crate::views::view(&value.point_of_reference);
    let _ = crate::views::view(&value.acis_data);
    let _ = crate::views::view(&value.wires);
    let _ = crate::views::view(&value.silhouettes);
    let _ = crate::views::view(&value.history_handle);
}
view_struct!(acadrust::entities::spline::SplineFlags {
    closed,
    periodic,
    rational,
    planar,
    linear
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_spline_2(value: &acadrust::entities::spline::SplineFlags) {
    let _ = crate::views::view(&value.closed);
    let _ = crate::views::view(&value.periodic);
    let _ = crate::views::view(&value.rational);
    let _ = crate::views::view(&value.planar);
    let _ = crate::views::view(&value.linear);
}
view_struct!(acadrust::entities::spline::Spline {
    common,
    degree,
    flags,
    knots,
    control_points,
    weights,
    fit_points,
    normal,
    knot_tolerance,
    control_tolerance,
    fit_tolerance,
    begin_tangent,
    end_tangent,
    knot_parameterization,
    cv_frame_visible,
    dwg_flags1,
    dwg_scenario,
    dxf_flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_spline_5(value: &acadrust::entities::spline::Spline) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.degree);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.knots);
    let _ = crate::views::view(&value.control_points);
    let _ = crate::views::view(&value.weights);
    let _ = crate::views::view(&value.fit_points);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.knot_tolerance);
    let _ = crate::views::view(&value.control_tolerance);
    let _ = crate::views::view(&value.fit_tolerance);
    let _ = crate::views::view(&value.begin_tangent);
    let _ = crate::views::view(&value.end_tangent);
    let _ = crate::views::view(&value.knot_parameterization);
    let _ = crate::views::view(&value.cv_frame_visible);
    let _ = crate::views::view(&value.dwg_flags1);
    let _ = crate::views::view(&value.dwg_scenario);
    let _ = crate::views::view(&value.dxf_flags);
}
view_enum!(acadrust::entities::surface::SurfaceKind {
    Generic,
    Plane,
    Extruded,
    Lofted,
    Revolved,
    Swept,
    Nurb,
});
view_struct!(acadrust::entities::surface::SurfaceSweepOptions {
    draft_angle,
    draft_start_distance,
    draft_end_distance,
    twist_angle,
    scale_factor,
    align_angle,
    sweep_entity_transform,
    path_entity_transform,
    is_solid,
    sweep_alignment_flags,
    path_flags,
    align_start,
    bank,
    base_point_set,
    sweep_entity_transform_computed,
    path_entity_transform_computed,
    reference_vector
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_surface_5(value: &acadrust::entities::surface::SurfaceSweepOptions) {
    let _ = crate::views::view(&value.draft_angle);
    let _ = crate::views::view(&value.draft_start_distance);
    let _ = crate::views::view(&value.draft_end_distance);
    let _ = crate::views::view(&value.twist_angle);
    let _ = crate::views::view(&value.scale_factor);
    let _ = crate::views::view(&value.align_angle);
    let _ = crate::views::view(&value.sweep_entity_transform);
    let _ = crate::views::view(&value.path_entity_transform);
    let _ = crate::views::view(&value.is_solid);
    let _ = crate::views::view(&value.sweep_alignment_flags);
    let _ = crate::views::view(&value.path_flags);
    let _ = crate::views::view(&value.align_start);
    let _ = crate::views::view(&value.bank);
    let _ = crate::views::view(&value.base_point_set);
    let _ = crate::views::view(&value.sweep_entity_transform_computed);
    let _ = crate::views::view(&value.path_entity_transform_computed);
    let _ = crate::views::view(&value.reference_vector);
}
view_enum!(acadrust::entities::surface::SurfaceData {
    Generic,
    Plane { class_version },
    Extruded { sweep_entity, options, sweep_vector, sweep_transform },
    Lofted { loft_transform, cross_section_entities, guide_entities, path_entity, plane_normal_lofting_type, start_draft_angle, end_draft_angle, start_draft_magnitude, end_draft_magnitude, arc_length_parameterization, no_twist, align_direction, simple_surfaces, closed_surfaces, solid, ruled_surface, virtual_guide, cross_sections, guide_curves, path_curve },
    Revolved { revolve_entity, class_version, entity_id, axis_point, axis_vector, revolve_angle, start_angle, entity_transform, draft_angle, draft_start_distance, draft_end_distance, twist_angle, solid, close_to_axis },
    Swept { class_version, sweep_entity, path_entity, sweep_transform, path_transform, options },
    Nurb { short_170, cv_hull_display, u_vector1, v_vector1, u_vector2, v_vector2 },
});
view_struct!(acadrust::entities::surface::Surface {
    common,
    kind,
    acis_data,
    modeler_format_version,
    u_isolines,
    v_isolines,
    surface_data,
    point_of_reference,
    history_handle,
    wires,
    silhouettes
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_surface_10(value: &acadrust::entities::surface::Surface) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.kind);
    let _ = crate::views::view(&value.acis_data);
    let _ = crate::views::view(&value.modeler_format_version);
    let _ = crate::views::view(&value.u_isolines);
    let _ = crate::views::view(&value.v_isolines);
    let _ = crate::views::view(&value.surface_data);
    let _ = crate::views::view(&value.point_of_reference);
    let _ = crate::views::view(&value.history_handle);
    let _ = crate::views::view(&value.wires);
    let _ = crate::views::view(&value.silhouettes);
}
view_enum!(acadrust::entities::table::CellType { Text, Block });
view_enum!(acadrust::entities::table::CellValueType {
    Unknown,
    Long,
    Double,
    String,
    Date,
    Point2D,
    Point3D,
    Handle,
    Buffer,
    ResultBuffer,
    General,
});
view_enum!(acadrust::entities::table::ValueUnitType {
    NoUnits,
    Distance,
    Angle,
    Area,
    Volume,
    Currency,
    Percentage,
});
view_enum!(acadrust::entities::table::BorderType { Single, Double });
view_enum!(acadrust::entities::table::TableCellContentType {
    Unknown,
    Value,
    Field,
    Block,
});
view_enum!(acadrust::entities::table::CellStyleType {
    Cell,
    Row,
    Column,
    FormattedTableData,
    Table,
});
view_enum!(acadrust::entities::table::BreakFlowDirection {
    Right,
    Vertical,
    Left,
});
impl crate::views::NativeView for acadrust::entities::table::CellEdgeFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::table::CellStateFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::table::CellStylePropertyFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::table::BorderPropertyFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::table::ContentLayoutFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
impl crate::views::NativeView for acadrust::entities::table::BreakOptionFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_struct!(acadrust::entities::table::CellBorder {
    border_type,
    color,
    line_weight,
    line_type_handle,
    invisible,
    double_spacing,
    override_flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_23(value: &acadrust::entities::table::CellBorder) {
    let _ = crate::views::view(&value.border_type);
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.line_weight);
    let _ = crate::views::view(&value.line_type_handle);
    let _ = crate::views::view(&value.invisible);
    let _ = crate::views::view(&value.double_spacing);
    let _ = crate::views::view(&value.override_flags);
}
view_struct!(acadrust::entities::table::CellValue {
    value_type,
    raw_type_code,
    unit_type,
    raw_unit_type_code,
    flags,
    data_size,
    text,
    format,
    formatted_value,
    numeric_value,
    handle_value,
    point_value,
    binary_value
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_26(value: &acadrust::entities::table::CellValue) {
    let _ = crate::views::view(&value.value_type);
    let _ = crate::views::view(&value.raw_type_code);
    let _ = crate::views::view(&value.unit_type);
    let _ = crate::views::view(&value.raw_unit_type_code);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.data_size);
    let _ = crate::views::view(&value.text);
    let _ = crate::views::view(&value.format);
    let _ = crate::views::view(&value.formatted_value);
    let _ = crate::views::view(&value.numeric_value);
    let _ = crate::views::view(&value.handle_value);
    let _ = crate::views::view(&value.point_value);
    let _ = crate::views::view(&value.binary_value);
}
view_struct!(acadrust::entities::table::CellContent {
    content_type,
    value,
    block_handle,
    text_style_handle,
    text_style_name,
    color,
    rotation,
    scale,
    text_height,
    field_handle,
    attributes,
    format_override_flags,
    format_property_flags,
    format_value_data_type,
    format_value_unit_type,
    value_format,
    alignment,
    geometry
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_29(value: &acadrust::entities::table::CellContent) {
    let _ = crate::views::view(&value.content_type);
    let _ = crate::views::view(&value.value);
    let _ = crate::views::view(&value.block_handle);
    let _ = crate::views::view(&value.text_style_handle);
    let _ = crate::views::view(&value.text_style_name);
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.scale);
    let _ = crate::views::view(&value.text_height);
    let _ = crate::views::view(&value.field_handle);
    let _ = crate::views::view(&value.attributes);
    let _ = crate::views::view(&value.format_override_flags);
    let _ = crate::views::view(&value.format_property_flags);
    let _ = crate::views::view(&value.format_value_data_type);
    let _ = crate::views::view(&value.format_value_unit_type);
    let _ = crate::views::view(&value.value_format);
    let _ = crate::views::view(&value.alignment);
    let _ = crate::views::view(&value.geometry);
}
view_struct!(acadrust::entities::table::TableAttribute {
    definition_handle,
    value,
    index
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_32(value: &acadrust::entities::table::TableAttribute) {
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.value);
    let _ = crate::views::view(&value.index);
}
view_struct!(acadrust::entities::table::CellContentGeometry {
    distance_to_top_left,
    distance_to_center,
    width,
    height,
    outer_width,
    outer_height,
    flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_33(value: &acadrust::entities::table::CellContentGeometry) {
    let _ = crate::views::view(&value.distance_to_top_left);
    let _ = crate::views::view(&value.distance_to_center);
    let _ = crate::views::view(&value.width);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.outer_width);
    let _ = crate::views::view(&value.outer_height);
    let _ = crate::views::view(&value.flags);
}
view_struct!(acadrust::entities::table::TableCustomData { name, value });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_34(value: &acadrust::entities::table::TableCustomData) {
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.value);
}
view_struct!(acadrust::entities::table::CellStyle {
    style_type,
    property_flags,
    override_flags,
    legacy_override_bits,
    content_format_override_flags,
    content_property_flags,
    value_data_type,
    value_unit_type,
    value_format,
    background_color,
    content_color,
    text_style_handle,
    text_style_name,
    text_height,
    rotation,
    scale,
    alignment,
    fill_enabled,
    layout_flags,
    margin_override_flags,
    margin_left,
    margin_top,
    margin_right,
    margin_bottom,
    horizontal_spacing,
    vertical_spacing,
    top_border,
    right_border,
    bottom_border,
    left_border,
    applied_border_edges,
    additional_borders
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_35(value: &acadrust::entities::table::CellStyle) {
    let _ = crate::views::view(&value.style_type);
    let _ = crate::views::view(&value.property_flags);
    let _ = crate::views::view(&value.override_flags);
    let _ = crate::views::view(&value.legacy_override_bits);
    let _ = crate::views::view(&value.content_format_override_flags);
    let _ = crate::views::view(&value.content_property_flags);
    let _ = crate::views::view(&value.value_data_type);
    let _ = crate::views::view(&value.value_unit_type);
    let _ = crate::views::view(&value.value_format);
    let _ = crate::views::view(&value.background_color);
    let _ = crate::views::view(&value.content_color);
    let _ = crate::views::view(&value.text_style_handle);
    let _ = crate::views::view(&value.text_style_name);
    let _ = crate::views::view(&value.text_height);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.scale);
    let _ = crate::views::view(&value.alignment);
    let _ = crate::views::view(&value.fill_enabled);
    let _ = crate::views::view(&value.layout_flags);
    let _ = crate::views::view(&value.margin_override_flags);
    let _ = crate::views::view(&value.margin_left);
    let _ = crate::views::view(&value.margin_top);
    let _ = crate::views::view(&value.margin_right);
    let _ = crate::views::view(&value.margin_bottom);
    let _ = crate::views::view(&value.horizontal_spacing);
    let _ = crate::views::view(&value.vertical_spacing);
    let _ = crate::views::view(&value.top_border);
    let _ = crate::views::view(&value.right_border);
    let _ = crate::views::view(&value.bottom_border);
    let _ = crate::views::view(&value.left_border);
    let _ = crate::views::view(&value.applied_border_edges);
    let _ = crate::views::view(&value.additional_borders);
}
view_struct!(acadrust::entities::table::TableCell {
    cell_type,
    state,
    contents,
    style,
    tooltip,
    rotation,
    geometry_scale,
    auto_fit,
    merge_width,
    merge_height,
    flag,
    merged,
    virtual_edge,
    has_linked_data,
    custom_data,
    edge_flags,
    value_handle,
    block_scale,
    custom_data_items,
    data_link_handle,
    data_link_rows,
    data_link_columns,
    data_link_unknown,
    style_id,
    geometry_data_flag,
    geometry_width_with_gap,
    geometry_height_with_gap,
    geometry_flags,
    geometry_handle,
    geometry,
    geometries
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_39(value: &acadrust::entities::table::TableCell) {
    let _ = crate::views::view(&value.cell_type);
    let _ = crate::views::view(&value.state);
    let _ = crate::views::view(&value.contents);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.tooltip);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.geometry_scale);
    let _ = crate::views::view(&value.auto_fit);
    let _ = crate::views::view(&value.merge_width);
    let _ = crate::views::view(&value.merge_height);
    let _ = crate::views::view(&value.flag);
    let _ = crate::views::view(&value.merged);
    let _ = crate::views::view(&value.virtual_edge);
    let _ = crate::views::view(&value.has_linked_data);
    let _ = crate::views::view(&value.custom_data);
    let _ = crate::views::view(&value.edge_flags);
    let _ = crate::views::view(&value.value_handle);
    let _ = crate::views::view(&value.block_scale);
    let _ = crate::views::view(&value.custom_data_items);
    let _ = crate::views::view(&value.data_link_handle);
    let _ = crate::views::view(&value.data_link_rows);
    let _ = crate::views::view(&value.data_link_columns);
    let _ = crate::views::view(&value.data_link_unknown);
    let _ = crate::views::view(&value.style_id);
    let _ = crate::views::view(&value.geometry_data_flag);
    let _ = crate::views::view(&value.geometry_width_with_gap);
    let _ = crate::views::view(&value.geometry_height_with_gap);
    let _ = crate::views::view(&value.geometry_flags);
    let _ = crate::views::view(&value.geometry_handle);
    let _ = crate::views::view(&value.geometry);
    let _ = crate::views::view(&value.geometries);
}
view_struct!(acadrust::entities::table::TableRow {
    height,
    cells,
    style,
    custom_data,
    custom_data_items,
    style_id
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_42(value: &acadrust::entities::table::TableRow) {
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.cells);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.custom_data);
    let _ = crate::views::view(&value.custom_data_items);
    let _ = crate::views::view(&value.style_id);
}
view_struct!(acadrust::entities::table::TableColumn {
    name,
    width,
    style,
    custom_data,
    custom_data_items,
    style_id
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_45(value: &acadrust::entities::table::TableColumn) {
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.width);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.custom_data);
    let _ = crate::views::view(&value.custom_data_items);
    let _ = crate::views::view(&value.style_id);
}
view_struct!(acadrust::entities::table::CellRange {
    top_row,
    left_col,
    bottom_row,
    right_col
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_48(value: &acadrust::entities::table::CellRange) {
    let _ = crate::views::view(&value.top_row);
    let _ = crate::views::view(&value.left_col);
    let _ = crate::views::view(&value.bottom_row);
    let _ = crate::views::view(&value.right_col);
}
view_struct!(acadrust::entities::table::TableBreakData {
    position,
    height,
    flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_51(value: &acadrust::entities::table::TableBreakData) {
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.flags);
}
view_struct!(acadrust::entities::table::TableBreakRange {
    position,
    start_row,
    end_row
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_52(value: &acadrust::entities::table::TableBreakRange) {
    let _ = crate::views::view(&value.position);
    let _ = crate::views::view(&value.start_row);
    let _ = crate::views::view(&value.end_row);
}
view_struct!(acadrust::entities::table::LegacyTableStyleOverride {
    flags,
    title_suppressed,
    header_suppressed,
    flow_direction,
    horizontal_cell_margin,
    vertical_cell_margin,
    row_colors,
    row_fill_none,
    row_fill_colors,
    row_alignments,
    text_style_names,
    text_style_handles,
    row_heights
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_53(value: &acadrust::entities::table::LegacyTableStyleOverride) {
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.title_suppressed);
    let _ = crate::views::view(&value.header_suppressed);
    let _ = crate::views::view(&value.flow_direction);
    let _ = crate::views::view(&value.horizontal_cell_margin);
    let _ = crate::views::view(&value.vertical_cell_margin);
    let _ = crate::views::view(&value.row_colors);
    let _ = crate::views::view(&value.row_fill_none);
    let _ = crate::views::view(&value.row_fill_colors);
    let _ = crate::views::view(&value.row_alignments);
    let _ = crate::views::view(&value.text_style_names);
    let _ = crate::views::view(&value.text_style_handles);
    let _ = crate::views::view(&value.row_heights);
}
view_struct!(acadrust::entities::table::Table {
    common,
    insertion_point,
    horizontal_direction,
    normal,
    table_style_handle,
    block_record_handle,
    block_name,
    data_version,
    value_flags,
    override_flag,
    override_border_color,
    override_border_line_weight,
    override_border_visibility,
    legacy_style_override,
    legacy_border_colors,
    legacy_border_line_weights,
    legacy_border_visibility,
    rows,
    columns,
    break_options,
    break_flow_direction,
    break_spacing,
    name,
    description,
    field_handles,
    base_style,
    merged_ranges,
    break_data,
    break_ranges,
    dwg_unknown_byte,
    dwg_unknown_handle,
    dwg_unknown_long1,
    dwg_r2010_unknown_bit,
    dwg_unknown_long2,
    dwg_unknown_short
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_56(value: &acadrust::entities::table::Table) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.horizontal_direction);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.table_style_handle);
    let _ = crate::views::view(&value.block_record_handle);
    let _ = crate::views::view(&value.block_name);
    let _ = crate::views::view(&value.data_version);
    let _ = crate::views::view(&value.value_flags);
    let _ = crate::views::view(&value.override_flag);
    let _ = crate::views::view(&value.override_border_color);
    let _ = crate::views::view(&value.override_border_line_weight);
    let _ = crate::views::view(&value.override_border_visibility);
    let _ = crate::views::view(&value.legacy_style_override);
    let _ = crate::views::view(&value.legacy_border_colors);
    let _ = crate::views::view(&value.legacy_border_line_weights);
    let _ = crate::views::view(&value.legacy_border_visibility);
    let _ = crate::views::view(&value.rows);
    let _ = crate::views::view(&value.columns);
    let _ = crate::views::view(&value.break_options);
    let _ = crate::views::view(&value.break_flow_direction);
    let _ = crate::views::view(&value.break_spacing);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.description);
    let _ = crate::views::view(&value.field_handles);
    let _ = crate::views::view(&value.base_style);
    let _ = crate::views::view(&value.merged_ranges);
    let _ = crate::views::view(&value.break_data);
    let _ = crate::views::view(&value.break_ranges);
    let _ = crate::views::view(&value.dwg_unknown_byte);
    let _ = crate::views::view(&value.dwg_unknown_handle);
    let _ = crate::views::view(&value.dwg_unknown_long1);
    let _ = crate::views::view(&value.dwg_r2010_unknown_bit);
    let _ = crate::views::view(&value.dwg_unknown_long2);
    let _ = crate::views::view(&value.dwg_unknown_short);
}
view_struct!(acadrust::entities::table::TableBuilder {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_table_65(value: &acadrust::entities::table::TableBuilder) {}
view_enum!(acadrust::entities::text::TextHorizontalAlignment {
    Left,
    Center,
    Right,
    Aligned,
    Middle,
    Fit,
});
view_enum!(acadrust::entities::text::TextVerticalAlignment {
    Baseline,
    Bottom,
    Middle,
    Top,
});
view_struct!(acadrust::entities::text::Text {
    common,
    value,
    insertion_point,
    alignment_point,
    height,
    rotation,
    width_factor,
    oblique_angle,
    style,
    horizontal_alignment,
    vertical_alignment,
    normal,
    thickness,
    generation_flags
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_text_5(value: &acadrust::entities::text::Text) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.value);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.alignment_point);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.width_factor);
    let _ = crate::views::view(&value.oblique_angle);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.horizontal_alignment);
    let _ = crate::views::view(&value.vertical_alignment);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.thickness);
    let _ = crate::views::view(&value.generation_flags);
}
view_struct!(acadrust::entities::tolerance::Tolerance {
    common,
    insertion_point,
    direction,
    normal,
    text,
    dimension_style_name,
    dimension_style_handle,
    text_height,
    dimension_gap,
    dwg_unknown_short
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_tolerance_2(value: &acadrust::entities::tolerance::Tolerance) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.direction);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.text);
    let _ = crate::views::view(&value.dimension_style_name);
    let _ = crate::views::view(&value.dimension_style_handle);
    let _ = crate::views::view(&value.text_height);
    let _ = crate::views::view(&value.dimension_gap);
    let _ = crate::views::view(&value.dwg_unknown_short);
}
impl crate::views::NativeView for acadrust::entities::underlay::UnderlayDisplayFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_enum!(acadrust::entities::underlay::UnderlayType { Pdf, Dwf, Dgn });
view_struct!(acadrust::entities::underlay::UnderlayDefinition {
    handle,
    owner_handle,
    underlay_type,
    file_path,
    page_name,
    name,
    reactors,
    unloaded
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_underlay_6(value: &acadrust::entities::underlay::UnderlayDefinition) {
    let _ = crate::views::view(&value.handle);
    let _ = crate::views::view(&value.owner_handle);
    let _ = crate::views::view(&value.underlay_type);
    let _ = crate::views::view(&value.file_path);
    let _ = crate::views::view(&value.page_name);
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.reactors);
    let _ = crate::views::view(&value.unloaded);
}
view_struct!(acadrust::entities::underlay::Underlay {
    common,
    underlay_type,
    definition_handle,
    insertion_point,
    x_scale,
    y_scale,
    z_scale,
    rotation,
    normal,
    flags,
    contrast,
    fade,
    clip_boundary_vertices,
    clip_inverted
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_underlay_9(value: &acadrust::entities::underlay::Underlay) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.underlay_type);
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.x_scale);
    let _ = crate::views::view(&value.y_scale);
    let _ = crate::views::view(&value.z_scale);
    let _ = crate::views::view(&value.rotation);
    let _ = crate::views::view(&value.normal);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.contrast);
    let _ = crate::views::view(&value.fade);
    let _ = crate::views::view(&value.clip_boundary_vertices);
    let _ = crate::views::view(&value.clip_inverted);
}
view_struct!(acadrust::entities::unknown_entity::UnknownEntity {
    common,
    dxf_name,
    dwg_type_code,
    raw_dwg_data,
    dwg_handle_bits,
    raw_dxf_codes,
    dwg_source_version
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_unknown_entity_2(
    value: &acadrust::entities::unknown_entity::UnknownEntity,
) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.dxf_name);
    let _ = crate::views::view(&value.dwg_type_code);
    let _ = crate::views::view(&value.raw_dwg_data);
    let _ = crate::views::view(&value.dwg_handle_bits);
    let _ = crate::views::view(&value.raw_dxf_codes);
    let _ = crate::views::view(&value.dwg_source_version);
}
view_struct!(acadrust::entities::view_border::ViewBorder {
    common,
    version,
    min,
    max,
    center,
    scale,
    rotation_angle,
    active_viewport,
    scale_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_view_border_2(value: &acadrust::entities::view_border::ViewBorder) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.min);
    let _ = crate::views::view(&value.max);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.scale);
    let _ = crate::views::view(&value.rotation_angle);
    let _ = crate::views::view(&value.active_viewport);
    let _ = crate::views::view(&value.scale_handle);
}
view_struct!(acadrust::entities::viewport::ViewportStatusFlags {
    non_rectangular_clipping,
    is_off,
    is_on,
    perspective,
    front_clipping,
    back_clipping,
    ucs_follow,
    front_clip_not_at_eye,
    ucs_icon_visible,
    ucs_icon_at_origin,
    fast_zoom,
    snap_on,
    grid_on,
    isometric_snap,
    hide_plot,
    iso_pair_top,
    iso_pair_right,
    locked
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_viewport_2(value: &acadrust::entities::viewport::ViewportStatusFlags) {
    let _ = crate::views::view(&value.non_rectangular_clipping);
    let _ = crate::views::view(&value.is_off);
    let _ = crate::views::view(&value.is_on);
    let _ = crate::views::view(&value.perspective);
    let _ = crate::views::view(&value.front_clipping);
    let _ = crate::views::view(&value.back_clipping);
    let _ = crate::views::view(&value.ucs_follow);
    let _ = crate::views::view(&value.front_clip_not_at_eye);
    let _ = crate::views::view(&value.ucs_icon_visible);
    let _ = crate::views::view(&value.ucs_icon_at_origin);
    let _ = crate::views::view(&value.fast_zoom);
    let _ = crate::views::view(&value.snap_on);
    let _ = crate::views::view(&value.grid_on);
    let _ = crate::views::view(&value.isometric_snap);
    let _ = crate::views::view(&value.hide_plot);
    let _ = crate::views::view(&value.iso_pair_top);
    let _ = crate::views::view(&value.iso_pair_right);
    let _ = crate::views::view(&value.locked);
}
view_enum!(acadrust::entities::viewport::ViewportRenderMode {
    Wireframe2D,
    Wireframe3D,
    HiddenLine,
    FlatShaded,
    GouraudShaded,
    FlatShadedWithEdges,
    GouraudShadedWithEdges,
});
view_struct!(acadrust::entities::viewport::GridFlags {
    beyond_limits,
    adaptive,
    subdivision,
    follow_dynamic
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_viewport_6(value: &acadrust::entities::viewport::GridFlags) {
    let _ = crate::views::view(&value.beyond_limits);
    let _ = crate::views::view(&value.adaptive);
    let _ = crate::views::view(&value.subdivision);
    let _ = crate::views::view(&value.follow_dynamic);
}
view_struct!(acadrust::entities::viewport::Viewport {
    common,
    center,
    width,
    height,
    status,
    id,
    dxf_status,
    off_screen,
    view_center,
    snap_base,
    snap_spacing,
    grid_spacing,
    view_direction,
    view_target,
    lens_length,
    front_clip_z,
    back_clip_z,
    view_height,
    snap_angle,
    twist_angle,
    circle_sides,
    frozen_layers,
    clip_boundary_handle,
    render_mode,
    style_sheet,
    ucs_at_origin,
    ucs_per_viewport,
    ucs_icon_visible,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_handle,
    base_ucs_handle,
    ucs_ortho_type,
    elevation,
    shade_plot_mode,
    grid_flags,
    grid_major,
    background_handle,
    shade_plot_handle,
    visual_style_handle,
    sun_handle,
    default_lighting,
    default_lighting_type,
    brightness,
    contrast,
    ambient_color,
    custom_scale
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_viewport_8(value: &acadrust::entities::viewport::Viewport) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.center);
    let _ = crate::views::view(&value.width);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.status);
    let _ = crate::views::view(&value.id);
    let _ = crate::views::view(&value.dxf_status);
    let _ = crate::views::view(&value.off_screen);
    let _ = crate::views::view(&value.view_center);
    let _ = crate::views::view(&value.snap_base);
    let _ = crate::views::view(&value.snap_spacing);
    let _ = crate::views::view(&value.grid_spacing);
    let _ = crate::views::view(&value.view_direction);
    let _ = crate::views::view(&value.view_target);
    let _ = crate::views::view(&value.lens_length);
    let _ = crate::views::view(&value.front_clip_z);
    let _ = crate::views::view(&value.back_clip_z);
    let _ = crate::views::view(&value.view_height);
    let _ = crate::views::view(&value.snap_angle);
    let _ = crate::views::view(&value.twist_angle);
    let _ = crate::views::view(&value.circle_sides);
    let _ = crate::views::view(&value.frozen_layers);
    let _ = crate::views::view(&value.clip_boundary_handle);
    let _ = crate::views::view(&value.render_mode);
    let _ = crate::views::view(&value.style_sheet);
    let _ = crate::views::view(&value.ucs_at_origin);
    let _ = crate::views::view(&value.ucs_per_viewport);
    let _ = crate::views::view(&value.ucs_icon_visible);
    let _ = crate::views::view(&value.ucs_origin);
    let _ = crate::views::view(&value.ucs_x_axis);
    let _ = crate::views::view(&value.ucs_y_axis);
    let _ = crate::views::view(&value.ucs_handle);
    let _ = crate::views::view(&value.base_ucs_handle);
    let _ = crate::views::view(&value.ucs_ortho_type);
    let _ = crate::views::view(&value.elevation);
    let _ = crate::views::view(&value.shade_plot_mode);
    let _ = crate::views::view(&value.grid_flags);
    let _ = crate::views::view(&value.grid_major);
    let _ = crate::views::view(&value.background_handle);
    let _ = crate::views::view(&value.shade_plot_handle);
    let _ = crate::views::view(&value.visual_style_handle);
    let _ = crate::views::view(&value.sun_handle);
    let _ = crate::views::view(&value.default_lighting);
    let _ = crate::views::view(&value.default_lighting_type);
    let _ = crate::views::view(&value.brightness);
    let _ = crate::views::view(&value.contrast);
    let _ = crate::views::view(&value.ambient_color);
    let _ = crate::views::view(&value.custom_scale);
}
view_enum!(acadrust::entities::viewport::StandardView {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
    SWIsometric,
    SEIsometric,
    NEIsometric,
    NWIsometric,
});
impl crate::views::NativeView for acadrust::entities::wipeout::WipeoutDisplayFlags {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.bits() as i64)
    }
}
view_enum!(acadrust::entities::wipeout::WipeoutClipType {
    Rectangular,
    Polygonal,
});
view_enum!(acadrust::entities::wipeout::WipeoutClipMode { Outside, Inside });
view_struct!(acadrust::entities::wipeout::Wipeout {
    common,
    class_version,
    insertion_point,
    u_vector,
    v_vector,
    size,
    flags,
    clipping_enabled,
    brightness,
    contrast,
    fade,
    clip_mode,
    clip_type,
    clip_boundary_vertices,
    definition_handle,
    definition_reactor_handle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_wipeout_8(value: &acadrust::entities::wipeout::Wipeout) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.class_version);
    let _ = crate::views::view(&value.insertion_point);
    let _ = crate::views::view(&value.u_vector);
    let _ = crate::views::view(&value.v_vector);
    let _ = crate::views::view(&value.size);
    let _ = crate::views::view(&value.flags);
    let _ = crate::views::view(&value.clipping_enabled);
    let _ = crate::views::view(&value.brightness);
    let _ = crate::views::view(&value.contrast);
    let _ = crate::views::view(&value.fade);
    let _ = crate::views::view(&value.clip_mode);
    let _ = crate::views::view(&value.clip_type);
    let _ = crate::views::view(&value.clip_boundary_vertices);
    let _ = crate::views::view(&value.definition_handle);
    let _ = crate::views::view(&value.definition_reactor_handle);
}
view_struct!(acadrust::entities::xline::XLine {
    common,
    base_point,
    direction
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_xline_2(value: &acadrust::entities::xline::XLine) {
    let _ = crate::views::view(&value.common);
    let _ = crate::views::view(&value.base_point);
    let _ = crate::views::view(&value.direction);
}
view_struct!(acadrust::entities::acis::parser::SatParser {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_parser_1(value: &acadrust::entities::acis::parser::SatParser) {}
view_struct!(acadrust::entities::acis::sab::SabWriter {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_sab_3(value: &acadrust::entities::acis::sab::SabWriter) {}
view_enum!(acadrust::entities::acis::sab::SabError {
    UnexpectedEof,
    InvalidMagic,
    UnknownTag(v0, v1),
    InvalidString,
});
view_struct!(acadrust::entities::acis::sab::SabReader {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_sab_10(value: &acadrust::entities::acis::sab::SabReader) {}
view_struct!(acadrust::entities::acis::types::SatVersion {
    major,
    minor,
    patch
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_types_3(value: &acadrust::entities::acis::types::SatVersion) {
    let _ = crate::views::view(&value.major);
    let _ = crate::views::view(&value.minor);
    let _ = crate::views::view(&value.patch);
}
view_struct!(acadrust::entities::acis::types::SatHeader {
    version,
    num_records,
    num_bodies,
    has_history,
    product_id,
    product_version,
    date,
    spatial_resolution,
    normal_tolerance,
    resfit_tolerance
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_types_7(value: &acadrust::entities::acis::types::SatHeader) {
    let _ = crate::views::view(&value.version);
    let _ = crate::views::view(&value.num_records);
    let _ = crate::views::view(&value.num_bodies);
    let _ = crate::views::view(&value.has_history);
    let _ = crate::views::view(&value.product_id);
    let _ = crate::views::view(&value.product_version);
    let _ = crate::views::view(&value.date);
    let _ = crate::views::view(&value.spatial_resolution);
    let _ = crate::views::view(&value.normal_tolerance);
    let _ = crate::views::view(&value.resfit_tolerance);
}
view_struct!(acadrust::entities::acis::types::SatPointer { 0 });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_types_10(value: &acadrust::entities::acis::types::SatPointer) {
    let _ = crate::views::view(&value.0);
}
view_enum!(acadrust::entities::acis::types::SatToken {
    Ident(v0),
    Pointer(v0),
    Integer(v0),
    Float(v0),
    String(v0),
    Position(v0, v1, v2),
    True,
    False,
    Terminator,
    Enum(v0),
    Sab { tag, data },
});
view_struct!(acadrust::entities::acis::types::SatRecord {
    index,
    entity_type,
    sub_type,
    attribute,
    subtype_id,
    tokens,
    raw_text
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_types_22(value: &acadrust::entities::acis::types::SatRecord) {
    let _ = crate::views::view(&value.index);
    let _ = crate::views::view(&value.entity_type);
    let _ = crate::views::view(&value.sub_type);
    let _ = crate::views::view(&value.attribute);
    let _ = crate::views::view(&value.subtype_id);
    let _ = crate::views::view(&value.tokens);
    let _ = crate::views::view(&value.raw_text);
}
view_enum!(acadrust::entities::acis::types::Sense { Forward, Reversed });
view_enum!(acadrust::entities::acis::types::Sidedness { Single, Double });
view_struct!(acadrust::entities::acis::types::SatBSplineSurface {
    rational,
    degree_u,
    degree_v,
    u_closure,
    v_closure,
    u_singularity,
    v_singularity,
    u_knots,
    v_knots,
    control_count_u,
    control_count_v,
    control_points,
    fit_tolerance
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_types_66(
    value: &acadrust::entities::acis::types::SatBSplineSurface,
) {
    let _ = crate::views::view(&value.rational);
    let _ = crate::views::view(&value.degree_u);
    let _ = crate::views::view(&value.degree_v);
    let _ = crate::views::view(&value.u_closure);
    let _ = crate::views::view(&value.v_closure);
    let _ = crate::views::view(&value.u_singularity);
    let _ = crate::views::view(&value.v_singularity);
    let _ = crate::views::view(&value.u_knots);
    let _ = crate::views::view(&value.v_knots);
    let _ = crate::views::view(&value.control_count_u);
    let _ = crate::views::view(&value.control_count_v);
    let _ = crate::views::view(&value.control_points);
    let _ = crate::views::view(&value.fit_tolerance);
}
view_enum!(acadrust::entities::acis::types::SatSplineSurfaceKind { Spline, Mesh, Bs3 });
view_struct!(acadrust::entities::acis::types::SatDocument { header, records });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_types_92(value: &acadrust::entities::acis::types::SatDocument) {
    let _ = crate::views::view(&value.header);
    let _ = crate::views::view(&value.records);
}
view_enum!(acadrust::entities::acis::types::SatParseError {
    EmptyInput,
    InvalidHeader(v0),
    InvalidProductInfo(v0),
    InvalidTolerances(v0),
    InvalidRecord { line, message },
    UnexpectedToken { line, token },
});
view_enum!(acadrust::entities::acis::types::SatValidationError {
    InvalidPointer { record_index, pointer_value, context },
});
view_enum!(acadrust::entities::acis::types::SatEntityCategory {
    Header,
    Topology,
    Geometry,
    Transform,
    Attribute,
    Unknown,
});
view_struct!(acadrust::entities::acis::writer::SatWriter {});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_acis_writer_1(value: &acadrust::entities::acis::writer::SatWriter) {}
view_enum!(acadrust::entities::mtext_format::types::SpecialChar {
    Degree,
    PlusMinus,
    Diameter,
    Percent,
});
view_enum!(
    acadrust::entities::mtext_format::types::MTextParagraphAlignment {
        Default,
        Left,
        Right,
        Center,
        Justified,
        Distributed,
    }
);
view_enum!(
    acadrust::entities::mtext_format::types::MTextLineAlignment {
        Bottom,
        Middle,
        Top,
    }
);
view_enum!(acadrust::entities::mtext_format::types::StackingType {
    Horizontal,
    Diagonal,
    Limit,
});
view_struct!(acadrust::entities::mtext_format::types::StackingData {
    numerator,
    denominator,
    stacking_type
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_12(
    value: &acadrust::entities::mtext_format::types::StackingData,
) {
    let _ = crate::views::view(&value.numerator);
    let _ = crate::views::view(&value.denominator);
    let _ = crate::views::view(&value.stacking_type);
}
view_struct!(
    acadrust::entities::mtext_format::types::ParagraphProperties {
        alignment,
        first_line_indent,
        left_margin,
        right_margin,
        spacing_before,
        spacing_after,
        line_spacing,
        tab_stops
    }
);
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_14(
    value: &acadrust::entities::mtext_format::types::ParagraphProperties,
) {
    let _ = crate::views::view(&value.alignment);
    let _ = crate::views::view(&value.first_line_indent);
    let _ = crate::views::view(&value.left_margin);
    let _ = crate::views::view(&value.right_margin);
    let _ = crate::views::view(&value.spacing_before);
    let _ = crate::views::view(&value.spacing_after);
    let _ = crate::views::view(&value.line_spacing);
    let _ = crate::views::view(&value.tab_stops);
}
view_enum!(acadrust::entities::mtext_format::types::MTextLineSpacing {
    Default,
    Exact(v0),
    Multiple(v0),
});
view_enum!(acadrust::entities::mtext_format::types::TabStop {
    Left(v0),
    Center(v0),
    Right(v0),
    Decimal(v0),
    DecimalWithCharacter { position, character },
});
view_enum!(acadrust::entities::mtext_format::types::MTextColor {
    None,
    Index(v0),
    TrueColor(v0),
    ByLayer,
    ByBlock,
});
view_struct!(acadrust::entities::mtext_format::types::MTextFont {
    name,
    style,
    bold,
    italic,
    bold_explicit,
    italic_explicit,
    charset,
    pitch,
    extra
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_23(
    value: &acadrust::entities::mtext_format::types::MTextFont,
) {
    let _ = crate::views::view(&value.name);
    let _ = crate::views::view(&value.style);
    let _ = crate::views::view(&value.bold);
    let _ = crate::views::view(&value.italic);
    let _ = crate::views::view(&value.bold_explicit);
    let _ = crate::views::view(&value.italic_explicit);
    let _ = crate::views::view(&value.charset);
    let _ = crate::views::view(&value.pitch);
    let _ = crate::views::view(&value.extra);
}
view_enum!(acadrust::entities::mtext_format::types::MTextScalar {
    Factor(v0),
    Absolute(v0),
});
view_struct!(acadrust::entities::mtext_format::types::SpanProperties {
    color,
    color_rgb,
    second_color,
    font,
    height,
    stroke,
    line_align,
    tracking,
    width_factor,
    oblique_angle
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_27(
    value: &acadrust::entities::mtext_format::types::SpanProperties,
) {
    let _ = crate::views::view(&value.color);
    let _ = crate::views::view(&value.color_rgb);
    let _ = crate::views::view(&value.second_color);
    let _ = crate::views::view(&value.font);
    let _ = crate::views::view(&value.height);
    let _ = crate::views::view(&value.stroke);
    let _ = crate::views::view(&value.line_align);
    let _ = crate::views::view(&value.tracking);
    let _ = crate::views::view(&value.width_factor);
    let _ = crate::views::view(&value.oblique_angle);
}
view_struct!(acadrust::entities::mtext_format::types::MTextSpan {
    text,
    properties,
    stacking
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_29(
    value: &acadrust::entities::mtext_format::types::MTextSpan,
) {
    let _ = crate::views::view(&value.text);
    let _ = crate::views::view(&value.properties);
    let _ = crate::views::view(&value.stacking);
}
view_struct!(acadrust::entities::mtext_format::types::MTextParagraph {
    properties,
    spans,
    starts_column
});
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_31(
    value: &acadrust::entities::mtext_format::types::MTextParagraph,
) {
    let _ = crate::views::view(&value.properties);
    let _ = crate::views::view(&value.spans);
    let _ = crate::views::view(&value.starts_column);
}
view_struct!(acadrust::entities::mtext_format::types::MTextDocument { paragraphs });
#[allow(dead_code, unused_variables)]
fn check_acadrust_entities_mtext_format_types_34(
    value: &acadrust::entities::mtext_format::types::MTextDocument,
) {
    let _ = crate::views::view(&value.paragraphs);
}
