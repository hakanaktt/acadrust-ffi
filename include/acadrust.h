#ifndef AR_H
#define AR_H

#include <stddef.h>
#include <stdint.h>

#if defined(_WIN32) && !defined(AR_STATIC)
#define AR_API __declspec(dllimport)
#else
#define AR_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* All functions use the platform C calling convention (cdecl on Windows).
 * Use this header with the library from the same release and architecture.
 * All strings and paths are NUL-terminated UTF-8. Angles are radians.
 * Coordinates and distances use the drawing's units.
 *
 * Pointers must be aligned and valid for their declared type/length. Required
 * pointers cannot be NULL. Never access a document concurrently with mutation
 * or freeing, and never reuse a freed document. No function retains input
 * strings, arrays, or buffers after returning.
 *
 * Unless specified otherwise, outputs are valid only after AR_OK. Initialize
 * owned output pointers to NULL and Buffer to {0}. Free an existing owned
 * output before reusing its slot. Free returned objects only with the matching
 * acadrust free function, including when statically linking the library.
 */

typedef struct Document Document;
typedef struct StringView { const char *data; size_t len; } StringView;
typedef struct ValueView { const void *data; const void *descriptor; } ValueView;
typedef ValueView View;
typedef int32_t ValueKind;
typedef struct NativeResult NativeResult;
typedef int32_t Status;
typedef int32_t Format;

#define AR_OK ((Status)0)
#define AR_INVALID_ARGUMENT ((Status)1)
#define AR_NOT_FOUND ((Status)2)
#define AR_TYPE_MISMATCH ((Status)3)
#define AR_IO_ERROR ((Status)4)
#define AR_PARSE_ERROR ((Status)5)
#define AR_BUFFER_TOO_SMALL ((Status)6)
#define AR_PANIC ((Status)7)

#define AR_FORMAT_AUTO ((Format)0)
#define AR_FORMAT_DXF_ASCII ((Format)1)
#define AR_FORMAT_DXF_BINARY ((Format)2)
#define AR_FORMAT_DWG ((Format)3)

#define AR_VERSION_R12 UINT32_C(1009)
#define AR_VERSION_R13 UINT32_C(1012)
#define AR_VERSION_R14 UINT32_C(1014)
#define AR_VERSION_2000 UINT32_C(1015)
#define AR_VERSION_2004 UINT32_C(1018)
#define AR_VERSION_2007 UINT32_C(1021)
#define AR_VERSION_2010 UINT32_C(1024)
#define AR_VERSION_2013 UINT32_C(1027)
#define AR_VERSION_2018 UINT32_C(1032)

#define AR_LAYER_FROZEN UINT32_C(1)
#define AR_LAYER_LOCKED UINT32_C(2)
#define AR_LAYER_OFF UINT32_C(4)

typedef struct Buffer {
    uint8_t *data;
    size_t len;
} Buffer;

typedef struct Vector3 { double x, y, z; } Vector3;
typedef struct Line { Vector3 start, end; } Line;
typedef struct Circle { Vector3 center; double radius; } Circle;
typedef struct Arc {
    Vector3 center;
    double radius, start_angle, end_angle;
} Arc;
typedef struct Point { Vector3 position; } Point;
typedef struct LwVertex {
    double x, y, bulge, start_width, end_width;
} LwVertex;

AR_API uint32_t ar_abi_version(void);
/* Borrowed static strings. Do not free. */
AR_API const char *ar_version(void);
AR_API const char *ar_upstream_version(void);
/* Borrowed thread-local string, valid until the next status-returning call on
 * this thread. Success clears it. Free/version/error functions preserve it.
 * AR_PANIC catches unwinding Rust panics, not invalid pointers or OOM aborts.
 * Discard the document after a panic, because mutation may be incomplete. */
AR_API const char *ar_last_error(void);
/* Free functions accept NULL. buffer_free also resets *buffer to {0}. */
AR_API void ar_string_free(char *text);
AR_API void ar_buffer_free(Buffer *buffer);
AR_API void ar_document_free(Document *doc);

