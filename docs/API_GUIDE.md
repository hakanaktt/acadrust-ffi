# acadrust-ffi API guide

This guide describes the C ABI exposed by `acadrust-ffi` 0.1.0, pinned to
`acadrust` 0.6.5. It is the companion to the public headers:

For detailed reader, writer, importer, callback, and diagnostic examples, see
the [I/O API guide](API_IO.md).

- [`acadrust.h`](../include/acadrust.h) contains the core ABI, document I/O,
  common geometry records, layers, and status values.
- [`acadrust_views.h`](../include/acadrust_views.h) contains zero-copy native
  views and document-level reports.
- [`acadrust_mutations.h`](../include/acadrust_mutations.h) contains typed
  native property mutation.
- [`acadrust_writable.h`](../include/acadrust_writable.h) contains exclusive
  writable spans into native allocations.
- [`acadrust_entities.h`](../include/acadrust_entities.h) contains the complete
  entity catalog and geometry operations.
- [`acadrust_database.h`](../include/acadrust_database.h) contains tables,
  objects, classes, layouts, cloning, and ownership operations.
- [`acadrust_io.h`](../include/acadrust_io.h) contains configured readers,
  recovery, importers, DWG inspection, and callbacks.
- [`acadrust_acis.h`](../include/acadrust_acis.h) contains SAT/SAB model access
  and ACIS primitive builders.

The umbrella header includes all of these files. Include `acadrust.h` in normal
applications. Include a narrower header when you want to document a smaller
dependency surface.

## Design goals

The FFI exposes the full upstream data model through an opaque, stable C ABI.
Rust record layouts and enum layouts are deliberately hidden. Native data is
read through `ValueView` and edited through validated mutation APIs. Numeric
arrays, byte arrays, and fixed-length strings can also be borrowed directly for
writable access.

The main access path does not serialize entities or objects to JSON. JSON
functions remain for compatibility with earlier releases and for interchange
with code that already owns a JSON library. They are not needed for complete
native traversal or mutation.

The API uses handles for persistent object identity. Handles are 64-bit values
unique within a document. A handle is not a pointer and does not identify an
object across independently loaded documents.

## ABI and basic types

All exported functions use the C ABI and the `ar_` prefix. The ABI version is
returned by `ar_abi_version()` and is currently `1`. The library version and
upstream version are borrowed static strings returned by `ar_version()` and
`ar_upstream_version()`.

```c
#include "acadrust.h"

if (ar_abi_version() != 1) {
    /* Reject an incompatible header/library pair. */
}
```

The public scalar records are ordinary C layouts:

```c
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
```

Coordinates and distances use drawing units. Angles use radians. Strings and
file paths passed to the library are NUL-terminated UTF-8. Returned owned C
strings are NUL-terminated UTF-8. Returned `StringView` values use a byte length
and do not need a NUL terminator.

The status type is a fixed-width signed 32-bit value. The constants are:

| Status | Meaning |
| --- | --- |
| `AR_OK` | The operation succeeded. |
| `AR_INVALID_ARGUMENT` | A pointer, flag, enum, path, version, or value was invalid. |
| `AR_NOT_FOUND` | A handle, table entry, object, layer, or property was not found. |
| `AR_TYPE_MISMATCH` | The requested operation does not apply to that native type. |
| `AR_IO_ERROR` | File-system or stream I/O failed. |
| `AR_PARSE_ERROR` | Input or an upstream model could not be parsed. |
| `AR_BUFFER_TOO_SMALL` | The caller supplied an insufficient output array. |
| `AR_PANIC` | Rust unwinding was caught at the FFI boundary. |

After every nonzero status, call `ar_last_error()` immediately. The returned
message is borrowed from thread-local storage and is replaced by the next
status-returning call on that thread. A successful status clears it. Version,
free, and error-query functions preserve it.

```c
Status status = ar_document_new(AR_VERSION_2018, &doc);
if (status != AR_OK) {
    fprintf(stderr, "%s\n", ar_last_error());
}
```

`AR_PANIC` means the affected Rust operation may have stopped part way through
a mutation. Discard that document instead of continuing to use it. Invalid
pointers, invalid buffer lengths, concurrent mutation, allocator aborts, and
use-after-free are outside the recoverable status model.

