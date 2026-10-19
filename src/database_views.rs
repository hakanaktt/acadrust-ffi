//! Native borrowed views of acadrust symbol tables, objects, and classes.
#![allow(clippy::match_single_binding)]

use crate::views::{view, view_enum, view_struct, NativeView, StringView, ValueView};

// tables/appid.rs
view_struct!(acadrust::tables::appid::AppId { handle, name });
const _: fn(&acadrust::tables::appid::AppId) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
};

// tables/block_record.rs
view_struct!(acadrust::tables::block_record::BlockFlags {
    anonymous,
    has_attributes,
    is_xref,
    is_xref_overlay,
    is_external,
    is_xref_unloaded
});
const _: fn(&acadrust::tables::block_record::BlockFlags) = |value| {
    let _ = view(&value.anonymous);
    let _ = view(&value.has_attributes);
    let _ = view(&value.is_xref);
    let _ = view(&value.is_xref_overlay);
    let _ = view(&value.is_external);
    let _ = view(&value.is_xref_unloaded);
};
view_struct!(acadrust::tables::block_record::BlockRecord {
    handle,
    block_entity_handle,
    block_end_handle,
    name,
    flags,
    layout,
    units,
    explodable,
    scale_uniformly,
    entity_handles,
    xref_path,
    description,
    insert_count_bytes,
    preview_data,
    insert_handles,
    base_point
});
const _: fn(&acadrust::tables::block_record::BlockRecord) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.block_entity_handle);
    let _ = view(&value.block_end_handle);
    let _ = view(&value.name);
    let _ = view(&value.flags);
    let _ = view(&value.layout);
    let _ = view(&value.units);
    let _ = view(&value.explodable);
    let _ = view(&value.scale_uniformly);
    let _ = view(&value.entity_handles);
    let _ = view(&value.xref_path);
    let _ = view(&value.description);
    let _ = view(&value.insert_count_bytes);
    let _ = view(&value.preview_data);
    let _ = view(&value.insert_handles);
    let _ = view(&value.base_point);
};

// tables/dimstyle.rs
view_struct!(acadrust::tables::dimstyle::DimStyle {
    handle,
    name,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle,
    dimclrd,
    dimclrd_true_color,
    dimdle,
    dimdli,
    dimgap,
    dimlwd,
    dimsd1,
    dimsd2,
    dimclre,
    dimclre_true_color,
    dimexe,
    dimexo,
    dimlwe,
    dimse1,
    dimse2,
    dimfxl,
    dimfxlon,
    dimasz,
    dimblk,
    dimblk1,
    dimblk2,
    dimldrblk,
    dimblk_name,
    dimblk1_name,
    dimblk2_name,
    dimsah,
    dimcen,
    dimtsz,
    dimclrt,
    dimclrt_true_color,
    dimtxt,
    dimjust,
    dimtad,
    dimtvp,
    dimtih,
    dimtoh,
    dimtix,
    dimsoxd,
    dimtfill,
    dimtfillclr,
    dimtfillclr_true_color,
    dimtmove,
    dimtxtdirection,
    dimaltmzf,
    dimaltmzs,
    dimmzf,
    dimmzs,
    dimtxsty_handle,
    dimtxsty,
    dimscale,
    dimlfac,
    dimlunit,
    dimdec,
    dimrnd,
    dimdsep,
    dimzin,
    dimaunit,
    dimadec,
    dimazin,
    dimalt,
    dimaltf,
    dimaltd,
    dimaltu,
    dimalttd,
    dimaltrnd,
    dimapost,
    dimaltz,
    dimalttz,
    dimtol,
    dimlim,
    dimtp,
    dimtm,
    dimtdec,
    dimtfac,
    dimtolj,
    dimtzin,
    dimatfit,
    dimtofl,
    dimupt,
    dimfit,
    dimpost,
    dimfrac,
    dimarcsym,
    dimjogang,
    dimltex_handle,
    dimltex1_handle,
    dimltex2_handle,
    dimunit,
    annotative
});
const _: fn(&acadrust::tables::dimstyle::DimStyle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.xref_reference);
    let _ = view(&value.xref_resolved);
    let _ = view(&value.xref_dependent);
    let _ = view(&value.xref_handle);
    let _ = view(&value.dimclrd);
    let _ = view(&value.dimclrd_true_color);
    let _ = view(&value.dimdle);
    let _ = view(&value.dimdli);
    let _ = view(&value.dimgap);
    let _ = view(&value.dimlwd);
    let _ = view(&value.dimsd1);
    let _ = view(&value.dimsd2);
    let _ = view(&value.dimclre);
    let _ = view(&value.dimclre_true_color);
    let _ = view(&value.dimexe);
    let _ = view(&value.dimexo);
    let _ = view(&value.dimlwe);
    let _ = view(&value.dimse1);
    let _ = view(&value.dimse2);
    let _ = view(&value.dimfxl);
    let _ = view(&value.dimfxlon);
    let _ = view(&value.dimasz);
    let _ = view(&value.dimblk);
    let _ = view(&value.dimblk1);
    let _ = view(&value.dimblk2);
    let _ = view(&value.dimldrblk);
    let _ = view(&value.dimblk_name);
    let _ = view(&value.dimblk1_name);
    let _ = view(&value.dimblk2_name);
    let _ = view(&value.dimsah);
    let _ = view(&value.dimcen);
    let _ = view(&value.dimtsz);
    let _ = view(&value.dimclrt);
    let _ = view(&value.dimclrt_true_color);
    let _ = view(&value.dimtxt);
    let _ = view(&value.dimjust);
    let _ = view(&value.dimtad);
    let _ = view(&value.dimtvp);
    let _ = view(&value.dimtih);
    let _ = view(&value.dimtoh);
    let _ = view(&value.dimtix);
    let _ = view(&value.dimsoxd);
    let _ = view(&value.dimtfill);
    let _ = view(&value.dimtfillclr);
    let _ = view(&value.dimtfillclr_true_color);
    let _ = view(&value.dimtmove);
    let _ = view(&value.dimtxtdirection);
    let _ = view(&value.dimaltmzf);
    let _ = view(&value.dimaltmzs);
    let _ = view(&value.dimmzf);
    let _ = view(&value.dimmzs);
    let _ = view(&value.dimtxsty_handle);
    let _ = view(&value.dimtxsty);
    let _ = view(&value.dimscale);
    let _ = view(&value.dimlfac);
    let _ = view(&value.dimlunit);
    let _ = view(&value.dimdec);
    let _ = view(&value.dimrnd);
    let _ = view(&value.dimdsep);
    let _ = view(&value.dimzin);
    let _ = view(&value.dimaunit);
    let _ = view(&value.dimadec);
    let _ = view(&value.dimazin);
    let _ = view(&value.dimalt);
    let _ = view(&value.dimaltf);
    let _ = view(&value.dimaltd);
    let _ = view(&value.dimaltu);
    let _ = view(&value.dimalttd);
    let _ = view(&value.dimaltrnd);
    let _ = view(&value.dimapost);
    let _ = view(&value.dimaltz);
    let _ = view(&value.dimalttz);
    let _ = view(&value.dimtol);
    let _ = view(&value.dimlim);
    let _ = view(&value.dimtp);
    let _ = view(&value.dimtm);
    let _ = view(&value.dimtdec);
    let _ = view(&value.dimtfac);
    let _ = view(&value.dimtolj);
    let _ = view(&value.dimtzin);
    let _ = view(&value.dimatfit);
    let _ = view(&value.dimtofl);
    let _ = view(&value.dimupt);
    let _ = view(&value.dimfit);
    let _ = view(&value.dimpost);
    let _ = view(&value.dimfrac);
    let _ = view(&value.dimarcsym);
    let _ = view(&value.dimjogang);
    let _ = view(&value.dimltex_handle);
    let _ = view(&value.dimltex1_handle);
    let _ = view(&value.dimltex2_handle);
    let _ = view(&value.dimunit);
    let _ = view(&value.annotative);
};

// tables/layer.rs
view_struct!(acadrust::tables::layer::LayerFlags {
    frozen,
    locked,
    frozen_in_new_viewport,
    off,
    xref_dependent
});
const _: fn(&acadrust::tables::layer::LayerFlags) = |value| {
    let _ = view(&value.frozen);
    let _ = view(&value.locked);
    let _ = view(&value.frozen_in_new_viewport);
    let _ = view(&value.off);
    let _ = view(&value.xref_dependent);
};
view_struct!(acadrust::tables::layer::Layer {
    handle,
    name,
    flags,
    color,
    color_name,
    book_name,
    line_type,
    line_weight,
    plot_style,
    is_plottable,
    transparency,
    description,
    material,
    plotstyle_handle,
    xref_block_record_handle
});
const _: fn(&acadrust::tables::layer::Layer) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.flags);
    let _ = view(&value.color);
    let _ = view(&value.color_name);
    let _ = view(&value.book_name);
    let _ = view(&value.line_type);
    let _ = view(&value.line_weight);
    let _ = view(&value.plot_style);
    let _ = view(&value.is_plottable);
    let _ = view(&value.transparency);
    let _ = view(&value.description);
    let _ = view(&value.material);
    let _ = view(&value.plotstyle_handle);
    let _ = view(&value.xref_block_record_handle);
};

// tables/linetype.rs
view_struct!(acadrust::tables::linetype::LineTypeElement { length, complex });
const _: fn(&acadrust::tables::linetype::LineTypeElement) = |value| {
    let _ = view(&value.length);
    let _ = view(&value.complex);
};
view_struct!(acadrust::tables::linetype::LineType {
    handle,
    name,
    description,
    elements,
    pattern_length,
    alignment,
    xref_dependent,
    xref_block_record_handle
});
const _: fn(&acadrust::tables::linetype::LineType) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.description);
    let _ = view(&value.elements);
    let _ = view(&value.pattern_length);
    let _ = view(&value.alignment);
    let _ = view(&value.xref_dependent);
    let _ = view(&value.xref_block_record_handle);
};
impl NativeView for acadrust::tables::linetype::LineTypeComplexContent {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::Shape { .. } => 1,
            Self::Text { .. } => 1,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::Shape { .. } => "Shape",
            Self::Text { .. } => "Text",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::Shape { shape_number } => match index {
                0 => Some((StringView::new("shape_number"), view(shape_number))),
                _ => None,
            },
            Self::Text { text } => match index {
                0 => Some((StringView::new("text"), view(text))),
                _ => None,
            },
        }
    }
}
view_struct!(acadrust::tables::linetype::LineTypeComplexData {
    content,
    style_handle,
    scale,
    rotation,
    absolute_rotation,
    source_flags,
    offset
});
const _: fn(&acadrust::tables::linetype::LineTypeComplexData) = |value| {
    let _ = view(&value.content);
    let _ = view(&value.style_handle);
    let _ = view(&value.scale);
    let _ = view(&value.rotation);
    let _ = view(&value.absolute_rotation);
    let _ = view(&value.source_flags);
    let _ = view(&value.offset);
};

// tables/mod.rs

// tables/textstyle.rs
view_struct!(acadrust::tables::textstyle::TextGenerationFlags {
    backward,
    upside_down
});
const _: fn(&acadrust::tables::textstyle::TextGenerationFlags) = |value| {
    let _ = view(&value.backward);
    let _ = view(&value.upside_down);
};
view_struct!(acadrust::tables::textstyle::TextStyle {
    handle,
    name,
    flags,
    height,
    width_factor,
    oblique_angle,
    last_height,
    font_file,
    big_font_file,
    true_type_font,
    true_type_font_flags,
    xref_dependent,
    xref_block_record_handle,
    is_shape_file,
    is_vertical,
    annotative
});
const _: fn(&acadrust::tables::textstyle::TextStyle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.flags);
    let _ = view(&value.height);
    let _ = view(&value.width_factor);
    let _ = view(&value.oblique_angle);
    let _ = view(&value.last_height);
    let _ = view(&value.font_file);
    let _ = view(&value.big_font_file);
    let _ = view(&value.true_type_font);
    let _ = view(&value.true_type_font_flags);
    let _ = view(&value.xref_dependent);
    let _ = view(&value.xref_block_record_handle);
    let _ = view(&value.is_shape_file);
    let _ = view(&value.is_vertical);
    let _ = view(&value.annotative);
};

// tables/ucs.rs
view_struct!(acadrust::tables::ucs::Ucs {
    handle,
    name,
    origin,
    x_axis,
    y_axis,
    elevation,
    ortho_view_type,
    ortho_type,
    named_ucs_handle,
    base_ucs_handle,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle
});
const _: fn(&acadrust::tables::ucs::Ucs) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.origin);
    let _ = view(&value.x_axis);
    let _ = view(&value.y_axis);
    let _ = view(&value.elevation);
    let _ = view(&value.ortho_view_type);
    let _ = view(&value.ortho_type);
    let _ = view(&value.named_ucs_handle);
    let _ = view(&value.base_ucs_handle);
    let _ = view(&value.xref_reference);
    let _ = view(&value.xref_resolved);
    let _ = view(&value.xref_dependent);
    let _ = view(&value.xref_handle);
};

// tables/view.rs
view_struct!(acadrust::tables::view::View {
    handle,
    name,
    center,
    height,
    width,
    direction,
    target,
    lens_length,
    front_clip,
    back_clip,
    twist_angle,
    perspective,
    front_clipping,
    back_clipping,
    front_clip_at_eye,
    render_mode,
    use_default_lights,
    default_lighting_type,
    brightness,
    contrast,
    ambient_color,
    paper_space,
    ucs_associated,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_elevation,
    ucs_ortho_type,
    camera_plottable,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle,
    background_handle,
    live_section_handle,
    visual_style_handle,
    sun_handle,
    named_ucs_handle,
    base_ucs_handle
});
const _: fn(&acadrust::tables::view::View) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.center);
    let _ = view(&value.height);
    let _ = view(&value.width);
    let _ = view(&value.direction);
    let _ = view(&value.target);
    let _ = view(&value.lens_length);
    let _ = view(&value.front_clip);
    let _ = view(&value.back_clip);
    let _ = view(&value.twist_angle);
    let _ = view(&value.perspective);
    let _ = view(&value.front_clipping);
    let _ = view(&value.back_clipping);
    let _ = view(&value.front_clip_at_eye);
    let _ = view(&value.render_mode);
    let _ = view(&value.use_default_lights);
    let _ = view(&value.default_lighting_type);
    let _ = view(&value.brightness);
    let _ = view(&value.contrast);
    let _ = view(&value.ambient_color);
    let _ = view(&value.paper_space);
    let _ = view(&value.ucs_associated);
    let _ = view(&value.ucs_origin);
    let _ = view(&value.ucs_x_axis);
    let _ = view(&value.ucs_y_axis);
    let _ = view(&value.ucs_elevation);
    let _ = view(&value.ucs_ortho_type);
    let _ = view(&value.camera_plottable);
    let _ = view(&value.xref_reference);
    let _ = view(&value.xref_resolved);
    let _ = view(&value.xref_dependent);
    let _ = view(&value.xref_handle);
    let _ = view(&value.background_handle);
    let _ = view(&value.live_section_handle);
    let _ = view(&value.visual_style_handle);
    let _ = view(&value.sun_handle);
    let _ = view(&value.named_ucs_handle);
    let _ = view(&value.base_ucs_handle);
};

// tables/vport.rs
view_struct!(acadrust::tables::vport::VPort {
    handle,
    name,
    lower_left,
    upper_right,
    view_center,
    snap_base,
    snap_spacing,
    grid_spacing,
    view_direction,
    view_target,
    view_height,
    aspect_ratio,
    lens_length,
    view_twist,
    front_clip,
    back_clip,
    ucsfollow,
    circle_zoom,
    fast_zoom,
    grid_on,
    snap_on,
    snap_style,
    snap_isopair,
    snap_rotation,
    render_mode,
    perspective,
    front_clipping,
    back_clipping,
    front_clip_at_eye,
    ucsicon_lower,
    ucsicon_origin,
    use_default_lights,
    default_lighting_type,
    brightness,
    contrast,
    ambient_color,
    ucs_at_origin,
    ucs_per_viewport,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_elevation,
    ucs_ortho_type,
    grid_flags,
    grid_major,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle,
    background_handle,
    visual_style_handle,
    sun_handle,
    named_ucs_handle,
    base_ucs_handle
});
const _: fn(&acadrust::tables::vport::VPort) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.lower_left);
    let _ = view(&value.upper_right);
    let _ = view(&value.view_center);
    let _ = view(&value.snap_base);
    let _ = view(&value.snap_spacing);
    let _ = view(&value.grid_spacing);
    let _ = view(&value.view_direction);
    let _ = view(&value.view_target);
    let _ = view(&value.view_height);
    let _ = view(&value.aspect_ratio);
    let _ = view(&value.lens_length);
    let _ = view(&value.view_twist);
    let _ = view(&value.front_clip);
    let _ = view(&value.back_clip);
    let _ = view(&value.ucsfollow);
    let _ = view(&value.circle_zoom);
    let _ = view(&value.fast_zoom);
    let _ = view(&value.grid_on);
    let _ = view(&value.snap_on);
    let _ = view(&value.snap_style);
    let _ = view(&value.snap_isopair);
    let _ = view(&value.snap_rotation);
    let _ = view(&value.render_mode);
    let _ = view(&value.perspective);
    let _ = view(&value.front_clipping);
    let _ = view(&value.back_clipping);
    let _ = view(&value.front_clip_at_eye);
    let _ = view(&value.ucsicon_lower);
    let _ = view(&value.ucsicon_origin);
    let _ = view(&value.use_default_lights);
    let _ = view(&value.default_lighting_type);
    let _ = view(&value.brightness);
    let _ = view(&value.contrast);
    let _ = view(&value.ambient_color);
    let _ = view(&value.ucs_at_origin);
    let _ = view(&value.ucs_per_viewport);
    let _ = view(&value.ucs_origin);
    let _ = view(&value.ucs_x_axis);
    let _ = view(&value.ucs_y_axis);
    let _ = view(&value.ucs_elevation);
    let _ = view(&value.ucs_ortho_type);
    let _ = view(&value.grid_flags);
    let _ = view(&value.grid_major);
    let _ = view(&value.xref_reference);
    let _ = view(&value.xref_resolved);
    let _ = view(&value.xref_dependent);
    let _ = view(&value.xref_handle);
    let _ = view(&value.background_handle);
    let _ = view(&value.visual_style_handle);
    let _ = view(&value.sun_handle);
    let _ = view(&value.named_ucs_handle);
    let _ = view(&value.base_ucs_handle);
};

// tables/vx.rs
view_struct!(acadrust::tables::vx::VxTableRecord {
    handle,
    name,
    is_xref_reference,
    is_xref_resolved,
    is_xref_dependent,
    xref_handle,
    is_on,
    viewport,
    previous_entry,
    legacy_viewport_entity_address,
    legacy_viewport_index,
    legacy_previous_entry_index
});
const _: fn(&acadrust::tables::vx::VxTableRecord) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.is_xref_reference);
    let _ = view(&value.is_xref_resolved);
    let _ = view(&value.is_xref_dependent);
    let _ = view(&value.xref_handle);
    let _ = view(&value.is_on);
    let _ = view(&value.viewport);
    let _ = view(&value.previous_entry);
    let _ = view(&value.legacy_viewport_entity_address);
    let _ = view(&value.legacy_viewport_index);
    let _ = view(&value.legacy_previous_entry_index);
};

// objects/associative.rs
view_struct!(acadrust::objects::AssociativeObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    dxf_name,
    cpp_class_name,
    data,
    source_version
});
const _: fn(&acadrust::objects::AssociativeObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.dxf_name);
    let _ = view(&value.cpp_class_name);
    let _ = view(&value.data);
    let _ = view(&value.source_version);
};
view_enum!(acadrust::objects::AssociativeData {
    Unknown,
    Dependency(v0),
    ValueDependency(v0),
    GeomDependency(v0),
    SurfaceActionBody(v0),
    Action(v0),
    Network(v0),
    AnnotationActionBody(v0),
    PersSubentManager(v0),
    EdgeActionParam(v0),
    ConstraintGroup(v0),
    Variable(v0),
    ActionParam(v0),
    CompoundActionParam(v0),
    OsnapPointRefActionParam(v0),
    PointRefActionParam(v0),
    ObjectActionParam(v0),
    PathActionParam(v0),
    DimDependencyBody(v0),
    FaceActionParam(v0),
    VertexActionParam(v0),
    AsmBodyActionParam(v0),
    ArrayParameters(v0),
    ArrayActionBody(v0),
    ArrayModifyActionBody(v0),
    DimensionAssociation(v0),
    PersSubentManagerStatic(v0),
    ViewRepActionBody(v0),
    ViewObjectActionParam(v0),
    ViewRepHatchManager(v0),
    ViewRepHatchActionParam(v0),
    ViewLabelActionParam(v0),
    SmartCenterActionBody(v0),
});
view_struct!(acadrust::objects::AssocDependency {
    class_version,
    status,
    is_read_dependency,
    is_write_dependency,
    is_attached_to_object,
    is_delegating_to_owning_action,
    order,
    dependent_on,
    name,
    read_dependency,
    node,
    dependency_body,
    dependency_body_id
});
const _: fn(&acadrust::objects::AssocDependency) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.status);
    let _ = view(&value.is_read_dependency);
    let _ = view(&value.is_write_dependency);
    let _ = view(&value.is_attached_to_object);
    let _ = view(&value.is_delegating_to_owning_action);
    let _ = view(&value.order);
    let _ = view(&value.dependent_on);
    let _ = view(&value.name);
    let _ = view(&value.read_dependency);
    let _ = view(&value.node);
    let _ = view(&value.dependency_body);
    let _ = view(&value.dependency_body_id);
};
view_struct!(acadrust::objects::AssocValueDependency {
    dependency,
    class_version,
    name,
    value
});
const _: fn(&acadrust::objects::AssocValueDependency) = |value| {
    let _ = view(&value.dependency);
    let _ = view(&value.class_version);
    let _ = view(&value.name);
    let _ = view(&value.value);
};
view_struct!(acadrust::objects::AssocPersistentSubentId {
    class_name,
    dependent_on_compound_object,
    class_code,
    values,
    leading_flag
});
const _: fn(&acadrust::objects::AssocPersistentSubentId) = |value| {
    let _ = view(&value.class_name);
    let _ = view(&value.dependent_on_compound_object);
    let _ = view(&value.class_code);
    let _ = view(&value.values);
    let _ = view(&value.leading_flag);
};
view_struct!(acadrust::objects::AssocGeomDependency {
    dependency,
    class_version,
    enabled,
    persistent_subent
});
const _: fn(&acadrust::objects::AssocGeomDependency) = |value| {
    let _ = view(&value.dependency);
    let _ = view(&value.class_version);
    let _ = view(&value.enabled);
    let _ = view(&value.persistent_subent);
};
view_enum!(acadrust::objects::AssocEvalValue {
    None,
    Real(v0),
    Long(v0),
    Short(v0),
    Byte(v0),
    Text(v0),
    Handle(v0),
});
view_struct!(acadrust::objects::AssocEvalVariant { code, value });
const _: fn(&acadrust::objects::AssocEvalVariant) = |value| {
    let _ = view(&value.code);
    let _ = view(&value.value);
};
view_struct!(acadrust::objects::AssocValueParamVariable { value, handle });
const _: fn(&acadrust::objects::AssocValueParamVariable) = |value| {
    let _ = view(&value.value);
    let _ = view(&value.handle);
};
view_struct!(acadrust::objects::AssocValueParam {
    class_version,
    name,
    unit_type,
    variables,
    controlled_object_dependency
});
const _: fn(&acadrust::objects::AssocValueParam) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.name);
    let _ = view(&value.unit_type);
    let _ = view(&value.variables);
    let _ = view(&value.controlled_object_dependency);
};
view_struct!(acadrust::objects::AssocActionDependency {
    is_owned,
    dependency
});
const _: fn(&acadrust::objects::AssocActionDependency) = |value| {
    let _ = view(&value.is_owned);
    let _ = view(&value.dependency);
};
view_struct!(acadrust::objects::AssocAction {
    class_version,
    geometry_status,
    owning_network,
    action_body,
    action_index,
    max_dependency_index,
    dependencies,
    owned_parameters,
    values
});
const _: fn(&acadrust::objects::AssocAction) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.geometry_status);
    let _ = view(&value.owning_network);
    let _ = view(&value.action_body);
    let _ = view(&value.action_index);
    let _ = view(&value.max_dependency_index);
    let _ = view(&value.dependencies);
    let _ = view(&value.owned_parameters);
    let _ = view(&value.values);
};
view_struct!(acadrust::objects::AssocNetwork {
    action,
    network_version,
    network_action_index,
    actions,
    owned_actions
});
const _: fn(&acadrust::objects::AssocNetwork) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.network_version);
    let _ = view(&value.network_action_index);
    let _ = view(&value.actions);
    let _ = view(&value.owned_actions);
};
view_struct!(acadrust::objects::AssocActionParam {
    is_r2013,
    version,
    name
});
const _: fn(&acadrust::objects::AssocActionParam) = |value| {
    let _ = view(&value.is_r2013);
    let _ = view(&value.version);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::AssocActionBody { version });