/* out receives a new owned document. Version is a numeric AC code above. */
AR_API Status ar_document_new(uint32_t version, Document **out);
AR_API Status ar_document_clone(const Document *doc, Document **out);
AR_API Status ar_document_get_version(const Document *doc, uint32_t *out);
/* Sets the export version; does not validate entity compatibility. */
AR_API Status ar_document_set_version(Document *doc, uint32_t version);

/* File input detects DWG or ASCII/binary DXF by content. Buffer input also
 * supports AUTO. DXF input detects ASCII/binary for either DXF format value.
 * Readers use strict mode. Output requires an explicit format (not AUTO).
 * File output overwrites an existing file. Buffer output is owned. */
AR_API Status ar_document_read_file(const char *path, Document **out);
AR_API Status ar_document_read_buffer(const uint8_t *data, size_t len, Format format, Document **out);
AR_API Status ar_document_write_file(const Document *doc, const char *path, Format format);
AR_API Status ar_document_write_buffer(const Document *doc, Format format, Buffer *out);

/* Parallel I/O uses a dedicated Rayon pool. It does not modify the host
 * application's global Rayon pool. Set threads to zero for available
 * parallelism. Existing calls keep their pool; later calls use the new one.
 * While a batch runs, acadrust's internal DWG page/object parallelism uses the
 * same pool. The current worker count can be queried at any time. */
AR_API Status ar_io_set_threads(size_t threads);
AR_API Status ar_io_get_threads(size_t *out_threads);
/* Batch readers copy all paths before starting worker threads. Every output
 * document slot is initialized to NULL. `statuses` receives one status per
 * item. The overall return is AR_OK only if every item succeeds; on failure,
 * successful items remain owned by the caller and failed slots remain NULL. */
AR_API Status ar_documents_read_files(const char *const *paths, size_t count,
                                      Document **documents, Status *statuses);
/* Batch writers read documents concurrently and write independent paths. Do
 * not mutate or free any document until this call returns. `documents` and
 * `paths` each have count entries; statuses may be inspected per item. Paths
 * should be distinct; concurrent writes to one path race and are undefined at
 * the file-system level. */
AR_API Status ar_documents_write_files(const Document *const *documents,
                                       const char *const *paths, size_t count,
                                       Format format, Status *statuses);

/* Short source-level aliases for the common calls. These are macros, so they
 * add no ABI symbols and existing long names remain available. */
#define ar_new ar_document_new
#define ar_clone ar_document_clone
#define ar_free ar_document_free
#define ar_read_file ar_document_read_file
#define ar_read ar_document_read_buffer
#define ar_write_file ar_document_write_file
#define ar_write ar_document_write_buffer
#define ar_read_many ar_documents_read_files
#define ar_write_many ar_documents_write_files
#define ar_set_threads ar_io_set_threads
#define ar_get_threads ar_io_get_threads

/* Entity enumeration covers all stored entities, including paper/block space.
 * Handles remain valid until removal; 0 is the null handle.
 * handles(NULL,0,&required) queries size and succeeds. Too-small arrays return
 * AR_BUFFER_TOO_SMALL, set required, and leave the array untouched. */
AR_API Status ar_entity_count(const Document *doc, size_t *out_count);
AR_API Status ar_entity_handles(const Document *doc, uint64_t *handles, size_t capacity, size_t *required);
AR_API Status ar_entity_type(const Document *doc, uint64_t handle, char **out_type);
AR_API Status ar_entity_remove(Document *doc, uint64_t handle);

/* JSON uses acadrust 0.6.5's Serde EntityType schema, with variant names such
 * as {"Line":{...}}. Use get_json to obtain a complete editable template.
 * Schema is tied to the upstream version, not the stable C struct layouts.
 * add_json assigns fresh identities and adds to model space. Other referenced
 * blocks/styles/objects must already exist. set_json requires the same variant
 * and preserves the existing entity's handle and owner. JSON strings are owned.
 * Serde omits some native storage fields: this is not a lossless DWG archive. */
