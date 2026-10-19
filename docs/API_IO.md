# acadrust-ffi I/O API guide

This chapter documents the complete native I/O surface in **acadrust-ffi
0.1.0**, built against the pinned **acadrust 0.6.5**. It covers basic
read/write calls, configurable readers and writers, memory-mapped DWG input,
DWG entity visitors, 3-D import, DWG metadata and section inspection, and
parallel batch helpers.

The API is a C99 ABI. Include `acadrust.h` (which includes `acadrust_io.h` and
the view declarations), link the matching library, and use headers from the
same release and architecture. Core document, entity, object, table, mutation,
and writable-storage contracts are in sibling [`API_GUIDE.md`](API_GUIDE.md).

## What this API owns

The I/O entry points exchange opaque `Document *`, `ReadReport *`,
`DwgInspection *`, and `Buffer` values. A successful function returns ownership
to the caller; a failed function leaves an output object `NULL` or a `Buffer`
set to `{ NULL, 0 }`. Initialize every output before the call and release it
with the matching function:

| Value | Release function | Notes |
| --- | --- | --- |
| `Document *` | `ar_document_free` | Also releases documents returned by batch reads. |
| `ReadReport *` | `ar_read_report_free` | Independent of the document lifetime. |
| `DwgInspection *` | `ar_dwg_inspection_free` | Views borrow this inspection. |
| `Buffer` | `ar_buffer_free` | Resets the structure to `{ NULL, 0 }`. |
| strings from another API | `ar_string_free` | I/O APIs do not return owned C strings. |

Free functions accept `NULL`; do not free an object twice or with a general C
allocator. `Buffer.data` is owned by the library and must not be modified or
freed directly. Input path strings, option structures, callback pointers, and
input byte arrays are borrowed only for the duration of the call. Reader,
importer, and section extraction code copies or decodes what it needs and never
retains caller-owned input memory.