## Ownership and lifetime rules

Owned values must be released by the matching function:

| Value | Release function |
| --- | --- |
| `Document *` | `ar_document_free` |
| `char *` returned by an `ar_*` output | `ar_string_free` |
| `Buffer` contents | `ar_buffer_free` |
| `ReadReport *` | `ar_read_report_free` |
| `DwgInspection *` | `ar_dwg_inspection_free` |
| `AcisDocument *` | `ar_acis_free` |
| `NativeResult *` | `ar_result_free` |

Initialize owned output slots before calls:

```c
Document *doc = NULL;
char *name = NULL;
Buffer buffer = {0};
ReadReport *report = NULL;
```

The library initializes output slots to `NULL` or zero on entry where the API
contract says it does. Free an existing owned result before reusing its slot.
Free functions accept `NULL`; `ar_buffer_free` also resets the buffer to
`{0}`. Do not free borrowed strings, borrowed views, or pointers returned from
`ar_value_data`.

Do not access a document concurrently with mutation or freeing. Separate
documents can be read or mutated on separate threads when the application
provides its own synchronization for each document. The library's batch I/O
pool safely processes independent documents and paths.

## Creating, cloning, and releasing documents

The core document functions are:

```c
Status ar_document_new(uint32_t version, Document **out);
Status ar_document_clone(const Document *doc, Document **out);
void ar_document_free(Document *doc);
Status ar_document_get_version(const Document *doc, uint32_t *out);
Status ar_document_set_version(Document *doc, uint32_t version);
```

Supported version codes are:

| Macro | Code | AutoCAD family |
| --- | ---: | --- |
| `AR_VERSION_R12` | 1009 | R12 |
| `AR_VERSION_R13` | 1012 | R13 |
| `AR_VERSION_R14` | 1014 | R14 |
| `AR_VERSION_2000` | 1015 | 2000 |
| `AR_VERSION_2004` | 1018 | 2004 |
| `AR_VERSION_2007` | 1021 | 2007 |
| `AR_VERSION_2010` | 1024 | 2010 |
| `AR_VERSION_2013` | 1027 | 2013 |
| `AR_VERSION_2018` | 1032 | 2018+ |

Setting a version selects the export target. It does not prove that every
entity in the document is compatible with that target. Use
`ar_document_has_version_locked_data` when you need the upstream compatibility
check.

`ar_document_clone` creates an independent drawing. Entity storage uses
upstream copy-on-write internally, so the first mutation of a shared entity
may detach that entity while preserving the clone's value.

## File and memory I/O

The basic functions are:

```c
Status ar_document_read_file(const char *path, Document **out);
Status ar_document_read_buffer(const uint8_t *data, size_t len,
                               Format format, Document **out);
Status ar_document_write_file(const Document *doc, const char *path,
                              Format format);
Status ar_document_write_buffer(const Document *doc, Format format,
                                Buffer *out);
```

The format constants are `AR_FORMAT_AUTO`, `AR_FORMAT_DXF_ASCII`,
`AR_FORMAT_DXF_BINARY`, and `AR_FORMAT_DWG`. Input files and `AUTO` buffers
detect DWG by its `AC10` signature and otherwise use DXF detection. Output
always requires an explicit format. File output overwrites an existing file.

Readers copy caller-owned input before returning. Output buffers are owned by
the caller after success.

Configured readers and writers add recovery, reports, encodings, callbacks,
version override, and DWG compression control:

```c
ReadOptions read_options;
WriteOptions write_options;
ReadReport *report = NULL;
Document *doc = NULL;

ar_read_options_default(&read_options);
read_options.failsafe = 1;

Status status = ar_document_read_file_ex(
    "drawing.dxf", AR_FORMAT_AUTO, &read_options, &doc, &report);
if (status == AR_OK) {
    /* inspect report with ar_read_report_view */
}
ar_read_report_free(report);
ar_document_free(doc);
```

`ReadOptions` fields:

- `failsafe`: permit recoverable record errors and return a partial document.
- `recover`: enable failsafe and apply upstream conservative repair.
- `dxf_default_encoding`: code page for old DXF without an encoding declaration.
- `dwg_r12_code_page`: override R12 string decoding.
- `progress`: DWG progress callback receiving values from 0 through 1000.
- `legacy_font_resolver`: R12 font-name callback.
- `user_data`: application pointer passed to callbacks.

