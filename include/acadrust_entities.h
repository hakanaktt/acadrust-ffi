#ifndef AR_ENTITIES_H
#define AR_ENTITIES_H

#include "acadrust.h"
#include "acadrust_views.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct BoundingBox { Vector3 min, max; } BoundingBox;
/* Row-major affine matrix acting on column vectors. Translation occupies
 * values[3], values[7], values[11]; last row must be [0, 0, 0, 1]. */
typedef struct Matrix4 { double values[16]; } Matrix4;

/* Borrowed static names; do not free. Includes every upstream EntityType
 * variant plus dimension, surface, underlay, extended-class and dynamic-block
 * entity subtypes. Constructor names are case-insensitive.
 * Empty defaults may require geometry or external references before export.
 * Use ar_entity_view to inspect fields and the native mutation API to edit. */
AR_API Status ar_entity_kind_count(size_t *out_count);
AR_API Status ar_entity_kind_name(size_t index, StringView *out_name);
AR_API Status ar_entity_kind_variant(size_t index, StringView *out_name);
AR_API Status ar_entity_add_default(Document *doc, const char *kind,
                                    uint64_t *out_handle);
/* Borrowed document string, valid until the next mutation/free of doc.
 * Resolves Extended/Unknown entities to their actual class names. */
AR_API Status ar_entity_class_name(const Document *doc, uint64_t handle,
                                  StringView *out_name);

/* Uses upstream bounds. Infinite rays/construction lines and entities with
 * unavailable geometry follow upstream's bounds conventions. INSERT bounds
 * do not expand referenced block contents. */
AR_API Status ar_entity_bounding_box(const Document *doc, uint64_t handle,
                                    BoundingBox *out_bounds);
/* Uses upstream entity-specific geometry behavior. General transformations
 * return AR_TYPE_MISMATCH for unsupported Extended, Unknown, Light,
 * SectionSymbol and ViewBorder classes. Translation supports Light and
 * geometric Extended classes. Nonuniform scaling follows upstream behavior;
 * circular primitives are not converted to ellipses. Angles are radians.
 * Mutations are atomic on validation failure. */
AR_API Status ar_entity_translate(Document *doc, uint64_t handle, Vector3 offset);
AR_API Status ar_entity_transform(Document *doc, uint64_t handle,
                                 const Matrix4 *matrix);
AR_API Status ar_entity_rotate(Document *doc, uint64_t handle, Vector3 axis,
                              double angle);
AR_API Status ar_entity_scale(Document *doc, uint64_t handle, Vector3 scale,
                             Vector3 origin);
AR_API Status ar_entity_mirror(Document *doc, uint64_t handle, Vector3 point,
                              Vector3 normal);
/* Adds decomposed entities to doc with new handles, retaining their owner's
 * block/layout. The source remains. Atomic entities and INSERTs yield zero
 * parts. Query with handles=NULL and capacity=0: sets required and leaves doc
 * unchanged. A short buffer returns AR_BUFFER_TOO_SMALL without mutation.
 * Successful non-query calls invalidate all document views. */
AR_API Status ar_entity_explode(Document *doc, uint64_t handle,
                               uint64_t *handles, size_t capacity,
                               size_t *required);

#ifdef __cplusplus
}
#endif

#endif