The exception to the usual output lifetime is a borrowed view. `ValueView` and
its descendants are read-only native views. Traversal allocates no JSON or
temporary serialized representation. A report view remains valid until its
`ReadReport` is freed; an inspection view remains valid until its
`DwgInspection` is freed. A document view is invalidated by any document
mutation or by freeing the document. See [native view traversal](#native-view-traversal)
for the exact rules.

All strings and paths are NUL-terminated UTF-8 on every platform. `StringView`
values returned through views carry a byte length and do not need a trailing
NUL. Angles are radians, and coordinates/distances use drawing units.

## Status and error handling

Every status-returning function uses one of these fixed-width values:

| Status | Meaning |
| --- | --- |
| `AR_OK` | Operation completed. |
| `AR_INVALID_ARGUMENT` | A required pointer, flag, enum, encoding, or numeric option is invalid. |
| `AR_NOT_FOUND` | A file, section, entity, or other named record was not found. |
| `AR_TYPE_MISMATCH` | A valid handle or view has the wrong native type. |
| `AR_IO_ERROR` | Operating-system file or stream I/O failed. |
| `AR_PARSE_ERROR` | The input is malformed or cannot be decoded. |
| `AR_BUFFER_TOO_SMALL` | A caller-provided output array is too small; the required length is reported where the API defines one. |
| `AR_PANIC` | A Rust panic was caught at the ABI boundary. |

Call `ar_last_error()` immediately after a nonzero status if a diagnostic string
is needed. It returns a borrowed thread-local NUL-terminated string. The next
status-returning call on that thread replaces it; a successful status clears it.
Version and free functions preserve it. A panic status requires discarding the
affected document because a mutation or decode may be incomplete. Invalid
pointers, concurrent use, and allocator aborts are outside the recoverable
status contract.

For a nonzero status, output slots are normally initialized before validation.
The extended readers explicitly set both `Document **` and `ReadReport **` to
`NULL`; batch readers set each document to `NULL` and each status to
`AR_PANIC` before starting work. Never interpret a partially initialized output
as valid unless the function's contract says so.

## Formats and version constants

Input and output format values are the `Format` constants in `acadrust.h`:

```c
#define AR_FORMAT_AUTO       ((Format)0)
#define AR_FORMAT_DXF_ASCII  ((Format)1)
#define AR_FORMAT_DXF_BINARY ((Format)2)
#define AR_FORMAT_DWG        ((Format)3)
```

`AR_FORMAT_AUTO` is valid for reading only. Content detection recognizes a DWG
signature (`AC10`); other input is passed to the DXF reader, which handles ASCII
and binary DXF. Output calls require an explicit DXF ASCII, DXF binary, or DWG
format.

Supported numeric export versions are:

| Constant | AC code | Common name |
| --- | ---: | --- |
| `AR_VERSION_R12` | 1009 | R12 |
| `AR_VERSION_R13` | 1012 | R13 |
| `AR_VERSION_R14` | 1014 | R14 |
| `AR_VERSION_2000` | 1015 | R2000 |
| `AR_VERSION_2004` | 1018 | R2004 |
| `AR_VERSION_2007` | 1021 | R2007 |
| `AR_VERSION_2010` | 1024 | R2010 |
| `AR_VERSION_2013` | 1027 | R2013 |
| `AR_VERSION_2018` | 1032 | R2018 |

Entity fidelity and version restrictions are those of acadrust 0.6.5. Setting a
document version does not validate every entity for compatibility. The upstream
writer promotes R2000/R2004 binary DXF to R2010 when required by its
compatibility path. Explicit R12 DXF export uses the upstream R12 ASCII
conversion path. A document's source version is not changed merely by writing it
with another `WriteOptions.version`.

## Basic I/O

The basic calls are the smallest native path for normal applications. They use
strict upstream readers and the document's current version for writing.

### `ar_document_read_file`

```c
Status ar_document_read_file(const char *path, Document **out);
```

`path` is a UTF-8 file path. The reader detects DWG, ASCII DXF, or binary DXF
from the file contents and returns a newly owned document. Set `*out = NULL`
before calling; free a successful result with `ar_document_free`. The extended
file reader records its copied path in document source metadata; this basic
entry point does not add that metadata.

### `ar_document_read_buffer`

```c
Status ar_document_read_buffer(const uint8_t *data, size_t len,
                               Format format, Document **out);
```

`data` must be non-`NULL`, `len` must be nonzero and no larger than
`isize::MAX`, and `format` may be `AR_FORMAT_AUTO` or an explicit input format.
The reader copies the byte range before parsing, so the caller may reuse the
input after the function returns. The copied bytes are not a zero-copy view of
the final document; decoding allocates native document storage.

### `ar_document_write_file`

```c
Status ar_document_write_file(const Document *doc, const char *path,
                              Format format);
```

`format` must be explicit. The destination is overwritten. The document is read
only for the call and is not modified. On an I/O error a destination may be
partially written, depending on the upstream writer and operating system.

### `ar_document_write_buffer`

```c
Status ar_document_write_buffer(const Document *doc, Format format,
                                Buffer *out);
```

The serialized bytes are returned in a newly owned `Buffer`. Initialize
`*out = (Buffer){0}` and release it with `ar_buffer_free`. No caller buffer is
used as a destination and no input memory is retained.

Short source-level macros map to the same ABI symbols:

```c
#define ar_new        ar_document_new
#define ar_clone      ar_document_clone
#define ar_free       ar_document_free
#define ar_read_file  ar_document_read_file
#define ar_read       ar_document_read_buffer
#define ar_write_file ar_document_write_file
#define ar_write      ar_document_write_buffer
```

## Configurable reading

The extended readers add options and a native report. They do not serialize a
report to JSON. `ar_read_report_view` exposes the upstream report structures
directly through `ValueView`.

### `ReadOptions`

Initialize with `ar_read_options_default`, or pass `NULL` to use the same
defaults:

```c
typedef struct ReadOptions {
    uint8_t failsafe;              /* 0 by default */
    uint8_t recover;               /* 0 by default */
    const char *dxf_default_encoding;
    const char *dwg_r12_code_page;
    ReadProgressCallback progress;
    LegacyFontCallback legacy_font_resolver;
    void *user_data;
} ReadOptions;
```

`failsafe` and `recover` must be exactly `0` or `1`. The strings may be `NULL`
or a supported code-page name such as `ANSI_1252`. An unsupported code page is
rejected before input parsing. `dxf_default_encoding` is used for pre-2007 DXF
that does not declare an encoding. `dwg_r12_code_page` overrides R12 string
decoding while preserving the declared header code page.

Strict mode is the default (`failsafe=0`, `recover=0`). Failsafe mode permits
partial parsing and records diagnostics. Recovery enables the upstream
conservative repair path in addition to partial parsing; it returns a
`ReadReport.repair` value when repair succeeds. Recovery fails with
`AR_PARSE_ERROR` when no usable drawing records survive. Options and option
strings are copied before worker work begins and are not retained.

```c
ReadOptions options;
Status s = ar_read_options_default(&options);
if (s != AR_OK) {
    fprintf(stderr, "defaults: %s\n", ar_last_error());
    return 1;
}
options.failsafe = 1;
options.dxf_default_encoding = "ANSI_1252";
```

### Extended read calls

```c
Status ar_document_read_file_ex(const char *path, Format format,
                                const ReadOptions *options,
                                Document **out, ReadReport **out_report);

Status ar_document_read_buffer_ex(const uint8_t *data, size_t len,
                                  Format format, const ReadOptions *options,
                                  Document **out, ReadReport **out_report);
```

Both outputs are initialized to `NULL`. On success, both are newly owned and
must be released independently. A report can be kept after freeing its
document. A parse or option error leaves both outputs `NULL`.

The file reader copies the path into the returned document's source metadata.
The buffer reader copies `data` before parsing. `AR_FORMAT_AUTO` has the same
signature detection described for basic input. The reader runs in the FFI's
dedicated I/O pool; progress callbacks for DWG can therefore run on worker
threads.

### Progress callback

```c
typedef void (*ReadProgressCallback)(uint16_t progress, void *user_data);
```

Progress values are monotonic in the inclusive range `0..1000`. The callback is
used only for DWG input and may run concurrently on DWG worker threads. Keep
the function and `user_data` valid until the read returns. The callback must be
thread-safe, return normally, and must not mutate or free the drawing being
read. It is not retained after the call.

```c
#include <stdatomic.h>

struct ProgressState { _Atomic uint16_t last; };

static void on_progress(uint16_t value, void *opaque)
{
    struct ProgressState *state = opaque;
    atomic_store_explicit(&state->last, value, memory_order_relaxed);
}
```

The callback is a notification mechanism, not a cancellation mechanism. There
is no supported way to abort a reader from the callback.

### Legacy R12 font callback

```c
typedef const char *(*LegacyFontCallback)(const char *font,
                                          void *user_data);
```

This callback is used only while reading DWG R12 legacy font references. `font`
is a borrowed UTF-8 string valid for the callback invocation. Return a borrowed
NUL-terminated UTF-8 name; the reader copies it immediately. Returning `NULL`,
an invalid UTF-8 string, or no callback keeps the stored name. Resolution does
not read font files. Keep callback state and the returned string alive until
the callback returns.

### `ReadReport` and its statistics

```c
Status ar_read_report_view(const ReadReport *report, ValueView *out);
void ar_read_report_free(ReadReport *report);
```

The root report view is a struct with `stats` and `repair` fields. `stats` is a
native `ReadStats`; `repair` is an option containing zero or one upstream
`RepairReport` value. For strict/failsafe reads it is empty. Recovery reads
populate it when the repair pass completes.

`ReadStats` exposes these fields:

| Field | Meaning |
| --- | --- |
| `source_format` | Option containing `Dwg` or `Dxf`. |
| `source_version` | Native version string, for example `AC1021` or an AC code representation. |
| `maintenance_version` | AutoCAD maintenance version byte. |
| `recovery_mode` | Reader used failsafe/recovery parsing. |
| `stream_completed` | The source stream reached its normal end. A truncated input can be usable while this is false. |
| `source_sections` | Sections encountered by the reader. |
| `source_records` | Records advertised or discovered in the source. |
| `decoded_source_records` | Records decoded into native values. |
| `skipped_source_records` | Records skipped because of failsafe handling or unsupported/corrupt data. |
| `output_entities` | Entities in the returned document, including block/paper space. |
| `output_objects` | Native object records in the returned document. |
| `output_table_records` | Records across all ten supported tables. |
| `recovered_errors` | Recovered-error notifications attached to the native document/report. |
| `record_stream_read` | The record stream was entered/read successfully. |
| `diagnostics` | A sequence of native `ReadDiagnostic` values. |

Each `ReadDiagnostic` has `code`, `stage`, optional `section`, optional
`source_offset` and `source_offset_basis`, optional DXF `source_line`, optional
`record_handle` and `record_type`, and a UTF-8 `message`. Stages are
`FileHeader`, `Classes`, `Header`, `Handles`, `Objects`, `RecordStream`, and
`Section`. The upstream safety limit is 1,000 diagnostics; after the limit a
`diagnostic-limit-reached` entry is emitted. Messages are bounded by the
upstream diagnostic limit.

The following helper walks diagnostics without allocating a report copy. It
uses `ar_value_field`, `ar_value_len`, `ar_value_child`, and a length-aware
`StringView` printer:

```c
static void print_view_string(ValueView value)
{
    const void *data = NULL;
    size_t len = 0, stride = 0;
    if (ar_value_data(value, &data, &len, &stride) == AR_OK && stride == 1)
        fwrite(data, 1, len, stdout);
}

static void print_diagnostics(const ReadReport *report)
{
    ValueView root = {0}, stats = {0}, diagnostics = {0};
    size_t count = 0;
    if (ar_read_report_view(report, &root) != AR_OK ||
        ar_value_field(root, "stats", &stats) != AR_OK ||
        ar_value_field(stats, "diagnostics", &diagnostics) != AR_OK ||
        ar_value_len(diagnostics, &count) != AR_OK)
        return;

    for (size_t i = 0; i < count; ++i) {
        ValueView item = {0}, message = {0};
        StringView name = {0};
        if (ar_value_child(diagnostics, i, &name, &item) == AR_OK &&
            ar_value_field(item, "message", &message) == AR_OK) {
            printf("diagnostic[%zu]: ", i);
            print_view_string(message);
            putchar('\n');
        }
    }
}
```

`name` is a borrowed child label and can be ignored when iterating a sequence.
Always check each status and use `ar_last_error()` for failures.

### Report ownership example

```c
Document *doc = NULL;
ReadReport *report = NULL;
ReadOptions options;
if (ar_read_options_default(&options) != AR_OK)
    return 1;

Status s = ar_document_read_file_ex("drawing.dwg", AR_FORMAT_AUTO,
                                   &options, &doc, &report);
if (s != AR_OK) {
    fprintf(stderr, "read failed: %s\n", ar_last_error());
    /* The extended reader leaves both outputs NULL on failure. */
    return 1;
}

print_diagnostics(report);
ar_document_free(doc);       /* report remains valid */
ar_read_report_free(report);  /* invalidates all report views */
```

## Memory-mapped DWG input

```c
Status ar_dwg_read_file_mmap(const char *path, const ReadOptions *options,
                             Document **out, ReadReport **out_report);
```

This is the extended DWG reader backed by an OS read-only memory map. All
`ReadOptions`, recovery, callbacks, outputs, and report rules remain the same.
The input file must not be modified or truncated while the call runs. The map
is unmapped before the function returns; the returned document and report own
their native decoded storage and do not refer to the mapping. The function is
unavailable on WebAssembly and returns `AR_INVALID_ARGUMENT` there.

Use mmap when the input is a native DWG file and avoiding a temporary input
copy matters. It does not make the resulting document a mapped, writable view
of the file, and it does not avoid allocations required to decode CAD records.

## DWG pass-2 entity visiting

```c
typedef uint8_t (*DwgEntityVisitCallback)(ValueView entity,
                                          void *user_data);

Status ar_dwg_read_file_visiting(const char *path,
                                 const ReadOptions *options,
                                 DwgEntityVisitCallback visitor,
                                 Document **out);

Status ar_dwg_read_buffer_visiting(const uint8_t *data, size_t len,
                                   const ReadOptions *options,
                                   DwgEntityVisitCallback visitor,
                                   Document **out);
```

The visitor receives each decoded entity before the reader stores it. Return
`1` to keep the entity or `0` to omit it. Any other return value is
`AR_INVALID_ARGUMENT`; the output document remains `NULL`. The callback is
serial on the reader thread. The `ValueView` and all descendants expire when
the callback returns, so never retain or mutate them. The reader also visits
upstream block and block-end framing entities; a visitor that filters by
variant should account for these callbacks.

The callback receives the same `ReadOptions.user_data` pointer as the progress
and legacy-font callbacks. Keep it valid for the entire read.

`ReadOptions.recover` must be zero. Visiting and repair are deliberately
separate: repair the completed document with the native document repair API
described in [`API_GUIDE.md`](API_GUIDE.md). Progress callbacks can still run
concurrently while the visitor runs. A file visit records the source path on
the returned document; a buffer visit copies the input first.

```c
static uint8_t keep_lines(ValueView entity, void *unused)
{
    StringView variant = {0};
    (void)unused;
    if (ar_value_variant(entity, &variant) != AR_OK)
        return 2; /* causes AR_INVALID_ARGUMENT */
    return (variant.len == 4 && memcmp(variant.data, "Line", 4) == 0);
}

Document *filtered = NULL;
ReadOptions options = {0};
Status s = ar_dwg_read_file_visiting("drawing.dwg", &options,
                                     keep_lines, &filtered);
if (s != AR_OK)
    fprintf(stderr, "%s\n", ar_last_error());
ar_document_free(filtered);
```

## Configurable writing

### `WriteOptions`

```c
Status ar_write_options_default(WriteOptions *out);

typedef struct WriteOptions {
    uint32_t version;       /* 0: use doc version */
    uint8_t dwg_no_lz77;    /* 0: normal DWG compression */
} WriteOptions;
```

Use `ar_write_options_default` or pass `NULL` to use `{ version=0,
dwg_no_lz77=0 }`. `version` must be zero or one of the `AR_VERSION_*` values.
`dwg_no_lz77` must be `0` or `1`. It is a diagnostic DWG mode supported only
for version `1021` (`AR_VERSION_2007`); using it with DXF, another version, or
an invalid flag returns `AR_INVALID_ARGUMENT` before writing.

```c
Status ar_document_write_buffer_ex(const Document *doc, Format format,
                                   const WriteOptions *options, Buffer *out);
Status ar_document_write_file_ex(const Document *doc, const char *path,
                                 Format format,
                                 const WriteOptions *options);
```

The extended writers serialize and validate before opening the destination
file. A successful file call overwrites the destination; a later operating
system write error can still leave a partial file. The source document and its
version remain unchanged. The buffer form returns owned bytes and initializes
`out` to `{0}` on entry.

## Native view traversal

Reports and inspections are intentionally exposed through the same native
`ValueView` API used for document data. This is the zero-copy path: no JSON
conversion, serde schema, or intermediate C struct is involved.

The relevant operations are:

```c
Status ar_value_kind(ValueView value, ValueKind *out);
Status ar_value_type_name(ValueView value, StringView *out);
Status ar_value_len(ValueView value, size_t *out);
Status ar_value_child(ValueView value, size_t index,
                      StringView *name, ValueView *out);
Status ar_value_field(ValueView value, const char *name, ValueView *out);
Status ar_value_variant(ValueView value, StringView *out);
Status ar_value_data(ValueView value, const void **data,
                     size_t *len, size_t *stride);
Status ar_value_read_integer(ValueView value, int64_t *out);
```

Use `ar_value_field` for struct fields, `ar_value_variant` for enum names,
`ar_value_child` for sequence/option/enum payloads, and `ar_value_len` before
iterating. A map has `2 * len` children: key then value for each entry; map
order is unspecified. An option has zero or one child. `ar_value_data` is valid
for primitive arrays and strings only; composite values must be traversed using
fields/children. Returned data is read-only and borrowed. `BOOL` is a byte
`0/1`; `CHAR` is a Unicode scalar `uint32_t`; `ar_value_read_integer` handles
integer and handle/flag wrappers without requiring a Rust layout cast.

The view root from `ar_read_report_view` is:

```text
ReadReport {
    stats: ReadStats,
    repair: Option<RepairReport>
}
```

The root from `ar_dwg_inspection_view` is one of:

```text
Header(DwgFileHeaderInfo)
Crcs(CrcExtractionReport)
```

Every descendant pointer expires when its owner is freed. Views into a report
remain valid after the document is freed, as demonstrated by the report
contract; inspection views likewise depend only on the inspection. Do not call
document mutation, writable-borrow, or free functions while using document
views. Reacquire document views after any mutation.

## 3-D import

The import API creates native `Mesh` entities from six upstream formats:

```c
typedef int32_t ImportFormat;
#define AR_IMPORT_AUTO    ((ImportFormat)0)
#define AR_IMPORT_STL     ((ImportFormat)1)
#define AR_IMPORT_COLLADA ((ImportFormat)2)
#define AR_IMPORT_OBJ     ((ImportFormat)3)
#define AR_IMPORT_GLTF    ((ImportFormat)4)
#define AR_IMPORT_GLB     ((ImportFormat)5)
#define AR_IMPORT_FBX     ((ImportFormat)6)
```

`AR_IMPORT_AUTO` is for the file API's extension detection only; buffer input
requires one explicit format. The file importer supports ASCII/binary STL,
COLLADA, OBJ/MTL, glTF, GLB, and ASCII/binary FBX as supported by acadrust
0.6.5. Relative material and buffer resources for file imports are resolved
from the input file directory. Buffer imports must be self-contained and use
embedded resources.

### `ImportOptions`

```c
Status ar_import_options_default(ImportOptions *out);

typedef struct ImportColor {
    int32_t mode;
    uint8_t index, r, g, b;
} ImportColor;

typedef struct ImportOptions {
    const char *layer_prefix;
    ImportColor default_color;
    uint8_t merge_vertices;
    double merge_tolerance;
    double scale_factor;
} ImportOptions;
```

Use `ar_import_options_default` or pass `NULL` for these defaults:

| Field | Default | Validation/meaning |
| --- | --- | --- |
| `layer_prefix` | `"imported"` | `NULL` selects the default. The importer may append a mesh/material suffix. |
| `default_color.mode` | `AR_IMPORT_COLOR_BY_LAYER` | See color modes below. |
| `merge_vertices` | `1` | Must be `0` or `1`. |
| `merge_tolerance` | `1e-9` | Must be finite and nonnegative. |
| `scale_factor` | `1.0` | Must be finite and nonzero; negative values are accepted by the upstream importer. |

Color modes are:

```c
#define AR_IMPORT_COLOR_BY_LAYER ((int32_t)0)
#define AR_IMPORT_COLOR_BY_BLOCK ((int32_t)1)
#define AR_IMPORT_COLOR_NONE     ((int32_t)2)
#define AR_IMPORT_COLOR_INDEX    ((int32_t)3)
#define AR_IMPORT_COLOR_RGB      ((int32_t)4)
```

For `AR_IMPORT_COLOR_INDEX`, `index` must be `1..255`; `r`, `g`, and `b` are
ignored. For `AR_IMPORT_COLOR_RGB`, the three color bytes are used and `index`
is ignored. The other modes ignore all color payload bytes. An invalid mode or
palette index is `AR_INVALID_ARGUMENT`.

### Import calls

```c
Status ar_document_import_file(const char *path,
                               const ImportOptions *options,
                               Document **out);

Status ar_document_import_buffer(const uint8_t *data, size_t len,
                                 ImportFormat format,
                                 const ImportOptions *options,
                                 Document **out);
```

Both output slots are set to `NULL` before validation. Buffer bytes and option
strings are copied and never retained. On success, release the document with
`ar_document_free`. The result is a normal native document that can be viewed,
mutated, written, and passed to the rest of the FFI.

```c
const uint8_t obj[] =
    "o triangle\n"
    "v 0 0 0\n"
    "v 1 0 0\n"
    "v 0 1 0\n"
    "f 1 2 3\n";
ImportOptions import_options;
ar_import_options_default(&import_options);
import_options.layer_prefix = "mesh";
import_options.scale_factor = 1000.0;
import_options.default_color = (ImportColor){
    AR_IMPORT_COLOR_RGB, 0, 12, 34, 56
};

Document *mesh_doc = NULL;
Status s = ar_document_import_buffer(obj, sizeof(obj) - 1, AR_IMPORT_OBJ,
                                     &import_options, &mesh_doc);
if (s != AR_OK)
    fprintf(stderr, "import: %s\n", ar_last_error());
ar_document_free(mesh_doc);
```



## DWG metadata inspection

Inspection reads metadata without reconstructing a `Document`:

```c
typedef int32_t DwgInspectKind;
#define AR_DWG_INSPECT_HEADER ((DwgInspectKind)0)
#define AR_DWG_INSPECT_CRCS   ((DwgInspectKind)1)

Status ar_dwg_inspect_file(const char *path, DwgInspectKind kind,
                           DwgInspection **out);
Status ar_dwg_inspect_buffer(const uint8_t *data, size_t len,
                             DwgInspectKind kind, DwgInspection **out);
Status ar_dwg_inspection_view(const DwgInspection *inspection,
                              ValueView *out);
void ar_dwg_inspection_free(DwgInspection *inspection);
```

The file and buffer forms initialize `*out` to `NULL`; the buffer form copies
the input. `kind` must be `HEADER` or `CRCS`.

### Header inspection

The `Header` payload is `DwgFileHeaderInfo` with these native fields:

* `version_string`, `version`, `acad_maintenance_version`, `preview_address`,
  `dwg_version`, `app_release_version`, `code_page`, `security_type`,
  `summary_info_addr`, and `vba_project_addr`;
* optional AC1021 values `ac21_metadata`, `ac21_header_crc`,
  `ac21_unknown_key`, and `ac21_compressed_data_crc`;
* `page_records`, `section_descriptors`, and `section_locators` maps/sequences;
* `objects_base_offset` and `is_ac18_format`.

`section_descriptors` entries contain `name`, compressed/decompressed sizes,
`encrypted`, `hash_code`, `encoding`, `page_count`, and per-page `pages`. A page
entry contains page number, offset, size, compressed/decompressed sizes,
checksum, and CRC. The upstream DWG version enum exposed through the view is
`AC9`, `AC12`, `AC15`, `AC18`, `AC21`, or `AC24`.

### CRC inspection

The `Crcs` payload is `CrcExtractionReport`. It contains the DWG `version`,
optional `header_crc64`, `header_crc`, `compressed_data_crc`, pages-map and
sections-map CRC values/seeds, `crc_seed`, `crc_seed_encoded`, `random_seed`,
`page_crcs`, and `notifications`. Each `page_crcs` entry contains
`section_name`, `page_number`, `crc`, `checksum`, `compressed_size`, and
`decompressed_size`.

CRC inspection extracts checksum fields stored in the file. It does not promise
that the checksums have been independently validated against every byte of the
file. Treat notifications and missing optional values as data, not as a
boolean integrity verdict.

Example inspection traversal:

```c
DwgInspection *inspection = NULL;
Status s = ar_dwg_inspect_file("drawing.dwg", AR_DWG_INSPECT_HEADER,
                              &inspection);
if (s == AR_OK) {
    ValueView root = {0}, header = {0}, version = {0};
    StringView child_name = {0};
    const void *data = NULL;
    size_t len = 0, stride = 0;
    ar_dwg_inspection_view(inspection, &root);
    /* Header is an enum payload at child 0. */
    ar_value_child(root, 0, &child_name, &header);
    ar_value_field(header, "version_string", &version);
    ar_value_data(version, &data, &len, &stride);
    if (stride == 1)
        printf("DWG version: %.*s\\n", (int)len, (const char *)data);
}
ar_dwg_inspection_free(inspection);
```

### DWG section extraction

```c
Status ar_dwg_section_file(const char *path, const char *section_name,
                           Buffer *out);
Status ar_dwg_section_buffer(const uint8_t *data, size_t len,
                             const char *section_name, Buffer *out);
```

These calls parse the DWG header, locate `section_name`, decompress it using
the upstream version-specific path, and return an owned byte buffer. They do
not construct a `Document` and do not return a borrowed pointer into the input.
Initialize and release `Buffer` with `ar_buffer_free`. Common names include
`"AcDb:Header"` and `"AcDb:AcDbObjects"`; use `section_descriptors` from a
header inspection for the exact names in a file. Unknown sections, malformed
headers, unsupported encodings, and I/O failures return a status and leave the
buffer empty.

```c
Buffer section = {0};
Status s = ar_dwg_section_file("drawing.dwg", "AcDb:Header", &section);
if (s != AR_OK) {
    fprintf(stderr, "section: %s\\n", ar_last_error());
} else {
    fwrite(section.data, 1, section.len, stdout);
}
ar_buffer_free(&section);
```

## Parallel batch I/O

The batch calls use a dedicated Rayon pool owned by the FFI. They do not modify
the host application's global Rayon pool. While a batch or single-file I/O
operation runs in this pool, acadrust's internal DWG page/object parallel work
uses the same configured pool.

### Pool configuration

```c
Status ar_io_set_threads(size_t threads);
Status ar_io_get_threads(size_t *out_threads);
```

`ar_io_set_threads(0)` selects the platform's available parallelism. A positive
value selects exactly that many workers; values too large for the platform are
rejected. Existing calls continue on their old pool; later calls observe the
replacement. `ar_io_get_threads` reports the active pool's worker count and may
be called while other calls are running.

### Batch reads

```c
Status ar_documents_read_files(const char *const *paths, size_t count,
                               Document **documents, Status *statuses);
```

`paths`, `documents`, and `statuses` each have `count` entries. For
`count > 0`, the corresponding arrays must be non-`NULL`; for zero count,
`NULL` arrays are allowed. Each path is copied before workers start, so the
caller may change its path storage after the call returns. A null path element
is an invalid argument.

Before work begins, every `documents[i]` is set to `NULL` and every
`statuses[i]` to `AR_PANIC`. Each successful item replaces its document slot
with a newly owned document and sets its status to `AR_OK`. A failed item leaves
its document slot `NULL` and records its own status. The overall return is
`AR_OK` only if every item succeeds; otherwise it is the first failing item in
input order. Successful documents remain owned by the caller even when the
overall status is an error.

```c
const char *paths[] = { "one.dwg", "two.dxf", "missing.dwg" };
Document *documents[3] = {0};
Status statuses[3];
Status overall = ar_documents_read_files(paths, 3, documents, statuses);
for (size_t i = 0; i < 3; ++i) {
    if (statuses[i] == AR_OK)
        ar_document_free(documents[i]);
    else
        fprintf(stderr, "read[%zu] failed with status %d\\n",
                i, (int)statuses[i]);
}
```

The batch worker keeps errors per item and writes statuses in input order even
though execution is concurrent. `ar_last_error()` after the overall call
describes the overall/first error; use the per-item status to decide which
documents are valid.

### Batch writes

```c
Status ar_documents_write_files(const Document *const *documents,
                                const char *const *paths, size_t count,
                                Format format, Status *statuses);
```

The arrays have `count` entries and `format` must be an explicit output format.
All document pointers must remain alive and must not be mutated for the entire
call. Paths should be distinct; concurrent writes to one path race at the file
system level. Paths are copied before workers start. Each `statuses[i]` reports
that output only, and the overall status is `AR_OK` only when every write
succeeds (otherwise the first item error in input order). A failed output may
be absent or partially written according to the file system and writer.

```c
const Document *docs[] = { first, second };
const char *outputs[] = { "first.dxf", "second.dxf" };
Status statuses[2];
Status overall = ar_documents_write_files(docs, outputs, 2,
                                          AR_FORMAT_DXF_ASCII, statuses);
```

The batch helpers parallelize independent files, not mutation of one document.
Use separate documents or external synchronization when a host application
needs concurrent document processing.

## A complete native read/report/cleanup pattern

The following C99 pattern combines options, a report, native diagnostics, and
correct cleanup. It deliberately never calls JSON APIs:

```c
#include <stdio.h>
#include "acadrust.h"

static void show_read(const ReadReport *report)
{
    ValueView root = {0}, stats = {0}, value = {0};
    size_t stride = 0, len = 0;
    const void *data = NULL;
    if (ar_read_report_view(report, &root) != AR_OK ||
        ar_value_field(root, "stats", &stats) != AR_OK ||
        ar_value_field(stats, "output_entities", &value) != AR_OK ||
        ar_value_data(value, &data, &len, &stride) != AR_OK)
        return;
    if (len == 1)
        printf("output_entities scalar (stride %zu bytes)\\n", stride);
    (void)data;
}

int read_one(const char *path)
{
    Document *doc = NULL;
    ReadReport *report = NULL;
    ReadOptions options;
    Status s = ar_read_options_default(&options);
    if (s != AR_OK)
        return 1;

    options.failsafe = 1;
    s = ar_document_read_file_ex(path, AR_FORMAT_AUTO, &options,
                                 &doc, &report);
    if (s != AR_OK) {
        fprintf(stderr, "read failed (%d): %s\\n", (int)s, ar_last_error());
        ar_document_free(doc);
        ar_read_report_free(report);
        return 1;
    }

    show_read(report);
    /* Use document views/mutations here, then discard them before freeing. */
    ar_document_free(doc);
    ar_read_report_free(report);
    return 0;
}
```

The `output_entities` example intentionally demonstrates that a `ValueView`
scalar is native storage; applications should decode the scalar using its
reported `stride` and type (`ar_value_read_integer` is convenient for integer
wrappers). For a complete report field decoder, walk fields with
`ar_value_field` and inspect `ar_value_kind` before reading data.

## Version and portability limits

This guide describes the ABI shipped with acadrust-ffi 0.1.0 and acadrust
0.6.5. Native report and inspection field names follow that upstream version;
they are not a promise that future acadrust releases keep identical Rust names.
Compile and run with matching headers and library binaries.

* DWG readers expose the upstream AC9/AC12/AC15/AC18/AC21/AC24 families.
* DXF readers support ASCII and binary input; `AUTO` is content detected.
* DWG mmap is unavailable on WebAssembly.
* DWG no-LZ77 output is only valid for DWG version 1021.
* Import buffer calls require an explicit import format and embedded resources.
* Progress callbacks may be concurrent; entity visitors are serial and their
  views are callback-scoped.
* Report/inspection views are native borrowed storage, while read/import input
  buffers and decompressed section output are copied/owned as described above.
* No I/O API uses JSON. The legacy JSON entity API remains documented in
  [`API_GUIDE.md`](API_GUIDE.md), and its schema is tied to acadrust 0.6.5.