Callbacks must remain valid and thread-safe until the read returns. They must
not mutate or free the document. The font callback returns a borrowed string
that the library copies immediately. A visiting read additionally supplies a
`DwgEntityVisitCallback`; return `1` to retain an entity or `0` to omit it.
Visiting callbacks receive a temporary entity view that expires when the
callback returns. Visiting and recovery cannot be combined.

`ar_dwg_read_file_mmap` uses an OS read-only memory map for DWG input. The map
is released before the function returns. It is unavailable on WebAssembly.

Configured writers use:

```c
WriteOptions options;
ar_write_options_default(&options);
options.version = AR_VERSION_2013; /* zero means doc->version */
options.dwg_no_lz77 = 0;
ar_document_write_file_ex(doc, "out.dwg", AR_FORMAT_DWG, &options);
```

`dwg_no_lz77` is valid only for DWG version 1021. The source document is not
modified by a version override. Upstream may promote some binary DXF targets
for compatibility.

## Parallel I/O

Configure the dedicated worker pool with:

```c
ar_io_set_threads(0);       /* use available parallelism */
ar_io_set_threads(4);       /* use four workers */
ar_io_get_threads(&workers);
```

`ar_documents_read_files` copies all paths before worker threads start. Each
output document slot is initialized to `NULL`, and each status entry identifies
the corresponding result. Successful items remain owned even when another item
fails.

`ar_documents_write_files` requires all documents to stay alive and unchanged
until it returns. The paths should be distinct. The overall status is `AR_OK`
only when every item succeeds. The pool is separate from the host program's
global Rayon pool.

## Zero-copy native views

`ValueView` is an opaque pair of pointers. Never cast its members to a Rust or
C struct. `StringView` is a borrowed UTF-8 byte span:

```c
typedef struct StringView { const char *data; size_t len; } StringView;
typedef struct ValueView { const void *data; const void *descriptor; } ValueView;
```

Start from a document, entity, table, object, class, report, or inspection:

```c
ValueView document_view;
ar_document_view(doc, &document_view);

ValueView entity_view;
ar_entity_view(doc, handle, &entity_view);

ValueView common;
ar_value_field(entity_view, "common", &common);
```

The view functions are:

- `ar_value_kind`: return an `AR_VALUE_*` kind.
- `ar_value_type_name`: return a diagnostic Rust type name.
- `ar_value_len`: return child count for a struct, option, sequence, or map.
- `ar_value_child`: access a child by index and receive its borrowed name.
- `ar_value_field`: access a named struct field.
- `ar_value_variant`: return an enum variant name.
- `ar_value_data`: return a read-only contiguous scalar, array, or string span.
- `ar_value_read_integer`: read handles and integer-like flags without casting.

The kinds are `AR_VALUE_OPAQUE`, `STRUCT`, `ENUM`, `SEQUENCE`, `STRING`,
`BOOL`, the fixed-width integer and float kinds, `OPTION`, `MAP`, and `CHAR`.
Composite records remain opaque and must be traversed child by child.

Views never allocate and are never freed. A document view and every descendant
expires before any document mutation or free. Reacquire the view afterward. A
report view remains valid until its report is freed, and an inspection view
remains valid until its inspection is freed.

For maps, the native view exposes a key child followed by a value child for each
entry. Object-map order is unspecified. For options, an absent option has zero
children and a present option has one `value` child. For enum payloads, a tuple
payload is exposed as `v0`, `v1`, and so on; a single payload can also be
reached through `value` in mutation paths.

Document and database view entry points include:

```c
ar_document_entity_at_view(doc, index, &view);
ar_document_object_at_view(doc, index, &handle, &view);
ar_object_view(doc, handle, &view);
ar_class_view(doc, index, &view);
```

## Direct writable spans

Writable spans provide exclusive direct access to an existing numeric scalar,
numeric array, byte array, or fixed-length UTF-8 string allocation:

```c
WritableSpan span = {0};
Status status = ar_entity_borrow_mut(doc, handle,
                                    "Line/start/x", &span);
if (status == AR_OK) {
    *(double *)span.data = 25.0;
    status = ar_writable_release(&span);
}
```

