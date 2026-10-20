#ifndef AR_BUILDER_H
#define AR_BUILDER_H
#include "acadrust.h"
#include "acadrust_mutations.h"
#ifdef __cplusplus
extern "C" {
#endif

typedef struct EntityBuilder EntityBuilder;

#define AR_ENTITY_EDIT_WRITE INT32_C(0)
#define AR_ENTITY_EDIT_RESIZE INT32_C(1)
#define AR_ENTITY_EDIT_SELECT INT32_C(2)
#define AR_ENTITY_EDIT_OPTION INT32_C(3)
typedef struct EntityEdit {
    int32_t kind;
    const char *path;
    NativeWrite value;
    size_t length;
    const char *variant;
    uint8_t enabled;
} EntityEdit;

/* Ordered native construction operations. Only the member for the selected
 * kind is used: WRITE=value, RESIZE=length, SELECT=variant, OPTION=enabled.
 * Apply is atomic on edit failure. Create configures and validates the complete
 * native entity before inserting it. NULL edits is permitted for count=0. */
AR_API Status ar_entity_builder_apply(EntityBuilder *builder, const EntityEdit *edits, size_t count);
AR_API Status ar_entity_create(Document *doc, const char *kind, const EntityEdit *edits, size_t count, uint64_t *out_handle);

/* Every one of the 97 catalog kinds has these two named entry points:
 * ar_<kind>_builder_new(out), ar_<kind>_create(doc, edits, count, out_handle).
 * Examples: ar_hatch_builder_new, ar_dimension_aligned_create,
 * ar_multileader_create, ar_acdbpointcloudex_create, ar_surface_create. */
#define AR_ENTITY_FACTORY(name, kind) \
    AR_API Status ar_##name##_builder_new(EntityBuilder **out); \
    AR_API Status ar_##name##_create(Document *doc, const EntityEdit *edits, size_t count, uint64_t *out_handle);
#include "acadrust_entity_catalog.inc"
#undef AR_ENTITY_FACTORY

/* Detached native entity construction, available for every entity catalog kind.
 * Nothing is inserted into a document until add succeeds. All setters use
 * native fields, never JSON. Inputs are copied; no input pointer is retained.
 * Paths are relative to the entity payload: "value" edits TEXT's text,
 * "start/x" edits LINE's coordinate. An optional root variant is accepted.
 * resize/select/option permit configuring complex geometry before validation.
 * Root replacement and protected identity/cache fields are unavailable.
 *
 * Builder views borrow its native data and expire on ANY setter/add/free.
 * Never access a builder concurrently with mutation/free. Free accepts NULL.
 * Add validates against the destination document; failure before insertion
 * leaves the document and builder unchanged. Add transfers native allocations
 * into the document, assigns fresh identities, frees the builder, and sets the
 * caller's builder pointer to NULL. Failed validation leaves it available to
 * correct and retry. Builder data are moved, not serialized or deep copied.
 * Block/layout targets must already exist. Geometry/version fidelity follows
 * upstream; a constructible default is not a guarantee of file compatibility.
 */
AR_API Status ar_entity_builder_new(const char *kind, EntityBuilder **out);
AR_API void ar_entity_builder_free(EntityBuilder *builder);
AR_API Status ar_entity_builder_kind(const EntityBuilder *builder, StringView *out);
AR_API Status ar_entity_builder_view(const EntityBuilder *builder, ValueView *out);
AR_API Status ar_entity_builder_write(EntityBuilder *builder, const char *path, const NativeWrite *input);
AR_API Status ar_entity_builder_resize(EntityBuilder *builder, const char *path, size_t len);
AR_API Status ar_entity_builder_select(EntityBuilder *builder, const char *path, const char *variant);
AR_API Status ar_entity_builder_option(EntityBuilder *builder, const char *path, uint8_t enabled);
/* Typed conveniences: integers are checked against the target's exact native
 * width, vectors require x/y/z double fields, and vec3s replaces a vector of
 * native 3D vectors. Zero count permits NULL. Setters copy caller input. */
AR_API Status ar_entity_builder_set_f64(EntityBuilder *builder, const char *path, double value);
AR_API Status ar_entity_builder_set_i64(EntityBuilder *builder, const char *path, int64_t value);
AR_API Status ar_entity_builder_set_u64(EntityBuilder *builder, const char *path, uint64_t value);
AR_API Status ar_entity_builder_set_bool(EntityBuilder *builder, const char *path, uint8_t value);
AR_API Status ar_entity_builder_set_text(EntityBuilder *builder, const char *path, const char *value);
AR_API Status ar_entity_builder_set_vec3(EntityBuilder *builder, const char *path, Vector3 value);
AR_API Status ar_entity_builder_set_vec3s(EntityBuilder *builder, const char *path, const Vector3 *values, size_t count);
AR_API Status ar_entity_builder_add(Document *doc, EntityBuilder **builder, uint64_t *out_handle);
AR_API Status ar_entity_builder_add_to_block(Document *doc, EntityBuilder **builder, const char *block, uint64_t *out_handle);
AR_API Status ar_entity_builder_add_to_layout(Document *doc, EntityBuilder **builder, const char *layout, uint64_t *out_handle);

#ifdef __cplusplus
}
#endif
#endif
