# acadrust-ffi

C and C++ bindings for [acadrust](https://github.com/hakanaktt/acadrust) 0.6.5.
The library builds as a shared library and a static library. It requires no
installed CAD application.

- Read and write DWG, ASCII DXF, and binary DXF, from files or memory.
- Create, clone, and release documents; choose the export version.
- Enumerate entities by persistent 64-bit handle.
- Construct all 48 upstream entity variants through a catalog of 97 entity kinds.
- Traverse native entities, geometry, strings, arrays, XDATA, ACIS, and storage
  metadata through borrowed views, without serialization or allocation.
- Borrow direct writable numeric, byte, and UTF-8 string storage with an
  exclusive lease; release it before accessing the drawing again.
- Reach all 10 symbol tables, non-graphical object variants, classes, layouts,
  drawing header variables, previews, and document metadata through native views.
- Edit native properties, nested records, enums, options, and arrays through
  typed native mutation calls. Existing simple geometry calls remain available.
- Transform and explode entities; manage blocks, layer states, and XRecords;
  audit and repair documents with native reports.
- Configure recovery, encodings, progress callbacks, entity filtering, mmap,
  export versions, DWG compression, and DWG header/CRC/section inspection.
- Import STL, COLLADA, OBJ, glTF, GLB, and FBX with native options.
- Receive status codes and thread-local error messages; unwinding Rust panics are
  caught at the boundary.
- Run independent file reads and writes concurrently through a dedicated Rayon
  worker pool.

The complete C interface and ownership rules are in
[`include/acadrust.h`](include/acadrust.h). Functions use the `ar_` prefix,
constants use `AR_`, and types use plain names such as `Document` and `Line`.
ABI version: **1**.

The [exhaustive API guide](docs/API_GUIDE.md) explains every public API family,
native view and writable-span lifetime rules, mutation paths, reports, ACIS,
advanced I/O, and C integration patterns.
The [I/O API guide](docs/API_IO.md) provides detailed reader, writer, importer,
callback, and diagnostic examples.

The header also provides short source aliases for common calls: `ar_new`,
`ar_free`, `ar_read_file`, `ar_read`, `ar_write_file`, `ar_write`,
`ar_read_many`, `ar_write_many`, `ar_set_threads`, and `ar_get_threads`.
They are macros over the full names and do not add ABI symbols.
The main header includes the view, entity, I/O, native mutation, writable-span,
and ACIS interfaces. The native view API is the primary route to complete data
access. The original JSON functions remain available for compatibility.

## Build

Install a current stable Rust toolchain. On Windows, use the MSVC Rust target
and Visual Studio C++ Build Tools. Build with:

```sh
cargo build --release --locked
```

Artifacts are in `target/release`:

| Platform | Shared Library | Link Library | Static Library |
| --- | --- | --- | --- |
| Windows MSVC | `acadrust_ffi.dll` | `acadrust_ffi.dll.lib` | `acadrust_ffi.lib` |
| Linux | `libacadrust_ffi.so` | same `.so` | `libacadrust_ffi.a` |
| macOS | `libacadrust_ffi.dylib` | same `.dylib` | `libacadrust_ffi.a` |

Link against the library for your application's architecture and distribute the
header with it. Shared linking is the simplest integration. Static consumers
must define `AR_STATIC` before including the header and link Rust's native
system dependencies. `cargo rustc --release -- --print native-static-libs`
prints the platform's required link flags. Keep Rust's default unwind panic
strategy to retain panic-to-status conversion.

## C Example

```c
#include "acadrust.h"
#include <stdio.h>

int main(void) {
    Document *doc = NULL;
    Line line = {{0, 0, 0}, {100, 50, 0}};
    uint64_t handle = 0;
    Status status = ar_document_new(AR_VERSION_2018, &doc);
    if (status == AR_OK)
        status = ar_line_add(doc, &line, &handle);
    if (status == AR_OK)
        status = ar_document_write_file(doc, "drawing.dxf", AR_FORMAT_DXF_ASCII);
    if (status != AR_OK)
        fprintf(stderr, "%s\n", ar_last_error());
    ar_document_free(doc);
    return status == AR_OK ? 0 : 1;
}
```

[`examples/roundtrip.c`](examples/roundtrip.c) creates a layer and geometry,
writes DXF, reopens it, and inspects an entity as JSON. Build the examples and
ABI tests after building the Rust library:

```sh
cmake -S . -B build
cmake --build build --config Release
ctest --test-dir build -C Release --output-on-failure
```

CMake imports the shared library from `target/release`. Set
`-DACADRUST_LIBRARY_DIR=/absolute/path/to/artifacts` to use another directory,
including `target/debug` or a custom `CARGO_TARGET_DIR`. CMake copies the DLL
beside executables on Windows. On Linux/macOS, it sets the build runtime path.

## Calling Rules

Strings and file paths are NUL-terminated UTF-8, including on Windows. Angles
are radians; coordinates use drawing units. Circle/arc centers and polyline
coordinates follow upstream OCS conventions; setters preserve existing extrusion
normals. C booleans are `uint8_t` values
`0` and `1`. Status and format values use fixed-width `int32_t`.

Every owned document, string, or byte buffer must be released with its matching
`ar_*_free` function. Initialize output pointers to `NULL` and buffers to
`{0}`. Free existing outputs before reusing their slots. Inputs are copied when
needed; the library never retains caller-owned memory. Free functions accept
`NULL`, and `ar_buffer_free` resets the buffer. Other freed pointers must
not be reused or freed twice.

`ValueView` and `StringView` borrow native storage and are never freed. Strings
carry byte lengths and need not be NUL-terminated. Primitive scalars and arrays
expose their original read-only data pointer through `ar_value_data`; composite
records expose named fields or indexed children. Rust struct and enum layouts
remain opaque to C. Successful view traversal performs no heap allocations.

Every document view and pointer derived from it expires before **any** document
mutation or freeing. Reacquire views after writing. A view into an owned native
report remains valid until that report is freed. Native setters copy new input
where Rust must own it; parsing, decompression, report computation, and geometry
operations can allocate. Zero-copy access does not imply allocation-free CAD
decoding or a C-compatible layout for Rust structs and enum discriminants.

`ar_entity_borrow_mut`, `ar_object_borrow_mut`, `ar_table_borrow_mut`, and
`ar_document_borrow_mut` expose **direct writable native storage** for numeric
scalars/arrays, byte arrays, and fixed-length UTF-8 strings. They return a
`WritableSpan`; release it with `ar_writable_release` before any other document
access. Release validates the data and updates native caches. On validation
failure the span stays active so the caller can correct its contents and retry.
No data buffer is copied. Identity/index fields and Rust validity-sensitive
values use native setters. See `acadrust_writable.h` for the complete contract.

Inspect `ar_last_error()` immediately after a nonzero status. Its borrowed
string belongs to the calling thread and is replaced by the next call returning
a status on that thread. Free and version functions preserve the message. A
panic status requires discarding the affected document. Invalid pointers,
concurrent mutation, and allocator aborts cannot be recovered by the wrapper.

Do not access a document concurrently with mutation or freeing. Separate
documents may be used on separate threads. Array getters support a size query
with a `NULL` buffer and zero capacity. Insufficient capacity returns
`AR_BUFFER_TOO_SMALL` with the required count and leaves the array untouched.

The legacy JSON API uses the upstream `EntityType` representation, such as `{"Line": {...}}`.
Obtain a complete value with `ar_entity_get_json`, edit it with a JSON
library, and pass it to `ar_entity_set_json`. Replacement preserves the
entity handle/owner and requires the same variant. `ar_entity_add_json`
assigns fresh identities and adds to model space; referenced blocks, styles,
and objects must already exist. The schema is tied to upstream 0.6.5. Some
native storage metadata is omitted by Serde, so JSON is not a lossless DWG
archive or an independent drawing interchange format.

Native entity catalog defaults may need geometry or references before export.
Bounds, transformations, entity availability, and format fidelity follow
acadrust 0.6.5. Unsupported upstream operations return an error; native access
does not add a geometry kernel or make unsupported upstream entities writable
in every file version. Borrowed views also expose raw data omitted by Serde.

File reads use upstream strict mode and detect the format by content. Output
format must be explicit; file output overwrites existing files. Supported
export versions are 1009, 1012, 1014, 1015, 1018, 1021, 1024, 1027, and 1032.
DXF export explicitly requests the document version, including genuine R12.
Entity fidelity and version restrictions follow acadrust; setting a version
does not validate every entity's compatibility.

`ar_documents_read_files` and `ar_documents_write_files` process independent
files in parallel through a dedicated Rayon pool. Configure it with
`ar_io_set_threads(threads)`; zero selects available parallelism, and
`ar_io_get_threads` reports the active count. The pool is separate from a host
application's global Rayon pool. The per-item status array identifies failures;
the function return is `AR_OK` only when every item succeeds. Batch reads copy
paths before workers start. Batch writes require documents to remain unchanged
and alive until the call returns. acadrust's internal DWG parallel work runs on
the same configured pool during these calls and single-file I/O calls. DXF
parsing and writing is serial per file, while a batch of independent DXF files
still runs concurrently.

## Verify

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

CMake/CTest additionally exercises the actual C ABI, C++ header compatibility,
and file round trips. CI runs these checks on Windows, Linux, and macOS.

Licensed under MPL-2.0, as is acadrust.