AR_API Status ar_entity_get_json(const Document *doc, uint64_t handle, char **out_json);
AR_API Status ar_entity_add_json(Document *doc, const char *json, uint64_t *out_handle);
AR_API Status ar_entity_set_json(Document *doc, uint64_t handle, const char *json);

/* Typed add operations use layer 0 and default ByLayer properties. Typed set
 * operations preserve common properties. get/set return TYPE_MISMATCH if the
 * handle denotes another entity type. Radii must be positive; all numbers must
 * be finite. These POD structs contain no borrowed pointers. */
AR_API Status ar_line_add(Document *doc, const Line *value, uint64_t *out_handle);
AR_API Status ar_line_get(const Document *doc, uint64_t handle, Line *out_value);
AR_API Status ar_line_set(Document *doc, uint64_t handle, const Line *value);
AR_API Status ar_circle_add(Document *doc, const Circle *value, uint64_t *out_handle);
AR_API Status ar_circle_get(const Document *doc, uint64_t handle, Circle *out_value);
AR_API Status ar_circle_set(Document *doc, uint64_t handle, const Circle *value);
AR_API Status ar_arc_add(Document *doc, const Arc *value, uint64_t *out_handle);
AR_API Status ar_arc_get(const Document *doc, uint64_t handle, Arc *out_value);
AR_API Status ar_arc_set(Document *doc, uint64_t handle, const Arc *value);
AR_API Status ar_point_add(Document *doc, const Point *value, uint64_t *out_handle);
AR_API Status ar_point_get(const Document *doc, uint64_t handle, Point *out_value);
AR_API Status ar_point_set(Document *doc, uint64_t handle, const Point *value);
/* Closed is 0 or 1; widths are nonnegative. NULL vertices is allowed for a
 * zero-length input or a (NULL,0) output size query. get always requires
 * required/closed/elevation, and fills these even for BUFFER_TOO_SMALL. */
AR_API Status ar_lwpolyline_add(Document *doc, const LwVertex *vertices, size_t count, uint8_t closed, double elevation, uint64_t *out_handle);
AR_API Status ar_lwpolyline_get(const Document *doc, uint64_t handle, LwVertex *vertices, size_t capacity, size_t *required, uint8_t *closed, double *elevation);
AR_API Status ar_lwpolyline_set(Document *doc, uint64_t handle, const LwVertex *vertices, size_t count, uint8_t closed, double elevation);

AR_API Status ar_entity_get_layer(const Document *doc, uint64_t handle, char **out_layer);
AR_API Status ar_entity_set_layer(Document *doc, uint64_t handle, const char *layer);
/* Entity ACI: 0=ByBlock, 1..255=palette, 256=ByLayer. Getter returns
 * TYPE_MISMATCH for true-color RGB; full color data is available via JSON. */
AR_API Status ar_entity_get_color_index(const Document *doc, uint64_t handle, int16_t *out_index);
AR_API Status ar_entity_set_color_index(Document *doc, uint64_t handle, int16_t index);

AR_API Status ar_layer_count(const Document *doc, size_t *out_count);
/* Layer names are returned as owned strings. Index is zero-based. */
AR_API Status ar_layer_name(const Document *doc, size_t index, char **out_name);
/* Layer colors must be in 1..255. Duplicate names return INVALID_ARGUMENT. */
AR_API Status ar_layer_add(Document *doc, const char *name, int16_t color_index);
AR_API Status ar_layer_get_color_index(const Document *doc, const char *name, int16_t *out_index);
AR_API Status ar_layer_set_color_index(Document *doc, const char *name, int16_t index);
AR_API Status ar_layer_get_flags(const Document *doc, const char *name, uint32_t *out_flags);
AR_API Status ar_layer_set_flags(Document *doc, const char *name, uint32_t flags);

#ifdef __cplusplus
}
#endif

#include "acadrust_views.h"
#include "acadrust_mutations.h"
#include "acadrust_io.h"
#include "acadrust_entities.h"
#include "acadrust_database.h"
#include "acadrust_acis.h"
#include "acadrust_writable.h"
#endif