The same operation exists for documents, objects, named table entries, and
indexed table entries:

- `ar_document_borrow_mut`
- `ar_entity_borrow_mut`
- `ar_object_borrow_mut`
- `ar_table_borrow_mut`
- `ar_table_entry_borrow_mut`
- `ar_writable_release`

The span reports `data`, element `len`, byte `stride`, scalar `kind`, and an
opaque release token. Do not modify the span metadata, resize the allocation,
free its pointer, copy the span, or use the pointer after release. No document
API may run while a span is active. `ar_document_free` intentionally does
nothing while a span is active; release the span first and call free again.

Strings are detached from their Rust `String` while borrowed. Their byte length
cannot change. Release validates UTF-8, floating-point finiteness, and entity
geometry, then restores the string and invalidates affected native caches. If
release fails, the span stays active and the pointer remains valid so the caller
can correct the data and retry.

Writable spans do not expose handles, ownership fields, enum discriminants,
flags with validity rules, indexed names, or raw storage metadata. Use native
mutation or structural APIs for those fields.

## Native property mutation

`NativeWrite` stages caller input before editing:

```c
typedef struct NativeWrite {
    ValueKind kind;
    const void *data;
    size_t len;
} NativeWrite;
```

The `AR_VALUE_*` kind must match the target. `len` is a byte count for strings
and an element count for numeric arrays. Floating-point writes must be finite;
boolean values must be `0` or `1`; character values must be valid Unicode
scalars. Input is copied before mutation, so the caller may reuse its input
buffer after the function returns.

Paths are slash-separated, with optional leading/trailing slashes. Enum variant
segments can be implicit (`start/x`), explicit (`Line/start/x`), or addressed as
`value`, `v0`, or an index. Options use `value`. Vector and array elements use
zero-based indices. Maps use their display key.

For documents use `ar_document_write`, `ar_document_resize`,
`ar_document_select`, and `ar_document_option`. For entities use the matching
`ar_entity_*` family; objects use `ar_object_*`; tables use `ar_table_*` or the
indexed `ar_table_entry_*` family; classes use `ar_class_*`.

The mutation families are:

| Operation | Effect |
| --- | --- |
| `*_write` | Replace a scalar, string, or vector/fixed-array value. |
| `*_resize` | Truncate or append default elements to a vector. |
| `*_select` | Select an enum variant and construct default payload fields. |
| `*_option` | Enable or clear an optional value. |

Identity, ownership, raw records, table names, and cache metadata are protected.
Entity edits preserve the entity handle and owner, validate the complete
candidate entity, and commit only after validation. Object edits refresh decoded
side views such as fields, dynamic-block visibility, context scales, and block
representations.

## Entity catalog and geometry

The entity catalog covers all upstream top-level variants and subtype families.
Use:

```c
size_t count;
ar_entity_kind_count(&count);
for (size_t i = 0; i < count; ++i) {
    StringView name, variant;
    ar_entity_kind_name(i, &name);
    ar_entity_kind_variant(i, &variant);
}
```

`ar_entity_kind_name` returns constructor names and `ar_entity_kind_variant`
returns the corresponding upstream enum variant. Names are case-insensitive in
`ar_entity_add_default`:

```c
uint64_t handle;
ar_entity_add_default(doc, "SPLINE", &handle);
```

Defaults may need geometry, layers, styles, blocks, or external references
before they can be exported successfully. `ar_entity_class_name` resolves the
actual DXF class name for extended and unknown entities.

The complete native entity view is available with `ar_entity_view`. The
geometry operations are:

- `ar_entity_bounding_box`
- `ar_entity_translate`
- `ar_entity_transform`
- `ar_entity_rotate`
- `ar_entity_scale`
- `ar_entity_mirror`
- `ar_entity_explode`

Transforms use a row-major `Matrix4` with translation in indices 3, 7, and 11;
the final row must be `[0, 0, 0, 1]`. Unsupported upstream entity classes return
`AR_TYPE_MISMATCH`. `ar_entity_explode` supports a query with `handles = NULL`
and `capacity = 0`; a short buffer returns `AR_BUFFER_TOO_SMALL` without adding
parts. Successful explosion assigns fresh handles and preserves the source
owner/block.

