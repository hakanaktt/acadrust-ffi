#ifndef AR_MUTATIONS_H
#define AR_MUTATIONS_H

#include "acadrust.h"
#include "acadrust_views.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Owned native fields are changed through exclusive document access, never by
 * writing through a ValueView. Every mutation invalidates all document views.
 * Reacquire them after the call. No mutation may run concurrently with reads.
 *
 * kind is an AR_VALUE_* primitive or STRING. Numeric/boolean/character input
 * uses its exact native kind; len is element count. BOOL uses uint8_t 0 or 1,
 * CHAR uses a valid Unicode scalar uint32_t. Floating-point values must be
 * finite. STRING is UTF-8: len is bytes, without a required trailing NUL.
 * data may be NULL only when len is zero. Inputs are copied before editing,
 * so a borrowed native array can also be used as an input to this call.
 * Numeric arrays replace Vec fields, or fixed arrays of the same length.
 */
typedef struct NativeWrite {
    ValueKind kind;
    const void *data;
    size_t len;
} NativeWrite;

/* Paths use slash-separated native field names and zero-based vector/array
 * indices. Leading/trailing slashes are optional. Single enum payloads may
 * be traversed implicitly: "start/x", "Line/start/x", and "v0/start/x".
 * An explicit payload segment may be "value", "v0", or its numeric index.
 * Optional values use a "value" segment. Existing map values use their key's
 * display string. Inspect names/kinds with the borrowed native view APIs.
 *
 * resize truncates or appends default native records. option enables/clears an
 * Option. select uses the exact borrowed enum variant name and resets its
 * payload to defaults. Constructors use upstream Default where available,
 * otherwise public fields and constructible enum variants. Missing constructors
 * return AR_TYPE_MISMATCH without changing the value. Geometry/reference fields
 * may still need initialization before export.
 *
 * Handles, ownership, block membership, raw record/cache fields, and table
 * names are protected. Use dedicated ownership/rename APIs. Root entity/object
 * replacement is unsupported. Entity edits clear stale raw entity records;
 * object edits clear affected cached storage. Named object changes synchronize
 * the owning dictionary. Table lookup by name addresses its first entry.
 * Document paths support header, summary_info, maintenance_version, preview,
 * and section_view_style; database collections use their specific APIs.
 */
AR_API Status ar_document_write(Document *doc, const char *path, const NativeWrite *input);
AR_API Status ar_document_resize(Document *doc, const char *path, size_t len);
AR_API Status ar_document_select(Document *doc, const char *path, const char *variant);
AR_API Status ar_document_option(Document *doc, const char *path, uint8_t enabled);

AR_API Status ar_entity_write(Document *doc, uint64_t handle, const char *path, const NativeWrite *input);
AR_API Status ar_entity_resize(Document *doc, uint64_t handle, const char *path, size_t len);
AR_API Status ar_entity_select(Document *doc, uint64_t handle, const char *path, const char *variant);
AR_API Status ar_entity_option(Document *doc, uint64_t handle, const char *path, uint8_t enabled);

AR_API Status ar_object_write(Document *doc, uint64_t handle, const char *path, const NativeWrite *input);
AR_API Status ar_object_resize(Document *doc, uint64_t handle, const char *path, size_t len);
AR_API Status ar_object_select(Document *doc, uint64_t handle, const char *path, const char *variant);
AR_API Status ar_object_option(Document *doc, uint64_t handle, const char *path, uint8_t enabled);

AR_API Status ar_table_write(Document *doc, int32_t kind, const char *name, const char *path, const NativeWrite *input);
AR_API Status ar_table_resize(Document *doc, int32_t kind, const char *name, const char *path, size_t len);
AR_API Status ar_table_select(Document *doc, int32_t kind, const char *name, const char *path, const char *variant);
AR_API Status ar_table_option(Document *doc, int32_t kind, const char *name, const char *path, uint8_t enabled);

/* Indexed variants also address VPORT records sharing the same display name. */
AR_API Status ar_table_entry_write(Document *doc, int32_t kind, size_t index, const char *path, const NativeWrite *input);
AR_API Status ar_table_entry_resize(Document *doc, int32_t kind, size_t index, const char *path, size_t len);
AR_API Status ar_table_entry_select(Document *doc, int32_t kind, size_t index, const char *path, const char *variant);
AR_API Status ar_table_entry_option(Document *doc, int32_t kind, size_t index, const char *path, uint8_t enabled);
/* Class metadata by index, including duplicate imported names. Native class
 * identity (dxf_name/class_number) uses the structural API. Collection indexes
 * are rebuilt; these setters copy owned class metadata, not serialized data. */
AR_API Status ar_class_write(Document *doc, size_t index, const char *path, const NativeWrite *input);
AR_API Status ar_class_resize(Document *doc, size_t index, const char *path, size_t len);
AR_API Status ar_class_select(Document *doc, size_t index, const char *path, const char *variant);
AR_API Status ar_class_option(Document *doc, size_t index, const char *path, uint8_t enabled);

#ifdef __cplusplus
}
#endif
#endif
