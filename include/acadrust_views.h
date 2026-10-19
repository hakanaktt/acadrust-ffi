#ifndef AR_VIEWS_H
#define AR_VIEWS_H
#include "acadrust.h"
#ifdef __cplusplus
extern "C" {
#endif

#define AR_VALUE_OPAQUE ((ValueKind)0)
#define AR_VALUE_STRUCT ((ValueKind)1)
#define AR_VALUE_ENUM ((ValueKind)2)
#define AR_VALUE_SEQUENCE ((ValueKind)3)
#define AR_VALUE_STRING ((ValueKind)4)
#define AR_VALUE_BOOL ((ValueKind)5)
#define AR_VALUE_I8 ((ValueKind)6)
#define AR_VALUE_U8 ((ValueKind)7)
#define AR_VALUE_I16 ((ValueKind)8)
#define AR_VALUE_U16 ((ValueKind)9)
#define AR_VALUE_I32 ((ValueKind)10)
#define AR_VALUE_U32 ((ValueKind)11)
#define AR_VALUE_I64 ((ValueKind)12)
#define AR_VALUE_U64 ((ValueKind)13)
#define AR_VALUE_F32 ((ValueKind)14)
#define AR_VALUE_F64 ((ValueKind)15)
#define AR_VALUE_OPTION ((ValueKind)16)
#define AR_VALUE_MAP ((ValueKind)17)
#define AR_VALUE_CHAR ((ValueKind)18)

/* Views borrow existing native storage: successful traversal allocates nothing
 * and performs no serialization. Do not free any view or view.data pointer.
 * StringView is UTF-8 with an explicit byte length, without a trailing NUL.
 * ValueView's two members are opaque tokens. Only use values returned by this
 * library; never cast value.data to a Rust/C struct or modify either member.
 *
 * A document view and ALL derived pointers expire before ANY mutation, even
 * through another API, or before its owner is freed. Reacquire views afterward.
 * No document may be mutated/freed concurrently with use of a borrowed view.
 * Result views remain valid until their result owner is freed.
 *
 * STRUCT children are named fields. ENUM has a borrowed variant name and
 * payload children (tuple names v0/v1/... or named fields). OPTION has zero or
 * one child. SEQUENCE children are indexed elements. MAP len is entry count:
 * child 2*i is its key, child 2*i+1 its value; iteration order is unspecified.
 * Every child points directly into its native owner. Empty views are invalid.
 * Type names are diagnostic Rust names, not a stable type identifier.
 */
AR_API Status ar_value_kind(ValueView value, ValueKind *out);
AR_API Status ar_value_type_name(ValueView value, StringView *out);
AR_API Status ar_value_len(ValueView value, size_t *out);
AR_API Status ar_value_child(ValueView value, size_t index, StringView *name, ValueView *out);
AR_API Status ar_value_field(ValueView value, const char *name, ValueView *out);
AR_API Status ar_value_variant(ValueView value, StringView *out);
/* Only primitive scalar/array and string storage has C-compatible data.
 * len is element count; stride is bytes per element. Strings use stride=1.
 * Access composites using child/field. Pointers are const and borrowed.
 * BOOL is a read-only byte 0/1. CHAR is a Unicode scalar uint32_t.
 * Handle/flag wrappers use read_integer (the returned bits may be unsigned).
 */
AR_API Status ar_value_data(ValueView value, const void **data, size_t *len, size_t *stride);
AR_API Status ar_value_read_integer(ValueView value, int64_t *out);

AR_API Status ar_document_view(const Document *doc, ValueView *out);
AR_API Status ar_entity_view(const Document *doc, uint64_t handle, ValueView *out);
AR_API Status ar_document_entity_at_view(const Document *doc, size_t index, ValueView *out);
AR_API Status ar_document_object_at_view(const Document *doc, size_t index, uint64_t *handle, ValueView *out);

#define AR_TABLE_LAYER INT32_C(1)
#define AR_TABLE_LINETYPE INT32_C(2)
#define AR_TABLE_TEXT_STYLE INT32_C(3)
#define AR_TABLE_BLOCK_RECORD INT32_C(4)
#define AR_TABLE_DIM_STYLE INT32_C(5)
#define AR_TABLE_APP_ID INT32_C(6)
#define AR_TABLE_VIEW INT32_C(7)
#define AR_TABLE_VPORT INT32_C(8)
#define AR_TABLE_UCS INT32_C(9)
#define AR_TABLE_VX INT32_C(10)
AR_API Status ar_table_count(const Document *doc, int32_t kind, size_t *out_count);
AR_API Status ar_table_view(const Document *doc, int32_t kind, ValueView *out);
AR_API Status ar_table_entry_view(const Document *doc, int32_t kind, size_t index, ValueView *out);
AR_API Status ar_table_find_view(const Document *doc, int32_t kind, const char *name, ValueView *out);
AR_API Status ar_table_add(Document *doc, int32_t kind, const char *name, uint64_t *out_handle);
AR_API Status ar_layout_add(Document *doc, const char *name, uint64_t *out_handle);
AR_API Status ar_object_count(const Document *doc, size_t *out_count);
/* Native object enumeration order is unspecified. */
AR_API Status ar_object_handle(const Document *doc, size_t index, uint64_t *out_handle);
AR_API Status ar_object_view(const Document *doc, uint64_t handle, ValueView *out);
AR_API Status ar_class_count(const Document *doc, size_t *out_count);
AR_API Status ar_class_view(const Document *doc, size_t index, ValueView *out);

AR_API void ar_result_free(NativeResult *result);
AR_API Status ar_result_view(const NativeResult *result, ValueView *out);
/* These operations compute new native results; view traversal does not copy
 * them. Free the opaque result after its borrowed views are no longer used. */
AR_API Status ar_document_audit(const Document *doc, NativeResult **out);
AR_API Status ar_document_repair(Document *doc, NativeResult **out);
AR_API Status ar_layer_states(const Document *doc, NativeResult **out);
AR_API Status ar_layer_state_capture(Document *doc, const char *name, const char *description, uint64_t *out_handle);
AR_API Status ar_layer_state_restore(Document *doc, const char *name, size_t *out_count);
AR_API Status ar_layer_state_rename(Document *doc, const char *old_name, const char *new_name);
AR_API Status ar_layer_state_remove(Document *doc, const char *name);
AR_API Status ar_document_export_block(const Document *doc, uint64_t handle, NativeResult **out);
typedef struct HandleMapping { uint64_t source, target; } HandleMapping;
/* Exported BlockGraph is opaque upstream; pass its result back for import. */
AR_API Status ar_document_import_block_graph(Document *doc, const NativeResult *graph, uint64_t record, const HandleMapping *mappings, size_t count);
/* mode: 0 graph, 1 tree, 2 operation list. */
AR_API Status ar_entity_solid_history(const Document *doc, uint64_t handle, int32_t mode, NativeResult **out);
/* insert=0 reads the definition defaults. */
AR_API Status ar_dynamic_block_properties(const Document *doc, uint64_t definition, uint64_t insert, NativeResult **out);
AR_API Status ar_entity_move_to_block_handle(Document *doc, uint64_t entity, uint64_t block);
AR_API Status ar_layer_rename(Document *doc, const char *old_name, const char *new_name);
AR_API Status ar_document_allocate_handle(Document *doc, uint64_t *out);
AR_API Status ar_document_resolve_references(Document *doc);
AR_API Status ar_document_has_version_locked_data(const Document *doc, uint32_t version, uint8_t *out);
/* Borrowed serialized drawing variable (e.g. HPORIGIN), or AR_NOT_FOUND. */
AR_API Status ar_document_drawing_variable(const Document *doc, const char *name, StringView *out);
AR_API Status ar_document_set_drawing_variable(Document *doc, const char *name, const char *value);
AR_API Status ar_document_extension_dictionary(Document *doc, uint64_t owner, uint8_t create, uint64_t *out);
AR_API Status ar_document_xrecord_view(const Document *doc, uint64_t owner, const char *key, ValueView *out);
AR_API Status ar_document_ensure_xrecord(Document *doc, uint64_t owner, const char *key, uint64_t *out);

#ifdef __cplusplus
}
#endif
#endif
