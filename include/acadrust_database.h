#ifndef AR_DATABASE_H
#define AR_DATABASE_H

#include "acadrust.h"
#include "acadrust_views.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Borrowed static catalog names. Every ObjectType variant is represented,
 * together with native class, data, associative, dynamic-block, solid-history,
 * annotation-context, underlay and DGN line-style subtype templates.
 * Constructor names are case-insensitive. Templates can require geometry,
 * class names or referenced handles before export. For a connected layout,
 * use ar_layout_add. This API constructs individual database records.
 * owner=0 creates an unowned record; other owners must already exist.
 * Dictionary registration is explicit through native dictionary mutation. */
AR_API Status ar_object_kind_count(size_t *out_count);
AR_API Status ar_object_kind_name(size_t index, StringView *out_name);
AR_API Status ar_object_kind_variant(size_t index, StringView *out_name);
AR_API Status ar_object_add_default(Document *doc, const char *kind,
                                    uint64_t owner, uint64_t *out_handle);
/* Shallow native clone: keeps references to existing records and copies XDATA.
 * owner=0 retains the source owner. Reactors/extension dictionary associations
 * are detached. Cloned dictionaries have soft references to their entries.
 * Opaque payloads that cannot be reidentified return AR_TYPE_MISMATCH. */
AR_API Status ar_object_clone(Document *doc, uint64_t source, uint64_t owner,
                              uint64_t *out_handle);
/* Removes an unreferenced record and its dictionary registrations and native
 * side indexes. Rejects records with known dependents. References hidden in
 * unknown vendor payloads cannot be inspected by upstream. */
AR_API Status ar_object_remove(Document *doc, uint64_t handle);
/* Removes an unused symbol-table entry by name. Built-in/current entries,
 * populated blocks, layout block records and known references are protected. */
AR_API Status ar_table_remove(Document *doc, int32_t kind, const char *name);
/* Index-based deletion can select duplicate imported records. Remaining
 * order and duplicate names are retained. */
AR_API Status ar_table_remove_at(Document *doc, int32_t kind, size_t index);

/* Creates a native CLASSES declaration. is_entity is 0 or 1; out_index is its
 * zero-based position. Class metadata is editable through ar_class_write;
 * all fields are reachable through ar_class_view.
 * Duplicate names are rejected. Removal preserves remaining order and rejects
 * live instances or positional references that cannot be safely shifted. */
AR_API Status ar_class_add(Document *doc, const char *dxf_name,
                           const char *cpp_name, const char *application,
                           uint8_t is_entity, size_t *out_index);
AR_API Status ar_class_remove(Document *doc, size_t index);

/* Keeps block/layout membership and viewport lists synchronized. Structural
 * markers and attribute subentities cannot move or clone independently.
 * All existing document views expire on success. */
AR_API Status ar_entity_move_to_block_name(Document *doc, uint64_t handle,
                                          const char *block);
AR_API Status ar_entity_move_to_layout(Document *doc, uint64_t handle,
                                      const char *layout);
/* Copies native geometry and metadata with a new identity in the same owner
 * block. Referenced definitions remain shared. Opaque raw payloads that cannot
 * be safely reidentified return AR_TYPE_MISMATCH. */
AR_API Status ar_entity_clone(Document *doc, uint64_t source,
                              uint64_t *out_handle);

#ifdef __cplusplus
}
#endif

#endif
