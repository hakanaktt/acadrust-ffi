#ifndef ACADRUST_CONSTRUCTORS_H
#define ACADRUST_CONSTRUCTORS_H

#include "acadrust.h"
#include "acadrust_entities.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Call ar_*_options_default before customizing a record. All angles and ellipse
 * parameters are radians. Input records are borrowed for one call. Strings
 * are NUL-terminated UTF-8; array counts are elements. Data are copied into
 * native ownership; no caller-owned memory is retained. NULL optional text,
 * style and layer pointers mean empty text, "Standard" style and "0" layer.
 * Failed validation leaves the document and out_handle unchanged. Successful
 * creation invalidates all borrowed document views, like other mutations.
 *
 * Each add inserts into model space. Use ar_entity_move_to_block_name or
 * ar_entity_move_to_layout for a different owner. All fields remain reachable
 * through native views, mutation and writable storage APIs after creation. */
typedef struct TextOptions {
    const char *text;
    Vector3 insertion_point;
    double height, rotation, width_factor, oblique_angle;
    Vector3 normal;
    double thickness;
    Vector3 alignment_point;
    uint8_t has_alignment_point;
    /* Horizontal: left=0, center=1, right=2, aligned=3, middle=4, fit=5.
     * Vertical: baseline=0, bottom=1, middle=2, top=3. */
    int32_t horizontal_alignment, vertical_alignment;
    int16_t generation_flags;
    const char *style;
    const char *layer;
} TextOptions;

typedef struct MTextOptions {
    const char *text;
    Vector3 insertion_point;
    double height, rectangle_width, rectangle_height;
    uint8_t has_rectangle_height;
    double rotation;
    Vector3 normal;
    /* Attachment: top left..right=1..3, middle=4..6, bottom=7..9.
     * Direction: left-to-right=1, top-to-bottom=3, by-style=5. */
    int32_t attachment_point, drawing_direction;
    double line_spacing_factor;
    int32_t line_spacing_style; /* at-least=1, exactly=2 */
    const char *style;
    const char *layer;
} MTextOptions;

typedef struct EllipseOptions {
    Vector3 center, major_axis;
    double minor_axis_ratio, start_parameter, end_parameter;
    Vector3 normal;
    const char *layer;
} EllipseOptions;

typedef struct SplineOptions {
    int32_t degree;
    const Vector3 *control_points;
    size_t control_point_count;
    const double *knots;
    size_t knot_count;
    const double *weights;
    size_t weight_count;
    const Vector3 *fit_points;
    size_t fit_point_count;
    uint8_t closed, periodic, rational, planar, linear;
    Vector3 normal, begin_tangent, end_tangent;
    double knot_tolerance, control_tolerance, fit_tolerance;
    int32_t knot_parameterization;
    uint8_t cv_frame_visible;
    const char *layer;
} SplineOptions;

/* SPLINE: supply degree+1 or more control points, or at least two fit points.
 * Empty knots generate upstream clamped uniform knots for control points.
 * Custom knots are nondecreasing and count=control_point_count+degree+1.
 * Nonempty weights require one positive weight per control point and enable
 * rational mode automatically. Empty input arrays may use NULL. The explicit
 * rational flag requires weights. Degree is never silently clamped.
 * knot_parameterization: chord=0, square-root=1, uniform=2, custom=15. */

typedef struct InsertOptions {
    const char *block_name;
    Vector3 insertion_point, scale;
    double rotation;
    Vector3 normal;
    uint16_t column_count, row_count;
    double column_spacing, row_spacing;
    const char *layer;
} InsertOptions;

/* INSERT requires an existing named block, nonzero finite scale components
 * (absolute value >= 1e-12), and positive row/column counts. */

typedef struct Face3DOptions {
    Vector3 corners[4];
    uint8_t invisible_edges;
    const char *layer;
} Face3DOptions;

typedef struct SolidOptions {
    Vector3 corners[4];
    Vector3 normal;
    double thickness;
    uint8_t is_trace;
    const char *layer;
} SolidOptions;

/* FACE3D corners are WCS; SOLID corners are OCS. For triangles repeat corner
 * 2 at corner 3 (zero-based). SOLID quadrilateral boundary order is 0,1,3,2.
 * FACE3D invisible_edges uses bits 1,2,4,8. SOLID is_trace selects TRACE. */

typedef struct RayOptions { Vector3 base_point, direction; const char *layer; } RayOptions;
typedef RayOptions XLineOptions;

/* RAY/XLINE normalize any nonzero finite direction, including extreme values. */

AR_API Status ar_text_options_default(TextOptions *out);
AR_API Status ar_mtext_options_default(MTextOptions *out);
AR_API Status ar_ellipse_options_default(EllipseOptions *out);
AR_API Status ar_spline_options_default(SplineOptions *out);
AR_API Status ar_insert_options_default(InsertOptions *out);
AR_API Status ar_face3d_options_default(Face3DOptions *out);
AR_API Status ar_solid_options_default(SolidOptions *out);
AR_API Status ar_ray_options_default(RayOptions *out);
AR_API Status ar_xline_options_default(XLineOptions *out);

AR_API Status ar_text_add(Document *, const TextOptions *, uint64_t *out_handle);
AR_API Status ar_mtext_add(Document *, const MTextOptions *, uint64_t *out_handle);
AR_API Status ar_ellipse_add(Document *, const EllipseOptions *, uint64_t *out_handle);
AR_API Status ar_spline_add(Document *, const SplineOptions *, uint64_t *out_handle);
AR_API Status ar_insert_add(Document *, const InsertOptions *, uint64_t *out_handle);
AR_API Status ar_face3d_add(Document *, const Face3DOptions *, uint64_t *out_handle);
AR_API Status ar_solid_add(Document *, const SolidOptions *, uint64_t *out_handle);
AR_API Status ar_ray_add(Document *, const RayOptions *, uint64_t *out_handle);
AR_API Status ar_xline_add(Document *, const XLineOptions *, uint64_t *out_handle);

#ifdef __cplusplus
}
#endif

#endif
