# acadrust-ffi

C and C++ bindings for [acadrust](https://github.com/hakanaktt/acadrust) 0.6.5.
The library builds as a shared library and a static library. It requires no
installed CAD application.

- Read and write DWG, ASCII DXF, and binary DXF, from files or memory.
- Create, clone, and release documents; choose the export version.
- Enumerate entities by persistent 64-bit handle.
- Create, read, and edit lines, circles, arcs, points, and lightweight polylines.
- Access all upstream entity variants through Serde JSON.
- Create layers and edit layer colors, frozen/locked/off flags, and entity layers.
- Receive status codes and thread-local error messages; unwinding Rust panics are
  caught at the boundary.
- Run independent file reads and writes concurrently through a dedicated Rayon
  worker pool.

The complete C interface and ownership rules are in
[`include/acadrust.h`](include/acadrust.h). Functions use the `ar_` prefix,
constants use `AR_`, and types use plain names such as `Document` and `Line`.
ABI version: **1**.

The header also provides short source aliases for common calls: `ar_new`,
`ar_free`, `ar_read_file`, `ar_read`, `ar_write_file`, `ar_write`,
`ar_read_many`, `ar_write_many`, `ar_set_threads`, and `ar_get_threads`.
They are macros over the full names and do not add ABI symbols.
Tables other than layers, drawing header variables, non-graphical objects,
layouts, and optional 3D importers do not yet have dedicated C APIs.

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

Inspect `ar_last_error()` immediately after a nonzero status. Its borrowed
string belongs to the calling thread and is replaced by the next call returning
a status on that thread. Free and version functions preserve the message. A
panic status requires discarding the affected document. Invalid pointers,
concurrent mutation, and allocator aborts cannot be recovered by the wrapper.

Do not access a document concurrently with mutation or freeing. Separate
documents may be used on separate threads. Array getters support a size query
with a `NULL` buffer and zero capacity. Insufficient capacity returns
`AR_BUFFER_TOO_SMALL` with the required count and leaves the array untouched.

JSON is the upstream `EntityType` representation, such as `{"Line": {...}}`.
Obtain a complete value with `ar_entity_get_json`, edit it with a JSON
library, and pass it to `ar_entity_set_json`. Replacement preserves the
entity handle/owner and requires the same variant. `ar_entity_add_json`
assigns fresh identities and adds to model space; referenced blocks, styles,
and objects must already exist. The schema is tied to upstream 0.6.5. Some
native storage metadata is omitted by Serde, so JSON is not a lossless DWG
archive or an independent drawing interchange format.

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