const _: fn(&acadrust::objects::AssocActionBody) = |value| {
    let _ = view(&value.version);
};
view_struct!(acadrust::objects::AssocParamBasedActionBody {
    version,
    minor,
    dependencies,
    marker,
    values,
    empty_value_marker,
    dependency
});
const _: fn(&acadrust::objects::AssocParamBasedActionBody) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.minor);
    let _ = view(&value.dependencies);
    let _ = view(&value.marker);
    let _ = view(&value.values);
    let _ = view(&value.empty_value_marker);
    let _ = view(&value.dependency);
};
view_struct!(acadrust::objects::AssocSurfaceBody {
    version,
    dependency,
    is_semi_associative,
    marker,
    is_semi_override,
    grip_status
});
const _: fn(&acadrust::objects::AssocSurfaceBody) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.dependency);
    let _ = view(&value.is_semi_associative);
    let _ = view(&value.marker);
    let _ = view(&value.is_semi_override);
    let _ = view(&value.grip_status);
};
view_enum!(acadrust::objects::AssocSurfaceActionKind {
    Plane,
    Extend,
    Extruded,
    Lofted,
    Network,
    Offset,
    Revolved,
    Trim,
    Blend,
    Patch,
    Fillet,
    Swept,
    EdgeChamfer,
    EdgeFillet,
});
view_struct!(acadrust::objects::AssocSurfaceActionBody {
    kind,
    action_body,
    parameter_body,
    surface_body,
    path_status,
    class_version,
    option,
    flags,
    status,
    secondary_status,
    distance,
    first_point,
    second_point
});
const _: fn(&acadrust::objects::AssocSurfaceActionBody) = |value| {
    let _ = view(&value.kind);
    let _ = view(&value.action_body);
    let _ = view(&value.parameter_body);
    let _ = view(&value.surface_body);
    let _ = view(&value.path_status);
    let _ = view(&value.class_version);
    let _ = view(&value.option);
    let _ = view(&value.flags);
    let _ = view(&value.status);
    let _ = view(&value.secondary_status);
    let _ = view(&value.distance);
    let _ = view(&value.first_point);
    let _ = view(&value.second_point);
};
view_struct!(acadrust::objects::AssocAnnotationBase {
    action_body,
    parameter_body,
    version,
    dependency
});
const _: fn(&acadrust::objects::AssocAnnotationBase) = |value| {
    let _ = view(&value.action_body);
    let _ = view(&value.parameter_body);
    let _ = view(&value.version);
    let _ = view(&value.dependency);
};
view_enum!(acadrust::objects::AssocAnnotationKind {
    RestoreEntityState,
    MLeader,
    AlignedDimension,
    ThreePointAngularDimension,
    OrdinateDimension,
    RotatedDimension,
});
view_struct!(acadrust::objects::AssocAnnotationDependency {
    dependency_id,
    dependency
});
const _: fn(&acadrust::objects::AssocAnnotationDependency) = |value| {
    let _ = view(&value.dependency_id);
    let _ = view(&value.dependency);
};
view_struct!(acadrust::objects::AssocAnnotationActionBody {
    kind,
    annotation,
    action_body,
    class_version,
    entity,
    actions,
    read_node,
    dimension_node,
    dependency
});
const _: fn(&acadrust::objects::AssocAnnotationActionBody) = |value| {
    let _ = view(&value.kind);
    let _ = view(&value.annotation);
    let _ = view(&value.action_body);
    let _ = view(&value.class_version);
    let _ = view(&value.entity);
    let _ = view(&value.actions);
    let _ = view(&value.read_node);
    let _ = view(&value.dimension_node);
    let _ = view(&value.dependency);
};
view_struct!(acadrust::objects::AssocPersSubentManager {
    class_version,
    markers,
    values,
    final_flag
});
const _: fn(&acadrust::objects::AssocPersSubentManager) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.markers);
    let _ = view(&value.values);
    let _ = view(&value.final_flag);
};
view_struct!(acadrust::objects::AssocSingleDependencyActionParam {
    action_param,
    dependency_class_version,
    dependency,
    class_version
});
const _: fn(&acadrust::objects::AssocSingleDependencyActionParam) = |value| {
    let _ = view(&value.action_param);
    let _ = view(&value.dependency_class_version);
    let _ = view(&value.dependency);
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::AssocSmartCenterActionBody {
    action_body,
    parameter_body,
    version
});
const _: fn(&acadrust::objects::AssocSmartCenterActionBody) = |value| {
    let _ = view(&value.action_body);
    let _ = view(&value.parameter_body);
    let _ = view(&value.version);
};
view_struct!(acadrust::objects::AssocViewRepActionBody {
    action_body,
    class_version,
    view_rep,
    view_type,
    rotation
});
const _: fn(&acadrust::objects::AssocViewRepActionBody) = |value| {
    let _ = view(&value.action_body);
    let _ = view(&value.class_version);
    let _ = view(&value.view_rep);
    let _ = view(&value.view_type);
    let _ = view(&value.rotation);
};
view_enum!(acadrust::objects::AssocViewObjectActionParamKind {
    ViewBorder,
    ViewRep,
    ViewSymbol,
    ViewStyle,
});
view_struct!(acadrust::objects::AssocViewObjectActionParam {
    kind,
    single_dependency,
    class_version
});
const _: fn(&acadrust::objects::AssocViewObjectActionParam) = |value| {
    let _ = view(&value.kind);
    let _ = view(&value.single_dependency);
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::AssocViewRepHatchManagerItem {
    first_id,
    second_id,
    status,
    parameter
});
const _: fn(&acadrust::objects::AssocViewRepHatchManagerItem) = |value| {
    let _ = view(&value.first_id);
    let _ = view(&value.second_id);
    let _ = view(&value.status);
    let _ = view(&value.parameter);
};
view_struct!(acadrust::objects::AssocViewRepHatchManager {
    compound,
    class_version,
    items
});
const _: fn(&acadrust::objects::AssocViewRepHatchManager) = |value| {
    let _ = view(&value.compound);
    let _ = view(&value.class_version);
    let _ = view(&value.items);
};
view_struct!(acadrust::objects::AssocViewRepHatchActionParam {
    single_dependency,
    class_version,
    normal,
    hatch_index,
    flags
});
const _: fn(&acadrust::objects::AssocViewRepHatchActionParam) = |value| {
    let _ = view(&value.single_dependency);
    let _ = view(&value.class_version);
    let _ = view(&value.normal);
    let _ = view(&value.hatch_index);
    let _ = view(&value.flags);
};
view_struct!(acadrust::objects::AssocViewLabelActionParam {
    single_dependency,
    class_version,
    label_version,
    offset,
    flag
});
const _: fn(&acadrust::objects::AssocViewLabelActionParam) = |value| {
    let _ = view(&value.single_dependency);
    let _ = view(&value.class_version);
    let _ = view(&value.label_version);
    let _ = view(&value.offset);
    let _ = view(&value.flag);
};
view_enum!(acadrust::objects::AssocSubcurveKind {
    None,
    Arc,
    Ellipse,
    Line,
    LineSegment3d,
    Nurb3d,
    Curve3d,
});
view_struct!(acadrust::objects::AssocEdgeActionParam {
    single_dependency,
    parameter,
    has_action,
    action_type,
    subcurve_kind,
    curve
});
const _: fn(&acadrust::objects::AssocEdgeActionParam) = |value| {
    let _ = view(&value.single_dependency);
    let _ = view(&value.parameter);
    let _ = view(&value.has_action);
    let _ = view(&value.action_type);
    let _ = view(&value.subcurve_kind);
    let _ = view(&value.curve);
};
view_enum!(acadrust::objects::AssocCurveValue {
    Bool(v0),
    Int(v0),
    Real(v0),
    Point(v0),
});
view_struct!(acadrust::objects::AssocConstraintNode {
    node_id,
    status,
    connections,
    class_name,
    registry_flag,
    data
});
const _: fn(&acadrust::objects::AssocConstraintNode) = |value| {
    let _ = view(&value.node_id);
    let _ = view(&value.status);
    let _ = view(&value.connections);
    let _ = view(&value.class_name);
    let _ = view(&value.registry_flag);
    let _ = view(&value.data);
};
impl NativeView for acadrust::objects::AssocConstraintNodeData {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Geometrical { .. } => 3,
            Self::Composite { .. } => 4,
            Self::HelpParameter { .. } => 2,
            Self::Angle { .. } => 6,
            Self::Parallel { .. } => 4,
            Self::Distance { .. } => 7,
            Self::RadiusDiameter { .. } => 6,
            Self::ImplicitPoint { .. } => 6,
            Self::Point { .. } => 3,
            Self::RigidSet { .. } => 5,
            Self::Line { .. } => 4,
            Self::BoundedLine { .. } => 7,
            Self::Circle { .. } => 9,
            Self::Arc { .. } => 11,
            Self::Ellipse { .. } => 5,
            Self::BoundedEllipse { .. } => 7,
            Self::Spline { .. } => 16,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::None => "None",
            Self::Geometrical { .. } => "Geometrical",
            Self::Composite { .. } => "Composite",
            Self::HelpParameter { .. } => "HelpParameter",
            Self::Angle { .. } => "Angle",
            Self::Parallel { .. } => "Parallel",
            Self::Distance { .. } => "Distance",
            Self::RadiusDiameter { .. } => "RadiusDiameter",
            Self::ImplicitPoint { .. } => "ImplicitPoint",
            Self::Point { .. } => "Point",
            Self::RigidSet { .. } => "RigidSet",
            Self::Line { .. } => "Line",
            Self::BoundedLine { .. } => "BoundedLine",
            Self::Circle { .. } => "Circle",
            Self::Arc { .. } => "Arc",
            Self::Ellipse { .. } => "Ellipse",
            Self::BoundedEllipse { .. } => "BoundedEllipse",
            Self::Spline { .. } => "Spline",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::None => match index {
                _ => None,
            },
            Self::Geometrical {
                owner_id,
                is_implied,
                is_active,
            } => match index {
                0 => Some((StringView::new("owner_id"), view(owner_id))),
                1 => Some((StringView::new("is_implied"), view(is_implied))),
                2 => Some((StringView::new("is_active"), view(is_active))),
                _ => None,
            },
            Self::Composite {
                owner_id,
                is_implied,
                is_active,
                owned_constraint_ids,
            } => match index {
                0 => Some((StringView::new("owner_id"), view(owner_id))),
                1 => Some((StringView::new("is_implied"), view(is_implied))),
                2 => Some((StringView::new("is_active"), view(is_active))),
                3 => Some((
                    StringView::new("owned_constraint_ids"),
                    view(owned_constraint_ids),
                )),
                _ => None,
            },
            Self::HelpParameter { value, reserved } => match index {
                0 => Some((StringView::new("value"), view(value))),
                1 => Some((StringView::new("reserved"), view(reserved))),
                _ => None,
            },
            Self::Angle {
                owner_id,
                is_implied,
                is_active,
                value_dependency,
                dimension_dependency,
                sector_type,
            } => match index {
                0 => Some((StringView::new("owner_id"), view(owner_id))),
                1 => Some((StringView::new("is_implied"), view(is_implied))),
                2 => Some((StringView::new("is_active"), view(is_active))),
                3 => Some((StringView::new("value_dependency"), view(value_dependency))),
                4 => Some((
                    StringView::new("dimension_dependency"),
                    view(dimension_dependency),
                )),
                5 => Some((StringView::new("sector_type"), view(sector_type))),
                _ => None,
            },
            Self::Parallel {
                owner_id,
                is_implied,
                is_active,
                datum_line_index,
            } => match index {
                0 => Some((StringView::new("owner_id"), view(owner_id))),
                1 => Some((StringView::new("is_implied"), view(is_implied))),
                2 => Some((StringView::new("is_active"), view(is_active))),
                3 => Some((StringView::new("datum_line_index"), view(datum_line_index))),
                _ => None,
            },
            Self::Distance {
                owner_id,
                is_implied,
                is_active,
                value_dependency,
                dimension_dependency,
                direction_type,
                distance,
            } => match index {
                0 => Some((StringView::new("owner_id"), view(owner_id))),
                1 => Some((StringView::new("is_implied"), view(is_implied))),
                2 => Some((StringView::new("is_active"), view(is_active))),
                3 => Some((StringView::new("value_dependency"), view(value_dependency))),
                4 => Some((
                    StringView::new("dimension_dependency"),
                    view(dimension_dependency),
                )),
                5 => Some((StringView::new("direction_type"), view(direction_type))),
                6 => Some((StringView::new("distance"), view(distance))),
                _ => None,
            },
            Self::RadiusDiameter {
                owner_id,
                is_implied,
                is_active,
                value_dependency,
                dimension_dependency,
                mode,
            } => match index {
                0 => Some((StringView::new("owner_id"), view(owner_id))),
                1 => Some((StringView::new("is_implied"), view(is_implied))),
                2 => Some((StringView::new("is_active"), view(is_active))),
                3 => Some((StringView::new("value_dependency"), view(value_dependency))),
                4 => Some((
                    StringView::new("dimension_dependency"),
                    view(dimension_dependency),
                )),
                5 => Some((StringView::new("mode"), view(mode))),
                _ => None,
            },
            Self::ImplicitPoint {
                geometry_dependency,
                geometry_node_id,
                point,
                point_type,
                point_index,
                curve_id,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("point"), view(point))),
                3 => Some((StringView::new("point_type"), view(point_type))),
                4 => Some((StringView::new("point_index"), view(point_index))),
                5 => Some((StringView::new("curve_id"), view(curve_id))),
                _ => None,
            },
            Self::Point {
                geometry_dependency,
                geometry_node_id,
                point,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("point"), view(point))),
                _ => None,
            },
            Self::RigidSet {
                geometry_dependency,
                geometry_node_id,
                reserved,
                transform,
                geometry_ids,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("reserved"), view(reserved))),
                3 => Some((StringView::new("transform"), view(transform))),
                4 => Some((StringView::new("geometry_ids"), view(geometry_ids))),
                _ => None,
            },
            Self::Line {
                geometry_dependency,
                geometry_node_id,
                point,
                direction,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("point"), view(point))),
                3 => Some((StringView::new("direction"), view(direction))),
                _ => None,
            },
            Self::BoundedLine {
                geometry_dependency,
                geometry_node_id,
                point,
                direction,
                is_ray,
                start_point,
                end_point,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("point"), view(point))),
                3 => Some((StringView::new("direction"), view(direction))),
                4 => Some((StringView::new("is_ray"), view(is_ray))),
                5 => Some((StringView::new("start_point"), view(start_point))),
                6 => Some((StringView::new("end_point"), view(end_point))),
                _ => None,
            },
            Self::Circle {
                geometry_dependency,
                geometry_node_id,
                center,
                normal,
                direction,
                radius,
                start_parameter,
                end_parameter,
                reserved,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("center"), view(center))),
                3 => Some((StringView::new("normal"), view(normal))),
                4 => Some((StringView::new("direction"), view(direction))),
                5 => Some((StringView::new("radius"), view(radius))),
                6 => Some((StringView::new("start_parameter"), view(start_parameter))),
                7 => Some((StringView::new("end_parameter"), view(end_parameter))),
                8 => Some((StringView::new("reserved"), view(reserved))),
                _ => None,
            },
            Self::Arc {
                geometry_dependency,
                geometry_node_id,
                center,
                normal,
                direction,
                radius,
                start_parameter,
                end_parameter,
                reserved,
                start_point,
                end_point,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("center"), view(center))),
                3 => Some((StringView::new("normal"), view(normal))),
                4 => Some((StringView::new("direction"), view(direction))),
                5 => Some((StringView::new("radius"), view(radius))),
                6 => Some((StringView::new("start_parameter"), view(start_parameter))),
                7 => Some((StringView::new("end_parameter"), view(end_parameter))),
                8 => Some((StringView::new("reserved"), view(reserved))),
                9 => Some((StringView::new("start_point"), view(start_point))),
                10 => Some((StringView::new("end_point"), view(end_point))),
                _ => None,
            },
            Self::Ellipse {
                geometry_dependency,
                geometry_node_id,
                center,
                major_axis,
                axis_ratio,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("center"), view(center))),
                3 => Some((StringView::new("major_axis"), view(major_axis))),
                4 => Some((StringView::new("axis_ratio"), view(axis_ratio))),
                _ => None,
            },
            Self::BoundedEllipse {
                geometry_dependency,
                geometry_node_id,
                center,
                major_axis,
                axis_ratio,
                start_point,
                end_point,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("center"), view(center))),
                3 => Some((StringView::new("major_axis"), view(major_axis))),
                4 => Some((StringView::new("axis_ratio"), view(axis_ratio))),
                5 => Some((StringView::new("start_point"), view(start_point))),
                6 => Some((StringView::new("end_point"), view(end_point))),
                _ => None,
            },
            Self::Spline {
                geometry_dependency,
                geometry_node_id,
                rational,
                periodic,
                degree,
                knot_tolerance,
                knot_physical_length,
                knot_grow_length,
                knots,
                weight_physical_length,
                weight_grow_length,
                weights,
                control_point_physical_length,
                control_point_grow_length,
                control_points,
                implicit_point_ids,
            } => match index {
                0 => Some((
                    StringView::new("geometry_dependency"),
                    view(geometry_dependency),
                )),
                1 => Some((StringView::new("geometry_node_id"), view(geometry_node_id))),
                2 => Some((StringView::new("rational"), view(rational))),
                3 => Some((StringView::new("periodic"), view(periodic))),
                4 => Some((StringView::new("degree"), view(degree))),
                5 => Some((StringView::new("knot_tolerance"), view(knot_tolerance))),
                6 => Some((
                    StringView::new("knot_physical_length"),
                    view(knot_physical_length),
                )),
                7 => Some((StringView::new("knot_grow_length"), view(knot_grow_length))),
                8 => Some((StringView::new("knots"), view(knots))),
                9 => Some((
                    StringView::new("weight_physical_length"),
                    view(weight_physical_length),
                )),
                10 => Some((
                    StringView::new("weight_grow_length"),
                    view(weight_grow_length),
                )),
                11 => Some((StringView::new("weights"), view(weights))),
                12 => Some((
                    StringView::new("control_point_physical_length"),
                    view(control_point_physical_length),
                )),
                13 => Some((
                    StringView::new("control_point_grow_length"),
                    view(control_point_grow_length),
                )),
                14 => Some((StringView::new("control_points"), view(control_points))),
                15 => Some((
                    StringView::new("implicit_point_ids"),
                    view(implicit_point_ids),
                )),
                _ => None,
            },
        }
    }
}
view_struct!(acadrust::objects::Assoc2dConstraintGroup {
    action,
    version,
    flag,
    work_plane,
    dependency,
    actions,
    next_node_id,
    nodes
});
const _: fn(&acadrust::objects::Assoc2dConstraintGroup) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.version);
    let _ = view(&value.flag);
    let _ = view(&value.work_plane);
    let _ = view(&value.dependency);
    let _ = view(&value.actions);
    let _ = view(&value.next_node_id);
    let _ = view(&value.nodes);
};
view_struct!(acadrust::objects::AssocVariable {
    action,
    class_version,
    name,
    expression,
    evaluator,
    description,
    value,
    has_cached_value,
    cached_value,
    flag,
    dependencies
});
const _: fn(&acadrust::objects::AssocVariable) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.class_version);
    let _ = view(&value.name);
    let _ = view(&value.expression);
    let _ = view(&value.evaluator);
    let _ = view(&value.description);
    let _ = view(&value.value);
    let _ = view(&value.has_cached_value);
    let _ = view(&value.cached_value);
    let _ = view(&value.flag);
    let _ = view(&value.dependencies);
};
view_struct!(acadrust::objects::AssocVariableDependency { dependency, flags });
const _: fn(&acadrust::objects::AssocVariableDependency) = |value| {
    let _ = view(&value.dependency);
    let _ = view(&value.flags);
};
view_struct!(acadrust::objects::AssocCompoundActionParam {
    action_param,
    class_version,
    status,
    parameters,
    child_parameter
});
const _: fn(&acadrust::objects::AssocCompoundActionParam) = |value| {
    let _ = view(&value.action_param);
    let _ = view(&value.class_version);
    let _ = view(&value.status);
    let _ = view(&value.parameters);
    let _ = view(&value.child_parameter);
};
view_struct!(acadrust::objects::AssocChildParameter {
    status,
    id,
    parameter,
    secondary_parameter,
    marker,
    tertiary_parameter
});
const _: fn(&acadrust::objects::AssocChildParameter) = |value| {
    let _ = view(&value.status);
    let _ = view(&value.id);
    let _ = view(&value.parameter);
    let _ = view(&value.secondary_parameter);
    let _ = view(&value.marker);
    let _ = view(&value.tertiary_parameter);
};
view_struct!(acadrust::objects::AssocOsnapPointRefActionParam {
    compound,
    status,
    osnap_mode,
    parameter
});
const _: fn(&acadrust::objects::AssocOsnapPointRefActionParam) = |value| {
    let _ = view(&value.compound);
    let _ = view(&value.status);
    let _ = view(&value.osnap_mode);
    let _ = view(&value.parameter);
};
view_struct!(acadrust::objects::AssocPathActionParam { compound, version });
const _: fn(&acadrust::objects::AssocPathActionParam) = |value| {
    let _ = view(&value.compound);
    let _ = view(&value.version);
};
view_struct!(acadrust::objects::AssocDimDependencyBody {
    dependency_body_version,
    base_version,
    name,
    class_version
});
const _: fn(&acadrust::objects::AssocDimDependencyBody) = |value| {
    let _ = view(&value.dependency_body_version);
    let _ = view(&value.base_version);
    let _ = view(&value.name);
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::AssocFaceActionParam {
    single_dependency,
    index
});
const _: fn(&acadrust::objects::AssocFaceActionParam) = |value| {
    let _ = view(&value.single_dependency);
    let _ = view(&value.index);
};
view_struct!(acadrust::objects::AssocVertexActionParam {
    single_dependency,
    point
});
const _: fn(&acadrust::objects::AssocVertexActionParam) = |value| {
    let _ = view(&value.single_dependency);
    let _ = view(&value.point);
};
view_struct!(acadrust::objects::AssocAsmBodyActionParam {
    single_dependency,
    acis_data,
    point_of_reference,
    wires,
    silhouettes,
    history
});
const _: fn(&acadrust::objects::AssocAsmBodyActionParam) = |value| {
    let _ = view(&value.single_dependency);
    let _ = view(&value.acis_data);
    let _ = view(&value.point_of_reference);
    let _ = view(&value.wires);
    let _ = view(&value.silhouettes);
    let _ = view(&value.history);
};
view_struct!(acadrust::objects::AssocArrayItem {
    class_version,
    location,
    flags,
    uses_default_transform,
    x_direction,
    transform,
    relative_transform,
    first_handle,
    second_handle
});
const _: fn(&acadrust::objects::AssocArrayItem) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.location);
    let _ = view(&value.flags);
    let _ = view(&value.uses_default_transform);
    let _ = view(&value.x_direction);
    let _ = view(&value.transform);
    let _ = view(&value.relative_transform);
    let _ = view(&value.first_handle);
    let _ = view(&value.second_handle);
};
view_struct!(acadrust::objects::AssocArrayParameters {
    version,
    class_name,
    items,
    item_count,
    row_count,
    level_count
});
const _: fn(&acadrust::objects::AssocArrayParameters) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.class_name);
    let _ = view(&value.items);
    let _ = view(&value.item_count);
    let _ = view(&value.row_count);
    let _ = view(&value.level_count);
};
view_struct!(acadrust::objects::AssocArrayActionBody {
    action_body,
    parameter_body,
    version,
    parameter_block,
    item_list_version,
    item_class,
    items,
    transform
});
const _: fn(&acadrust::objects::AssocArrayActionBody) = |value| {
    let _ = view(&value.action_body);
    let _ = view(&value.parameter_body);
    let _ = view(&value.version);
    let _ = view(&value.parameter_block);
    let _ = view(&value.item_list_version);
    let _ = view(&value.item_class);
    let _ = view(&value.items);
    let _ = view(&value.transform);
};
view_struct!(acadrust::objects::AssocArrayModifyActionBody {
    body,
    status,
    item_locations
});
const _: fn(&acadrust::objects::AssocArrayModifyActionBody) = |value| {
    let _ = view(&value.body);
    let _ = view(&value.status);
    let _ = view(&value.item_locations);
};
view_struct!(acadrust::objects::AssocDimensionReference {
    class_name,
    osnap_type,
    xrefs,
    main_subent_type,
    main_gs_marker,
    xref_paths,
    osnap_distance,
    osnap_point,
    intersection_objects,
    intersection_subent_type,
    intersection_gs_marker,
    intersection_xref_paths,
    has_last_point_reference
});
const _: fn(&acadrust::objects::AssocDimensionReference) = |value| {
    let _ = view(&value.class_name);
    let _ = view(&value.osnap_type);
    let _ = view(&value.xrefs);
    let _ = view(&value.main_subent_type);
    let _ = view(&value.main_gs_marker);
    let _ = view(&value.xref_paths);
    let _ = view(&value.osnap_distance);
    let _ = view(&value.osnap_point);
    let _ = view(&value.intersection_objects);
    let _ = view(&value.intersection_subent_type);
    let _ = view(&value.intersection_gs_marker);
    let _ = view(&value.intersection_xref_paths);
    let _ = view(&value.has_last_point_reference);
};
view_struct!(acadrust::objects::AssocDimensionAssociation {
    associativity,
    trans_space,
    rotated_type,
    dimension,
    references
});
const _: fn(&acadrust::objects::AssocDimensionAssociation) = |value| {
    let _ = view(&value.associativity);
    let _ = view(&value.trans_space);
    let _ = view(&value.rotated_type);
    let _ = view(&value.dimension);
    let _ = view(&value.references);
};
view_struct!(acadrust::objects::PersSubentManager {
    class_version,
    marker_zero,
    marker_two,
    associative_step_count,
    associative_subent_count,
    values
});
const _: fn(&acadrust::objects::PersSubentManager) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.marker_zero);
    let _ = view(&value.marker_two);
    let _ = view(&value.associative_step_count);
    let _ = view(&value.associative_subent_count);
    let _ = view(&value.values);
};