### Common typed geometry

The compact typed APIs are useful when a C program does not need the complete
entity model:

- Lines: `ar_line_add`, `ar_line_get`, `ar_line_set`.
- Circles: `ar_circle_add`, `ar_circle_get`, `ar_circle_set`.
- Arcs: `ar_arc_add`, `ar_arc_get`, `ar_arc_set`.
- Points: `ar_point_add`, `ar_point_get`, `ar_point_set`.
- Lightweight polylines: `ar_lwpolyline_add`, `ar_lwpolyline_get`,
  `ar_lwpolyline_set`.

Polyline vertices require finite coordinates, finite bulges, and nonnegative
start/end widths. The getter supports the usual size query and preserves the
output array on `AR_BUFFER_TOO_SMALL`.

`ar_entity_count`, `ar_entity_handles`, `ar_entity_type`, and
`ar_entity_remove` enumerate or remove entities by persistent handle. Removing
an entity also cleans its block membership and viewport-layout membership.

### Layers and entity appearance

Use `ar_entity_get_layer`, `ar_entity_set_layer`,
`ar_entity_get_color_index`, and `ar_entity_set_color_index` for common drawing
operations. ACI color indices use `1..255` for palette colors, `0` for ByBlock,
and `256` for ByLayer. True-color records require native views or JSON.

Layer functions are:

```c
ar_layer_count(doc, &count);
ar_layer_name(doc, index, &owned_name);
ar_layer_add(doc, "Annotations", 3);
ar_layer_get_color_index(doc, "Annotations", &color);
ar_layer_set_color_index(doc, "Annotations", 5);
ar_layer_get_flags(doc, "Annotations", &flags);
ar_layer_set_flags(doc, "Annotations", AR_LAYER_FROZEN | AR_LAYER_LOCKED);
```

The flag bits are `AR_LAYER_FROZEN`, `AR_LAYER_LOCKED`, and `AR_LAYER_OFF`.
`ar_layer_rename` updates name-based references and rejects protected names.

## Symbol tables, objects, classes, and layouts

The ten table kinds are:

| Constant | Native table |
| --- | --- |
| `AR_TABLE_LAYER` | Layer |
| `AR_TABLE_LINETYPE` | Linetype |
| `AR_TABLE_TEXT_STYLE` | Text style |
| `AR_TABLE_BLOCK_RECORD` | Block record |
| `AR_TABLE_DIM_STYLE` | Dimension style |
| `AR_TABLE_APP_ID` | Application ID |
| `AR_TABLE_VIEW` | Named view |
| `AR_TABLE_VPORT` | Viewport table |
| `AR_TABLE_UCS` | UCS |
| `AR_TABLE_VX` | Legacy VX record |

Read table counts and entries with `ar_table_count`, `ar_table_view`,
`ar_table_entry_view`, and `ar_table_find_view`. Create entries with
`ar_table_add`. Indexed access is important for duplicate imported viewport
names; name lookup selects the first matching entry.

`ar_layout_add` creates the backing paper-space block record, layout object,
and dictionary registration. Move entities by block name with
`ar_entity_move_to_block_name`, by layout name with
`ar_entity_move_to_layout`, or by block handle with
`ar_entity_move_to_block_handle`. Viewport lists and owner handles are kept in
sync. Structural markers and attribute subentities cannot move independently.

Object catalogs and operations are analogous:

```c
ar_object_kind_count(&count);
ar_object_kind_name(index, &name);
ar_object_kind_variant(index, &variant);
ar_object_add_default(doc, "FIELD", owner, &handle);
ar_object_clone(doc, source, owner, &clone);
ar_object_remove(doc, handle);
```

The catalog covers every upstream `ObjectType` variant, including dictionaries,
layouts, XRecords, fields, associative objects, dynamic blocks, context data,
underlays, materials, visual styles, DGN line styles, and proxy/unknown forms.
Object cloning is shallow with respect to referenced records. Opaque payloads
that cannot be safely reidentified return `AR_TYPE_MISMATCH`.

Class declarations use `ar_class_count`, `ar_class_view`, `ar_class_add`, and
`ar_class_remove`. Class metadata can be edited through the `ar_class_*`
mutation family. Class identity fields and positional references are protected.

