#ifndef AR_WRITABLE_H
#define AR_WRITABLE_H
#include "acadrust.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef struct WritableSpan {
    void *data;
    size_t len, stride;
    int32_t kind;
    void *token;
} WritableSpan;

/* Exclusive, direct writable access to a native numeric scalar/array,
 * byte array, or fixed-length UTF-8 string allocation at a native path.
 * No data buffer clone or serialization occurs. The lease and its path
 * have small control allocations. An upstream shared document clone may
 * require copy-on-write on its first entity mutation.
 *
 * Initialize the span to {0}. Borrowing invalidates all earlier read views.
 * Until release succeeds, no document access is permitted. Document APIs
 * reject the active borrow; document_free is ignored and must be called again
 * after release. Never use earlier views while a writable borrow is active.
 *
 * Only data[0..len*stride] may be changed. Do not resize, free the data, edit
 * span metadata, copy the span, or use its pointer after release. Bytes belong
 * to the document. Nested Rust structs/enums remain opaque: borrow their
 * numeric/string children. Booleans, characters, handle wrappers, flags, enum
 * discriminants, indexed names, identities, and raw storage metadata use the
 * native setters or dedicated structural APIs, not unchecked pointers.
 *
 * Strings expose their existing allocation as bytes while detached from
 * String; len cannot change. Release checks UTF-8 and numeric finiteness,
 * validates basic entity geometry, restores strings, and invalidates native
 * storage caches. A failed release keeps the borrow active and pointer valid:
 * correct its contents and retry. Release does not roll edits back.
 * Successful release resets the span to {0}. Never abandon an active span.
 * Paths use the same slash syntax as the native mutation API.
 */
AR_API Status ar_entity_borrow_mut(Document *doc, uint64_t handle, const char *path, WritableSpan *out);
AR_API Status ar_object_borrow_mut(Document *doc, uint64_t handle, const char *path, WritableSpan *out);
AR_API Status ar_document_borrow_mut(Document *doc, const char *path, WritableSpan *out);
AR_API Status ar_table_borrow_mut(Document *doc, int32_t kind, const char *name, const char *path, WritableSpan *out);
/* Indexed access reaches duplicate imported table names individually. */
AR_API Status ar_table_entry_borrow_mut(Document *doc, int32_t kind, size_t index, const char *path, WritableSpan *out);
AR_API Status ar_writable_release(WritableSpan *span);
#ifdef __cplusplus
}
#endif
#endif
