// Generated with tools/generate-views from acadrust 0.6.5.
use crate::views::{view, view_enum, view_struct};
view_struct!(acadrust::document::SolidHistoryGraph {
    root,
    nodes,
    evaluation_graph
});
const _: fn(&acadrust::document::SolidHistoryGraph) = |v| {
    let _ = view(&v.root);
    let _ = view(&v.nodes);
    let _ = view(&v.evaluation_graph);
};
view_struct!(acadrust::document::HeaderVariables {
    legacy,
    required_versions,
    associate_dimensions,
    update_dimensions_while_dragging,
    ortho_mode,
    fill_mode,
    quick_text_mode,
    mirror_text,
    regen_mode,
    limit_check,
    paper_space_limit_check,
    polyline_linetype_generation,
    paper_space_linetype_scaling,
    show_model_space,
    user_timer,
    sketch_type,
    world_view,
    retain_xref_visibility,
    display_silhouette,
    spline_frame,
    delete_objects,
    record_solid_history,
    show_solid_history,
    drag_mode,
    blip_mode,
    attribute_request,
    attribute_dialog,
    linear_unit_format,
    linear_unit_precision,
    angular_unit_format,
    angular_unit_precision,
    insertion_units,
    attribute_visibility,
    point_display_mode,
    user_int1,
    user_int2,
    user_int3,
    user_int4,
    user_int5,
    coords_mode,
    object_snap_mode,
    pick_style,
    spline_type,
    spline_segments,
    spline_segs_surface,
    surface_u_density,
    surface_v_density,
    surface_type,
    surface_tab1,
    surface_tab2,
    shade_edge,
    shade_diffuse,
    max_active_viewports,
    isolines,
    multiline_justification,
    text_quality,
    sort_entities,
    index_control,
    hide_text,
    xclip_frame,
    dwf_frame,
    dgn_frame,
    halo_gap,
    obscured_color,
    obscured_linetype,
    intersection_display,
    intersection_color,
    interference_color,
    real_world_scale,
    light_glyph_display,
    tile_model_light_synch,
    polysolid_width,
    polysolid_height,
    dwf_3d_precision,
    dim_fixed_ext_line_length,
    dim_jog_angle,
    dim_text_fill,
    dim_text_fill_color,
    dimension_associativity,
    project_name,
    linetype_scale,
    text_height,
    trace_width,
    sketch_increment,
    sketch_tolerance,
    thickness,
    point_display_size,
    polyline_width,
    current_entity_linetype_scale,
    view_twist,
    fillet_radius,
    chamfer_distance_a,
    chamfer_distance_b,
    chamfer_length,
    chamfer_angle,
    angle_base,
    angle_direction,
    elevation,
    paper_elevation,
    facet_resolution,
    multiline_scale,
    user_real1,
    user_real2,
    user_real3,
    user_real4,
    user_real5,
    viewport_scale_factor,
    current_annotation_scale,
    annotation_scale_value,
    shadow_plane_location,
    loft_angle1,
    loft_angle2,
    loft_magnitude1,
    loft_magnitude2,
    loft_param,
    loft_normals,
    latitude,
    longitude,
    north_direction,
    timezone,
    steps_per_second,
    step_size,
    lens_length,
    camera_height,
    camera_display,
    current_entity_color,
    current_line_weight,
    current_plotstyle_type,
    end_caps,
    join_style,
    lineweight_display,
    xedit,
    extended_names,
    plotstyle_mode,
    ole_startup,
    dim_scale,
    dim_arrow_size,
    dim_ext_line_offset,
    dim_line_increment,
    dim_ext_line_extension,
    dim_rounding,
    dim_line_extension,
    dim_tolerance_plus,
    dim_tolerance_minus,
    dim_text_height,
    dim_center_mark,
    dim_tick_size,
    dim_alt_scale,
    dim_linear_scale,
    dim_text_vertical_pos,
    dim_tolerance_scale,
    dim_line_gap,
    dim_alt_rounding,
    dim_tolerance,
    dim_limits,
    dim_text_inside_horizontal,
    dim_text_outside_horizontal,
    dim_suppress_ext1,
    dim_suppress_ext2,
    dim_text_above,
    dim_zero_suppression,
    dim_alt_zero_suppression,
    dim_alternate_units,
    dim_alt_decimal_places,
    dim_force_line_inside,
    dim_separate_arrows,
    dim_force_text_inside,
    dim_suppress_outside_ext,
    dim_line_color,
    dim_ext_line_color,
    dim_text_color,
    dim_angular_decimal_places,
    dim_decimal_places,
    dim_tolerance_decimal_places,
    dim_alt_units_format,
    dim_alt_tolerance_decimal_places,
    dim_angular_units,
    dim_fraction_format,
    dim_linear_unit_format,
    dim_decimal_separator,
    dim_text_movement,
    dim_horizontal_justification,
    dim_suppress_line1,
    dim_suppress_line2,
    dim_tolerance_justification,
    dim_tolerance_zero_suppression,
    dim_alt_tolerance_zero_suppression,
    dim_alt_tolerance_zero_tight,
    dim_fit,
    dim_user_positioned_text,
    dim_post,
    dim_alt_post,
    dim_arrow_block,
    dim_arrow_block1,
    dim_arrow_block2,
    dim_leader_arrow_block,
    model_space_insertion_base,
    model_space_extents_min,
    model_space_extents_max,
    model_space_limits_min,
    model_space_limits_max,
    paper_space_insertion_base,
    paper_space_extents_min,
    paper_space_extents_max,
    paper_space_limits_min,
    paper_space_limits_max,
    ucs_base,
    model_space_ucs_name,
    paper_space_ucs_name,
    model_space_ucs_origin,
    model_space_ucs_x_axis,
    model_space_ucs_y_axis,
    paper_space_ucs_origin,
    paper_space_ucs_x_axis,
    paper_space_ucs_y_axis,
    ucs_ortho_ref,
    ucs_ortho_view,
    paper_ucs_ortho_ref,
    paper_ucs_ortho_view,
    handle_seed,
    current_layer_handle,
    current_text_style_handle,
    current_linetype_handle,
    current_dimstyle_handle,
    current_multiline_style_handle,
    current_material_handle,
    dim_text_style_handle,
    dim_linetype_handle,
    dim_linetype1_handle,
    dim_linetype2_handle,
    dim_arrow_block_handle,
    dim_arrow_block1_handle,
    dim_arrow_block2_handle,
    dim_line_weight,
    dim_ext_line_weight,
    block_control_handle,
    layer_control_handle,
    style_control_handle,
    linetype_control_handle,
    view_control_handle,
    ucs_control_handle,
    vport_control_handle,
    appid_control_handle,
    dimstyle_control_handle,
    vpent_hdr_control_handle,
    current_vx_handle,
    named_objects_dict_handle,
    acad_group_dict_handle,
    acad_mlinestyle_dict_handle,
    acad_layout_dict_handle,
    acad_plotsettings_dict_handle,
    acad_plotstylename_dict_handle,
    acad_material_dict_handle,
    acad_color_dict_handle,
    acad_visualstyle_dict_handle,
    model_space_block_handle,
    paper_space_block_handle,
    bylayer_linetype_handle,
    byblock_linetype_handle,
    continuous_linetype_handle,
    create_date_julian,
    update_date_julian,
    universal_create_date_julian,
    universal_update_date_julian,
    total_editing_time,
    user_elapsed_time,
    fingerprint_guid,
    version_guid,
    menu_name,
    code_page,
    last_saved_by,
    hyperlink_base,
    stylesheet,
    measurement,
    proxy_graphics,
    tree_depth,
    multiline_style,
    current_linetype_name,
    current_layer_name,
    current_text_style_name,
    current_dimstyle_name,
    current_table_style_name,
    current_mleader_style_name
});
const _: fn(&acadrust::document::HeaderVariables) = |v| {
    let _ = view(&v.legacy);
    let _ = view(&v.required_versions);
    let _ = view(&v.associate_dimensions);
    let _ = view(&v.update_dimensions_while_dragging);
    let _ = view(&v.ortho_mode);
    let _ = view(&v.fill_mode);
    let _ = view(&v.quick_text_mode);
    let _ = view(&v.mirror_text);
    let _ = view(&v.regen_mode);
    let _ = view(&v.limit_check);
    let _ = view(&v.paper_space_limit_check);
    let _ = view(&v.polyline_linetype_generation);
    let _ = view(&v.paper_space_linetype_scaling);
    let _ = view(&v.show_model_space);
    let _ = view(&v.user_timer);
    let _ = view(&v.sketch_type);
    let _ = view(&v.world_view);
    let _ = view(&v.retain_xref_visibility);
    let _ = view(&v.display_silhouette);
    let _ = view(&v.spline_frame);
    let _ = view(&v.delete_objects);
    let _ = view(&v.record_solid_history);
    let _ = view(&v.show_solid_history);
    let _ = view(&v.drag_mode);
    let _ = view(&v.blip_mode);
    let _ = view(&v.attribute_request);
    let _ = view(&v.attribute_dialog);
    let _ = view(&v.linear_unit_format);
    let _ = view(&v.linear_unit_precision);
    let _ = view(&v.angular_unit_format);
    let _ = view(&v.angular_unit_precision);
    let _ = view(&v.insertion_units);
    let _ = view(&v.attribute_visibility);
    let _ = view(&v.point_display_mode);
    let _ = view(&v.user_int1);
    let _ = view(&v.user_int2);
    let _ = view(&v.user_int3);
    let _ = view(&v.user_int4);
    let _ = view(&v.user_int5);
    let _ = view(&v.coords_mode);
    let _ = view(&v.object_snap_mode);
    let _ = view(&v.pick_style);
    let _ = view(&v.spline_type);
    let _ = view(&v.spline_segments);
    let _ = view(&v.spline_segs_surface);
    let _ = view(&v.surface_u_density);
    let _ = view(&v.surface_v_density);
    let _ = view(&v.surface_type);
    let _ = view(&v.surface_tab1);
    let _ = view(&v.surface_tab2);
    let _ = view(&v.shade_edge);
    let _ = view(&v.shade_diffuse);
    let _ = view(&v.max_active_viewports);
    let _ = view(&v.isolines);
    let _ = view(&v.multiline_justification);
    let _ = view(&v.text_quality);
    let _ = view(&v.sort_entities);
    let _ = view(&v.index_control);
    let _ = view(&v.hide_text);
    let _ = view(&v.xclip_frame);
    let _ = view(&v.dwf_frame);
    let _ = view(&v.dgn_frame);
    let _ = view(&v.halo_gap);
    let _ = view(&v.obscured_color);
    let _ = view(&v.obscured_linetype);
    let _ = view(&v.intersection_display);
    let _ = view(&v.intersection_color);
    let _ = view(&v.interference_color);
    let _ = view(&v.real_world_scale);
    let _ = view(&v.light_glyph_display);
    let _ = view(&v.tile_model_light_synch);
    let _ = view(&v.polysolid_width);
    let _ = view(&v.polysolid_height);
    let _ = view(&v.dwf_3d_precision);
    let _ = view(&v.dim_fixed_ext_line_length);
    let _ = view(&v.dim_jog_angle);
    let _ = view(&v.dim_text_fill);
    let _ = view(&v.dim_text_fill_color);
    let _ = view(&v.dimension_associativity);
    let _ = view(&v.project_name);
    let _ = view(&v.linetype_scale);
    let _ = view(&v.text_height);
    let _ = view(&v.trace_width);
    let _ = view(&v.sketch_increment);
    let _ = view(&v.sketch_tolerance);
    let _ = view(&v.thickness);
    let _ = view(&v.point_display_size);
    let _ = view(&v.polyline_width);
    let _ = view(&v.current_entity_linetype_scale);
    let _ = view(&v.view_twist);
    let _ = view(&v.fillet_radius);
    let _ = view(&v.chamfer_distance_a);
    let _ = view(&v.chamfer_distance_b);
    let _ = view(&v.chamfer_length);
    let _ = view(&v.chamfer_angle);
    let _ = view(&v.angle_base);
    let _ = view(&v.angle_direction);
    let _ = view(&v.elevation);
    let _ = view(&v.paper_elevation);
    let _ = view(&v.facet_resolution);
    let _ = view(&v.multiline_scale);
    let _ = view(&v.user_real1);
    let _ = view(&v.user_real2);
    let _ = view(&v.user_real3);
    let _ = view(&v.user_real4);
    let _ = view(&v.user_real5);
    let _ = view(&v.viewport_scale_factor);
    let _ = view(&v.current_annotation_scale);
    let _ = view(&v.annotation_scale_value);
    let _ = view(&v.shadow_plane_location);
    let _ = view(&v.loft_angle1);
    let _ = view(&v.loft_angle2);
    let _ = view(&v.loft_magnitude1);
    let _ = view(&v.loft_magnitude2);
    let _ = view(&v.loft_param);
    let _ = view(&v.loft_normals);
    let _ = view(&v.latitude);
    let _ = view(&v.longitude);
    let _ = view(&v.north_direction);
    let _ = view(&v.timezone);
    let _ = view(&v.steps_per_second);
    let _ = view(&v.step_size);
    let _ = view(&v.lens_length);
    let _ = view(&v.camera_height);
    let _ = view(&v.camera_display);
    let _ = view(&v.current_entity_color);
    let _ = view(&v.current_line_weight);
    let _ = view(&v.current_plotstyle_type);
    let _ = view(&v.end_caps);
    let _ = view(&v.join_style);
    let _ = view(&v.lineweight_display);
    let _ = view(&v.xedit);
    let _ = view(&v.extended_names);
    let _ = view(&v.plotstyle_mode);
    let _ = view(&v.ole_startup);
    let _ = view(&v.dim_scale);
    let _ = view(&v.dim_arrow_size);
    let _ = view(&v.dim_ext_line_offset);
    let _ = view(&v.dim_line_increment);
    let _ = view(&v.dim_ext_line_extension);
    let _ = view(&v.dim_rounding);
    let _ = view(&v.dim_line_extension);
    let _ = view(&v.dim_tolerance_plus);
    let _ = view(&v.dim_tolerance_minus);
    let _ = view(&v.dim_text_height);
    let _ = view(&v.dim_center_mark);
    let _ = view(&v.dim_tick_size);
    let _ = view(&v.dim_alt_scale);
    let _ = view(&v.dim_linear_scale);
    let _ = view(&v.dim_text_vertical_pos);
    let _ = view(&v.dim_tolerance_scale);
    let _ = view(&v.dim_line_gap);
    let _ = view(&v.dim_alt_rounding);
    let _ = view(&v.dim_tolerance);
    let _ = view(&v.dim_limits);
    let _ = view(&v.dim_text_inside_horizontal);
    let _ = view(&v.dim_text_outside_horizontal);
    let _ = view(&v.dim_suppress_ext1);
    let _ = view(&v.dim_suppress_ext2);
    let _ = view(&v.dim_text_above);
    let _ = view(&v.dim_zero_suppression);
    let _ = view(&v.dim_alt_zero_suppression);
    let _ = view(&v.dim_alternate_units);
    let _ = view(&v.dim_alt_decimal_places);
    let _ = view(&v.dim_force_line_inside);
    let _ = view(&v.dim_separate_arrows);
    let _ = view(&v.dim_force_text_inside);
    let _ = view(&v.dim_suppress_outside_ext);
    let _ = view(&v.dim_line_color);
    let _ = view(&v.dim_ext_line_color);
    let _ = view(&v.dim_text_color);
    let _ = view(&v.dim_angular_decimal_places);
    let _ = view(&v.dim_decimal_places);
    let _ = view(&v.dim_tolerance_decimal_places);
    let _ = view(&v.dim_alt_units_format);
    let _ = view(&v.dim_alt_tolerance_decimal_places);
    let _ = view(&v.dim_angular_units);
    let _ = view(&v.dim_fraction_format);
    let _ = view(&v.dim_linear_unit_format);
    let _ = view(&v.dim_decimal_separator);
    let _ = view(&v.dim_text_movement);
    let _ = view(&v.dim_horizontal_justification);
    let _ = view(&v.dim_suppress_line1);
    let _ = view(&v.dim_suppress_line2);
    let _ = view(&v.dim_tolerance_justification);
    let _ = view(&v.dim_tolerance_zero_suppression);
    let _ = view(&v.dim_alt_tolerance_zero_suppression);
    let _ = view(&v.dim_alt_tolerance_zero_tight);
    let _ = view(&v.dim_fit);
    let _ = view(&v.dim_user_positioned_text);
    let _ = view(&v.dim_post);
    let _ = view(&v.dim_alt_post);
    let _ = view(&v.dim_arrow_block);
    let _ = view(&v.dim_arrow_block1);
    let _ = view(&v.dim_arrow_block2);
    let _ = view(&v.dim_leader_arrow_block);
    let _ = view(&v.model_space_insertion_base);
    let _ = view(&v.model_space_extents_min);
    let _ = view(&v.model_space_extents_max);
    let _ = view(&v.model_space_limits_min);
    let _ = view(&v.model_space_limits_max);
    let _ = view(&v.paper_space_insertion_base);
    let _ = view(&v.paper_space_extents_min);
    let _ = view(&v.paper_space_extents_max);
    let _ = view(&v.paper_space_limits_min);
    let _ = view(&v.paper_space_limits_max);
    let _ = view(&v.ucs_base);
    let _ = view(&v.model_space_ucs_name);
    let _ = view(&v.paper_space_ucs_name);
    let _ = view(&v.model_space_ucs_origin);
    let _ = view(&v.model_space_ucs_x_axis);
    let _ = view(&v.model_space_ucs_y_axis);
    let _ = view(&v.paper_space_ucs_origin);
    let _ = view(&v.paper_space_ucs_x_axis);
    let _ = view(&v.paper_space_ucs_y_axis);
    let _ = view(&v.ucs_ortho_ref);
    let _ = view(&v.ucs_ortho_view);
    let _ = view(&v.paper_ucs_ortho_ref);
    let _ = view(&v.paper_ucs_ortho_view);
    let _ = view(&v.handle_seed);
    let _ = view(&v.current_layer_handle);
    let _ = view(&v.current_text_style_handle);
    let _ = view(&v.current_linetype_handle);
    let _ = view(&v.current_dimstyle_handle);
    let _ = view(&v.current_multiline_style_handle);
    let _ = view(&v.current_material_handle);
    let _ = view(&v.dim_text_style_handle);
    let _ = view(&v.dim_linetype_handle);
    let _ = view(&v.dim_linetype1_handle);
    let _ = view(&v.dim_linetype2_handle);
    let _ = view(&v.dim_arrow_block_handle);
    let _ = view(&v.dim_arrow_block1_handle);
    let _ = view(&v.dim_arrow_block2_handle);
    let _ = view(&v.dim_line_weight);
    let _ = view(&v.dim_ext_line_weight);
    let _ = view(&v.block_control_handle);
    let _ = view(&v.layer_control_handle);
    let _ = view(&v.style_control_handle);
    let _ = view(&v.linetype_control_handle);
    let _ = view(&v.view_control_handle);
    let _ = view(&v.ucs_control_handle);
    let _ = view(&v.vport_control_handle);
    let _ = view(&v.appid_control_handle);
    let _ = view(&v.dimstyle_control_handle);
    let _ = view(&v.vpent_hdr_control_handle);
    let _ = view(&v.current_vx_handle);
    let _ = view(&v.named_objects_dict_handle);
    let _ = view(&v.acad_group_dict_handle);
    let _ = view(&v.acad_mlinestyle_dict_handle);
    let _ = view(&v.acad_layout_dict_handle);
    let _ = view(&v.acad_plotsettings_dict_handle);
    let _ = view(&v.acad_plotstylename_dict_handle);
    let _ = view(&v.acad_material_dict_handle);
    let _ = view(&v.acad_color_dict_handle);
    let _ = view(&v.acad_visualstyle_dict_handle);
    let _ = view(&v.model_space_block_handle);
    let _ = view(&v.paper_space_block_handle);
    let _ = view(&v.bylayer_linetype_handle);
    let _ = view(&v.byblock_linetype_handle);
    let _ = view(&v.continuous_linetype_handle);
    let _ = view(&v.create_date_julian);
    let _ = view(&v.update_date_julian);
    let _ = view(&v.universal_create_date_julian);
    let _ = view(&v.universal_update_date_julian);
    let _ = view(&v.total_editing_time);
    let _ = view(&v.user_elapsed_time);
    let _ = view(&v.fingerprint_guid);
    let _ = view(&v.version_guid);
    let _ = view(&v.menu_name);
    let _ = view(&v.code_page);
    let _ = view(&v.last_saved_by);
    let _ = view(&v.hyperlink_base);
    let _ = view(&v.stylesheet);
    let _ = view(&v.measurement);
    let _ = view(&v.proxy_graphics);
    let _ = view(&v.tree_depth);
    let _ = view(&v.multiline_style);
    let _ = view(&v.current_linetype_name);
    let _ = view(&v.current_layer_name);
    let _ = view(&v.current_text_style_name);
    let _ = view(&v.current_dimstyle_name);
    let _ = view(&v.current_table_style_name);
    let _ = view(&v.current_mleader_style_name);
};
view_enum!(acadrust::document::PreviewFormat { Bmp, Wmf, Png });
view_struct!(acadrust::document::Preview { format, data });
const _: fn(&acadrust::document::Preview) = |v| {
    let _ = view(&v.format);
    let _ = view(&v.data);
};
view_struct!(acadrust::document::FieldDef {
    handle,
    owner,
    evaluator,
    code,
    objects
});
const _: fn(&acadrust::document::FieldDef) = |v| {
    let _ = view(&v.handle);
    let _ = view(&v.owner);
    let _ = view(&v.evaluator);
    let _ = view(&v.code);
    let _ = view(&v.objects);
};
view_struct!(acadrust::document::SummaryInfo {
    title,
    subject,
    author,
    keywords,
    comments,
    last_saved_by,
    revision_number,
    hyperlink_base,
    custom_properties
});
const _: fn(&acadrust::document::SummaryInfo) = |v| {
    let _ = view(&v.title);
    let _ = view(&v.subject);
    let _ = view(&v.author);
    let _ = view(&v.keywords);
    let _ = view(&v.comments);
    let _ = view(&v.last_saved_by);
    let _ = view(&v.revision_number);
    let _ = view(&v.hyperlink_base);
    let _ = view(&v.custom_properties);
};
view_struct!(acadrust::document::CadDocument {
    version,
    maintenance_version,
    header,
    layers,
    line_types,
    text_styles,
    block_records,
    dim_styles,
    app_ids,
    views,
    vports,
    ucss,
    vx_table,
    vx_control_entries,
    classes,
    notifications,
    objects,
    block_visibility_params,
    context_scales,
    block_representations,
    fields,
    summary_info,
    source_path,
    dgn_ls_definitions,
    dgn_ls_components,
    object_xdata,
    dwg_source_version,
    preview,
    section_view_style,
    view_rep_refs,
    section_view_reps
});
const _: fn(&acadrust::document::CadDocument) = |v| {
    let _ = view(&v.version);
    let _ = view(&v.maintenance_version);
    let _ = view(&v.header);
    let _ = view(&v.layers);
    let _ = view(&v.line_types);
    let _ = view(&v.text_styles);
    let _ = view(&v.block_records);
    let _ = view(&v.dim_styles);
    let _ = view(&v.app_ids);
    let _ = view(&v.views);
    let _ = view(&v.vports);
    let _ = view(&v.ucss);
    let _ = view(&v.vx_table);
    let _ = view(&v.vx_control_entries);
    let _ = view(&v.classes);
    let _ = view(&v.notifications);
    let _ = view(&v.objects);
    let _ = view(&v.block_visibility_params);
    let _ = view(&v.context_scales);
    let _ = view(&v.block_representations);
    let _ = view(&v.fields);
    let _ = view(&v.summary_info);
    let _ = view(&v.source_path);
    let _ = view(&v.dgn_ls_definitions);
    let _ = view(&v.dgn_ls_components);
    let _ = view(&v.object_xdata);
    let _ = view(&v.dwg_source_version);
    let _ = view(&v.preview);
    let _ = view(&v.section_view_style);
    let _ = view(&v.view_rep_refs);
    let _ = view(&v.section_view_reps);
};
view_struct!(acadrust::document::LegacyHeaderState {
    view_center,
    view_height,
    view_aspect_ratio,
    view_direction,
    view_target,
    snap_mode,
    snap_spacing,
    snap_base,
    snap_angle,
    snap_style,
    snap_isopair,
    grid_mode,
    grid_spacing,
    axis_mode,
    axis_spacing,
    circle_zoom,
    fast_zoom,
    front_clip,
    back_clip,
    view_mode,
    flatland,
    ucs_icon,
    unit_mode,
    user_unit_ratios,
    user_unit_names
});
const _: fn(&acadrust::document::LegacyHeaderState) = |v| {
    let _ = view(&v.view_center);
    let _ = view(&v.view_height);
    let _ = view(&v.view_aspect_ratio);
    let _ = view(&v.view_direction);
    let _ = view(&v.view_target);
    let _ = view(&v.snap_mode);
    let _ = view(&v.snap_spacing);
    let _ = view(&v.snap_base);
    let _ = view(&v.snap_angle);
    let _ = view(&v.snap_style);
    let _ = view(&v.snap_isopair);
    let _ = view(&v.grid_mode);
    let _ = view(&v.grid_spacing);
    let _ = view(&v.axis_mode);
    let _ = view(&v.axis_spacing);
    let _ = view(&v.circle_zoom);
    let _ = view(&v.fast_zoom);
    let _ = view(&v.front_clip);
    let _ = view(&v.back_clip);
    let _ = view(&v.view_mode);
    let _ = view(&v.flatland);
    let _ = view(&v.ucs_icon);
    let _ = view(&v.unit_mode);
    let _ = view(&v.user_unit_ratios);
    let _ = view(&v.user_unit_names);
};
view_enum!(acadrust::audit::AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
});
view_enum!(acadrust::audit::AuditIssueCode {
    NullHandle,
    DuplicateHandle,
    StorageKeyMismatch,
    EntityIndexMismatch,
    ObjectMapMismatch,
    DanglingReference,
    OwnerMismatch,
    OwnerCycle,
    InvalidGeometry,
    InvalidHandleSeed,
    StaleTableKey,
    UnsupportedRecord,
});
view_enum!(acadrust::audit::AuditLocation {
    Document,
    TableEntry { table, name, handle },
    Entity { handle },
    Object { handle },
    Reference { owner, target },
});
view_struct!(acadrust::audit::AuditIssue {
    code,
    severity,
    location,
    handle,
    message,
    repairable
});
const _: fn(&acadrust::audit::AuditIssue) = |v| {
    let _ = view(&v.code);
    let _ = view(&v.severity);
    let _ = view(&v.location);
    let _ = view(&v.handle);
    let _ = view(&v.message);
    let _ = view(&v.repairable);
};
view_struct!(acadrust::audit::AuditReport {
    scanned_records,
    scanned_entities,
    scanned_objects,
    scanned_table_entries,
    issues
});
const _: fn(&acadrust::audit::AuditReport) = |v| {
    let _ = view(&v.scanned_records);
    let _ = view(&v.scanned_entities);
    let _ = view(&v.scanned_objects);
    let _ = view(&v.scanned_table_entries);
    let _ = view(&v.issues);
};
view_enum!(acadrust::audit::RepairAction {
    ResyncTableKeys { count },
    AssignTableHandles { count },
    AssignEntityHandles { count },
    ReindexEntities { count },
    ReindexObject { old, new },
    RepairBlockOwners { count },
    SynchronizeHandleAllocator,
});
view_struct!(acadrust::audit::RepairReport {
    before,
    after,
    actions,
    unresolved
});
const _: fn(&acadrust::audit::RepairReport) = |v| {
    let _ = view(&v.before);
    let _ = view(&v.after);
    let _ = view(&v.actions);
    let _ = view(&v.unresolved);
};
view_enum!(acadrust::notification::NotificationType {
    NotImplemented,
    NotSupported,
    Warning,
    Error,
});
view_struct!(acadrust::notification::Notification {
    notification_type,
    message
});
const _: fn(&acadrust::notification::Notification) = |v| {
    let _ = view(&v.notification_type);
    let _ = view(&v.message);
};
view_struct!(acadrust::layer_state::LayerStateLayer {
    layer_handle,
    layer_name,
    off,
    frozen,
    locked,
    plottable,
    new_viewport_frozen,
    color,
    color_name,
    book_name,
    line_type,
    line_weight,
    plot_style,
    transparency
});
const _: fn(&acadrust::layer_state::LayerStateLayer) = |v| {
    let _ = view(&v.layer_handle);
    let _ = view(&v.layer_name);
    let _ = view(&v.off);
    let _ = view(&v.frozen);
    let _ = view(&v.locked);
    let _ = view(&v.plottable);
    let _ = view(&v.new_viewport_frozen);
    let _ = view(&v.color);
    let _ = view(&v.color_name);
    let _ = view(&v.book_name);
    let _ = view(&v.line_type);
    let _ = view(&v.line_weight);
    let _ = view(&v.plot_style);
    let _ = view(&v.transparency);
};
view_struct!(acadrust::layer_state::LayerState {
    name,
    description,
    mask,
    hidden,
    current_layer,
    layers
});
const _: fn(&acadrust::layer_state::LayerState) = |v| {
    let _ = view(&v.name);
    let _ = view(&v.description);
    let _ = view(&v.mask);
    let _ = view(&v.hidden);
    let _ = view(&v.current_layer);
    let _ = view(&v.layers);
};
view_struct!(acadrust::compound_file::StructuredStoragePayload {
    leading_records,
    compound_file,
    trailing_records
});
const _: fn(&acadrust::compound_file::StructuredStoragePayload) = |v| {
    let _ = view(&v.leading_records);
    let _ = view(&v.compound_file);
    let _ = view(&v.trailing_records);
};
view_struct!(acadrust::compound_file::BinaryRecord { sequence, data });
const _: fn(&acadrust::compound_file::BinaryRecord) = |v| {
    let _ = view(&v.sequence);
    let _ = view(&v.data);
};
view_struct!(acadrust::compound_file::CompoundFile {
    minor_version,
    major_version,
    transaction_signature,
    root
});
const _: fn(&acadrust::compound_file::CompoundFile) = |v| {
    let _ = view(&v.minor_version);
    let _ = view(&v.major_version);
    let _ = view(&v.transaction_signature);
    let _ = view(&v.root);
};
view_struct!(acadrust::compound_file::CompoundStorage {
    name,
    class_id,
    state_bits,
    creation_time,
    modified_time,
    entries
});
const _: fn(&acadrust::compound_file::CompoundStorage) = |v| {
    let _ = view(&v.name);
    let _ = view(&v.class_id);
    let _ = view(&v.state_bits);
    let _ = view(&v.creation_time);
    let _ = view(&v.modified_time);
    let _ = view(&v.entries);
};
view_enum!(acadrust::compound_file::CompoundEntry {
    Storage(v0),
    Stream(v0),
});
view_struct!(acadrust::compound_file::CompoundStream {
    name,
    class_id,
    state_bits,
    creation_time,
    modified_time,
    content
});
const _: fn(&acadrust::compound_file::CompoundStream) = |v| {
    let _ = view(&v.name);
    let _ = view(&v.class_id);
    let _ = view(&v.state_bits);
    let _ = view(&v.creation_time);
    let _ = view(&v.modified_time);
    let _ = view(&v.content);
};
view_enum!(acadrust::compound_file::CompoundStreamContent {
    PropertySet(v0),
    VbaDirectory(v0),
    VbaModule(v0),
    Binary(v0),
});
view_struct!(acadrust::compound_file::CompoundPropertySetStream {
    version,
    system_identifier,
    class_id,
    sets
});
const _: fn(&acadrust::compound_file::CompoundPropertySetStream) = |v| {
    let _ = view(&v.version);
    let _ = view(&v.system_identifier);
    let _ = view(&v.class_id);
    let _ = view(&v.sets);
};
view_struct!(acadrust::compound_file::CompoundPropertySet {
    format_id,
    properties
});
const _: fn(&acadrust::compound_file::CompoundPropertySet) = |v| {
    let _ = view(&v.format_id);
    let _ = view(&v.properties);
};
view_struct!(acadrust::compound_file::CompoundProperty { id, value });
const _: fn(&acadrust::compound_file::CompoundProperty) = |v| {
    let _ = view(&v.id);
    let _ = view(&v.value);
};
view_enum!(acadrust::compound_file::CompoundPropertyValue {
    Empty,
    Null,
    I16(v0),
    I32(v0),
    I64(v0),
    U16(v0),
    U32(v0),
    U64(v0),
    F32(v0),
    F64(v0),
    Bool(v0),
    AnsiString { value, code_page, original_value, original_code_page, encoded },
    UnicodeString(v0),
    FileTime(v0),
    ClassId(v0),
    Blob(v0),
    Clipboard { format, data },
    Dictionary(v0),
    Vector { element_type, values },
    Unsupported { variant_type, records },
});
view_struct!(acadrust::compound_file::CompoundDictionaryEntry { property_id, name });
const _: fn(&acadrust::compound_file::CompoundDictionaryEntry) = |v| {
    let _ = view(&v.property_id);
    let _ = view(&v.name);
};
view_struct!(acadrust::document::DynamicBlockProperty {
    parameter_handle,
    name,
    description,
    type_code,
    units,
    show,
    read_only,
    visible,
    value,
    allowed_values
});
const _: fn(&acadrust::document::DynamicBlockProperty) = |v| {
    let _ = view(&v.parameter_handle);
    let _ = view(&v.name);
    let _ = view(&v.description);
    let _ = view(&v.type_code);
    let _ = view(&v.units);
    let _ = view(&v.show);
    let _ = view(&v.read_only);
    let _ = view(&v.visible);
    let _ = view(&v.value);
    let _ = view(&v.allowed_values);
};
view_struct!(acadrust::vba::VbaDirectoryStream { code_page, records });
const _: fn(&acadrust::vba::VbaDirectoryStream) = |v| {
    let _ = view(&v.code_page);
    let _ = view(&v.records);
};
view_struct!(acadrust::vba::VbaDirectoryRecord { id, value });
const _: fn(&acadrust::vba::VbaDirectoryRecord) = |v| {
    let _ = view(&v.id);
    let _ = view(&v.value);
};
view_enum!(acadrust::vba::VbaDirectoryValue {
    Marker,
    U16(v0),
    U32(v0),
    ProjectVersion { major, minor },
    AnsiString { value, original_value, original_code_page, encoded },
    UnicodeString { value, original_value, encoded },
    Binary(v0),
});
view_struct!(acadrust::vba::VbaModuleDescriptor {
    name,
    stream_name,
    source_offset,
    code_page
});
const _: fn(&acadrust::vba::VbaModuleDescriptor) = |v| {
    let _ = view(&v.name);
    let _ = view(&v.stream_name);
    let _ = view(&v.source_offset);
    let _ = view(&v.code_page);
};
view_struct!(acadrust::vba::VbaModuleStream {
    source_offset,
    code_page,
    performance_cache,
    source
});
const _: fn(&acadrust::vba::VbaModuleStream) = |v| {
    let _ = view(&v.source_offset);
    let _ = view(&v.code_page);
    let _ = view(&v.performance_cache);
    let _ = view(&v.source);
};