// objects/block_visibility.rs
view_enum!(acadrust::objects::BlockEvalValue {
    None,
    Real(v0),
    Point(v0),
    Text(v0),
    Long(v0),
    Handle(v0),
    Short(v0),
});
view_struct!(acadrust::objects::BlockParameterConnection { code, name });
const _: fn(&acadrust::objects::BlockParameterConnection) = |value| {
    let _ = view(&value.code);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::BlockParameterPropertyInfo { connections });
const _: fn(&acadrust::objects::BlockParameterPropertyInfo) = |value| {
    let _ = view(&value.connections);
};
view_struct!(acadrust::objects::BlockVisibilityState {
    name,
    visible_blocks,
    visible_params
});
const _: fn(&acadrust::objects::BlockVisibilityState) = |value| {
    let _ = view(&value.name);
    let _ = view(&value.visible_blocks);
    let _ = view(&value.visible_params);
};
view_struct!(acadrust::objects::BlockVisibilityParameter {
    handle,
    owner,
    eval_parent_id,
    eval_major,
    eval_minor,
    eval_value_code,
    eval_value,
    eval_node_id,
    element_name,
    element_major,
    element_minor,
    element_eed_1071,
    show_properties,
    chain_actions,
    name,
    description,
    def_point,
    property_info,
    property_info_count,
    is_initialized,
    unknown_bool,
    all_blocks,
    states
});
const _: fn(&acadrust::objects::BlockVisibilityParameter) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.eval_parent_id);
    let _ = view(&value.eval_major);
    let _ = view(&value.eval_minor);
    let _ = view(&value.eval_value_code);
    let _ = view(&value.eval_value);
    let _ = view(&value.eval_node_id);
    let _ = view(&value.element_name);
    let _ = view(&value.element_major);
    let _ = view(&value.element_minor);
    let _ = view(&value.element_eed_1071);
    let _ = view(&value.show_properties);
    let _ = view(&value.chain_actions);
    let _ = view(&value.name);
    let _ = view(&value.description);
    let _ = view(&value.def_point);
    let _ = view(&value.property_info);
    let _ = view(&value.property_info_count);
    let _ = view(&value.is_initialized);
    let _ = view(&value.unknown_bool);
    let _ = view(&value.all_blocks);
    let _ = view(&value.states);
};

// objects/class_object.rs
view_struct!(acadrust::objects::ClassObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    data
});
const _: fn(&acadrust::objects::ClassObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.data);
};
view_enum!(acadrust::objects::ClassObjectData {
    Empty,
    SpatialIndex(v0),
    LayerFilter(v0),
    PartialViewingIndex(v0),
    VbaProject(v0),
    SectionManager(v0),
    SectionSettings(v0),
    LightList(v0),
    Sun(v0),
    RenderSettings(v0),
    MentalRayRenderSettings(v0),
    RapidRtRenderSettings(v0),
    GradientBackground(v0),
    GroundPlaneBackground(v0),
    IblBackground(v0),
    ImageBackground(v0),
    SkyLightBackground(v0),
    SolidBackground(v0),
    RenderEntry(v0),
    RenderEnvironment(v0),
    RenderGlobal(v0),
    MotionPath(v0),
    CurvePath(v0),
    PointPath(v0),
    TvDeviceProperties(v0),
    PointCloudDefinition(v0),
    PointCloudDefinitionEx(v0),
    PointCloudDefinitionReactor(v0),
    PointCloudDefinitionReactorEx(v0),
    PointCloudColorMap(v0),
    NavisworksModelDefinition(v0),
    ContextDataManager(v0),
    SunStudy(v0),
    DataTable(v0),
    DataLink(v0),
    PersistentSubentityManager(v0),
    GeoMapImage(v0),
    DetailViewStyle(v0),
    SectionViewStyle(v0),
    AcMeCommandHistory(v0),
    AcMeScope(v0),
    AcMeStateManager(v0),
    CsacDocumentOptions(v0),
    ViewRepSourceManager(v0),
    ViewRepStandard(v0),
    ViewRepOrientationDefinition,
    ViewRepOrientation(v0),
    ViewRepSectionDefinition(v0),
    ViewRepModelSpaceViewSelectionSet(v0),
    ViewRep(v0),
    ViewRepModelSpaceSource(v0),
});
view_struct!(acadrust::objects::ViewRepSourceManager {
    has_source,
    source,
    status
});
const _: fn(&acadrust::objects::ViewRepSourceManager) = |value| {
    let _ = view(&value.has_source);
    let _ = view(&value.source);
    let _ = view(&value.status);
};
view_struct!(acadrust::objects::ViewRepStandard { values });
const _: fn(&acadrust::objects::ViewRepStandard) = |value| {
    let _ = view(&value.values);
};
view_struct!(acadrust::objects::ViewRepOrientation {
    camera,
    target,
    normal
});
const _: fn(&acadrust::objects::ViewRepOrientation) = |value| {
    let _ = view(&value.camera);
    let _ = view(&value.target);
    let _ = view(&value.normal);
};
view_struct!(acadrust::objects::ViewRepSectionDefinition {
    version,
    section_depth,
    flags
});
const _: fn(&acadrust::objects::ViewRepSectionDefinition) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.section_depth);
    let _ = view(&value.flags);
};
view_struct!(acadrust::objects::ViewRepModelSpaceViewSelectionSet { version, entities });
const _: fn(&acadrust::objects::ViewRepModelSpaceViewSelectionSet) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.entities);
};
view_struct!(acadrust::objects::ViewRepGuid {
    data1,
    data2,
    data3,
    data4
});
const _: fn(&acadrust::objects::ViewRepGuid) = |value| {
    let _ = view(&value.data1);
    let _ = view(&value.data2);
    let _ = view(&value.data3);
    let _ = view(&value.data4);
};
view_struct!(acadrust::objects::ViewRepModelSpaceSource {
    enabled,
    header_values,
    transform,
    source_version,
    source_status,
    model,
    guid,
    references,
    tail_values,
    orientation
});
const _: fn(&acadrust::objects::ViewRepModelSpaceSource) = |value| {
    let _ = view(&value.enabled);
    let _ = view(&value.header_values);
    let _ = view(&value.transform);
    let _ = view(&value.source_version);
    let _ = view(&value.source_status);
    let _ = view(&value.model);
    let _ = view(&value.guid);
    let _ = view(&value.references);
    let _ = view(&value.tail_values);
    let _ = view(&value.orientation);
};
view_struct!(acadrust::objects::ViewRepSketchReference { object, flag });
const _: fn(&acadrust::objects::ViewRepSketchReference) = |value| {
    let _ = view(&value.object);
    let _ = view(&value.flag);
};
impl NativeView for acadrust::objects::ViewRepSketchGeometry {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Line { .. } => 3,
            Self::Circle { .. } => 8,
            Self::Nurb { .. } => 10,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::None => "None",
            Self::Line { .. } => "Line",
            Self::Circle { .. } => "Circle",
            Self::Nurb { .. } => "Nurb",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::None => match index {
                _ => None,
            },
            Self::Line {
                type_code,
                first,
                second,
            } => match index {
                0 => Some((StringView::new("type_code"), view(type_code))),
                1 => Some((StringView::new("first"), view(first))),
                2 => Some((StringView::new("second"), view(second))),
                _ => None,
            },
            Self::Circle {
                type_code,
                center,
                normal,
                direction,
                radius,
                start_parameter,
                end_parameter,
                reserved,
            } => match index {
                0 => Some((StringView::new("type_code"), view(type_code))),
                1 => Some((StringView::new("center"), view(center))),
                2 => Some((StringView::new("normal"), view(normal))),
                3 => Some((StringView::new("direction"), view(direction))),
                4 => Some((StringView::new("radius"), view(radius))),
                5 => Some((StringView::new("start_parameter"), view(start_parameter))),
                6 => Some((StringView::new("end_parameter"), view(end_parameter))),
                7 => Some((StringView::new("reserved"), view(reserved))),
                _ => None,
            },
            Self::Nurb {
                type_code,
                flags,
                degree,
                tolerance,
                knot_header,
                knots,
                weight_header,
                weights,
                point_header,
                control_points,
            } => match index {
                0 => Some((StringView::new("type_code"), view(type_code))),
                1 => Some((StringView::new("flags"), view(flags))),
                2 => Some((StringView::new("degree"), view(degree))),
                3 => Some((StringView::new("tolerance"), view(tolerance))),
                4 => Some((StringView::new("knot_header"), view(knot_header))),
                5 => Some((StringView::new("knots"), view(knots))),
                6 => Some((StringView::new("weight_header"), view(weight_header))),
                7 => Some((StringView::new("weights"), view(weights))),
                8 => Some((StringView::new("point_header"), view(point_header))),
                9 => Some((StringView::new("control_points"), view(control_points))),
                _ => None,
            },
        }
    }
}
view_struct!(acadrust::objects::ViewRepSketch {
    id,
    version,
    references,
    reserved,
    enabled,
    geometry,
    final_flag
});
const _: fn(&acadrust::objects::ViewRepSketch) = |value| {
    let _ = view(&value.id);
    let _ = view(&value.version);
    let _ = view(&value.references);
    let _ = view(&value.reserved);
    let _ = view(&value.enabled);
    let _ = view(&value.geometry);
    let _ = view(&value.final_flag);
};
view_struct!(acadrust::objects::ViewRepObjectPath {
    class_name,
    objects
});
const _: fn(&acadrust::objects::ViewRepObjectPath) = |value| {
    let _ = view(&value.class_name);
    let _ = view(&value.objects);
};
view_struct!(acadrust::objects::ViewRepBlockPathEntry { flag, kind, object });
const _: fn(&acadrust::objects::ViewRepBlockPathEntry) = |value| {
    let _ = view(&value.flag);
    let _ = view(&value.kind);
    let _ = view(&value.object);
};
view_struct!(acadrust::objects::ViewRepBlockPath {
    class_name,
    version,
    entries
});
const _: fn(&acadrust::objects::ViewRepBlockPath) = |value| {
    let _ = view(&value.class_name);
    let _ = view(&value.version);
    let _ = view(&value.entries);
};
view_struct!(acadrust::objects::ViewRep {
    header_values,
    name,
    scale,
    header_status,
    description,
    source_id,
    source_enabled,
    source_version,
    model_id,
    guid,
    marker,
    transform,
    transform_version,
    database_id,
    geometry_version,
    geometry_marker,
    sketches,
    related_objects,
    source_manager,
    owned_objects,
    optional_objects,
    position,
    rotation,
    orientation,
    is_active,
    projection,
    linked_views,
    section_sketches,
    action_mode,
    action,
    has_parent,
    parent,
    tail_version,
    tail_state,
    tail_id,
    path_count,
    path_version,
    path_id,
    has_block_path,
    block_path,
    style
});
const _: fn(&acadrust::objects::ViewRep) = |value| {
    let _ = view(&value.header_values);
    let _ = view(&value.name);
    let _ = view(&value.scale);
    let _ = view(&value.header_status);
    let _ = view(&value.description);
    let _ = view(&value.source_id);
    let _ = view(&value.source_enabled);
    let _ = view(&value.source_version);
    let _ = view(&value.model_id);
    let _ = view(&value.guid);
    let _ = view(&value.marker);
    let _ = view(&value.transform);
    let _ = view(&value.transform_version);
    let _ = view(&value.database_id);
    let _ = view(&value.geometry_version);
    let _ = view(&value.geometry_marker);
    let _ = view(&value.sketches);
    let _ = view(&value.related_objects);
    let _ = view(&value.source_manager);
    let _ = view(&value.owned_objects);
    let _ = view(&value.optional_objects);
    let _ = view(&value.position);
    let _ = view(&value.rotation);
    let _ = view(&value.orientation);
    let _ = view(&value.is_active);
    let _ = view(&value.projection);
    let _ = view(&value.linked_views);
    let _ = view(&value.section_sketches);
    let _ = view(&value.action_mode);
    let _ = view(&value.action);
    let _ = view(&value.has_parent);
    let _ = view(&value.parent);
    let _ = view(&value.tail_version);
    let _ = view(&value.tail_state);
    let _ = view(&value.tail_id);
    let _ = view(&value.path_count);
    let _ = view(&value.path_version);
    let _ = view(&value.path_id);
    let _ = view(&value.has_block_path);
    let _ = view(&value.block_path);
    let _ = view(&value.style);
};
view_struct!(acadrust::objects::AcMeCommandHistory { class_version });
const _: fn(&acadrust::objects::AcMeCommandHistory) = |value| {
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::AcMeScope { class_version });
const _: fn(&acadrust::objects::AcMeScope) = |value| {
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::AcMeStateManager { class_version });
const _: fn(&acadrust::objects::AcMeStateManager) = |value| {
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::CsacDocumentOptions {
    class_version,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
const _: fn(&acadrust::objects::CsacDocumentOptions) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.raw_dwg_data);
    let _ = view(&value.raw_dwg_handle_bits);
    let _ = view(&value.raw_dwg_version);
};
view_struct!(acadrust::objects::SpatialIndex {
    last_updated_julian_day,
    last_updated_milliseconds,
    min_corner,
    max_corner,
    indexed_objects
});
const _: fn(&acadrust::objects::SpatialIndex) = |value| {
    let _ = view(&value.last_updated_julian_day);
    let _ = view(&value.last_updated_milliseconds);
    let _ = view(&value.min_corner);
    let _ = view(&value.max_corner);
    let _ = view(&value.indexed_objects);
};
view_struct!(acadrust::objects::LayerFilter { names });
const _: fn(&acadrust::objects::LayerFilter) = |value| {
    let _ = view(&value.names);
};
view_struct!(acadrust::objects::PartialViewingIndexEntry {
    extents_min,
    extents_max,
    object
});
const _: fn(&acadrust::objects::PartialViewingIndexEntry) = |value| {
    let _ = view(&value.extents_min);
    let _ = view(&value.extents_max);
    let _ = view(&value.object);
};
view_struct!(acadrust::objects::PartialViewingIndex {
    has_entries,
    entries
});
const _: fn(&acadrust::objects::PartialViewingIndex) = |value| {
    let _ = view(&value.has_entries);
    let _ = view(&value.entries);
};
view_struct!(acadrust::objects::VbaProject { storage });
const _: fn(&acadrust::objects::VbaProject) = |value| {
    let _ = view(&value.storage);
};
view_struct!(acadrust::objects::SectionManager { is_live, sections });
const _: fn(&acadrust::objects::SectionManager) = |value| {
    let _ = view(&value.is_live);
    let _ = view(&value.sections);
};
view_struct!(acadrust::objects::SectionGeometrySettings {
    geometry_count,
    index,
    flags,
    color,
    layer,
    linetype,
    linetype_scale,
    plot_style,
    lineweight,
    face_transparency,
    edge_transparency,
    hatch_type,
    hatch_pattern,
    hatch_angle,
    hatch_spacing,
    hatch_scale
});
const _: fn(&acadrust::objects::SectionGeometrySettings) = |value| {
    let _ = view(&value.geometry_count);
    let _ = view(&value.index);
    let _ = view(&value.flags);
    let _ = view(&value.color);
    let _ = view(&value.layer);
    let _ = view(&value.linetype);
    let _ = view(&value.linetype_scale);
    let _ = view(&value.plot_style);
    let _ = view(&value.lineweight);
    let _ = view(&value.face_transparency);
    let _ = view(&value.edge_transparency);
    let _ = view(&value.hatch_type);
    let _ = view(&value.hatch_pattern);
    let _ = view(&value.hatch_angle);
    let _ = view(&value.hatch_spacing);
    let _ = view(&value.hatch_scale);
};
view_struct!(acadrust::objects::SectionTypeSettings {
    section_type,
    generation,
    sources,
    destination_block,
    destination_file,
    geometry
});
const _: fn(&acadrust::objects::SectionTypeSettings) = |value| {
    let _ = view(&value.section_type);
    let _ = view(&value.generation);
    let _ = view(&value.sources);
    let _ = view(&value.destination_block);
    let _ = view(&value.destination_file);
    let _ = view(&value.geometry);
};
view_struct!(acadrust::objects::SectionSettings {
    current_type,
    types
});
const _: fn(&acadrust::objects::SectionSettings) = |value| {
    let _ = view(&value.current_type);
    let _ = view(&value.types);
};
view_struct!(acadrust::objects::LightListEntry { handle, name });
const _: fn(&acadrust::objects::LightListEntry) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::LightList {
    class_version,
    lights
});
const _: fn(&acadrust::objects::LightList) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.lights);
};
view_struct!(acadrust::objects::Sun {
    class_version,
    is_on,
    color,
    intensity,
    has_shadow,
    julian_day,
    milliseconds,
    is_daylight_savings_on,
    shadow_type,
    shadow_map_size,
    shadow_softness
});
const _: fn(&acadrust::objects::Sun) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.is_on);
    let _ = view(&value.color);
    let _ = view(&value.intensity);
    let _ = view(&value.has_shadow);
    let _ = view(&value.julian_day);
    let _ = view(&value.milliseconds);
    let _ = view(&value.is_daylight_savings_on);
    let _ = view(&value.shadow_type);
    let _ = view(&value.shadow_map_size);
    let _ = view(&value.shadow_softness);
};
view_struct!(acadrust::objects::RenderSettings {
    class_version,
    name,
    fog_enabled,
    fog_background_enabled,
    backfaces_enabled,
    environment_image_enabled,
    environment_image_filename,
    description,
    display_index,
    has_predefined
});
const _: fn(&acadrust::objects::RenderSettings) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.name);
    let _ = view(&value.fog_enabled);
    let _ = view(&value.fog_background_enabled);
    let _ = view(&value.backfaces_enabled);
    let _ = view(&value.environment_image_enabled);
    let _ = view(&value.environment_image_filename);
    let _ = view(&value.description);
    let _ = view(&value.display_index);
    let _ = view(&value.has_predefined);
};
view_struct!(acadrust::objects::MentalRayRenderSettings {
    base,
    version,
    sampling_min,
    sampling_max,
    sampling_filter,
    sampling_filter_width,
    sampling_filter_height,
    sampling_contrast,
    shadow_mode,
    shadow_maps_enabled,
    ray_tracing_enabled,
    ray_trace_depth,
    global_illumination_enabled,
    global_illumination_sample_count,
    global_illumination_sample_radius_enabled,
    global_illumination_sample_radius,
    photons_per_light,
    photon_trace_depth,
    final_gathering_enabled,
    final_gathering_ray_count,
    final_gathering_sample_radius_state,
    final_gathering_sample_radius,
    light_luminance_scale,
    diagnostics_mode,
    diagnostics_grid_mode,
    diagnostics_grid_size,
    diagnostics_photon_mode,
    diagnostics_bsp_mode,
    export_mi_enabled,
    description,
    tile_size,
    tile_order,
    memory_limit,
    diagnostics_samples_mode,
    energy_multiplier
});
const _: fn(&acadrust::objects::MentalRayRenderSettings) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.version);
    let _ = view(&value.sampling_min);
    let _ = view(&value.sampling_max);
    let _ = view(&value.sampling_filter);
    let _ = view(&value.sampling_filter_width);
    let _ = view(&value.sampling_filter_height);
    let _ = view(&value.sampling_contrast);
    let _ = view(&value.shadow_mode);
    let _ = view(&value.shadow_maps_enabled);
    let _ = view(&value.ray_tracing_enabled);
    let _ = view(&value.ray_trace_depth);
    let _ = view(&value.global_illumination_enabled);
    let _ = view(&value.global_illumination_sample_count);
    let _ = view(&value.global_illumination_sample_radius_enabled);
    let _ = view(&value.global_illumination_sample_radius);
    let _ = view(&value.photons_per_light);
    let _ = view(&value.photon_trace_depth);
    let _ = view(&value.final_gathering_enabled);
    let _ = view(&value.final_gathering_ray_count);
    let _ = view(&value.final_gathering_sample_radius_state);
    let _ = view(&value.final_gathering_sample_radius);
    let _ = view(&value.light_luminance_scale);
    let _ = view(&value.diagnostics_mode);
    let _ = view(&value.diagnostics_grid_mode);
    let _ = view(&value.diagnostics_grid_size);
    let _ = view(&value.diagnostics_photon_mode);
    let _ = view(&value.diagnostics_bsp_mode);
    let _ = view(&value.export_mi_enabled);
    let _ = view(&value.description);
    let _ = view(&value.tile_size);
    let _ = view(&value.tile_order);
    let _ = view(&value.memory_limit);
    let _ = view(&value.diagnostics_samples_mode);
    let _ = view(&value.energy_multiplier);
};
view_struct!(acadrust::objects::RapidRtRenderSettings {
    base,
    version,
    render_target,
    render_level,
    render_time,
    lighting_model,
    filter_type,
    filter_width,
    filter_height
});
const _: fn(&acadrust::objects::RapidRtRenderSettings) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.version);
    let _ = view(&value.render_target);
    let _ = view(&value.render_level);
    let _ = view(&value.render_time);
    let _ = view(&value.lighting_model);
    let _ = view(&value.filter_type);
    let _ = view(&value.filter_width);
    let _ = view(&value.filter_height);
};
view_struct!(acadrust::objects::GradientBackground {
    class_version,
    color_top,
    color_middle,
    color_bottom,
    horizon,
    height,
    rotation
});
const _: fn(&acadrust::objects::GradientBackground) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.color_top);
    let _ = view(&value.color_middle);
    let _ = view(&value.color_bottom);
    let _ = view(&value.horizon);
    let _ = view(&value.height);
    let _ = view(&value.rotation);
};
view_struct!(acadrust::objects::GroundPlaneBackground {
    class_version,
    color_sky_zenith,
    color_sky_horizon,
    color_underground_horizon,
    color_underground_azimuth,
    color_near,
    color_far
});
const _: fn(&acadrust::objects::GroundPlaneBackground) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.color_sky_zenith);
    let _ = view(&value.color_sky_horizon);
    let _ = view(&value.color_underground_horizon);
    let _ = view(&value.color_underground_azimuth);
    let _ = view(&value.color_near);
    let _ = view(&value.color_far);
};
view_struct!(acadrust::objects::IblBackground {
    class_version,
    enabled,
    name,
    rotation,
    display_image,
    secondary_background
});
const _: fn(&acadrust::objects::IblBackground) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.enabled);
    let _ = view(&value.name);
    let _ = view(&value.rotation);
    let _ = view(&value.display_image);
    let _ = view(&value.secondary_background);
};
view_struct!(acadrust::objects::ImageBackground {
    class_version,
    filename,
    fit_to_screen,
    maintain_aspect_ratio,
    use_tiling,
    offset,
    scale
});
const _: fn(&acadrust::objects::ImageBackground) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.filename);
    let _ = view(&value.fit_to_screen);
    let _ = view(&value.maintain_aspect_ratio);
    let _ = view(&value.use_tiling);
    let _ = view(&value.offset);
    let _ = view(&value.scale);
};
view_struct!(acadrust::objects::SkyLightBackground { class_version, sun });
const _: fn(&acadrust::objects::SkyLightBackground) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.sun);
};
view_struct!(acadrust::objects::SolidBackground {
    class_version,
    color
});
const _: fn(&acadrust::objects::SolidBackground) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.color);
};
view_struct!(acadrust::objects::RenderEntry {
    class_version,
    image_filename,
    preset_name,
    view_name,
    width,
    height,
    start_year,
    start_month,
    start_day,
    start_hour,
    start_minute,
    start_second,
    start_millisecond,
    end_year,
    end_month,
    end_day,
    end_hour,
    end_minute,
    end_second,
    end_millisecond,
    render_time,
    memory_amount,
    material_count,
    light_count,
    triangle_count,
    display_index
});
const _: fn(&acadrust::objects::RenderEntry) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.image_filename);
    let _ = view(&value.preset_name);
    let _ = view(&value.view_name);
    let _ = view(&value.width);
    let _ = view(&value.height);
    let _ = view(&value.start_year);
    let _ = view(&value.start_month);
    let _ = view(&value.start_day);
    let _ = view(&value.start_hour);
    let _ = view(&value.start_minute);
    let _ = view(&value.start_second);
    let _ = view(&value.start_millisecond);
    let _ = view(&value.end_year);
    let _ = view(&value.end_month);
    let _ = view(&value.end_day);
    let _ = view(&value.end_hour);
    let _ = view(&value.end_minute);
    let _ = view(&value.end_second);
    let _ = view(&value.end_millisecond);
    let _ = view(&value.render_time);
    let _ = view(&value.memory_amount);
    let _ = view(&value.material_count);
    let _ = view(&value.light_count);
    let _ = view(&value.triangle_count);
    let _ = view(&value.display_index);
};
view_struct!(acadrust::objects::RenderEnvironment {
    class_version,
    fog_enabled,
    fog_background_enabled,
    fog_color,
    fog_density_near,
    fog_density_far,
    fog_distance_near,
    fog_distance_far,
    environment_image_enabled,
    environment_image_filename
});
const _: fn(&acadrust::objects::RenderEnvironment) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.fog_enabled);
    let _ = view(&value.fog_background_enabled);
    let _ = view(&value.fog_color);
    let _ = view(&value.fog_density_near);
    let _ = view(&value.fog_density_far);
    let _ = view(&value.fog_distance_near);
    let _ = view(&value.fog_distance_far);
    let _ = view(&value.environment_image_enabled);
    let _ = view(&value.environment_image_filename);
};
view_struct!(acadrust::objects::RenderGlobal {
    class_version,
    procedure,
    destination,
    save_enabled,
    save_filename,
    image_width,
    image_height,
    predefined_presets_first,
    high_level_info
});
const _: fn(&acadrust::objects::RenderGlobal) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.procedure);
    let _ = view(&value.destination);
    let _ = view(&value.save_enabled);
    let _ = view(&value.save_filename);
    let _ = view(&value.image_width);
    let _ = view(&value.image_height);
    let _ = view(&value.predefined_presets_first);
    let _ = view(&value.high_level_info);
};
view_struct!(acadrust::objects::MotionPath {
    class_version,
    camera_path,
    target_path,
    view,
    frames,
    frame_rate,
    corner_deceleration
});
const _: fn(&acadrust::objects::MotionPath) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.camera_path);
    let _ = view(&value.target_path);
    let _ = view(&value.view);
    let _ = view(&value.frames);
    let _ = view(&value.frame_rate);
    let _ = view(&value.corner_deceleration);
};
view_struct!(acadrust::objects::CurvePath {
    class_version,
    entity
});
const _: fn(&acadrust::objects::CurvePath) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.entity);
};
view_struct!(acadrust::objects::PointPath {
    class_version,
    point
});
const _: fn(&acadrust::objects::PointPath) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.point);
};
view_struct!(acadrust::objects::TvDeviceProperties {
    flags,
    max_regen_threads,
    use_lut_palette,
    alternate_highlight,
    alternate_highlight_color,
    geometry_shader_usage,
    blending_mode,
    antialiasing_level,
    reserved_double
});
const _: fn(&acadrust::objects::TvDeviceProperties) = |value| {
    let _ = view(&value.flags);
    let _ = view(&value.max_regen_threads);
    let _ = view(&value.use_lut_palette);
    let _ = view(&value.alternate_highlight);
    let _ = view(&value.alternate_highlight_color);
    let _ = view(&value.geometry_shader_usage);
    let _ = view(&value.blending_mode);
    let _ = view(&value.antialiasing_level);
    let _ = view(&value.reserved_double);
};
view_struct!(acadrust::objects::PointCloudDefinition {
    class_version,
    source_filename,
    is_loaded,
    point_count,
    extents_min,
    extents_max
});
const _: fn(&acadrust::objects::PointCloudDefinition) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.source_filename);
    let _ = view(&value.is_loaded);
    let _ = view(&value.point_count);
    let _ = view(&value.extents_min);
    let _ = view(&value.extents_max);
};
view_struct!(acadrust::objects::PointCloudDefinitionReactor { class_version });
const _: fn(&acadrust::objects::PointCloudDefinitionReactor) = |value| {
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::PointCloudColorRamp {
    id,
    class_version,
    colors,
    name
});
const _: fn(&acadrust::objects::PointCloudColorRamp) = |value| {
    let _ = view(&value.id);
    let _ = view(&value.class_version);
    let _ = view(&value.colors);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::PointCloudRampColor { color, visible });