Table/object/class removal checks known native references, including header
symbol names and extension-dictionary ownership. Unknown vendor payloads cannot
be inspected with certainty; deletion may therefore conservatively reject a
record or leave references that upstream itself cannot model.

## Document features and reports

`NativeResult` owns a native report or graph. Obtain its borrowed view with
`ar_result_view` and release it with `ar_result_free`.

Document-wide operations include:

- `ar_document_view`, entity/object/class/table views.
- `ar_document_audit` and `ar_document_repair`.
- `ar_layer_states`, `ar_layer_state_capture`, `ar_layer_state_restore`,
  `ar_layer_state_rename`, and `ar_layer_state_remove`.
- `ar_document_export_block` and `ar_document_import_block_graph`.
- `ar_entity_solid_history` with modes 0 (graph), 1 (tree), and 2 (operations).
- `ar_dynamic_block_properties`; pass `insert = 0` for definition defaults.
- `ar_document_allocate_handle`, `ar_document_resolve_references`, and
  `ar_document_has_version_locked_data`.
- `ar_document_drawing_variable` and `ar_document_set_drawing_variable`.
- `ar_document_extension_dictionary`, `ar_document_xrecord_view`, and
  `ar_document_ensure_xrecord`.

Audit, repair, read, and DWG inspection reports are traversed through the same
native `ValueView` API. The report owner must outlive all report views.

Layer-state captures store native AutoCAD layer-state data. Restoring a state
returns the number of layers affected. Layer-state names are case-insensitive
at the upstream layer-state API.

Block graph export/import carries dynamic-block object graphs, classes, and
side metadata to fresh handles. `HandleMapping` maps references outside the
exported graph to destination handles.

## ACIS and modeler data

`AcisDocument` owns a parsed SAT document. It can be viewed, parsed, written,
and released:

```c
AcisDocument *model = NULL;
ar_acis_primitive(AR_ACIS_SPHERE,
                  (Vector3){0, 0, 0},
                  (Vector3){10, 0, 0}, &model);

char *sat = NULL;
ar_acis_write_sat(model, &sat);
ar_string_free(sat);
ar_acis_free(model);
```

Primitive kinds are `AR_ACIS_BOX`, `WEDGE`, `PYRAMID`, `CYLINDER`, `CONE`,
`SPHERE`, and `TORUS`. Box and wedge use length/width/height. Pyramid uses
base size and height. Cylinder and cone use radius and height. Sphere uses
radius. Torus uses major and minor radius, with major greater than minor.

Use `ar_acis_parse_sat`, `ar_acis_parse_sab`, `ar_acis_write_sat`, and
`ar_acis_write_sab` for encoding conversion. `ar_entity_parse_acis` and
`ar_entity_set_acis` attach models to `SOLID3D`, `REGION`, `BODY`, and `SURFACE`
entities. SAT/SAB writing allocates owned output strings or buffers.

## JSON compatibility API

The original JSON functions are retained:

```c
ar_entity_get_json(doc, handle, &json);
ar_entity_add_json(doc, json, &new_handle);
ar_entity_set_json(doc, handle, replacement);
```

The schema is the upstream Serde representation, for example
`{"Line":{...}}`. `ar_entity_set_json` requires the same entity variant and
preserves handle and owner. `ar_entity_add_json` clears the incoming identity
and assigns a fresh handle. JSON omits some native storage metadata and is not
a lossless DWG archive.

Use native views when you need zero-copy access, raw records, private XDATA
records, or stable C traversal without a JSON dependency.

## Error-safe C patterns

### Read and write a drawing

```c
Document *doc = NULL;
Buffer bytes = {0};
Status status = ar_document_read_file("input.dxf", &doc);
if (status == AR_OK)
    status = ar_document_write_buffer(doc, AR_FORMAT_DWG, &bytes);
if (status != AR_OK)
    fprintf(stderr, "acadrust: %s\n", ar_last_error());
ar_buffer_free(&bytes);
ar_document_free(doc);
```

### Enumerate every entity without JSON

