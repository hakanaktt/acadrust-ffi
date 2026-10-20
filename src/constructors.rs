//! Convenient native constructors for frequently created entity types.

use std::ffi::c_char;
use std::{mem, ptr, slice};

use acadrust::entities::{self as native, AttachmentPoint, DrawingDirection, LineSpacingStyle};
use acadrust::EntityType;

use crate::{
    document_mut, ffi_call, input_str, invalid, not_found, output, Document, FfiResult, Status,
    Vector3,
};

const Z: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
const X: Vector3 = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

/// Input-only record. String pointers are optional, borrowed for one call.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TextOptions {
    pub text: *const c_char,
    pub insertion_point: Vector3,
    pub height: f64,
    pub rotation: f64,
    pub width_factor: f64,
    pub oblique_angle: f64,
    pub normal: Vector3,
    pub thickness: f64,
    pub alignment_point: Vector3,
    pub has_alignment_point: u8,
    pub horizontal_alignment: i32,
    pub vertical_alignment: i32,
    pub generation_flags: i16,
    pub style: *const c_char,
    pub layer: *const c_char,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            text: ptr::null(),
            insertion_point: Vector3::default(),
            height: 1.0,
            rotation: 0.0,
            width_factor: 1.0,
            oblique_angle: 0.0,
            normal: Z,
            thickness: 0.0,
            alignment_point: Vector3::default(),
            has_alignment_point: 0,
            horizontal_alignment: 0,
            vertical_alignment: 0,
            generation_flags: 0,
            style: ptr::null(),
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MTextOptions {
    pub text: *const c_char,
    pub insertion_point: Vector3,
    pub height: f64,
    pub rectangle_width: f64,
    pub rectangle_height: f64,
    pub has_rectangle_height: u8,
    pub rotation: f64,
    pub normal: Vector3,
    pub attachment_point: i32,
    pub drawing_direction: i32,
    pub line_spacing_factor: f64,
    pub line_spacing_style: i32,
    pub style: *const c_char,
    pub layer: *const c_char,
}