const _: fn(&acadrust::objects::PointCloudRampColor) = |value| {
    let _ = view(&value.color);
    let _ = view(&value.visible);
};
view_struct!(acadrust::objects::PointCloudColorMap {
    class_version,
    default_intensity_scheme,
    default_elevation_scheme,
    default_classification_scheme,
    color_ramps,
    classification_color_ramps
});
const _: fn(&acadrust::objects::PointCloudColorMap) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.default_intensity_scheme);
    let _ = view(&value.default_elevation_scheme);
    let _ = view(&value.default_classification_scheme);
    let _ = view(&value.color_ramps);
    let _ = view(&value.classification_color_ramps);
};
view_struct!(acadrust::objects::NavisworksModelDefinition {
    flags,
    path,
    status,
    extents_min,
    extents_max,
    host_drawing_visibility
});
const _: fn(&acadrust::objects::NavisworksModelDefinition) = |value| {
    let _ = view(&value.flags);
    let _ = view(&value.path);
    let _ = view(&value.status);
    let _ = view(&value.extents_min);
    let _ = view(&value.extents_max);
    let _ = view(&value.host_drawing_visibility);
};
view_struct!(acadrust::objects::ContextDataEntry { item, name });
const _: fn(&acadrust::objects::ContextDataEntry) = |value| {
    let _ = view(&value.item);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::ContextDataSubManager { handle, entries });
const _: fn(&acadrust::objects::ContextDataSubManager) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.entries);
};
view_struct!(acadrust::objects::ContextDataManager {
    object_context,
    sub_managers
});
const _: fn(&acadrust::objects::ContextDataManager) = |value| {
    let _ = view(&value.object_context);
    let _ = view(&value.sub_managers);
};
view_struct!(acadrust::objects::SunStudyDate {
    julian_day,
    milliseconds
});
const _: fn(&acadrust::objects::SunStudyDate) = |value| {
    let _ = view(&value.julian_day);
    let _ = view(&value.milliseconds);
};
view_struct!(acadrust::objects::SunStudy {
    class_version,
    setup_name,
    description,
    output_type,
    use_subset,
    sheet_set_name,
    sheet_subset_name,
    select_dates_from_calendar,
    dates,
    select_range_of_dates,
    start_time,
    end_time,
    interval,
    hours,
    shade_plot_type,
    viewport_count,
    rows,
    columns,
    spacing,
    lock_viewports,
    label_viewports,
    page_setup_wizard,
    view,
    visual_style,
    text_style
});
const _: fn(&acadrust::objects::SunStudy) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.setup_name);
    let _ = view(&value.description);
    let _ = view(&value.output_type);
    let _ = view(&value.use_subset);
    let _ = view(&value.sheet_set_name);
    let _ = view(&value.sheet_subset_name);
    let _ = view(&value.select_dates_from_calendar);
    let _ = view(&value.dates);
    let _ = view(&value.select_range_of_dates);
    let _ = view(&value.start_time);
    let _ = view(&value.end_time);
    let _ = view(&value.interval);
    let _ = view(&value.hours);
    let _ = view(&value.shade_plot_type);
    let _ = view(&value.viewport_count);
    let _ = view(&value.rows);
    let _ = view(&value.columns);
    let _ = view(&value.spacing);
    let _ = view(&value.lock_viewports);
    let _ = view(&value.label_viewports);
    let _ = view(&value.page_setup_wizard);
    let _ = view(&value.view);
    let _ = view(&value.visual_style);
    let _ = view(&value.text_style);
};
view_enum!(acadrust::objects::DataTableCellType {
    Integer,
    Double,
    Text,
    Point,
    ObjectId,
    HardOwnerId,
    SoftOwnerId,
    HardPointerId,
    SoftPointerId,
    Bool,
    Vector,
});
view_struct!(acadrust::objects::DataTableValue {
    integer,
    real,
    text,
    point,
    handle
});
const _: fn(&acadrust::objects::DataTableValue) = |value| {
    let _ = view(&value.integer);
    let _ = view(&value.real);
    let _ = view(&value.text);
    let _ = view(&value.point);
    let _ = view(&value.handle);
};
view_struct!(acadrust::objects::DataTableColumn {
    value_type,
    name,
    rows
});
const _: fn(&acadrust::objects::DataTableColumn) = |value| {
    let _ = view(&value.value_type);
    let _ = view(&value.name);
    let _ = view(&value.rows);
};
view_struct!(acadrust::objects::DataTable {
    flags,
    name,
    row_count,
    columns
});
const _: fn(&acadrust::objects::DataTable) = |value| {
    let _ = view(&value.flags);
    let _ = view(&value.name);
    let _ = view(&value.row_count);
    let _ = view(&value.columns);
};
view_struct!(acadrust::objects::DataLinkCustomData { target, value });
const _: fn(&acadrust::objects::DataLinkCustomData) = |value| {
    let _ = view(&value.target);
    let _ = view(&value.value);
};
view_struct!(acadrust::objects::DataLink {
    data_adapter,
    description,
    tooltip,
    connection_string,
    option,
    update_option,
    flags,
    year,
    month,
    day,
    hour,
    minute,
    second,
    millisecond,
    path_option,
    status_flags,
    update_status,
    custom_data,
    hard_owner
});
const _: fn(&acadrust::objects::DataLink) = |value| {
    let _ = view(&value.data_adapter);
    let _ = view(&value.description);
    let _ = view(&value.tooltip);
    let _ = view(&value.connection_string);
    let _ = view(&value.option);
    let _ = view(&value.update_option);
    let _ = view(&value.flags);
    let _ = view(&value.year);
    let _ = view(&value.month);
    let _ = view(&value.day);
    let _ = view(&value.hour);
    let _ = view(&value.minute);
    let _ = view(&value.second);
    let _ = view(&value.millisecond);
    let _ = view(&value.path_option);
    let _ = view(&value.status_flags);
    let _ = view(&value.update_status);
    let _ = view(&value.custom_data);
    let _ = view(&value.hard_owner);
};
view_struct!(acadrust::objects::PersistentSubentityManager {
    class_version,
    reserved_zero,
    reserved_two,
    associated_step_count,
    associated_subentity_count,
    steps,
    subentities
});
const _: fn(&acadrust::objects::PersistentSubentityManager) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.reserved_zero);
    let _ = view(&value.reserved_two);
    let _ = view(&value.associated_step_count);
    let _ = view(&value.associated_subentity_count);
    let _ = view(&value.steps);
    let _ = view(&value.subentities);
};
view_struct!(acadrust::objects::GeoMapImage {
    class_version,
    origin,
    image_size,
    display_properties,
    clipping_enabled,
    brightness,
    contrast,
    fade
});
const _: fn(&acadrust::objects::GeoMapImage) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.origin);
    let _ = view(&value.image_size);
    let _ = view(&value.display_properties);
    let _ = view(&value.clipping_enabled);
    let _ = view(&value.brightness);
    let _ = view(&value.contrast);
    let _ = view(&value.fade);
};
view_struct!(acadrust::objects::ModelDocViewStyle {
    class_version,
    description,
    modified_for_recompute,
    display_name,
    flags
});
const _: fn(&acadrust::objects::ModelDocViewStyle) = |value| {
    let _ = view(&value.class_version);
    let _ = view(&value.description);
    let _ = view(&value.modified_for_recompute);
    let _ = view(&value.display_name);
    let _ = view(&value.flags);
};
view_struct!(acadrust::objects::DetailViewStyle {
    base,
    class_version,
    flags,
    identifier_style,
    identifier_color,
    identifier_height,
    identifier_excluded_characters,
    identifier_offset,
    identifier_placement,
    arrow_symbol,
    arrow_symbol_color,
    arrow_symbol_size,
    boundary_linetype,
    boundary_lineweight,
    boundary_color,
    view_label_text_style,
    view_label_text_color,
    view_label_text_height,
    view_label_attachment,
    view_label_offset,
    view_label_alignment,
    view_label_pattern,
    connection_linetype,
    connection_lineweight,
    connection_color,
    border_linetype,
    border_lineweight,
    border_color,
    model_edge
});
const _: fn(&acadrust::objects::DetailViewStyle) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.class_version);
    let _ = view(&value.flags);
    let _ = view(&value.identifier_style);
    let _ = view(&value.identifier_color);
    let _ = view(&value.identifier_height);
    let _ = view(&value.identifier_excluded_characters);
    let _ = view(&value.identifier_offset);
    let _ = view(&value.identifier_placement);
    let _ = view(&value.arrow_symbol);
    let _ = view(&value.arrow_symbol_color);
    let _ = view(&value.arrow_symbol_size);
    let _ = view(&value.boundary_linetype);
    let _ = view(&value.boundary_lineweight);
    let _ = view(&value.boundary_color);
    let _ = view(&value.view_label_text_style);
    let _ = view(&value.view_label_text_color);
    let _ = view(&value.view_label_text_height);
    let _ = view(&value.view_label_attachment);
    let _ = view(&value.view_label_offset);
    let _ = view(&value.view_label_alignment);
    let _ = view(&value.view_label_pattern);
    let _ = view(&value.connection_linetype);
    let _ = view(&value.connection_lineweight);
    let _ = view(&value.connection_color);
    let _ = view(&value.border_linetype);
    let _ = view(&value.border_lineweight);
    let _ = view(&value.border_color);
    let _ = view(&value.model_edge);
};
view_struct!(acadrust::objects::SectionViewStyle {
    base,
    class_version,
    flags,
    identifier_style,
    identifier_color,
    identifier_height,
    arrow_start_symbol,
    arrow_end_symbol,
    arrow_symbol_color,
    arrow_symbol_size,
    identifier_excluded_characters,
    arrow_symbol_extension_length,
    plane_linetype,
    plane_lineweight,
    plane_color,
    bend_linetype,
    bend_lineweight,
    bend_color,
    bend_line_length,
    end_line_length,
    view_label_text_style,
    view_label_text_color,
    view_label_text_height,
    view_label_attachment,
    view_label_offset,
    view_label_alignment,
    view_label_pattern,
    hatch_color,
    hatch_background_color,
    hatch_pattern,
    hatch_scale,
    hatch_transparency,
    reserved_flags,
    identifier_position,
    identifier_offset,
    arrow_position,
    end_line_overshoot,
    hatch_angles
});
const _: fn(&acadrust::objects::SectionViewStyle) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.class_version);
    let _ = view(&value.flags);
    let _ = view(&value.identifier_style);
    let _ = view(&value.identifier_color);
    let _ = view(&value.identifier_height);
    let _ = view(&value.arrow_start_symbol);
    let _ = view(&value.arrow_end_symbol);
    let _ = view(&value.arrow_symbol_color);
    let _ = view(&value.arrow_symbol_size);
    let _ = view(&value.identifier_excluded_characters);
    let _ = view(&value.arrow_symbol_extension_length);
    let _ = view(&value.plane_linetype);
    let _ = view(&value.plane_lineweight);
    let _ = view(&value.plane_color);
    let _ = view(&value.bend_linetype);
    let _ = view(&value.bend_lineweight);
    let _ = view(&value.bend_color);
    let _ = view(&value.bend_line_length);
    let _ = view(&value.end_line_length);
    let _ = view(&value.view_label_text_style);
    let _ = view(&value.view_label_text_color);
    let _ = view(&value.view_label_text_height);
    let _ = view(&value.view_label_attachment);
    let _ = view(&value.view_label_offset);
    let _ = view(&value.view_label_alignment);
    let _ = view(&value.view_label_pattern);
    let _ = view(&value.hatch_color);
    let _ = view(&value.hatch_background_color);
    let _ = view(&value.hatch_pattern);
    let _ = view(&value.hatch_scale);
    let _ = view(&value.hatch_transparency);
    let _ = view(&value.reserved_flags);
    let _ = view(&value.identifier_position);
    let _ = view(&value.identifier_offset);
    let _ = view(&value.arrow_position);
    let _ = view(&value.end_line_overshoot);
    let _ = view(&value.hatch_angles);
};

// objects/data_objects.rs
view_struct!(acadrust::objects::DataObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    data
});
const _: fn(&acadrust::objects::DataObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.data);
};
view_enum!(acadrust::objects::DataObjectData {
    BreakPointRef,
    BreakData(v0),
    CellStyleMap(v0),
    AcDsRecord,
    AcDsSchema,
    Dummy,
    IdBuffer(v0),
    Index(v0),
    LayerIndex(v0),
    LongTransaction,
    ObjectPointer,
    PartialViewingFilter(v0),
    TableGeometry(v0),
});
view_struct!(acadrust::objects::BreakData {
    version,
    dimension_reference,
    reserved_reference,
    point_references
});
const _: fn(&acadrust::objects::BreakData) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.dimension_reference);
    let _ = view(&value.reserved_reference);
    let _ = view(&value.point_references);
};
view_struct!(acadrust::objects::BreakPointReference {
    version,
    reserved,
    reference_type,
    flags,
    identifier,
    first_point,
    second_point,
    trailing_version
});
const _: fn(&acadrust::objects::BreakPointReference) = |value| {
    let _ = view(&value.version);
    let _ = view(&value.reserved);
    let _ = view(&value.reference_type);
    let _ = view(&value.flags);
    let _ = view(&value.identifier);
    let _ = view(&value.first_point);
    let _ = view(&value.second_point);
    let _ = view(&value.trailing_version);
};
view_struct!(acadrust::objects::IdBuffer { flags, object_ids });
const _: fn(&acadrust::objects::IdBuffer) = |value| {
    let _ = view(&value.flags);
    let _ = view(&value.object_ids);
};
view_struct!(acadrust::objects::Index {
    last_updated_julian_day,
    last_updated_milliseconds
});
const _: fn(&acadrust::objects::Index) = |value| {
    let _ = view(&value.last_updated_julian_day);
    let _ = view(&value.last_updated_milliseconds);
};
view_struct!(acadrust::objects::LayerIndexEntry {
    layer_count,
    name,
    id_buffer
});
const _: fn(&acadrust::objects::LayerIndexEntry) = |value| {
    let _ = view(&value.layer_count);
    let _ = view(&value.name);
    let _ = view(&value.id_buffer);
};
view_struct!(acadrust::objects::LayerIndex {
    last_updated_julian_day,
    last_updated_milliseconds,
    entries
});
const _: fn(&acadrust::objects::LayerIndex) = |value| {
    let _ = view(&value.last_updated_julian_day);
    let _ = view(&value.last_updated_milliseconds);
    let _ = view(&value.entries);
};
view_struct!(acadrust::objects::PartialViewingFilter {});
view_struct!(acadrust::objects::CellStyleMap { cells });
const _: fn(&acadrust::objects::CellStyleMap) = |value| {
    let _ = view(&value.cells);
};
view_struct!(acadrust::objects::TableGeometryCell {
    geometry_data_flag,
    width_with_gap,
    height_with_gap,
    table_geometry,
    geometry
});
const _: fn(&acadrust::objects::TableGeometryCell) = |value| {
    let _ = view(&value.geometry_data_flag);
    let _ = view(&value.width_with_gap);
    let _ = view(&value.height_with_gap);
    let _ = view(&value.table_geometry);
    let _ = view(&value.geometry);
};
view_struct!(acadrust::objects::TableGeometry {
    rows,
    columns,
    cells
});
const _: fn(&acadrust::objects::TableGeometry) = |value| {
    let _ = view(&value.rows);
    let _ = view(&value.columns);
    let _ = view(&value.cells);
};