```c
size_t count = 0;
ar_entity_count(doc, &count);
uint64_t *handles = malloc(count * sizeof(*handles));
size_t required = 0;
if (handles)
    ar_entity_handles(doc, handles, count, &required);
for (size_t i = 0; i < required; ++i) {
    ValueView entity;
    ar_entity_view(doc, handles[i], &entity);
    /* Traverse entity with ar_value_field/ar_value_child. */
}
free(handles);
```

### Edit a native property

```c
double x = 42.0;
NativeWrite write = { AR_VALUE_F64, &x, 1 };
Status status = ar_entity_write(doc, handle, "Line/start/x", &write);
if (status != AR_OK)
    fprintf(stderr, "%s\n", ar_last_error());
```

### Borrow direct storage

```c
WritableSpan span = {0};
if (ar_entity_borrow_mut(doc, handle, "Line/start/x", &span) == AR_OK) {
    *(double *)span.data = 42.0;
    if (ar_writable_release(&span) != AR_OK) {
        /* The span remains active; repair data and retry. */
    }
}
```

### Traverse a report

```c
NativeResult *result = NULL;
if (ar_document_audit(doc, &result) == AR_OK) {
    ValueView report;
    ar_result_view(result, &report);
    ValueView issues;
    ar_value_field(report, "issues", &issues);
    ar_result_free(result);
}
```

## Threading and callbacks

The document object is not internally synchronized. Do not call a mutating API
concurrently with any view, read, write, or free operation on the same document.
The per-call Rayon pool used by batch I/O is safe for independent documents.

DWG progress callbacks may execute on worker threads. They must be reentrant and
must not call back into the document being read. Visiting callbacks receive
temporary native views and must finish using them before returning.

## Build, link, and verify

Build the library:

```sh
cargo build --release --locked
```

Run Rust verification:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

Run the C and C++ ABI checks:

```sh
cmake -S . -B build
cmake --build build --config Release
ctest --test-dir build -C Release --output-on-failure
```

On Windows, the shared library is `acadrust_ffi.dll` and the import library is
`acadrust_ffi.dll.lib`. On Linux it is `libacadrust_ffi.so`; on macOS it is
`libacadrust_ffi.dylib`. Static consumers define `AR_STATIC` and link the native
Rust system libraries reported by `cargo rustc --release -- --print
native-static-libs`.

## Compatibility and limitations

The FFI follows acadrust 0.6.5 for format fidelity, entity availability,
version conversion, geometry behavior, and unsupported operations. It does not
add a separate CAD kernel. Some empty constructor defaults require references
or geometry before a file writer can emit a valid record. Unsupported entity
transformations return `AR_TYPE_MISMATCH`.

The schema and type-name strings exposed by native views are tied to the pinned
upstream version. The C ABI records and status constants are the stable boundary;
Rust diagnostic type names are for traversal and diagnostics, not for long-term
string-based version negotiation.

Unknown and vendor-specific records may contain references that upstream cannot
decode. Removal APIs protect every modeled reference and conservatively reject
some opaque cases, but cannot prove the absence of references hidden in unknown
payloads.

Licensed under MPL-2.0, as is acadrust.

## Complete function-name inventory

The following names complete the family inventory above. They are listed here
so generated bindings, language wrappers, and API documentation tools can check
the public surface without parsing prose.

```text
ar_acis_view
ar_class_write ar_class_resize ar_class_select ar_class_option
ar_document_read_buffer_ex ar_document_write_buffer_ex
ar_document_import_file ar_document_import_buffer
ar_dwg_read_file_visiting ar_dwg_read_buffer_visiting
ar_dwg_inspect_file ar_dwg_inspect_buffer ar_dwg_inspection_view
ar_dwg_section_file ar_dwg_section_buffer
ar_import_options_default
ar_entity_clone ar_entity_write ar_entity_resize ar_entity_select ar_entity_option
ar_object_count ar_object_handle ar_object_write ar_object_resize ar_object_select ar_object_option
ar_table_remove ar_table_remove_at
ar_table_write ar_table_resize ar_table_select ar_table_option
ar_table_entry_write ar_table_entry_resize ar_table_entry_select ar_table_entry_option
```

Together with the function declarations in the preceding sections, this is the
complete exported `ar_*` function set in ABI version 1. `ar_abi_version` should
be checked before a wrapper uses a newer inventory.