impl Default for MTextOptions {
    fn default() -> Self {
        Self {
            text: ptr::null(),
            insertion_point: Vector3::default(),
            height: 1.0,
            rectangle_width: 10.0,
            rectangle_height: 0.0,
            has_rectangle_height: 0,
            rotation: 0.0,
            normal: Z,
            attachment_point: 1,
            drawing_direction: 1,
            line_spacing_factor: 1.0,
            line_spacing_style: 1,
            style: ptr::null(),
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EllipseOptions {
    pub center: Vector3,
    pub major_axis: Vector3,
    pub minor_axis_ratio: f64,
    pub start_parameter: f64,
    pub end_parameter: f64,
    pub normal: Vector3,
    pub layer: *const c_char,
}

impl Default for EllipseOptions {
    fn default() -> Self {
        Self {
            center: Vector3::default(),
            major_axis: X,
            minor_axis_ratio: 0.5,
            start_parameter: 0.0,
            end_parameter: std::f64::consts::TAU,
            normal: Z,
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SplineOptions {
    pub degree: i32,
    pub control_points: *const Vector3,
    pub control_point_count: usize,
    pub knots: *const f64,
    pub knot_count: usize,
    pub weights: *const f64,
    pub weight_count: usize,
    pub fit_points: *const Vector3,
    pub fit_point_count: usize,
    pub closed: u8,
    pub periodic: u8,
    pub rational: u8,
    pub planar: u8,
    pub linear: u8,
    pub normal: Vector3,
    pub begin_tangent: Vector3,
    pub end_tangent: Vector3,
    pub knot_tolerance: f64,
    pub control_tolerance: f64,
    pub fit_tolerance: f64,
    pub knot_parameterization: i32,
    pub cv_frame_visible: u8,
    pub layer: *const c_char,
}

impl Default for SplineOptions {
    fn default() -> Self {
        Self {
            degree: 3,
            control_points: ptr::null(),
            control_point_count: 0,
            knots: ptr::null(),
            knot_count: 0,
            weights: ptr::null(),
            weight_count: 0,
            fit_points: ptr::null(),
            fit_point_count: 0,
            closed: 0,
            periodic: 0,
            rational: 0,
            planar: 0,
            linear: 0,
            normal: Z,
            begin_tangent: Vector3::default(),
            end_tangent: Vector3::default(),
            knot_tolerance: 0.0,
            control_tolerance: 0.0,
            fit_tolerance: 0.0,
            knot_parameterization: 0,
            cv_frame_visible: 0,
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct InsertOptions {
    pub block_name: *const c_char,
    pub insertion_point: Vector3,
    pub scale: Vector3,
    pub rotation: f64,
    pub normal: Vector3,
    pub column_count: u16,
    pub row_count: u16,
    pub column_spacing: f64,
    pub row_spacing: f64,
    pub layer: *const c_char,
}

impl Default for InsertOptions {
    fn default() -> Self {
        Self {
            block_name: ptr::null(),
            insertion_point: Vector3::default(),
            scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            rotation: 0.0,
            normal: Z,
            column_count: 1,
            row_count: 1,
            column_spacing: 0.0,
            row_spacing: 0.0,
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Face3DOptions {
    pub corners: [Vector3; 4],
    pub invisible_edges: u8,
    pub layer: *const c_char,
}

impl Default for Face3DOptions {
    fn default() -> Self {
        Self {
            corners: [Vector3::default(); 4],
            invisible_edges: 0,
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SolidOptions {
    pub corners: [Vector3; 4],
    pub normal: Vector3,
    pub thickness: f64,
    pub is_trace: u8,
    pub layer: *const c_char,
}

impl Default for SolidOptions {
    fn default() -> Self {
        Self {
            corners: [Vector3::default(); 4],
            normal: Z,
            thickness: 0.0,
            is_trace: 0,
            layer: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RayOptions {
    pub base_point: Vector3,
    pub direction: Vector3,
    pub layer: *const c_char,
}

impl Default for RayOptions {
    fn default() -> Self {
        Self {
            base_point: Vector3::default(),
            direction: X,
            layer: ptr::null(),
        }
    }
}

pub type XLineOptions = RayOptions;

fn finite(value: f64, name: &str) -> FfiResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(invalid(format!("{name} must be finite")))
    }
}

fn positive(value: f64, name: &str) -> FfiResult<()> {
    finite(value, name)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(invalid(format!("{name} must be positive")))
    }
}

fn nonnegative(value: f64, name: &str) -> FfiResult<()> {
    finite(value, name)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(invalid(format!("{name} must not be negative")))
    }
}

fn vector(value: Vector3, name: &str) -> FfiResult<()> {
    for coordinate in [value.x, value.y, value.z] {
        finite(coordinate, name)?;
    }
    Ok(())
}

fn nonzero_vector(value: Vector3, name: &str) -> FfiResult<()> {
    vector(value, name)?;
    if value.x == 0.0 && value.y == 0.0 && value.z == 0.0 {
        return Err(invalid(format!("{name} must not be zero")));
    }
    Ok(())
}

fn unit(value: Vector3) -> Vector3 {
    // Scale first so finite extreme inputs cannot overflow or underflow.
    let scale = value.x.abs().max(value.y.abs()).max(value.z.abs());
    let (x, y, z) = (value.x / scale, value.y / scale, value.z / scale);
    let length = x.hypot(y).hypot(z);
    Vector3 {
        x: x / length,
        y: y / length,
        z: z / length,
    }
}

fn boolean(value: u8, name: &str) -> FfiResult<bool> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(invalid(format!("{name} must be 0 or 1"))),
    }
}

unsafe fn options<T: Copy>(value: *const T) -> FfiResult<T> {
    value
        .as_ref()
        .copied()
        .ok_or_else(|| invalid("options pointer is NULL"))
}

unsafe fn string_or(value: *const c_char, default: &str) -> FfiResult<String> {
    if value.is_null() {
        Ok(default.to_owned())
    } else {
        Ok(input_str(value)?.to_owned())
    }
}

unsafe fn input_array<'a, T>(data: *const T, len: usize, name: &str) -> FfiResult<&'a [T]> {
    if len == 0 {
        return Ok(&[]);
    }
    if data.is_null() {
        return Err(invalid(format!(
            "{name} pointer is NULL for nonempty input"
        )));
    }
    if !(data as usize).is_multiple_of(mem::align_of::<T>()) {
        return Err(invalid(format!("{name} pointer is not aligned")));
    }
    if len > isize::MAX as usize / mem::size_of::<T>() {
        return Err(invalid(format!("{name} byte length exceeds isize::MAX")));
    }
    Ok(slice::from_raw_parts(data, len))
}

unsafe fn points(
    data: *const Vector3,
    len: usize,
    name: &str,
) -> FfiResult<Vec<acadrust::Vector3>> {
    input_array(data, len, name)?
        .iter()
        .map(|&point| {
            vector(point, name)?;
            Ok(point.into())
        })
        .collect()
}

unsafe fn add(doc: *mut Document, value: EntityType, out_handle: *mut u64) -> FfiResult<()> {
    let out_handle = output(out_handle)?;
    let doc = document_mut(doc)?;
    crate::entities::validate_entity(doc, &value)?;
    if let EntityType::Insert(insert) = &value {
        if !doc.block_records.contains(&insert.block_name) {
            return Err(not_found(format!(
                "block {:?} was not found",
                insert.block_name
            )));
        }
    }
    *out_handle = doc.add_entity(value)?.value();
    Ok(())
}

macro_rules! defaults {
    ($($name:ident: $record:ty),+ $(,)?) => {$ (
        #[no_mangle]
        pub unsafe extern "C" fn $name(out: *mut $record) -> Status {
            ffi_call(|| { *output(out)? = <$record>::default(); Ok(()) })
        }
    )+};
}

defaults! {
    ar_text_options_default: TextOptions,
    ar_mtext_options_default: MTextOptions,
    ar_ellipse_options_default: EllipseOptions,
    ar_spline_options_default: SplineOptions,
    ar_insert_options_default: InsertOptions,
    ar_face3d_options_default: Face3DOptions,
    ar_solid_options_default: SolidOptions,
    ar_ray_options_default: RayOptions,
    ar_xline_options_default: XLineOptions,
}

#[no_mangle]
pub unsafe extern "C" fn ar_text_add(
    doc: *mut Document,
    input: *const TextOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        vector(o.insertion_point, "insertion point")?;
        positive(o.height, "text height")?;
        positive(o.width_factor, "width factor")?;
        finite(o.rotation, "rotation")?;
        finite(o.oblique_angle, "oblique angle")?;
        finite(o.thickness, "thickness")?;
        nonzero_vector(o.normal, "normal")?;
        let alignment_point = if boolean(o.has_alignment_point, "has_alignment_point")? {
            vector(o.alignment_point, "alignment point")?;
            Some(o.alignment_point.into())
        } else {
            None
        };
        let horizontal_alignment = match o.horizontal_alignment {
            0 => native::TextHorizontalAlignment::Left,
            1 => native::TextHorizontalAlignment::Center,
            2 => native::TextHorizontalAlignment::Right,
            3 => native::TextHorizontalAlignment::Aligned,
            4 => native::TextHorizontalAlignment::Middle,
            5 => native::TextHorizontalAlignment::Fit,
            _ => return Err(invalid("horizontal_alignment must be 0 through 5")),
        };
        let vertical_alignment = match o.vertical_alignment {
            0 => native::TextVerticalAlignment::Baseline,
            1 => native::TextVerticalAlignment::Bottom,
            2 => native::TextVerticalAlignment::Middle,
            3 => native::TextVerticalAlignment::Top,
            _ => return Err(invalid("vertical_alignment must be 0 through 3")),
        };
        if o.generation_flags & !6 != 0 {
            return Err(invalid(
                "text generation_flags may only contain bits 2 and 4",
            ));
        }
        let mut value = native::Text {
            value: string_or(o.text, "")?,
            insertion_point: o.insertion_point.into(),
            alignment_point,
            height: o.height,
            rotation: o.rotation,
            width_factor: o.width_factor,
            oblique_angle: o.oblique_angle,
            style: string_or(o.style, "Standard")?,
            horizontal_alignment,
            vertical_alignment,
            normal: o.normal.into(),
            thickness: o.thickness,
            generation_flags: o.generation_flags,
            ..native::Text::default()
        };
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::Text(value), out_handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_mtext_add(
    doc: *mut Document,
    input: *const MTextOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        vector(o.insertion_point, "insertion point")?;
        positive(o.height, "text height")?;
        nonnegative(o.rectangle_width, "rectangle width")?;
        finite(o.rotation, "rotation")?;
        nonzero_vector(o.normal, "normal")?;
        positive(o.line_spacing_factor, "line spacing factor")?;
        let rectangle_height = if boolean(o.has_rectangle_height, "has_rectangle_height")? {
            nonnegative(o.rectangle_height, "rectangle height")?;
            Some(o.rectangle_height)
        } else {
            None
        };
        let attachment_point = match o.attachment_point {
            1 => AttachmentPoint::TopLeft,
            2 => AttachmentPoint::TopCenter,
            3 => AttachmentPoint::TopRight,
            4 => AttachmentPoint::MiddleLeft,
            5 => AttachmentPoint::MiddleCenter,
            6 => AttachmentPoint::MiddleRight,
            7 => AttachmentPoint::BottomLeft,
            8 => AttachmentPoint::BottomCenter,
            9 => AttachmentPoint::BottomRight,
            _ => return Err(invalid("attachment_point must be 1 through 9")),
        };
        let drawing_direction = match o.drawing_direction {
            1 => DrawingDirection::LeftToRight,
            3 => DrawingDirection::TopToBottom,
            5 => DrawingDirection::ByStyle,
            _ => return Err(invalid("drawing_direction must be 1, 3, or 5")),
        };
        let line_spacing_style = match o.line_spacing_style {
            1 => LineSpacingStyle::AtLeast,
            2 => LineSpacingStyle::Exactly,
            _ => return Err(invalid("line_spacing_style must be 1 or 2")),
        };
        let mut value = native::MText {
            value: string_or(o.text, "")?,
            insertion_point: o.insertion_point.into(),
            height: o.height,
            rectangle_width: o.rectangle_width,
            rectangle_height,
            rotation: o.rotation,
            style: string_or(o.style, "Standard")?,
            attachment_point,
            drawing_direction,
            line_spacing_factor: o.line_spacing_factor,
            line_spacing_style,
            normal: o.normal.into(),
            ..native::MText::default()
        };
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::MText(Box::new(value)), out_handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_ellipse_add(
    doc: *mut Document,
    input: *const EllipseOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        vector(o.center, "center")?;
        nonzero_vector(o.major_axis, "major axis")?;
        nonzero_vector(o.normal, "normal")?;
        positive(o.minor_axis_ratio, "minor axis ratio")?;
        if o.minor_axis_ratio > 1.0 {
            return Err(invalid("minor axis ratio must not exceed 1"));
        }
        finite(o.start_parameter, "start parameter")?;
        finite(o.end_parameter, "end parameter")?;
        let mut value = native::Ellipse {
            center: o.center.into(),
            major_axis: o.major_axis.into(),
            minor_axis_ratio: o.minor_axis_ratio,
            start_parameter: o.start_parameter,
            end_parameter: o.end_parameter,
            normal: o.normal.into(),
            ..native::Ellipse::default()
        };
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::Ellipse(value), out_handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_spline_add(
    doc: *mut Document,
    input: *const SplineOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        if o.degree < 1 {
            return Err(invalid("spline degree must be at least 1"));
        }
        let control_points = points(o.control_points, o.control_point_count, "control points")?;
        let fit_points = points(o.fit_points, o.fit_point_count, "fit points")?;
        if control_points.is_empty() {
            if fit_points.len() < 2 {
                return Err(invalid(
                    "spline requires at least two fit points or degree+1 control points",
                ));
            }
            if o.knot_count != 0 || o.weight_count != 0 {
                return Err(invalid("knots and weights require control points"));
            }
        } else if control_points.len() <= o.degree as usize {
            return Err(invalid("spline requires at least degree+1 control points"));
        }
        let knots = if o.knot_count == 0 && !control_points.is_empty() {
            native::Spline::generate_clamped_knots(o.degree as usize, control_points.len())
        } else {
            input_array(o.knots, o.knot_count, "knots")?.to_vec()
        };
        if !control_points.is_empty()
            && knots.len()
                != control_points
                    .len()
                    .checked_add(o.degree as usize + 1)
                    .ok_or_else(|| invalid("spline knot count overflow"))?
        {
            return Err(invalid(
                "knot count must equal control_point_count + degree + 1",
            ));
        }
        for &knot in &knots {
            finite(knot, "knot")?;
        }
        if knots.windows(2).any(|v| v[0] > v[1])
            || (!knots.is_empty() && knots.first() == knots.last())
        {
            return Err(invalid(
                "knots must be nondecreasing with a nonzero parameter interval",
            ));
        }
        if !control_points.is_empty() && knots[o.degree as usize] >= knots[control_points.len()] {
            return Err(invalid("spline active knot interval must be nonzero"));
        }
        let weights = input_array(o.weights, o.weight_count, "weights")?.to_vec();
        if !weights.is_empty() && weights.len() != control_points.len() {
            return Err(invalid("weight count must match control point count"));
        }
        for &weight in &weights {
            positive(weight, "weight")?;
        }
        let rational = boolean(o.rational, "rational")? || !weights.is_empty();
        if rational && control_points.is_empty() {
            return Err(invalid(
                "rational splines require control points and weights",
            ));
        }
        if rational && weights.len() != control_points.len() {
            return Err(invalid(
                "rational splines require one weight per control point",
            ));
        }
        nonzero_vector(o.normal, "normal")?;
        vector(o.begin_tangent, "begin tangent")?;
        vector(o.end_tangent, "end tangent")?;
        nonnegative(o.knot_tolerance, "knot tolerance")?;
        nonnegative(o.control_tolerance, "control tolerance")?;
        nonnegative(o.fit_tolerance, "fit tolerance")?;
        if ![0, 1, 2, 15].contains(&o.knot_parameterization) {
            return Err(invalid("knot_parameterization must be 0, 1, 2, or 15"));
        }
        let flags = native::SplineFlags {
            closed: boolean(o.closed, "closed")?,
            periodic: boolean(o.periodic, "periodic")?,
            rational,
            planar: boolean(o.planar, "planar")?,
            linear: boolean(o.linear, "linear")?,
        };
        let mut value = native::Spline {
            degree: o.degree,
            control_points,
            knots,
            weights,
            fit_points,
            flags,
            normal: o.normal.into(),
            begin_tangent: o.begin_tangent.into(),
            end_tangent: o.end_tangent.into(),
            knot_tolerance: o.knot_tolerance,
            control_tolerance: o.control_tolerance,
            fit_tolerance: o.fit_tolerance,
            knot_parameterization: o.knot_parameterization,
            cv_frame_visible: boolean(o.cv_frame_visible, "cv_frame_visible")?,
            ..native::Spline::default()
        };
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::Spline(value), out_handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_insert_add(
    doc: *mut Document,
    input: *const InsertOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        let name = input_str(o.block_name)?.to_owned();
        if name.is_empty() {
            return Err(invalid("block_name must not be empty"));
        }
        vector(o.insertion_point, "insertion point")?;
        vector(o.scale, "scale")?;
        for scale in [o.scale.x, o.scale.y, o.scale.z] {
            if scale.abs() < 1e-12 {
                return Err(invalid("absolute insert scale must be at least 1e-12"));
            }
        }
        finite(o.rotation, "rotation")?;
        nonzero_vector(o.normal, "normal")?;
        if o.column_count == 0 || o.row_count == 0 {
            return Err(invalid("insert row and column counts must be positive"));
        }
        finite(o.column_spacing, "column spacing")?;
        finite(o.row_spacing, "row spacing")?;
        let mut value = native::Insert::new(name, o.insertion_point.into())
            .with_scale(o.scale.x, o.scale.y, o.scale.z)
            .with_rotation(o.rotation)
            .with_normal(o.normal.into());
        value.column_count = o.column_count;
        value.row_count = o.row_count;
        value.column_spacing = o.column_spacing;
        value.row_spacing = o.row_spacing;
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::Insert(value), out_handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_face3d_add(
    doc: *mut Document,
    input: *const Face3DOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        for corner in o.corners {
            vector(corner, "corner")?;
        }
        if o.invisible_edges & !15 != 0 {
            return Err(invalid(
                "invisible_edges may only contain the low four bits",
            ));
        }
        let [a, b, c, d] = o.corners.map(Into::into);
        let mut value = native::Face3D::new(a, b, c, d);
        value.invisible_edges = native::InvisibleEdgeFlags::from_bits(o.invisible_edges);
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::Face3D(value), out_handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_solid_add(
    doc: *mut Document,
    input: *const SolidOptions,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let o = options(input)?;
        for corner in o.corners {
            vector(corner, "corner")?;
        }
        nonzero_vector(o.normal, "normal")?;
        finite(o.thickness, "thickness")?;
        let [a, b, c, d] = o.corners.map(Into::into);
        let mut value = native::Solid::new(a, b, c, d)
            .with_normal(o.normal.into())
            .with_thickness(o.thickness);
        value.is_trace = boolean(o.is_trace, "is_trace")?;
        value.common.layer = string_or(o.layer, "0")?;
        add(doc, EntityType::Solid(value), out_handle)
    })
}

macro_rules! directed_line {
    ($name:ident, $record:ty, $variant:ident, $native:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            doc: *mut Document,
            input: *const $record,
            out_handle: *mut u64,
        ) -> Status {
            ffi_call(|| {
                let o = options(input)?;
                vector(o.base_point, "base point")?;
                nonzero_vector(o.direction, "direction")?;
                let mut value = <$native>::new(o.base_point.into(), unit(o.direction).into());
                value.common.layer = string_or(o.layer, "0")?;
                add(doc, EntityType::$variant(value), out_handle)
            })
        }
    };
}

directed_line!(ar_ray_add, RayOptions, Ray, native::Ray);
directed_line!(ar_xline_add, XLineOptions, XLine, native::XLine);