// objects/dgn_linestyle.rs
view_struct!(acadrust::objects::DgnLineStyleObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    data
});
const _: fn(&acadrust::objects::DgnLineStyleObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.data);
};
impl NativeView for acadrust::objects::DgnLineStyleData {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::Definition { .. } => 14,
            Self::Component { .. } => 8,
            Self::Registered { .. } => 4,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::Definition { .. } => "Definition",
            Self::Component { .. } => "Component",
            Self::Registered { .. } => "Registered",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::Definition {
                description,
                version,
                style_number,
                component_uid,
                is_continuous,
                unit_definition,
                unit_scale,
                units_type,
                is_element,
                is_physical,
                is_scale_independent,
                is_snappable,
                root_component,
                properties,
            } => match index {
                0 => Some((StringView::new("description"), view(description))),
                1 => Some((StringView::new("version"), view(version))),
                2 => Some((StringView::new("style_number"), view(style_number))),
                3 => Some((StringView::new("component_uid"), view(component_uid))),
                4 => Some((StringView::new("is_continuous"), view(is_continuous))),
                5 => Some((StringView::new("unit_definition"), view(unit_definition))),
                6 => Some((StringView::new("unit_scale"), view(unit_scale))),
                7 => Some((StringView::new("units_type"), view(units_type))),
                8 => Some((StringView::new("is_element"), view(is_element))),
                9 => Some((StringView::new("is_physical"), view(is_physical))),
                10 => Some((
                    StringView::new("is_scale_independent"),
                    view(is_scale_independent),
                )),
                11 => Some((StringView::new("is_snappable"), view(is_snappable))),
                12 => Some((StringView::new("root_component"), view(root_component))),
                13 => Some((StringView::new("properties"), view(properties))),
                _ => None,
            },
            Self::Component {
                kind,
                description,
                version,
                component_uid,
                scale,
                property_flags,
                component,
                properties,
            } => match index {
                0 => Some((StringView::new("kind"), view(kind))),
                1 => Some((StringView::new("description"), view(description))),
                2 => Some((StringView::new("version"), view(version))),
                3 => Some((StringView::new("component_uid"), view(component_uid))),
                4 => Some((StringView::new("scale"), view(scale))),
                5 => Some((StringView::new("property_flags"), view(property_flags))),
                6 => Some((StringView::new("component"), view(component))),
                7 => Some((StringView::new("properties"), view(properties))),
                _ => None,
            },
            Self::Registered {
                dxf_name,
                properties,
                payload,
                object_ids,
            } => match index {
                0 => Some((StringView::new("dxf_name"), view(dxf_name))),
                1 => Some((StringView::new("properties"), view(properties))),
                2 => Some((StringView::new("payload"), view(payload))),
                3 => Some((StringView::new("object_ids"), view(object_ids))),
                _ => None,
            },
        }
    }
}
view_enum!(acadrust::objects::DgnLsComponentType {
    Symbol,
    Compound,
    Stroke,
    Point,
    Internal,
});
view_enum!(acadrust::objects::DgnLsComponentData {
    Symbol(v0),
    Compound(v0),
    Stroke(v0),
    Point(v0),
    Internal(v0),
});
view_struct!(acadrust::objects::DgnLsSymbolComponent {
    stored_unit_scale,
    unit_scale,
    has_unit_scale,
    is_3d,
    block
});
const _: fn(&acadrust::objects::DgnLsSymbolComponent) = |value| {
    let _ = view(&value.stored_unit_scale);
    let _ = view(&value.unit_scale);
    let _ = view(&value.has_unit_scale);
    let _ = view(&value.is_3d);
    let _ = view(&value.block);
};
view_struct!(acadrust::objects::DgnLsCompoundComponent { entries });
const _: fn(&acadrust::objects::DgnLsCompoundComponent) = |value| {
    let _ = view(&value.entries);
};
view_struct!(acadrust::objects::DgnLsCompoundEntry { component, offset });
const _: fn(&acadrust::objects::DgnLsCompoundEntry) = |value| {
    let _ = view(&value.component);
    let _ = view(&value.offset);
};
view_struct!(acadrust::objects::DgnLsStrokePattern {
    has_iteration_limit,
    is_single_segment,
    iteration_limit,
    auto_phase,
    phase,
    phase_mode,
    strokes
});
const _: fn(&acadrust::objects::DgnLsStrokePattern) = |value| {
    let _ = view(&value.has_iteration_limit);
    let _ = view(&value.is_single_segment);
    let _ = view(&value.iteration_limit);
    let _ = view(&value.auto_phase);
    let _ = view(&value.phase);
    let _ = view(&value.phase_mode);
    let _ = view(&value.strokes);
};
view_enum!(acadrust::objects::DgnLsPhaseMode {
    Distance,
    Fraction,
    Centered,
    Reserved,
});
view_struct!(acadrust::objects::DgnLsStroke {
    is_dash,
    bypass_corner,
    can_be_scaled,
    invert_at_origin,
    invert_at_end,
    length,
    start_width,
    end_width,
    width_mode,
    cap_mode
});
const _: fn(&acadrust::objects::DgnLsStroke) = |value| {
    let _ = view(&value.is_dash);
    let _ = view(&value.bypass_corner);
    let _ = view(&value.can_be_scaled);
    let _ = view(&value.invert_at_origin);
    let _ = view(&value.invert_at_end);
    let _ = view(&value.length);
    let _ = view(&value.start_width);
    let _ = view(&value.end_width);
    let _ = view(&value.width_mode);
    let _ = view(&value.cap_mode);
};
view_struct!(acadrust::objects::DgnLsPointComponent {
    stroke_component,
    symbols
});
const _: fn(&acadrust::objects::DgnLsPointComponent) = |value| {
    let _ = view(&value.stroke_component);
    let _ = view(&value.symbols);
};
view_struct!(acadrust::objects::DgnLsSymbolReference {
    symbol_component,
    partial_strokes,
    clip_partial,
    allow_stretch,
    partial_projected,
    use_symbol_color,
    use_symbol_lineweight,
    justify,
    rotation_type,
    vertex_mask,
    x_offset,
    y_offset,
    angle,
    stroke_number
});
const _: fn(&acadrust::objects::DgnLsSymbolReference) = |value| {
    let _ = view(&value.symbol_component);
    let _ = view(&value.partial_strokes);
    let _ = view(&value.clip_partial);
    let _ = view(&value.allow_stretch);
    let _ = view(&value.partial_projected);
    let _ = view(&value.use_symbol_color);
    let _ = view(&value.use_symbol_lineweight);
    let _ = view(&value.justify);
    let _ = view(&value.rotation_type);
    let _ = view(&value.vertex_mask);
    let _ = view(&value.x_offset);
    let _ = view(&value.y_offset);
    let _ = view(&value.angle);
    let _ = view(&value.stroke_number);
};
view_struct!(acadrust::objects::DgnLsInternalComponent {
    pattern,
    internal_version,
    hardware_style,
    is_hardware_style,
    line_code
});
const _: fn(&acadrust::objects::DgnLsInternalComponent) = |value| {
    let _ = view(&value.pattern);
    let _ = view(&value.internal_version);
    let _ = view(&value.hardware_style);
    let _ = view(&value.is_hardware_style);
    let _ = view(&value.line_code);
};
view_struct!(acadrust::objects::DgnLsDefinition {
    handle,
    name,
    root_component
});
const _: fn(&acadrust::objects::DgnLsDefinition) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.name);
    let _ = view(&value.root_component);
};
view_struct!(acadrust::objects::DgnLsComponent {
    handle,
    component_type,
    description,
    refs,
    scale
});
const _: fn(&acadrust::objects::DgnLsComponent) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.component_type);
    let _ = view(&value.description);
    let _ = view(&value.refs);
    let _ = view(&value.scale);
};

// objects/dictionary_variable.rs
view_struct!(acadrust::objects::DictionaryVariable {
    handle,
    owner_handle,
    schema_number,
    value,
    name
});
const _: fn(&acadrust::objects::DictionaryVariable) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner_handle);
    let _ = view(&value.schema_number);
    let _ = view(&value.value);
    let _ = view(&value.name);
};

// objects/dynamic_block.rs
view_struct!(acadrust::objects::DynamicBlockObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    dxf_name,
    cpp_class_name,
    data
});
const _: fn(&acadrust::objects::DynamicBlockObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.dxf_name);
    let _ = view(&value.cpp_class_name);
    let _ = view(&value.data);
};
view_enum!(acadrust::objects::DynamicBlockData {
    Unknown,
    Representation(v0),
    ProxyNode(v0),
    GripLocationComponent(v0),
    AlignmentGrip(v0),
    FlipGrip(v0),
    LinearGrip(v0),
    LookupGrip(v0),
    PolarGrip(v0),
    RotationGrip(v0),
    VisibilityGrip(v0),
    XYGrip(v0),
    PropertiesTableGrip(v0),
    AlignmentParameter(v0),
    BasePointParameter(v0),
    FlipParameter(v0),
    LinearParameter(v0),
    LookupParameter(v0),
    PointParameter(v0),
    PolarParameter(v0),
    RotationParameter(v0),
    UserParameter(v0),
    VisibilityParameter(v0),
    XYParameter(v0),
    AngularConstraintParameter(v0),
    DiametricConstraintParameter(v0),
    RadialConstraintParameter(v0),
    AlignedConstraintParameter(v0),
    LinearConstraintParameter(v0),
    HorizontalConstraintParameter(v0),
    VerticalConstraintParameter(v0),
    ParameterDependencyBody(v0),
    MoveAction(v0),
    FlipAction(v0),
    RotateAction(v0),
    ScaleAction(v0),
    ArrayAction(v0),
    LookupAction(v0),
    StretchAction(v0),
    PolarStretchAction(v0),
    AlignmentParameterEntity,
    BasePointParameterEntity,
    FlipParameterEntity,
    LinearParameterEntity,
    PointParameterEntity,
    RotationParameterEntity,
    VisibilityParameterEntity,
    XYParameterEntity,
    AngularConstraintParameterEntity(v0),
    FlipGripEntity,
    LinearGripEntity,
    PolarGripEntity,
    RotationGripEntity,
    VisibilityGripEntity,
    XYGripEntity,
    EvaluationGraph(v0),
    SolidHistory(v0),
    SolidHistoryNode(v0),
});
view_struct!(acadrust::objects::BlockEvalExpression {
    parent_id,
    major,
    minor,
    value_code,
    value,
    node_id
});
const _: fn(&acadrust::objects::BlockEvalExpression) = |value| {
    let _ = view(&value.parent_id);
    let _ = view(&value.major);
    let _ = view(&value.minor);
    let _ = view(&value.value_code);
    let _ = view(&value.value);
    let _ = view(&value.node_id);
};
view_struct!(acadrust::objects::BlockElement {
    eval,
    name,
    major,
    minor,
    eed_1071
});
const _: fn(&acadrust::objects::BlockElement) = |value| {
    let _ = view(&value.eval);
    let _ = view(&value.name);
    let _ = view(&value.major);
    let _ = view(&value.minor);
    let _ = view(&value.eed_1071);
};
view_struct!(acadrust::objects::BlockConnection { code, name });
const _: fn(&acadrust::objects::BlockConnection) = |value| {
    let _ = view(&value.code);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::BlockParameterProperty { connections });
const _: fn(&acadrust::objects::BlockParameterProperty) = |value| {
    let _ = view(&value.connections);
};
view_struct!(acadrust::objects::BlockParameterValueSet {
    description,
    flags,
    minimum,
    maximum,
    increment,
    values
});
const _: fn(&acadrust::objects::BlockParameterValueSet) = |value| {
    let _ = view(&value.description);
    let _ = view(&value.flags);
    let _ = view(&value.minimum);
    let _ = view(&value.maximum);
    let _ = view(&value.increment);
    let _ = view(&value.values);
};
view_struct!(acadrust::objects::BlockParameter {
    element,
    show_properties,
    chain_actions
});
const _: fn(&acadrust::objects::BlockParameter) = |value| {
    let _ = view(&value.element);
    let _ = view(&value.show_properties);
    let _ = view(&value.chain_actions);
};
view_struct!(acadrust::objects::BlockOnePointParameter {
    parameter,
    definition_point,
    properties,
    property_count
});
const _: fn(&acadrust::objects::BlockOnePointParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.definition_point);
    let _ = view(&value.properties);
    let _ = view(&value.property_count);
};
view_struct!(acadrust::objects::BlockTwoPointParameter {
    parameter,
    definition_base_point,
    definition_end_point,
    properties,
    property_states,
    parameter_base_location,
    updated_base_point,
    base_point,
    updated_end_point,
    end_point
});
const _: fn(&acadrust::objects::BlockTwoPointParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.definition_base_point);
    let _ = view(&value.definition_end_point);
    let _ = view(&value.properties);
    let _ = view(&value.property_states);
    let _ = view(&value.parameter_base_location);
    let _ = view(&value.updated_base_point);
    let _ = view(&value.base_point);
    let _ = view(&value.updated_end_point);
    let _ = view(&value.end_point);
};
view_struct!(acadrust::objects::BlockGrip {
    element,
    flags_91,
    flags_92,
    location,
    insert_cycling,
    insert_cycling_weight
});
const _: fn(&acadrust::objects::BlockGrip) = |value| {
    let _ = view(&value.element);
    let _ = view(&value.flags_91);
    let _ = view(&value.flags_92);
    let _ = view(&value.location);
    let _ = view(&value.insert_cycling);
    let _ = view(&value.insert_cycling_weight);
};
view_struct!(acadrust::objects::BlockGripExpression {
    eval,
    grip_type,
    expression
});
const _: fn(&acadrust::objects::BlockGripExpression) = |value| {
    let _ = view(&value.eval);
    let _ = view(&value.grip_type);
    let _ = view(&value.expression);
};
view_struct!(acadrust::objects::BlockOrientedGrip { grip, orientation });
const _: fn(&acadrust::objects::BlockOrientedGrip) = |value| {
    let _ = view(&value.grip);
    let _ = view(&value.orientation);
};
view_struct!(acadrust::objects::BlockFlipGrip {
    grip,
    combined_state,
    orientation
});
const _: fn(&acadrust::objects::BlockFlipGrip) = |value| {
    let _ = view(&value.grip);
    let _ = view(&value.combined_state);
    let _ = view(&value.orientation);
};
view_struct!(acadrust::objects::BlockAction {
    element,
    display_location,
    dependencies,
    action_ids
});
const _: fn(&acadrust::objects::BlockAction) = |value| {
    let _ = view(&value.element);
    let _ = view(&value.display_location);
    let _ = view(&value.dependencies);
    let _ = view(&value.action_ids);
};
view_struct!(acadrust::objects::BlockActionOffsets {
    distance_multiplier,
    angle_offset,
    flags
});
const _: fn(&acadrust::objects::BlockActionOffsets) = |value| {
    let _ = view(&value.distance_multiplier);
    let _ = view(&value.angle_offset);
    let _ = view(&value.flags);
};
view_struct!(acadrust::objects::BlockActionWithBasePoint {
    action,
    offset,
    connections,
    dependent,
    base_point
});
const _: fn(&acadrust::objects::BlockActionWithBasePoint) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.offset);
    let _ = view(&value.connections);
    let _ = view(&value.dependent);
    let _ = view(&value.base_point);
};
view_struct!(acadrust::objects::BlockConstraintParameter {
    parameter,
    dependency
});
const _: fn(&acadrust::objects::BlockConstraintParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.dependency);
};
view_struct!(acadrust::objects::BlockLinearConstraintParameter {
    constraint,
    expression_name,
    expression_description,
    value,
    value_set
});
const _: fn(&acadrust::objects::BlockLinearConstraintParameter) = |value| {
    let _ = view(&value.constraint);
    let _ = view(&value.expression_name);
    let _ = view(&value.expression_description);
    let _ = view(&value.value);
    let _ = view(&value.value_set);
};
view_struct!(acadrust::objects::BlockRepresentationData { flags, block });
const _: fn(&acadrust::objects::BlockRepresentationData) = |value| {
    let _ = view(&value.flags);
    let _ = view(&value.block);
};
view_struct!(acadrust::objects::BlockAlignmentParameter {
    parameter,
    align_perpendicular
});
const _: fn(&acadrust::objects::BlockAlignmentParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.align_perpendicular);
};
view_struct!(acadrust::objects::BlockLinearParameter {
    parameter,
    distance_name,
    distance_description,
    distance,
    value_set
});
const _: fn(&acadrust::objects::BlockLinearParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.distance_name);
    let _ = view(&value.distance_description);
    let _ = view(&value.distance);
    let _ = view(&value.value_set);
};
view_struct!(acadrust::objects::BlockBasePointParameter {
    parameter,
    point,
    base_point
});
const _: fn(&acadrust::objects::BlockBasePointParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.point);
    let _ = view(&value.base_point);
};
view_struct!(acadrust::objects::BlockFlipParameter {
    parameter,
    flip_label,
    flip_label_description,
    base_state_label,
    flipped_state_label,
    definition_label_point,
    flags_96,
    tooltip
});
const _: fn(&acadrust::objects::BlockFlipParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.flip_label);
    let _ = view(&value.flip_label_description);
    let _ = view(&value.base_state_label);
    let _ = view(&value.flipped_state_label);
    let _ = view(&value.definition_label_point);
    let _ = view(&value.flags_96);
    let _ = view(&value.tooltip);
};
view_struct!(acadrust::objects::BlockXYParameter {
    parameter,
    x_label,
    x_label_description,
    y_label,
    y_label_description,
    x_value,
    y_value,
    x_value_set,
    y_value_set
});
const _: fn(&acadrust::objects::BlockXYParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.x_label);
    let _ = view(&value.x_label_description);
    let _ = view(&value.y_label);
    let _ = view(&value.y_label_description);
    let _ = view(&value.x_value);
    let _ = view(&value.y_value);
    let _ = view(&value.x_value_set);
    let _ = view(&value.y_value_set);
};
view_struct!(acadrust::objects::BlockRotationParameter {
    parameter,
    definition_base_angle_point,
    angle_name,
    angle_description,
    angle,
    value_set
});
const _: fn(&acadrust::objects::BlockRotationParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.definition_base_angle_point);
    let _ = view(&value.angle_name);
    let _ = view(&value.angle_description);
    let _ = view(&value.angle);
    let _ = view(&value.value_set);
};
view_struct!(acadrust::objects::BlockPolarParameter {
    parameter,
    angle_name,
    angle_description,
    distance_name,
    distance_description,
    offset,
    angle_value_set,
    distance_value_set
});
const _: fn(&acadrust::objects::BlockPolarParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.angle_name);
    let _ = view(&value.angle_description);
    let _ = view(&value.distance_name);
    let _ = view(&value.distance_description);
    let _ = view(&value.offset);
    let _ = view(&value.angle_value_set);
    let _ = view(&value.distance_value_set);
};
view_struct!(acadrust::objects::BlockLookupParameter {
    parameter,
    index,
    lookup_name,
    lookup_description
});
const _: fn(&acadrust::objects::BlockLookupParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.index);
    let _ = view(&value.lookup_name);
    let _ = view(&value.lookup_description);
};
view_struct!(acadrust::objects::BlockPointParameter {
    parameter,
    position_name,
    position_description,
    definition_label_point
});
const _: fn(&acadrust::objects::BlockPointParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.position_name);
    let _ = view(&value.position_description);
    let _ = view(&value.definition_label_point);
};
view_struct!(acadrust::objects::BlockUserParameter {
    parameter,
    flags,
    associated_variable,
    expression,
    value_code,
    value,
    value_type
});
const _: fn(&acadrust::objects::BlockUserParameter) = |value| {
    let _ = view(&value.parameter);
    let _ = view(&value.flags);
    let _ = view(&value.associated_variable);
    let _ = view(&value.expression);
    let _ = view(&value.value_code);
    let _ = view(&value.value);
    let _ = view(&value.value_type);
};
view_struct!(acadrust::objects::BlockAngularConstraintParameter {
    constraint,
    center_point,
    end_point,
    expression_name,
    expression_description,
    angle,
    orientation_on_both_grips,
    value_set
});
const _: fn(&acadrust::objects::BlockAngularConstraintParameter) = |value| {
    let _ = view(&value.constraint);
    let _ = view(&value.center_point);
    let _ = view(&value.end_point);
    let _ = view(&value.expression_name);
    let _ = view(&value.expression_description);
    let _ = view(&value.angle);
    let _ = view(&value.orientation_on_both_grips);
    let _ = view(&value.value_set);
};
view_struct!(acadrust::objects::BlockAngularConstraintParameterEntity {
    constraint,
    center_point,
    label_point,
    expression_name,
    expression_description,
    angle,
    orientation_on_both_grips,
    value_set
});
const _: fn(&acadrust::objects::BlockAngularConstraintParameterEntity) = |value| {
    let _ = view(&value.constraint);
    let _ = view(&value.center_point);
    let _ = view(&value.label_point);
    let _ = view(&value.expression_name);
    let _ = view(&value.expression_description);
    let _ = view(&value.angle);
    let _ = view(&value.orientation_on_both_grips);
    let _ = view(&value.value_set);
};
view_struct!(acadrust::objects::BlockDistanceConstraintParameter {
    constraint,
    expression_name,
    expression_description,
    distance,
    value_set
});
const _: fn(&acadrust::objects::BlockDistanceConstraintParameter) = |value| {
    let _ = view(&value.constraint);
    let _ = view(&value.expression_name);
    let _ = view(&value.expression_description);
    let _ = view(&value.distance);
    let _ = view(&value.value_set);
};
view_struct!(acadrust::objects::BlockParameterDependencyBody {
    dependency_body_version,
    dimension_base_version,
    name,
    class_version
});
const _: fn(&acadrust::objects::BlockParameterDependencyBody) = |value| {
    let _ = view(&value.dependency_body_version);
    let _ = view(&value.dimension_base_version);
    let _ = view(&value.name);
    let _ = view(&value.class_version);
};
view_struct!(acadrust::objects::BlockMoveAction {
    action,
    connections,
    offsets
});
const _: fn(&acadrust::objects::BlockMoveAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.connections);
    let _ = view(&value.offsets);
};
view_struct!(acadrust::objects::BlockFlipAction {
    action,
    connections
});
const _: fn(&acadrust::objects::BlockFlipAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.connections);
};
view_struct!(acadrust::objects::BlockBasePointAction {
    action,
    connections,
    dwg_tail_byte
});
const _: fn(&acadrust::objects::BlockBasePointAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.connections);
    let _ = view(&value.dwg_tail_byte);
};
view_struct!(acadrust::objects::BlockArrayAction {
    action,
    connections,
    column_offset,
    row_offset
});
const _: fn(&acadrust::objects::BlockArrayAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.connections);
    let _ = view(&value.column_offset);
    let _ = view(&value.row_offset);
};
view_struct!(acadrust::objects::BlockLookupColumn {
    node_id,
    value_type,
    property_type,
    lookup_property,
    unmatched_name,
    writable,
    connection_name
});
const _: fn(&acadrust::objects::BlockLookupColumn) = |value| {
    let _ = view(&value.node_id);
    let _ = view(&value.value_type);
    let _ = view(&value.property_type);
    let _ = view(&value.lookup_property);
    let _ = view(&value.unmatched_name);
    let _ = view(&value.writable);
    let _ = view(&value.connection_name);
};
view_struct!(acadrust::objects::BlockLookupAction {
    action,
    row_count,
    column_count,
    expressions,
    columns,
    flag_280
});
const _: fn(&acadrust::objects::BlockLookupAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.row_count);
    let _ = view(&value.column_count);
    let _ = view(&value.expressions);
    let _ = view(&value.columns);
    let _ = view(&value.flag_280);
};
view_struct!(acadrust::objects::BlockStretchHandle { handle, indexes });
const _: fn(&acadrust::objects::BlockStretchHandle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.indexes);
};
view_struct!(acadrust::objects::BlockStretchCode { code, indexes });
const _: fn(&acadrust::objects::BlockStretchCode) = |value| {
    let _ = view(&value.code);
    let _ = view(&value.indexes);
};
view_struct!(acadrust::objects::BlockStretchAction {
    action,
    connections,
    points,
    handles,
    codes,
    offsets
});
const _: fn(&acadrust::objects::BlockStretchAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.connections);
    let _ = view(&value.points);
    let _ = view(&value.handles);
    let _ = view(&value.codes);
    let _ = view(&value.offsets);
};
view_struct!(acadrust::objects::BlockPolarStretchAction {
    action,
    connections,
    points,
    handles,
    bindings,
    codes,
    distance_multiplier,
    angle_offset,
    extra
});
const _: fn(&acadrust::objects::BlockPolarStretchAction) = |value| {
    let _ = view(&value.action);
    let _ = view(&value.connections);
    let _ = view(&value.points);
    let _ = view(&value.handles);
    let _ = view(&value.bindings);
    let _ = view(&value.codes);
    let _ = view(&value.distance_multiplier);
    let _ = view(&value.angle_offset);
    let _ = view(&value.extra);
};
view_struct!(acadrust::objects::BlockEvaluationNode {
    id,
    edge_flags,
    next_id,
    expression,
    node_data,
    active_cycles
});
const _: fn(&acadrust::objects::BlockEvaluationNode) = |value| {
    let _ = view(&value.id);
    let _ = view(&value.edge_flags);
    let _ = view(&value.next_id);
    let _ = view(&value.expression);
    let _ = view(&value.node_data);
    let _ = view(&value.active_cycles);
};
view_struct!(acadrust::objects::BlockEvaluationEdge {
    id,
    next_id,
    incoming_edge,
    source_node,
    destination_node,
    outgoing_edges
});
const _: fn(&acadrust::objects::BlockEvaluationEdge) = |value| {
    let _ = view(&value.id);
    let _ = view(&value.next_id);
    let _ = view(&value.incoming_edge);
    let _ = view(&value.source_node);
    let _ = view(&value.destination_node);
    let _ = view(&value.outgoing_edges);
};
view_struct!(acadrust::objects::BlockEvaluationGraph {
    first_node_id,
    first_node_id_copy,
    nodes,
    edges
});
const _: fn(&acadrust::objects::BlockEvaluationGraph) = |value| {
    let _ = view(&value.first_node_id);
    let _ = view(&value.first_node_id_copy);
    let _ = view(&value.nodes);
    let _ = view(&value.edges);
};
view_struct!(acadrust::objects::SolidHistory {
    major,
    minor,
    owner,
    history_node_id,
    show_history,
    record_history
});
const _: fn(&acadrust::objects::SolidHistory) = |value| {
    let _ = view(&value.major);
    let _ = view(&value.minor);
    let _ = view(&value.owner);
    let _ = view(&value.history_node_id);
    let _ = view(&value.show_history);
    let _ = view(&value.record_history);
};
view_struct!(acadrust::objects::SolidHistoryNodeBase {
    eval,
    major,
    minor,
    transform,
    color,
    step_id,
    material
});
const _: fn(&acadrust::objects::SolidHistoryNodeBase) = |value| {
    let _ = view(&value.eval);
    let _ = view(&value.major);
    let _ = view(&value.minor);
    let _ = view(&value.transform);
    let _ = view(&value.color);
    let _ = view(&value.step_id);
    let _ = view(&value.material);
};
view_enum!(acadrust::objects::SolidHistoryOperation {
    Unknown,
    Box(v0),
    Wedge(v0),
    Sphere(v0),
    Cylinder(v0),
    Cone(v0),
    Pyramid(v0),
    Torus(v0),
    Boolean(v0),
    Brep(v0),
    Fillet(v0),
    Chamfer(v0),
    Sweep(v0),
    Extrusion(v0),
    Loft(v0),
    Revolve(v0),
});
view_struct!(acadrust::objects::SolidHistoryBox {
    base,
    operation_major,
    operation_minor,
    length,
    width,
    height
});
const _: fn(&acadrust::objects::SolidHistoryBox) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.length);
    let _ = view(&value.width);
    let _ = view(&value.height);
};
view_struct!(acadrust::objects::SolidHistorySphere {
    base,
    operation_major,
    operation_minor,
    radius
});
const _: fn(&acadrust::objects::SolidHistorySphere) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.radius);
};
view_struct!(acadrust::objects::SolidHistoryCylinder {
    base,
    operation_major,
    operation_minor,
    height,
    major_radius,
    minor_radius,
    x_radius
});
const _: fn(&acadrust::objects::SolidHistoryCylinder) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.height);
    let _ = view(&value.major_radius);
    let _ = view(&value.minor_radius);
    let _ = view(&value.x_radius);
};
view_struct!(acadrust::objects::SolidHistoryCone {
    base,
    operation_major,
    operation_minor,
    height,
    base_x_radius,
    base_y_radius,
    top_radius
});
const _: fn(&acadrust::objects::SolidHistoryCone) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.height);
    let _ = view(&value.base_x_radius);
    let _ = view(&value.base_y_radius);
    let _ = view(&value.top_radius);
};
view_struct!(acadrust::objects::SolidHistoryPyramid {
    base,
    operation_major,
    operation_minor,
    height,
    sides,
    radius,
    top_radius
});
const _: fn(&acadrust::objects::SolidHistoryPyramid) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.height);
    let _ = view(&value.sides);
    let _ = view(&value.radius);
    let _ = view(&value.top_radius);
};
view_struct!(acadrust::objects::SolidHistoryTorus {
    base,
    operation_major,
    operation_minor,
    major_radius,
    minor_radius
});
const _: fn(&acadrust::objects::SolidHistoryTorus) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.major_radius);
    let _ = view(&value.minor_radius);
};
view_struct!(acadrust::objects::SolidHistoryBoolean {
    base,
    operation_major,
    operation_minor,
    operation,
    first_operand,
    second_operand
});
const _: fn(&acadrust::objects::SolidHistoryBoolean) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.operation);
    let _ = view(&value.first_operand);
    let _ = view(&value.second_operand);
};
view_struct!(acadrust::objects::SolidHistoryTree {
    operation,
    operands
});
const _: fn(&acadrust::objects::SolidHistoryTree) = |value| {
    let _ = view(&value.operation);
    let _ = view(&value.operands);
};
view_struct!(acadrust::objects::SolidHistoryBrep {
    base,
    operation_major,
    operation_minor,
    acis_data
});
const _: fn(&acadrust::objects::SolidHistoryBrep) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.acis_data);
};
view_struct!(acadrust::objects::SolidHistoryFillet {
    base,
    operation_major,
    operation_minor,
    method,
    edges,
    radii,
    start_setbacks,
    end_setbacks
});
const _: fn(&acadrust::objects::SolidHistoryFillet) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.method);
    let _ = view(&value.edges);
    let _ = view(&value.radii);
    let _ = view(&value.start_setbacks);
    let _ = view(&value.end_setbacks);
};
view_struct!(acadrust::objects::SolidHistoryChamfer {
    base,
    operation_major,
    operation_minor,
    method,
    base_distance,
    other_distance,
    edges,
    base_face
});
const _: fn(&acadrust::objects::SolidHistoryChamfer) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.method);
    let _ = view(&value.base_distance);
    let _ = view(&value.other_distance);
    let _ = view(&value.edges);
    let _ = view(&value.base_face);
};
view_struct!(acadrust::objects::SolidHistorySweep {
    base,
    operation_major,
    operation_minor,
    direction,
    sweep_entity,
    path_entity,
    draft_angle,
    start_draft_distance,
    end_draft_distance,
    scale_factor,
    twist_angle,
    align_angle,
    sweep_entity_transform,
    path_entity_transform,
    align_option,
    miter_option,
    has_align_start,
    align_start,
    bank,
    flags_294_296,
    reference_point,
    dwg_vector
});
const _: fn(&acadrust::objects::SolidHistorySweep) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.direction);
    let _ = view(&value.sweep_entity);
    let _ = view(&value.path_entity);
    let _ = view(&value.draft_angle);
    let _ = view(&value.start_draft_distance);
    let _ = view(&value.end_draft_distance);
    let _ = view(&value.scale_factor);
    let _ = view(&value.twist_angle);
    let _ = view(&value.align_angle);
    let _ = view(&value.sweep_entity_transform);
    let _ = view(&value.path_entity_transform);
    let _ = view(&value.align_option);
    let _ = view(&value.miter_option);
    let _ = view(&value.has_align_start);
    let _ = view(&value.align_start);
    let _ = view(&value.bank);
    let _ = view(&value.flags_294_296);
    let _ = view(&value.reference_point);
    let _ = view(&value.dwg_vector);
};
view_struct!(acadrust::objects::SolidHistoryLoft {
    base,
    operation_major,
    operation_minor,
    cross_sections,
    guides,
    parameters,
    path_entity,
    options
});
const _: fn(&acadrust::objects::SolidHistoryLoft) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.cross_sections);
    let _ = view(&value.guides);
    let _ = view(&value.parameters);
    let _ = view(&value.path_entity);
    let _ = view(&value.options);
};
view_struct!(acadrust::objects::SolidHistoryLoftOptions {
    surface_option,
    start_draft_angle,
    end_draft_angle,
    start_magnitude,
    end_magnitude,
    flags
});
const _: fn(&acadrust::objects::SolidHistoryLoftOptions) = |value| {
    let _ = view(&value.surface_option);
    let _ = view(&value.start_draft_angle);
    let _ = view(&value.end_draft_angle);
    let _ = view(&value.start_magnitude);
    let _ = view(&value.end_magnitude);
    let _ = view(&value.flags);
};
view_struct!(acadrust::objects::SolidHistoryLoftParameters {
    path_entity,
    normals,
    start_draft_angle,
    end_draft_angle,
    start_magnitude,
    end_magnitude,
    start_continuity,
    end_continuity,
    start_bulge,
    end_bulge,
    closed,
    periodic,
    surface,
    align_direction,
    section_counts
});
const _: fn(&acadrust::objects::SolidHistoryLoftParameters) = |value| {
    let _ = view(&value.path_entity);
    let _ = view(&value.normals);
    let _ = view(&value.start_draft_angle);
    let _ = view(&value.end_draft_angle);
    let _ = view(&value.start_magnitude);
    let _ = view(&value.end_magnitude);
    let _ = view(&value.start_continuity);
    let _ = view(&value.end_continuity);
    let _ = view(&value.start_bulge);
    let _ = view(&value.end_bulge);
    let _ = view(&value.closed);
    let _ = view(&value.periodic);
    let _ = view(&value.surface);
    let _ = view(&value.align_direction);
    let _ = view(&value.section_counts);
};
view_struct!(acadrust::objects::SolidHistoryRevolve {
    base,
    operation_major,
    operation_minor,
    axis_point,
    direction,
    revolve_angle,
    start_angle,
    draft_angle,
    field_44,
    field_45,
    twist_angle,
    flag_290,
    close_to_axis,
    sweep_entity
});
const _: fn(&acadrust::objects::SolidHistoryRevolve) = |value| {
    let _ = view(&value.base);
    let _ = view(&value.operation_major);
    let _ = view(&value.operation_minor);
    let _ = view(&value.axis_point);
    let _ = view(&value.direction);
    let _ = view(&value.revolve_angle);
    let _ = view(&value.start_angle);
    let _ = view(&value.draft_angle);
    let _ = view(&value.field_44);
    let _ = view(&value.field_45);
    let _ = view(&value.twist_angle);
    let _ = view(&value.flag_290);
    let _ = view(&value.close_to_axis);
    let _ = view(&value.sweep_entity);
};

// objects/field.rs
view_struct!(acadrust::objects::FieldChildValue { key, value });
const _: fn(&acadrust::objects::FieldChildValue) = |value| {
    let _ = view(&value.key);
    let _ = view(&value.value);
};
view_struct!(acadrust::objects::Field {
    handle,
    owner,
    evaluator_id,
    code,
    format,
    child_fields,
    referenced_objects,
    evaluation_option,
    filing_option,
    state,
    evaluation_status,
    evaluation_error_code,
    evaluation_error_message,
    value,
    value_string,
    value_string_length,
    child_values,
    xdata
});
const _: fn(&acadrust::objects::Field) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.evaluator_id);
    let _ = view(&value.code);
    let _ = view(&value.format);
    let _ = view(&value.child_fields);
    let _ = view(&value.referenced_objects);
    let _ = view(&value.evaluation_option);
    let _ = view(&value.filing_option);
    let _ = view(&value.state);
    let _ = view(&value.evaluation_status);
    let _ = view(&value.evaluation_error_code);
    let _ = view(&value.evaluation_error_message);
    let _ = view(&value.value);
    let _ = view(&value.value_string);
    let _ = view(&value.value_string_length);
    let _ = view(&value.child_values);
    let _ = view(&value.xdata);
};
view_struct!(acadrust::objects::FieldList {
    handle,
    owner,
    unknown,
    fields
});
const _: fn(&acadrust::objects::FieldList) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.unknown);
    let _ = view(&value.fields);
};

// objects/group.rs
view_struct!(acadrust::objects::Group {
    handle,
    owner,
    name,
    unnamed,
    description,
    entities,
    selectable
});
const _: fn(&acadrust::objects::Group) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.name);
    let _ = view(&value.unnamed);
    let _ = view(&value.description);
    let _ = view(&value.entities);
    let _ = view(&value.selectable);
};

// objects/image_definition.rs
view_enum!(acadrust::objects::ResolutionUnit {
    None,
    Centimeters,
    Inches,
});
view_struct!(acadrust::objects::ImageDefinitionReactor {
    handle,
    owner,
    image_handle
});
const _: fn(&acadrust::objects::ImageDefinitionReactor) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.image_handle);
};
view_struct!(acadrust::objects::ImageDefinition {
    handle,
    owner,
    file_name,
    is_loaded,
    size_in_pixels,
    pixel_size,
    resolution_unit,
    class_version
});
const _: fn(&acadrust::objects::ImageDefinition) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.file_name);
    let _ = view(&value.is_loaded);
    let _ = view(&value.size_in_pixels);
    let _ = view(&value.pixel_size);
    let _ = view(&value.resolution_unit);
    let _ = view(&value.class_version);
};

// objects/mlinestyle.rs
view_struct!(acadrust::objects::MLineStyleFlags {
    fill_on,
    display_joints,
    start_square_cap,
    start_inner_arcs_cap,
    start_round_cap,
    end_square_cap,
    end_inner_arcs_cap,
    end_round_cap
});
const _: fn(&acadrust::objects::MLineStyleFlags) = |value| {
    let _ = view(&value.fill_on);
    let _ = view(&value.display_joints);
    let _ = view(&value.start_square_cap);
    let _ = view(&value.start_inner_arcs_cap);
    let _ = view(&value.start_round_cap);
    let _ = view(&value.end_square_cap);
    let _ = view(&value.end_inner_arcs_cap);
    let _ = view(&value.end_round_cap);
};
view_struct!(acadrust::objects::MLineStyleElement {
    offset,
    color,
    linetype
});
const _: fn(&acadrust::objects::MLineStyleElement) = |value| {
    let _ = view(&value.offset);
    let _ = view(&value.color);
    let _ = view(&value.linetype);
};
view_struct!(acadrust::objects::MLineStyle {
    handle,
    owner,
    name,
    description,
    elements,
    start_angle,
    end_angle,
    fill_color,
    flags
});
const _: fn(&acadrust::objects::MLineStyle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.name);
    let _ = view(&value.description);
    let _ = view(&value.elements);
    let _ = view(&value.start_angle);
    let _ = view(&value.end_angle);
    let _ = view(&value.fill_color);
    let _ = view(&value.flags);
};

// objects/mod.rs
view_struct!(acadrust::objects::Dictionary {
    handle,
    owner,
    entries,
    hard_owner_entries,
    duplicate_cloning,
    hard_owner,
    reactors,
    xdictionary_handle
});
const _: fn(&acadrust::objects::Dictionary) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.entries);
    let _ = view(&value.hard_owner_entries);
    let _ = view(&value.duplicate_cloning);
    let _ = view(&value.hard_owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
};
view_struct!(acadrust::objects::Layout {
    handle,
    owner,
    name,
    flags,
    tab_order,
    min_limits,
    max_limits,
    insertion_base,
    min_extents,
    max_extents,
    elevation,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_ortho_type,
    block_record,
    viewport,
    viewports,
    base_ucs,
    named_ucs,
    reactors,
    xdictionary_handle,
    raw_plot_settings_codes,
    paper_width,
    paper_height,
    plot_rotation,
    plot_flags,
    plot_page_name,
    plot_printer_name,
    paper_size,
    plot_view_name,
    plot_style_sheet,
    plot_margin_left,
    plot_margin_bottom,
    plot_margin_right,
    plot_margin_top,
    plot_origin_x,
    plot_origin_y,
    plot_window_min_x,
    plot_window_min_y,
    plot_window_max_x,
    plot_window_max_y,
    plot_paper_units,
    plot_type,
    plot_scale_numerator,
    plot_scale_denominator,
    plot_scale_type,
    plot_scale_factor,
    paper_image_origin_x,
    paper_image_origin_y,
    shade_plot_mode,
    shade_plot_resolution,
    shade_plot_dpi,
    plot_view_handle,
    visual_style_handle
});
const _: fn(&acadrust::objects::Layout) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.name);
    let _ = view(&value.flags);
    let _ = view(&value.tab_order);
    let _ = view(&value.min_limits);
    let _ = view(&value.max_limits);
    let _ = view(&value.insertion_base);
    let _ = view(&value.min_extents);
    let _ = view(&value.max_extents);
    let _ = view(&value.elevation);
    let _ = view(&value.ucs_origin);
    let _ = view(&value.ucs_x_axis);
    let _ = view(&value.ucs_y_axis);
    let _ = view(&value.ucs_ortho_type);
    let _ = view(&value.block_record);
    let _ = view(&value.viewport);
    let _ = view(&value.viewports);
    let _ = view(&value.base_ucs);
    let _ = view(&value.named_ucs);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.raw_plot_settings_codes);
    let _ = view(&value.paper_width);
    let _ = view(&value.paper_height);
    let _ = view(&value.plot_rotation);
    let _ = view(&value.plot_flags);
    let _ = view(&value.plot_page_name);
    let _ = view(&value.plot_printer_name);
    let _ = view(&value.paper_size);
    let _ = view(&value.plot_view_name);
    let _ = view(&value.plot_style_sheet);
    let _ = view(&value.plot_margin_left);
    let _ = view(&value.plot_margin_bottom);
    let _ = view(&value.plot_margin_right);
    let _ = view(&value.plot_margin_top);
    let _ = view(&value.plot_origin_x);
    let _ = view(&value.plot_origin_y);
    let _ = view(&value.plot_window_min_x);
    let _ = view(&value.plot_window_min_y);
    let _ = view(&value.plot_window_max_x);
    let _ = view(&value.plot_window_max_y);
    let _ = view(&value.plot_paper_units);
    let _ = view(&value.plot_type);
    let _ = view(&value.plot_scale_numerator);
    let _ = view(&value.plot_scale_denominator);
    let _ = view(&value.plot_scale_type);
    let _ = view(&value.plot_scale_factor);
    let _ = view(&value.paper_image_origin_x);
    let _ = view(&value.paper_image_origin_y);
    let _ = view(&value.shade_plot_mode);
    let _ = view(&value.shade_plot_resolution);
    let _ = view(&value.shade_plot_dpi);
    let _ = view(&value.plot_view_handle);
    let _ = view(&value.visual_style_handle);
};
impl NativeView for acadrust::objects::ObjectType {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::Dictionary(..) => 1,
            Self::Layout(..) => 1,
            Self::XRecord(..) => 1,
            Self::Group(..) => 1,
            Self::MLineStyle(..) => 1,
            Self::ImageDefinition(..) => 1,
            Self::UnderlayDefinition(..) => 1,
            Self::PlotSettings(..) => 1,
            Self::MultiLeaderStyle(..) => 1,
            Self::TableStyle(..) => 1,
            Self::TableContent(..) => 1,
            Self::Scale(..) => 1,
            Self::ObjectContextData(..) => 1,
            Self::SortEntitiesTable(..) => 1,
            Self::DictionaryVariable(..) => 1,
            Self::VisualStyle(..) => 1,
            Self::Material(..) => 1,
            Self::ImageDefinitionReactor(..) => 1,
            Self::GeoData(..) => 1,
            Self::SpatialFilter(..) => 1,
            Self::RasterVariables(..) => 1,
            Self::BookColor(..) => 1,
            Self::PlaceHolder(..) => 1,
            Self::DictionaryWithDefault(..) => 1,
            Self::WipeoutVariables(..) => 1,
            Self::BlockVisibilityParameter(..) => 1,
            Self::DynamicBlock(..) => 1,
            Self::Associative(..) => 1,
            Self::ClassObject(..) => 1,
            Self::DataObject(..) => 1,
            Self::Field(..) => 1,
            Self::FieldList(..) => 1,
            Self::RegisteredClass(..) => 1,
            Self::DgnLineStyle(..) => 1,
            Self::ProxyObject(..) => 1,
            Self::Unknown { .. } => 7,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::Dictionary(..) => "Dictionary",
            Self::Layout(..) => "Layout",
            Self::XRecord(..) => "XRecord",
            Self::Group(..) => "Group",
            Self::MLineStyle(..) => "MLineStyle",
            Self::ImageDefinition(..) => "ImageDefinition",
            Self::UnderlayDefinition(..) => "UnderlayDefinition",
            Self::PlotSettings(..) => "PlotSettings",
            Self::MultiLeaderStyle(..) => "MultiLeaderStyle",
            Self::TableStyle(..) => "TableStyle",
            Self::TableContent(..) => "TableContent",
            Self::Scale(..) => "Scale",
            Self::ObjectContextData(..) => "ObjectContextData",
            Self::SortEntitiesTable(..) => "SortEntitiesTable",
            Self::DictionaryVariable(..) => "DictionaryVariable",
            Self::VisualStyle(..) => "VisualStyle",
            Self::Material(..) => "Material",
            Self::ImageDefinitionReactor(..) => "ImageDefinitionReactor",
            Self::GeoData(..) => "GeoData",
            Self::SpatialFilter(..) => "SpatialFilter",
            Self::RasterVariables(..) => "RasterVariables",
            Self::BookColor(..) => "BookColor",
            Self::PlaceHolder(..) => "PlaceHolder",
            Self::DictionaryWithDefault(..) => "DictionaryWithDefault",
            Self::WipeoutVariables(..) => "WipeoutVariables",
            Self::BlockVisibilityParameter(..) => "BlockVisibilityParameter",
            Self::DynamicBlock(..) => "DynamicBlock",
            Self::Associative(..) => "Associative",
            Self::ClassObject(..) => "ClassObject",
            Self::DataObject(..) => "DataObject",
            Self::Field(..) => "Field",
            Self::FieldList(..) => "FieldList",
            Self::RegisteredClass(..) => "RegisteredClass",
            Self::DgnLineStyle(..) => "DgnLineStyle",
            Self::ProxyObject(..) => "ProxyObject",
            Self::Unknown { .. } => "Unknown",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::Dictionary(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Layout(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::XRecord(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Group(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::MLineStyle(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::ImageDefinition(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::UnderlayDefinition(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::PlotSettings(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::MultiLeaderStyle(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::TableStyle(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::TableContent(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Scale(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::ObjectContextData(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::SortEntitiesTable(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::DictionaryVariable(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::VisualStyle(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Material(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::ImageDefinitionReactor(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::GeoData(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::SpatialFilter(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::RasterVariables(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::BookColor(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::PlaceHolder(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::DictionaryWithDefault(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::WipeoutVariables(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::BlockVisibilityParameter(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::DynamicBlock(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Associative(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::ClassObject(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::DataObject(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Field(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::FieldList(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::RegisteredClass(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::DgnLineStyle(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::ProxyObject(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Unknown {
                type_name,
                handle,
                owner,
                raw_dxf_codes,
                raw_dwg_data,
                raw_dwg_handle_bits,
                raw_dwg_version,
            } => match index {
                0 => Some((StringView::new("type_name"), view(type_name))),
                1 => Some((StringView::new("handle"), view(handle))),
                2 => Some((StringView::new("owner"), view(owner))),
                3 => Some((StringView::new("raw_dxf_codes"), view(raw_dxf_codes))),
                4 => Some((StringView::new("raw_dwg_data"), view(raw_dwg_data))),
                5 => Some((
                    StringView::new("raw_dwg_handle_bits"),
                    view(raw_dwg_handle_bits),
                )),
                6 => Some((StringView::new("raw_dwg_version"), view(raw_dwg_version))),
                _ => None,
            },
        }
    }
}

// objects/multileader_style.rs
view_enum!(acadrust::objects::LeaderContentType {
    None,
    Block,
    MText,
    Tolerance,
});
view_enum!(acadrust::objects::MultiLeaderPathType {
    Invisible,
    StraightLineSegments,
    Spline,
});
view_enum!(acadrust::objects::TextAttachmentType {
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
view_enum!(acadrust::objects::TextAttachmentDirectionType {
    Horizontal,
    Vertical,
});
view_enum!(acadrust::objects::TextAlignmentType {
    Left,
    Center,
    Right,
});
view_enum!(acadrust::objects::TextAngleType {
    ParallelToLastLeaderLine,
    Horizontal,
    Optimized,
});
view_enum!(acadrust::objects::BlockContentConnectionType {
    BlockExtents,
    BasePoint,
});
view_enum!(acadrust::objects::LeaderDrawOrderType {
    LeaderHeadFirst,
    LeaderTailFirst,
});
view_enum!(acadrust::objects::MultiLeaderDrawOrderType {
    ContentFirst,
    LeaderFirst,
});
view_struct!(acadrust::objects::MultiLeaderStyle {
    handle,
    owner_handle,
    name,
    description,
    path_type,
    line_color,
    line_type_handle,
    line_weight,
    enable_landing,
    enable_dogleg,
    landing_distance,
    landing_gap,
    arrowhead_handle,
    arrowhead_size,
    content_type,
    text_style_handle,
    text_left_attachment,
    text_right_attachment,
    text_top_attachment,
    text_bottom_attachment,
    text_attachment_direction,
    text_angle_type,
    text_alignment,
    text_color,
    text_height,
    text_frame,
    text_always_left,
    default_text,
    block_content_handle,
    block_content_color,
    block_content_connection,
    block_content_rotation,
    block_content_scale_x,
    block_content_scale_y,
    block_content_scale_z,
    enable_block_scale,
    enable_block_rotation,
    scale_factor,
    align_space,
    break_gap_size,
    max_leader_points,
    first_segment_angle,
    second_segment_angle,
    leader_draw_order,
    multileader_draw_order,
    is_annotative,
    property_changed,
    unknown_flag_298
});
const _: fn(&acadrust::objects::MultiLeaderStyle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner_handle);
    let _ = view(&value.name);
    let _ = view(&value.description);
    let _ = view(&value.path_type);
    let _ = view(&value.line_color);
    let _ = view(&value.line_type_handle);
    let _ = view(&value.line_weight);
    let _ = view(&value.enable_landing);
    let _ = view(&value.enable_dogleg);
    let _ = view(&value.landing_distance);
    let _ = view(&value.landing_gap);
    let _ = view(&value.arrowhead_handle);
    let _ = view(&value.arrowhead_size);
    let _ = view(&value.content_type);
    let _ = view(&value.text_style_handle);
    let _ = view(&value.text_left_attachment);
    let _ = view(&value.text_right_attachment);
    let _ = view(&value.text_top_attachment);
    let _ = view(&value.text_bottom_attachment);
    let _ = view(&value.text_attachment_direction);
    let _ = view(&value.text_angle_type);
    let _ = view(&value.text_alignment);
    let _ = view(&value.text_color);
    let _ = view(&value.text_height);
    let _ = view(&value.text_frame);
    let _ = view(&value.text_always_left);
    let _ = view(&value.default_text);
    let _ = view(&value.block_content_handle);
    let _ = view(&value.block_content_color);
    let _ = view(&value.block_content_connection);
    let _ = view(&value.block_content_rotation);
    let _ = view(&value.block_content_scale_x);
    let _ = view(&value.block_content_scale_y);
    let _ = view(&value.block_content_scale_z);
    let _ = view(&value.enable_block_scale);
    let _ = view(&value.enable_block_rotation);
    let _ = view(&value.scale_factor);
    let _ = view(&value.align_space);
    let _ = view(&value.break_gap_size);
    let _ = view(&value.max_leader_points);
    let _ = view(&value.first_segment_angle);
    let _ = view(&value.second_segment_angle);
    let _ = view(&value.leader_draw_order);
    let _ = view(&value.multileader_draw_order);
    let _ = view(&value.is_annotative);
    let _ = view(&value.property_changed);
    let _ = view(&value.unknown_flag_298);
};

// objects/object_context_data.rs
view_struct!(acadrust::objects::ObjectContextData {
    handle,
    owner_handle,
    reactors,
    xdictionary_handle,
    class_version,
    is_default,
    scale,
    kind
});
const _: fn(&acadrust::objects::ObjectContextData) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner_handle);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.class_version);
    let _ = view(&value.is_default);
    let _ = view(&value.scale);
    let _ = view(&value.kind);
};
impl NativeView for acadrust::objects::ObjectContextKind {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::AnnotScale => 0,
            Self::BlkRef { .. } => 3,
            Self::Text { .. } => 4,
            Self::MText(..) => 1,
            Self::Dim(..) => 1,
            Self::MLeader(..) => 1,
            Self::MTextAttribute(..) => 1,
            Self::Leader(..) => 1,
            Self::Fcf { .. } => 2,
            Self::HatchScale(..) => 1,
            Self::HatchView(..) => 1,
            Self::Opaque => 0,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::AnnotScale => "AnnotScale",
            Self::BlkRef { .. } => "BlkRef",
            Self::Text { .. } => "Text",
            Self::MText(..) => "MText",
            Self::Dim(..) => "Dim",
            Self::MLeader(..) => "MLeader",
            Self::MTextAttribute(..) => "MTextAttribute",
            Self::Leader(..) => "Leader",
            Self::Fcf { .. } => "Fcf",
            Self::HatchScale(..) => "HatchScale",
            Self::HatchView(..) => "HatchView",
            Self::Opaque => "Opaque",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::AnnotScale => match index {
                _ => None,
            },
            Self::BlkRef {
                rotation,
                insertion,
                scale_factor,
            } => match index {
                0 => Some((StringView::new("rotation"), view(rotation))),
                1 => Some((StringView::new("insertion"), view(insertion))),
                2 => Some((StringView::new("scale_factor"), view(scale_factor))),
                _ => None,
            },
            Self::Text {
                horizontal_mode,
                rotation,
                insertion,
                alignment,
            } => match index {
                0 => Some((StringView::new("horizontal_mode"), view(horizontal_mode))),
                1 => Some((StringView::new("rotation"), view(rotation))),
                2 => Some((StringView::new("insertion"), view(insertion))),
                3 => Some((StringView::new("alignment"), view(alignment))),
                _ => None,
            },
            Self::MText(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Dim(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::MLeader(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::MTextAttribute(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Leader(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Fcf {
                location,
                horizontal_direction,
            } => match index {
                0 => Some((StringView::new("location"), view(location))),
                1 => Some((
                    StringView::new("horizontal_direction"),
                    view(horizontal_direction),
                )),
                _ => None,
            },
            Self::HatchScale(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::HatchView(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Opaque => match index {
                _ => None,
            },
        }
    }
}
view_struct!(acadrust::objects::HatchScaleContext {
    pattern_lines,
    pattern_scale,
    pattern_base,
    loops
});
const _: fn(&acadrust::objects::HatchScaleContext) = |value| {
    let _ = view(&value.pattern_lines);
    let _ = view(&value.pattern_scale);
    let _ = view(&value.pattern_base);
    let _ = view(&value.loops);
};
view_struct!(acadrust::objects::HatchLoopContext {
    loop_type,
    supports_context,
    boundary
});
const _: fn(&acadrust::objects::HatchLoopContext) = |value| {
    let _ = view(&value.loop_type);
    let _ = view(&value.supports_context);
    let _ = view(&value.boundary);
};
view_struct!(acadrust::objects::HatchViewContext {
    hatch,
    view,
    view_normal,
    view_rotation,
    evaluate_hatch
});
const _: fn(&acadrust::objects::HatchViewContext) = |value| {
    let _ = view(&value.hatch);
    let _ = view(&value.view);
    let _ = view(&value.view_normal);
    let _ = view(&value.view_rotation);
    let _ = view(&value.evaluate_hatch);
};
view_struct!(acadrust::objects::MTextContext {
    attachment,
    x_axis_dir,
    insertion,
    rect_width,
    rect_height,
    extents_width,
    extents_height,
    column_type,
    columns
});
const _: fn(&acadrust::objects::MTextContext) = |value| {
    let _ = view(&value.attachment);
    let _ = view(&value.x_axis_dir);
    let _ = view(&value.insertion);
    let _ = view(&value.rect_width);
    let _ = view(&value.rect_height);
    let _ = view(&value.extents_width);
    let _ = view(&value.extents_height);
    let _ = view(&value.column_type);
    let _ = view(&value.columns);
};
view_struct!(acadrust::objects::MTextAttributeContext {
    horizontal_mode,
    rotation,
    insertion,
    alignment,
    enable_context,
    context
});
const _: fn(&acadrust::objects::MTextAttributeContext) = |value| {
    let _ = view(&value.horizontal_mode);
    let _ = view(&value.rotation);
    let _ = view(&value.insertion);
    let _ = view(&value.alignment);
    let _ = view(&value.enable_context);
    let _ = view(&value.context);
};
view_struct!(acadrust::objects::EmbeddedMTextContext {
    owner_handle,
    reactors,
    xdictionary_handle,
    has_binary_data,
    class_version,
    is_default,
    scale,
    mtext
});
const _: fn(&acadrust::objects::EmbeddedMTextContext) = |value| {
    let _ = view(&value.owner_handle);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.has_binary_data);
    let _ = view(&value.class_version);
    let _ = view(&value.is_default);
    let _ = view(&value.scale);
    let _ = view(&value.mtext);
};
view_struct!(acadrust::objects::LeaderContext {
    points,
    x_direction,
    annotation_enabled,
    insertion_offset,
    endpoint_projection
});
const _: fn(&acadrust::objects::LeaderContext) = |value| {
    let _ = view(&value.points);
    let _ = view(&value.x_direction);
    let _ = view(&value.annotation_enabled);
    let _ = view(&value.insertion_offset);
    let _ = view(&value.endpoint_projection);
};
view_struct!(acadrust::objects::DimContext {
    def_pt,
    is_def_textloc,
    text_rotation,
    block,
    b293,
    dimtofl,
    dimosxd,
    dimatfit,
    dimtix,
    dimtmove,
    override_code,
    has_arrow2,
    flip_arrow2,
    flip_arrow1,
    subtype
});
const _: fn(&acadrust::objects::DimContext) = |value| {
    let _ = view(&value.def_pt);
    let _ = view(&value.is_def_textloc);
    let _ = view(&value.text_rotation);
    let _ = view(&value.block);
    let _ = view(&value.b293);
    let _ = view(&value.dimtofl);
    let _ = view(&value.dimosxd);
    let _ = view(&value.dimatfit);
    let _ = view(&value.dimtix);
    let _ = view(&value.dimtmove);
    let _ = view(&value.override_code);
    let _ = view(&value.has_arrow2);
    let _ = view(&value.flip_arrow2);
    let _ = view(&value.flip_arrow1);
    let _ = view(&value.subtype);
};
impl NativeView for acadrust::objects::DimSubtype {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::Aligned { .. } => 1,
            Self::Angular { .. } => 1,
            Self::Diametric { .. } => 2,
            Self::Radial { .. } => 1,
            Self::RadialLarge { .. } => 2,
            Self::Ordinate { .. } => 2,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::Aligned { .. } => "Aligned",
            Self::Angular { .. } => "Angular",
            Self::Diametric { .. } => "Diametric",
            Self::Radial { .. } => "Radial",
            Self::RadialLarge { .. } => "RadialLarge",
            Self::Ordinate { .. } => "Ordinate",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::Aligned { dimline_pt } => match index {
                0 => Some((StringView::new("dimline_pt"), view(dimline_pt))),
                _ => None,
            },
            Self::Angular { arc_pt } => match index {
                0 => Some((StringView::new("arc_pt"), view(arc_pt))),
                _ => None,
            },
            Self::Diametric {
                first_arc_pt,
                def_pt,
            } => match index {
                0 => Some((StringView::new("first_arc_pt"), view(first_arc_pt))),
                1 => Some((StringView::new("def_pt"), view(def_pt))),
                _ => None,
            },
            Self::Radial { first_arc_pt } => match index {
                0 => Some((StringView::new("first_arc_pt"), view(first_arc_pt))),
                _ => None,
            },
            Self::RadialLarge {
                ovr_center,
                jog_point,
            } => match index {
                0 => Some((StringView::new("ovr_center"), view(ovr_center))),
                1 => Some((StringView::new("jog_point"), view(jog_point))),
                _ => None,
            },
            Self::Ordinate {
                feature_location_pt,
                leader_endpt,
            } => match index {
                0 => Some((
                    StringView::new("feature_location_pt"),
                    view(feature_location_pt),
                )),
                1 => Some((StringView::new("leader_endpt"), view(leader_endpt))),
                _ => None,
            },
        }
    }
}
view_struct!(acadrust::objects::MTextColumns {
    num_heights,
    width,
    gutter,
    auto_height,
    flow_reversed,
    heights
});
const _: fn(&acadrust::objects::MTextColumns) = |value| {
    let _ = view(&value.num_heights);
    let _ = view(&value.width);
    let _ = view(&value.gutter);
    let _ = view(&value.auto_height);
    let _ = view(&value.flow_reversed);
    let _ = view(&value.heights);
};

// objects/plot_settings.rs
view_enum!(acadrust::objects::PlotPaperUnits {
    Inches,
    Millimeters,
    Pixels,
});
view_enum!(acadrust::objects::PlotRotation {
    None,
    Degrees90,
    Degrees180,
    Degrees270,
});
view_enum!(acadrust::objects::PlotType {
    LastScreenDisplay,
    Extents,
    Limits,
    View,
    Window,
    Layout,
});
view_enum!(acadrust::objects::ScaledType {
    ScaleToFit,
    CustomScale,
    OneToOne,
    OneToTwo,
    OneToFour,
    OneToEight,
    OneToTen,
    OneToSixteen,
    OneToTwenty,
    OneToThirty,
    OneToForty,
    OneToFifty,
    OneToHundred,
    TwoToOne,
    FourToOne,
    EightToOne,
    TenToOne,
    HundredToOne,
});
view_enum!(acadrust::objects::ShadePlotMode {
    AsDisplayed,
    Wireframe,
    Hidden,
    Rendered,
});
view_enum!(acadrust::objects::ShadePlotResolutionLevel {
    Draft,
    Preview,
    Normal,
    Presentation,
    Maximum,
    Custom,
});
view_struct!(acadrust::objects::PlotFlags {
    unknown_bits,
    plot_viewport_borders,
    show_plot_styles,
    plot_centered,
    plot_hidden,
    use_standard_scale,
    plot_plot_styles,
    scale_lineweights,
    print_lineweights,
    draw_viewports_first,
    model_type,
    update_paper,
    zoom_to_paper_on_update,
    initializing,
    prev_plot_init
});
const _: fn(&acadrust::objects::PlotFlags) = |value| {
    let _ = view(&value.unknown_bits);
    let _ = view(&value.plot_viewport_borders);
    let _ = view(&value.show_plot_styles);
    let _ = view(&value.plot_centered);
    let _ = view(&value.plot_hidden);
    let _ = view(&value.use_standard_scale);
    let _ = view(&value.plot_plot_styles);
    let _ = view(&value.scale_lineweights);
    let _ = view(&value.print_lineweights);
    let _ = view(&value.draw_viewports_first);
    let _ = view(&value.model_type);
    let _ = view(&value.update_paper);
    let _ = view(&value.zoom_to_paper_on_update);
    let _ = view(&value.initializing);
    let _ = view(&value.prev_plot_init);
};
view_struct!(acadrust::objects::PaperMargin {
    left,
    bottom,
    right,
    top
});
const _: fn(&acadrust::objects::PaperMargin) = |value| {
    let _ = view(&value.left);
    let _ = view(&value.bottom);
    let _ = view(&value.right);
    let _ = view(&value.top);
};
view_struct!(acadrust::objects::PlotWindow {
    lower_left_x,
    lower_left_y,
    upper_right_x,
    upper_right_y
});
const _: fn(&acadrust::objects::PlotWindow) = |value| {
    let _ = view(&value.lower_left_x);
    let _ = view(&value.lower_left_y);
    let _ = view(&value.upper_right_x);
    let _ = view(&value.upper_right_y);
};
view_struct!(acadrust::objects::PlotSettings {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    page_name,
    printer_name,
    paper_size,
    plot_view_name,
    current_style_sheet,
    paper_width,
    paper_height,
    margins,
    origin_x,
    origin_y,
    plot_window,
    scale_numerator,
    scale_denominator,
    paper_units,
    rotation,
    plot_type,
    scale_type,
    shade_plot_mode,
    shade_plot_resolution,
    shade_plot_dpi,
    flags,
    standard_scale_factor,
    paper_image_origin_x,
    paper_image_origin_y,
    plot_view_handle,
    visual_style_handle
});
const _: fn(&acadrust::objects::PlotSettings) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.page_name);
    let _ = view(&value.printer_name);
    let _ = view(&value.paper_size);
    let _ = view(&value.plot_view_name);
    let _ = view(&value.current_style_sheet);
    let _ = view(&value.paper_width);
    let _ = view(&value.paper_height);
    let _ = view(&value.margins);
    let _ = view(&value.origin_x);
    let _ = view(&value.origin_y);
    let _ = view(&value.plot_window);
    let _ = view(&value.scale_numerator);
    let _ = view(&value.scale_denominator);
    let _ = view(&value.paper_units);
    let _ = view(&value.rotation);
    let _ = view(&value.plot_type);
    let _ = view(&value.scale_type);
    let _ = view(&value.shade_plot_mode);
    let _ = view(&value.shade_plot_resolution);
    let _ = view(&value.shade_plot_dpi);
    let _ = view(&value.flags);
    let _ = view(&value.standard_scale_factor);
    let _ = view(&value.paper_image_origin_x);
    let _ = view(&value.paper_image_origin_y);
    let _ = view(&value.plot_view_handle);
    let _ = view(&value.visual_style_handle);
};

// objects/scale.rs
view_struct!(acadrust::objects::Scale {
    handle,
    owner_handle,
    name,
    paper_units,
    drawing_units,
    is_unit_scale,
    is_temporary
});
const _: fn(&acadrust::objects::Scale) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner_handle);
    let _ = view(&value.name);
    let _ = view(&value.paper_units);
    let _ = view(&value.drawing_units);
    let _ = view(&value.is_unit_scale);
    let _ = view(&value.is_temporary);
};

// objects/semantic_property.rs
view_struct!(acadrust::objects::SemanticProperty {
    subclass,
    code,
    value
});
const _: fn(&acadrust::objects::SemanticProperty) = |value| {
    let _ = view(&value.subclass);
    let _ = view(&value.code);
    let _ = view(&value.value);
};
view_enum!(acadrust::objects::SemanticPropertyValue {
    Text(v0),
    Bool(v0),
    Byte(v0),
    Int16(v0),
    Int32(v0),
    Int64(v0),
    Double(v0),
    Handle(v0),
    Binary(v0),
});
view_struct!(acadrust::objects::RegisteredClassObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    dxf_name,
    cpp_class_name,
    properties,
    payload,
    object_ids,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
const _: fn(&acadrust::objects::RegisteredClassObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.dxf_name);
    let _ = view(&value.cpp_class_name);
    let _ = view(&value.properties);
    let _ = view(&value.payload);
    let _ = view(&value.object_ids);
    let _ = view(&value.raw_dwg_data);
    let _ = view(&value.raw_dwg_handle_bits);
    let _ = view(&value.raw_dwg_version);
};
view_struct!(acadrust::objects::ProxyObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    proxy_id,
    class_id,
    dxf_subclass,
    version,
    dwg_version,
    maintenance_version,
    from_dxf,
    payload,
    text_payload,
    object_ids
});
const _: fn(&acadrust::objects::ProxyObject) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.proxy_id);
    let _ = view(&value.class_id);
    let _ = view(&value.dxf_subclass);
    let _ = view(&value.version);
    let _ = view(&value.dwg_version);
    let _ = view(&value.maintenance_version);
    let _ = view(&value.from_dxf);
    let _ = view(&value.payload);
    let _ = view(&value.text_payload);
    let _ = view(&value.object_ids);
};
view_enum!(acadrust::objects::ProxyPayloadEncoding { Bytes, Bits });
view_struct!(acadrust::objects::ProxyPayload {
    encoding,
    bit_count,
    records
});
const _: fn(&acadrust::objects::ProxyPayload) = |value| {
    let _ = view(&value.encoding);
    let _ = view(&value.bit_count);
    let _ = view(&value.records);
};
view_struct!(acadrust::objects::ProxyPayloadRecord {
    sequence,
    bit_offset,
    bit_count,
    data
});
const _: fn(&acadrust::objects::ProxyPayloadRecord) = |value| {
    let _ = view(&value.sequence);
    let _ = view(&value.bit_offset);
    let _ = view(&value.bit_count);
    let _ = view(&value.data);
};
view_enum!(acadrust::objects::ProxyReferenceKind {
    Undefined,
    SoftOwnership,
    HardOwnership,
    SoftPointer,
    HardPointer,
});
view_struct!(acadrust::objects::ProxyObjectReference { handle, kind });
const _: fn(&acadrust::objects::ProxyObjectReference) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.kind);
};

// objects/sort_entities_table.rs
view_struct!(acadrust::objects::SortEntsEntry {
    entity_handle,
    sort_handle
});
const _: fn(&acadrust::objects::SortEntsEntry) = |value| {
    let _ = view(&value.entity_handle);
    let _ = view(&value.sort_handle);
};
#[repr(transparent)]
struct SortEntries(acadrust::objects::SortEntitiesTable);

impl NativeView for SortEntries {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_SEQUENCE
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        self.0
            .entries()
            .nth(index)
            .map(|entry| (StringView::new(""), view(entry)))
    }
}

impl NativeView for acadrust::objects::SortEntitiesTable {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_STRUCT
    }
    fn len(&self) -> usize {
        6
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match index {
            0 => Some((StringView::new("handle"), view(&self.handle))),
            1 => Some((StringView::new("owner_handle"), view(&self.owner_handle))),
            2 => Some((
                StringView::new("block_owner_handle"),
                view(&self.block_owner_handle),
            )),
            3 => Some((StringView::new("raw_dxf_codes"), view(&self.raw_dxf_codes))),
            4 => Some((
                StringView::new("raw_dxf_version"),
                view(&self.raw_dxf_version),
            )),
            5 => {
                // The transparent adapter borrows the same object, exposing its private collection through its public iterator.
                let entries = unsafe { &*(std::ptr::from_ref(self).cast::<SortEntries>()) };
                Some((StringView::new("entries"), view(entries)))
            }
            _ => None,
        }
    }
}
const _: fn(&acadrust::objects::SortEntitiesTable) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner_handle);
    let _ = view(&value.block_owner_handle);
    let _ = view(&value.raw_dxf_codes);
    let _ = view(&value.raw_dxf_version);
};

// objects/stub_objects.rs
view_enum!(acadrust::objects::VisualStylePropertyValue {
    Short(v0),
    Long(v0),
    Double(v0),
    Bool(v0),
    Color(v0),
    Text(v0),
});
view_struct!(acadrust::objects::VisualStyleProperty { value, enabled });
const _: fn(&acadrust::objects::VisualStyleProperty) = |value| {
    let _ = view(&value.value);
    let _ = view(&value.enabled);
};
view_struct!(acadrust::objects::VisualStyle {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    description,
    style_type,
    face_lighting_model,
    face_lighting_quality,
    face_color_mode,
    face_modifier,
    edge_model,
    edge_style,
    internal_use_only,
    extended_lighting_model,
    properties
});
const _: fn(&acadrust::objects::VisualStyle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.description);
    let _ = view(&value.style_type);
    let _ = view(&value.face_lighting_model);
    let _ = view(&value.face_lighting_quality);
    let _ = view(&value.face_color_mode);
    let _ = view(&value.face_modifier);
    let _ = view(&value.edge_model);
    let _ = view(&value.edge_style);
    let _ = view(&value.internal_use_only);
    let _ = view(&value.extended_lighting_model);
    let _ = view(&value.properties);
};
view_struct!(acadrust::objects::MaterialColor { flag, factor, rgb });
const _: fn(&acadrust::objects::MaterialColor) = |value| {
    let _ = view(&value.flag);
    let _ = view(&value.factor);
    let _ = view(&value.rgb);
};
view_enum!(acadrust::objects::MaterialProceduralValue {
    Bool(v0),
    Integer(v0),
    Real(v0),
    Color(v0),
    Text(v0),
    Table(v0),
});
view_struct!(acadrust::objects::MaterialTexture {
    mode,
    color1,
    color2,
    procedural,
    table_end
});
const _: fn(&acadrust::objects::MaterialTexture) = |value| {
    let _ = view(&value.mode);
    let _ = view(&value.color1);
    let _ = view(&value.color2);
    let _ = view(&value.procedural);
    let _ = view(&value.table_end);
};
view_struct!(acadrust::objects::MaterialMap {
    blend_factor,
    projection,
    tiling,
    auto_transform,
    transform,
    source,
    file_name,
    texture
});
const _: fn(&acadrust::objects::MaterialMap) = |value| {
    let _ = view(&value.blend_factor);
    let _ = view(&value.projection);
    let _ = view(&value.tiling);
    let _ = view(&value.auto_transform);
    let _ = view(&value.transform);
    let _ = view(&value.source);
    let _ = view(&value.file_name);
    let _ = view(&value.texture);
};
view_struct!(acadrust::objects::Material {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    name,
    description,
    ambient_color,
    diffuse_color,
    diffuse_map,
    specular_color,
    specular_map,
    specular_gloss_factor,
    reflection_map,
    opacity_percent,
    opacity_map,
    bump_map,
    refraction_index,
    refraction_map,
    translucence,
    self_illumination,
    reflectivity,
    illumination_model,
    channel_flags,
    mode,
    indirect_bump_scale,
    reflectance_scale,
    transmittance_scale,
    two_sided_material,
    luminance,
    luminance_mode,
    normal_map_method,
    normal_map_strength,
    normal_map,
    is_anonymous,
    global_illumination,
    final_gather,
    color_bleed_scale,
    advanced_data_present
});
const _: fn(&acadrust::objects::Material) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.name);
    let _ = view(&value.description);
    let _ = view(&value.ambient_color);
    let _ = view(&value.diffuse_color);
    let _ = view(&value.diffuse_map);
    let _ = view(&value.specular_color);
    let _ = view(&value.specular_map);
    let _ = view(&value.specular_gloss_factor);
    let _ = view(&value.reflection_map);
    let _ = view(&value.opacity_percent);
    let _ = view(&value.opacity_map);
    let _ = view(&value.bump_map);
    let _ = view(&value.refraction_index);
    let _ = view(&value.refraction_map);
    let _ = view(&value.translucence);
    let _ = view(&value.self_illumination);
    let _ = view(&value.reflectivity);
    let _ = view(&value.illumination_model);
    let _ = view(&value.channel_flags);
    let _ = view(&value.mode);
    let _ = view(&value.indirect_bump_scale);
    let _ = view(&value.reflectance_scale);
    let _ = view(&value.transmittance_scale);
    let _ = view(&value.two_sided_material);
    let _ = view(&value.luminance);
    let _ = view(&value.luminance_mode);
    let _ = view(&value.normal_map_method);
    let _ = view(&value.normal_map_strength);
    let _ = view(&value.normal_map);
    let _ = view(&value.is_anonymous);
    let _ = view(&value.global_illumination);
    let _ = view(&value.final_gather);
    let _ = view(&value.color_bleed_scale);
    let _ = view(&value.advanced_data_present);
};
view_struct!(acadrust::objects::GeoDataMeshPoint {
    source,
    destination
});
const _: fn(&acadrust::objects::GeoDataMeshPoint) = |value| {
    let _ = view(&value.source);
    let _ = view(&value.destination);
};
view_struct!(acadrust::objects::GeoDataMeshFace {
    first,
    second,
    third
});
const _: fn(&acadrust::objects::GeoDataMeshFace) = |value| {
    let _ = view(&value.first);
    let _ = view(&value.second);
    let _ = view(&value.third);
};
view_struct!(acadrust::objects::GeoData {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    version,
    host_block,
    coordinate_type,
    design_point,
    reference_point,
    obsolete_observation_point,
    obsolete_scale_vector,
    north_direction,
    up_direction,
    horizontal_unit_scale,
    vertical_unit_scale,
    horizontal_units,
    vertical_units,
    scale_estimation_method,
    user_scale_factor,
    sea_level_correction,
    sea_level_elevation,
    coordinate_projection_radius,
    coordinate_system_definition,
    coordinate_system_datum,
    coordinate_system_wkt,
    geo_rss_tag,
    observation_from_tag,
    observation_to_tag,
    observation_coverage_tag,
    mesh_points,
    mesh_faces,
    civil_data_present,
    civil_obsolete_flag,
    civil_reference_point1,
    civil_reference_point2,
    civil_unknown1,
    civil_unknown2,
    civil_unknown_flag1,
    civil_zero_point1,
    civil_zero_point2,
    civil_unknown_flag2,
    civil_north_angle_degrees,
    civil_north_angle_radians
});
const _: fn(&acadrust::objects::GeoData) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.version);
    let _ = view(&value.host_block);
    let _ = view(&value.coordinate_type);
    let _ = view(&value.design_point);
    let _ = view(&value.reference_point);
    let _ = view(&value.obsolete_observation_point);
    let _ = view(&value.obsolete_scale_vector);
    let _ = view(&value.north_direction);
    let _ = view(&value.up_direction);
    let _ = view(&value.horizontal_unit_scale);
    let _ = view(&value.vertical_unit_scale);
    let _ = view(&value.horizontal_units);
    let _ = view(&value.vertical_units);
    let _ = view(&value.scale_estimation_method);
    let _ = view(&value.user_scale_factor);
    let _ = view(&value.sea_level_correction);
    let _ = view(&value.sea_level_elevation);
    let _ = view(&value.coordinate_projection_radius);
    let _ = view(&value.coordinate_system_definition);
    let _ = view(&value.coordinate_system_datum);
    let _ = view(&value.coordinate_system_wkt);
    let _ = view(&value.geo_rss_tag);
    let _ = view(&value.observation_from_tag);
    let _ = view(&value.observation_to_tag);
    let _ = view(&value.observation_coverage_tag);
    let _ = view(&value.mesh_points);
    let _ = view(&value.mesh_faces);
    let _ = view(&value.civil_data_present);
    let _ = view(&value.civil_obsolete_flag);
    let _ = view(&value.civil_reference_point1);
    let _ = view(&value.civil_reference_point2);
    let _ = view(&value.civil_unknown1);
    let _ = view(&value.civil_unknown2);
    let _ = view(&value.civil_unknown_flag1);
    let _ = view(&value.civil_zero_point1);
    let _ = view(&value.civil_zero_point2);
    let _ = view(&value.civil_unknown_flag2);
    let _ = view(&value.civil_north_angle_degrees);
    let _ = view(&value.civil_north_angle_radians);
};
view_struct!(acadrust::objects::SpatialFilter {
    handle,
    owner,
    boundary_points,
    normal,
    origin,
    display_enabled,
    front_clip,
    back_clip,
    inverse_block_transform,
    clip_bound_transform
});
const _: fn(&acadrust::objects::SpatialFilter) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.boundary_points);
    let _ = view(&value.normal);
    let _ = view(&value.origin);
    let _ = view(&value.display_enabled);
    let _ = view(&value.front_clip);
    let _ = view(&value.back_clip);
    let _ = view(&value.inverse_block_transform);
    let _ = view(&value.clip_bound_transform);
};
view_struct!(acadrust::objects::RasterVariables {
    handle,
    owner,
    class_version,
    display_image_frame,
    image_quality,
    units
});
const _: fn(&acadrust::objects::RasterVariables) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.class_version);
    let _ = view(&value.display_image_frame);
    let _ = view(&value.image_quality);
    let _ = view(&value.units);
};
view_struct!(acadrust::objects::BookColor {
    handle,
    owner,
    color,
    color_name,
    book_name
});
const _: fn(&acadrust::objects::BookColor) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.color);
    let _ = view(&value.color_name);
    let _ = view(&value.book_name);
};
view_struct!(acadrust::objects::PlaceHolder { handle, owner });
const _: fn(&acadrust::objects::PlaceHolder) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
};
view_struct!(acadrust::objects::DictionaryWithDefault {
    handle,
    owner,
    entries,
    default_handle,
    duplicate_cloning,
    hard_owner
});
const _: fn(&acadrust::objects::DictionaryWithDefault) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.entries);
    let _ = view(&value.default_handle);
    let _ = view(&value.duplicate_cloning);
    let _ = view(&value.hard_owner);
};
view_struct!(acadrust::objects::WipeoutVariables {
    handle,
    owner,
    display_frame
});
const _: fn(&acadrust::objects::WipeoutVariables) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.display_frame);
};

// objects/table_style.rs
view_enum!(acadrust::objects::TableFlowDirection { Down, Up });
view_enum!(acadrust::objects::CellAlignment {
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
view_enum!(acadrust::objects::TableBorderType { Single, Double });
view_struct!(acadrust::objects::TableCellBorder {
    property_flags,
    border_type,
    line_weight,
    color,
    is_invisible,
    double_line_spacing
});
const _: fn(&acadrust::objects::TableCellBorder) = |value| {
    let _ = view(&value.property_flags);
    let _ = view(&value.border_type);
    let _ = view(&value.line_weight);
    let _ = view(&value.color);
    let _ = view(&value.is_invisible);
    let _ = view(&value.double_line_spacing);
};
view_struct!(acadrust::objects::RowCellStyle {
    text_style_name,
    text_style_handle,
    text_height,
    alignment,
    text_color,
    fill_color,
    fill_enabled,
    data_type,
    unit_type,
    format_string,
    left_border,
    right_border,
    top_border,
    bottom_border,
    horizontal_inside_border,
    vertical_inside_border
});
const _: fn(&acadrust::objects::RowCellStyle) = |value| {
    let _ = view(&value.text_style_name);
    let _ = view(&value.text_style_handle);
    let _ = view(&value.text_height);
    let _ = view(&value.alignment);
    let _ = view(&value.text_color);
    let _ = view(&value.fill_color);
    let _ = view(&value.fill_enabled);
    let _ = view(&value.data_type);
    let _ = view(&value.unit_type);
    let _ = view(&value.format_string);
    let _ = view(&value.left_border);
    let _ = view(&value.right_border);
    let _ = view(&value.top_border);
    let _ = view(&value.bottom_border);
    let _ = view(&value.horizontal_inside_border);
    let _ = view(&value.vertical_inside_border);
};
view_struct!(acadrust::objects::TableContentFormat {
    property_override_flags,
    property_flags,
    value_data_type,
    value_unit_type,
    value_format_string,
    rotation,
    block_scale,
    cell_alignment,
    content_color,
    text_style,
    text_height
});
const _: fn(&acadrust::objects::TableContentFormat) = |value| {
    let _ = view(&value.property_override_flags);
    let _ = view(&value.property_flags);
    let _ = view(&value.value_data_type);
    let _ = view(&value.value_unit_type);
    let _ = view(&value.value_format_string);
    let _ = view(&value.rotation);
    let _ = view(&value.block_scale);
    let _ = view(&value.cell_alignment);
    let _ = view(&value.content_color);
    let _ = view(&value.text_style);
    let _ = view(&value.text_height);
};
view_struct!(acadrust::objects::TableGridFormat {
    index_mask,
    border,
    line_type
});
const _: fn(&acadrust::objects::TableGridFormat) = |value| {
    let _ = view(&value.index_mask);
    let _ = view(&value.border);
    let _ = view(&value.line_type);
};
view_struct!(acadrust::objects::TableCellStyleData {
    style_type,
    data_flags,
    property_override_flags,
    merge_flags,
    background_color,
    content_layout,
    content_format,
    margin_override_flags,
    vertical_margin,
    horizontal_margin,
    bottom_margin,
    right_margin,
    horizontal_spacing,
    vertical_spacing,
    borders
});
const _: fn(&acadrust::objects::TableCellStyleData) = |value| {
    let _ = view(&value.style_type);
    let _ = view(&value.data_flags);
    let _ = view(&value.property_override_flags);
    let _ = view(&value.merge_flags);
    let _ = view(&value.background_color);
    let _ = view(&value.content_layout);
    let _ = view(&value.content_format);
    let _ = view(&value.margin_override_flags);
    let _ = view(&value.vertical_margin);
    let _ = view(&value.horizontal_margin);
    let _ = view(&value.bottom_margin);
    let _ = view(&value.right_margin);
    let _ = view(&value.horizontal_spacing);
    let _ = view(&value.vertical_spacing);
    let _ = view(&value.borders);
};
view_struct!(acadrust::objects::NamedTableCellStyle {
    cell_style,
    id,
    style_type,
    name
});
const _: fn(&acadrust::objects::NamedTableCellStyle) = |value| {
    let _ = view(&value.cell_style);
    let _ = view(&value.id);
    let _ = view(&value.style_type);
    let _ = view(&value.name);
};
view_struct!(acadrust::objects::TableStyle {
    handle,
    owner_handle,
    name,
    description,
    version,
    flow_direction,
    flags,
    horizontal_margin,
    vertical_margin,
    title_suppressed,
    header_suppressed,
    data_row_style,
    header_row_style,
    title_row_style,
    modern_unknown_byte,
    modern_unknown_long1,
    modern_unknown_long2,
    modern_cell_style_handle,
    modern_style,
    modern_overrides,
    annotative,
    raw_dxf_codes
});
const _: fn(&acadrust::objects::TableStyle) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner_handle);
    let _ = view(&value.name);
    let _ = view(&value.description);
    let _ = view(&value.version);
    let _ = view(&value.flow_direction);
    let _ = view(&value.flags);
    let _ = view(&value.horizontal_margin);
    let _ = view(&value.vertical_margin);
    let _ = view(&value.title_suppressed);
    let _ = view(&value.header_suppressed);
    let _ = view(&value.data_row_style);
    let _ = view(&value.header_row_style);
    let _ = view(&value.title_row_style);
    let _ = view(&value.modern_unknown_byte);
    let _ = view(&value.modern_unknown_long1);
    let _ = view(&value.modern_unknown_long2);
    let _ = view(&value.modern_cell_style_handle);
    let _ = view(&value.modern_style);
    let _ = view(&value.modern_overrides);
    let _ = view(&value.annotative);
    let _ = view(&value.raw_dxf_codes);
};

// objects/xrecord.rs
view_enum!(acadrust::objects::DictionaryCloningFlags {
    NotApplicable,
    KeepExisting,
    UseClone,
    XrefName,
    Name,
    UnmangleName,
});
view_enum!(acadrust::objects::XRecordValueType {
    String,
    Point3D,
    Double,
    Byte,
    Int16,
    Int32,
    Int64,
    Handle,
    ObjectId,
    Bool,
    Chunk,
    Unknown,
});
impl NativeView for acadrust::objects::XRecordValue {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_ENUM
    }
    fn len(&self) -> usize {
        match self {
            Self::String(..) => 1,
            Self::Double(..) => 1,
            Self::Int16(..) => 1,
            Self::Int32(..) => 1,
            Self::Int64(..) => 1,
            Self::Byte(..) => 1,
            Self::Bool(..) => 1,
            Self::Handle(..) => 1,
            Self::Point3D(..) => 3,
            Self::Chunk(..) => 1,
        }
    }
    fn variant(&self) -> Option<&'static str> {
        Some(match self {
            Self::String(..) => "String",
            Self::Double(..) => "Double",
            Self::Int16(..) => "Int16",
            Self::Int32(..) => "Int32",
            Self::Int64(..) => "Int64",
            Self::Byte(..) => "Byte",
            Self::Bool(..) => "Bool",
            Self::Handle(..) => "Handle",
            Self::Point3D(..) => "Point3D",
            Self::Chunk(..) => "Chunk",
        })
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match self {
            Self::String(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Double(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Int16(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Int32(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Int64(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Byte(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Bool(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Handle(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
            Self::Point3D(v0, v1, v2) => match index {
                0 => Some((StringView::new("0"), view(v0))),
                1 => Some((StringView::new("1"), view(v1))),
                2 => Some((StringView::new("2"), view(v2))),
                _ => None,
            },
            Self::Chunk(v0) => match index {
                0 => Some((StringView::new("value"), view(v0))),
                _ => None,
            },
        }
    }
}
view_struct!(acadrust::objects::XRecordEntry { code, value });
const _: fn(&acadrust::objects::XRecordEntry) = |value| {
    let _ = view(&value.code);
    let _ = view(&value.value);
};
view_enum!(acadrust::objects::KnownXRecordKind {
    LayerViewportAlphaOverride,
    LayerViewportColorOverride,
    LayerViewportLinetypeOverride,
    LayerViewportLineweightOverride,
    LayerReconciled,
    LayerStateAnnotationScale,
    AnnotationScale,
    MeshTextureCoordinates,
    XrecordRoundTrip,
    MTextRoundTrip,
    HeaderRoundTrip,
    RecomposeData,
    DynamicBlockHistory,
    LayoutThumbnail,
    ViewTransition,
    PhotometricLight,
    LastSavedVersion,
    PreviousProductInfo,
    DrawingProperties,
    FingerprintGuid,
    DimensionStyleData,
    UcsData,
    PlotData,
    Hyperlink,
    AdvancedMaterial,
    MaterialAsset,
    Metadata,
    Unknown,
});
view_struct!(acadrust::objects::XRecordSection { name, entries });
const _: fn(&acadrust::objects::XRecordSection) = |value| {
    let _ = view(&value.name);
    let _ = view(&value.entries);
};
view_struct!(acadrust::objects::XRecord {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    name,
    cloning_flags,
    entries,
    object_references,
    preserve_object_reference_stream,
    entries_complete,
    raw_data,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
const _: fn(&acadrust::objects::XRecord) = |value| {
    let _ = view(&value.handle);
    let _ = view(&value.owner);
    let _ = view(&value.reactors);
    let _ = view(&value.xdictionary_handle);
    let _ = view(&value.name);
    let _ = view(&value.cloning_flags);
    let _ = view(&value.entries);
    let _ = view(&value.object_references);
    let _ = view(&value.preserve_object_reference_stream);
    let _ = view(&value.entries_complete);
    let _ = view(&value.raw_data);
    let _ = view(&value.raw_dwg_data);
    let _ = view(&value.raw_dwg_handle_bits);
    let _ = view(&value.raw_dwg_version);
};

// classes/mod.rs
view_struct!(acadrust::classes::ProxyFlags { 0 });
view_struct!(acadrust::classes::DxfClass {
    dxf_name,
    cpp_class_name,
    application_name,
    proxy_flags,
    instance_count,
    was_zombie,
    is_an_entity,
    class_number,
    item_class_id,
    dwg_version,
    maintenance_version,
    unknown1,
    unknown2
});
const _: fn(&acadrust::classes::DxfClass) = |value| {
    let _ = view(&value.dxf_name);
    let _ = view(&value.cpp_class_name);
    let _ = view(&value.application_name);
    let _ = view(&value.proxy_flags);
    let _ = view(&value.instance_count);
    let _ = view(&value.was_zombie);
    let _ = view(&value.is_an_entity);
    let _ = view(&value.class_number);
    let _ = view(&value.item_class_id);
    let _ = view(&value.dwg_version);
    let _ = view(&value.maintenance_version);
    let _ = view(&value.unknown1);
    let _ = view(&value.unknown2);
};

impl<T: acadrust::tables::TableEntry + NativeView> NativeView for acadrust::tables::Table<T> {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_SEQUENCE
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        self.iter()
            .nth(index)
            .map(|entry| (StringView::new(entry.name()), view(entry)))
    }
    fn number(&self) -> Option<i64> {
        Some(self.handle().value() as i64)
    }
}

impl NativeView for acadrust::classes::DxfClassCollection {
    fn kind(&self) -> i32 {
        crate::views::AR_VALUE_SEQUENCE
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        self.iter()
            .nth(index)
            .map(|class| (StringView::new(&class.dxf_name), view(class)))
    }
}

macro_rules! flag_view {
    ($($ty:path),* $(,)?) => { $(
        impl NativeView for $ty {
            fn kind(&self) -> i32 { crate::views::AR_VALUE_ENUM }
            fn number(&self) -> Option<i64> { Some(self.bits() as i64) }
        }
    )* };
}

flag_view!(
    acadrust::objects::MultiLeaderPropertyOverrideFlags,
    acadrust::objects::LeaderLinePropertyOverrideFlags,
    acadrust::objects::TableCellStylePropertyFlags,
    acadrust::objects::TableBorderPropertyFlags,
    acadrust::objects::TableStyleFlags,
);

#[cfg(test)]
mod tests {
    use super::*;
    use acadrust::objects::SortEntitiesTable;
    use acadrust::Handle;

    #[test]
    fn sort_entries_adapter_borrows_the_original_records() {
        let mut table = SortEntitiesTable::for_block(Handle::new(30));
        table.add_entry(Handle::new(40), Handle::new(50));
        let expected = std::ptr::from_ref(table.entries().next().unwrap()).cast();
        let (_, entries) = NativeView::child(&table, 5).unwrap();
        assert_eq!(entries.data, std::ptr::from_ref(&table).cast());
        let entries = unsafe { &*entries.data.cast::<SortEntries>() };
        assert_eq!(NativeView::len(entries), 1);
        assert_eq!(NativeView::child(entries, 0).unwrap().1.data, expected);
    }

    #[test]
    fn classes_collection_borrows_duplicate_declarations_in_order() {
        let mut classes = acadrust::classes::DxfClassCollection::new();
        classes.push_preserving(acadrust::classes::DxfClass::new("CUSTOM", "First"));
        classes.push_preserving(acadrust::classes::DxfClass::new("CUSTOM", "Second"));
        assert_eq!(NativeView::len(&classes), 2);
        for (index, class) in classes.iter().enumerate() {
            assert_eq!(
                NativeView::child(&classes, index).unwrap().1.data,
                std::ptr::from_ref(class).cast()
            );
        }
    }
}
