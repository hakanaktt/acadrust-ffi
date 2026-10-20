# Constructing every entity

The construction API supports all **97 catalog kinds**, covering all **48
upstream entity variants** and their dimension, surface, underlay, extended,
and dynamic-block subtypes. It uses native records and never requires JSON.

Include `acadrust.h` for C or `acadrust.hpp` for the optional C++11 wrapper.
Distribute the accompanying headers and `acadrust_entity_catalog.inc` together.

## Named factories for every kind

Every lowercase catalog name has two C entry points:

```c
Status ar_<kind>_builder_new(EntityBuilder **out);
Status ar_<kind>_create(Document *doc, const EntityEdit *edits,
                       size_t count, uint64_t *out_handle);
```

For example:

```c
ar_hatch_builder_new(&builder);
ar_dimension_aligned_builder_new(&builder);
ar_multileader_builder_new(&builder);
ar_polygonmesh_builder_new(&builder);
ar_planesurface_builder_new(&builder);
```

The canonical list is [acadrust_entity_catalog.inc](../include/acadrust_entity_catalog.inc).
Each `AR_ENTITY_FACTORY(name, "KIND")` entry declares `ar_name_builder_new`
and `ar_name_create`, and C++ exposes `acadrust::EntityBuilder::name()`.
No catalog entry is excluded from the construction layer. The C and Rust tests
enumerate the runtime catalog and verify every factory in the same order.

The generic forms are also available when the kind is selected at runtime:

```c
ar_entity_builder_new("HATCH", &builder);
ar_entity_create(doc, "HATCH", edits, edit_count, &handle);
```

## C++ fluent construction

Each named C++ factory returns a move-only RAII builder. Setters throw
`acadrust::Error`, retaining both its `Status` and the original native error
message. Destruction frees an uncommitted builder.

```cpp
#include "acadrust.hpp"

uint64_t add_label(Document *doc) {
    return acadrust::EntityBuilder::text()
        .text("value", "Assembly A")
        .point("insertion_point", 10.0, 20.0)
        .real("height", 2.5)
        .layer("0")
        .rgb(30, 90, 140)
        .add(doc);
}
```

```cpp
uint64_t add_spline(Document *doc) {
    return acadrust::EntityBuilder::spline()
        .integer("degree", 2)
        .points("control_points", {
            {0.0, 0.0, 0.0}, {2.0, 3.0, 0.0}, {4.0, 0.0, 0.0}
        })
        .reals("knots", {0.0, 0.0, 0.0, 1.0, 1.0, 1.0})
        .add(doc);
}
```

This pattern applies to every catalog factory, including
`hatch()`, `dimension_aligned()`, `multileader()`, `mesh()`, `solid3d()`,
`planesurface()`, `pdfunderlay()`, and `acdbpointcloudex()`. Nested data uses
`resize`, `select`, `option`, and the same native field paths exposed by views.

For example, a filled rectangular hatch can be assembled without a separate
opaque boundary object or a JSON value:

```cpp
uint64_t handle = acadrust::EntityBuilder::hatch()
    .resize("paths", 1)
    .uinteger("paths/0/flags", 3) // external + polyline
    .resize("paths/0/edges", 1)
    .select("paths/0/edges/0", "Polyline")
    .points("paths/0/edges/0/vertices", {
        {0, 0, 0}, {4, 0, 0}, {4, 3, 0}, {0, 3, 0}
    })
    .boolean("paths/0/edges/0/is_closed", true)
    .add(doc);
```

| Method | Input and behavior |
| --- | --- |
| `real(path, double)` | Write an exact native `f64`. |
| `integer(path, int64_t)` | Range-check and convert to the native integer width. |
| `uinteger(path, uint64_t)` | Range-check and convert to the native integer width. |
| `boolean(path, bool)` | Write a native Boolean. |
| `text(path, const char *)` | Copy a NUL-terminated UTF-8 string. |
| `text(path, std::string)` | Copy explicit-length string bytes, including embedded NULs. |
| `point(path, Vector3)` | Set all `x`, `y`, `z` coordinates of a native vector. |
| `point(path, x, y, z)` | Coordinate overload; `z` defaults to zero. |
| `points(path, values)` | Replace a native vector sequence from a pointer/count, vector, or initializer list. |
| `reals(path, values)` | Replace an `f64` sequence from a pointer/count, vector, or initializer list. |
| `write(path, NativeWrite)` | Exact scalar or array kinds not covered by a convenience method. |
| `resize(path, count)` | Resize a nested native sequence. |
| `select(path, variant)` | Select a nested enum variant and initialize its payload. |
| `option(path, enabled)` | Enable or clear an optional native value. |
| `layer(name)` | Set `common/layer`. The layer must exist at insertion. |
| `by_layer()`, `by_block()` | Set the common color mode. |
| `color_index(index)` | Set a palette index; zero selects ByBlock. |
| `rgb(r,g,b)` | Set an RGB color. |
| `view()` | Borrow the detached native entity. |
| `get()` | Borrow the opaque C builder pointer. |
| `add(doc)` | Commit to model space and consume the builder. |
| `add_to_block(doc, name)` | Commit to an existing named block. |
| `add_to_layout(doc, name)` | Commit to an existing layout. |

## C detached construction

The C API separates configuration from insertion. A builder owns a native
entity, but has no document handle until insertion succeeds.

```c
#include "acadrust.h"

Status add_text(Document *doc, uint64_t *out_handle) {
    EntityBuilder *builder = NULL;
    Vector3 position = {10.0, 20.0, 0.0};
    Status status = ar_text_builder_new(&builder);
    if (status == AR_OK)
        status = ar_entity_builder_set_text(builder, "value", "Assembly A");
    if (status == AR_OK)
        status = ar_entity_builder_set_vec3(builder, "insertion_point", position);
    if (status == AR_OK)
        status = ar_entity_builder_set_f64(builder, "height", 2.5);
    if (status == AR_OK)
        status = ar_entity_builder_add(doc, &builder, out_handle);
    ar_entity_builder_free(builder);
    return status;
}
```

The typed C setters are `ar_entity_builder_set_f64`, `_set_i64`, `_set_u64`,
`_set_bool`, `_set_text`, `_set_vec3`, and `_set_vec3s`. Use
`ar_entity_builder_write` with `NativeWrite` for exact native widths, strings
with explicit lengths, and numeric/byte arrays. Use
`ar_entity_builder_resize`, `_select`, and `_option` for nested records.

Builder paths are **relative to the entity payload**. TEXT uses `"value"`;
LINE uses `"start/x"`; SPLINE uses `"degree"`, `"control_points"`, and
`"knots"`. A leading entity variant is optional, such as `"Line/start/x"`.
This differs from the earlier live-document mutation alias rules: the
builder's TEXT path needs only `"value"`, not `"value/value"`.

Unknown fields return `AR_NOT_FOUND`; incompatible value types return
`AR_TYPE_MISMATCH`. Integer convenience setters reject out-of-range values
instead of truncating them. Root variant replacement and protected identity,
ownership, and raw cache fields are unavailable.

## One-call creation for every entity

Each `ar_<kind>_create` accepts an ordered array of `EntityEdit` operations.
These operations initialize a detached native entity, then validate and insert
it. On an edit or validation failure, no entity is added and `out_handle` is
zero.

```c
double height = 2.5;
const char label[] = "Assembly A";
EntityEdit edits[] = {
    {AR_ENTITY_EDIT_WRITE, "value",
     {AR_VALUE_STRING, label, sizeof(label) - 1}, 0, NULL, 0},
    {AR_ENTITY_EDIT_WRITE, "height",
     {AR_VALUE_F64, &height, 1}, 0, NULL, 0}
};
uint64_t handle = 0;
Status status = ar_text_create(doc, edits,
                              sizeof(edits) / sizeof(edits[0]), &handle);
```

The same signature exists for all 97 kinds. For complex entities, prepare
operations in structural order: resize a vector before setting its children,
select an enum before setting its payload, and enable an option before setting
its value.

| Edit kind | Active member |
| --- | --- |
| `AR_ENTITY_EDIT_WRITE` | `value`: a `NativeWrite`. |
| `AR_ENTITY_EDIT_RESIZE` | `length`: new sequence length. |
| `AR_ENTITY_EDIT_SELECT` | `variant`: native variant name. |
| `AR_ENTITY_EDIT_OPTION` | `enabled`: zero or one. |

`ar_entity_builder_apply` applies the same edit list atomically to an existing
detached builder. That atomic batch API clones the builder candidate; ordinary
individual setters edit its native value directly.

## Ownership, validation, and zero-copy transfer

- `ar_entity_builder_new` and every named `_builder_new` return an owned
  builder. Release it with `ar_entity_builder_free` if not inserted.
- Setters copy caller-owned strings and arrays into native ownership. They
  never retain caller input pointers.
- Builder views borrow its native entity. They expire before any setter,
  apply, insertion attempt, or builder free.
- No intermediate geometry is inserted into the drawing. Incomplete native
  states may be assembled while configuring a detached builder.
- Insertion checks the complete entity with the existing FFI geometry/layer
  validation and verifies an explicit destination before adding anything.
  This validation is not a complete CAD geometry or format-compatibility audit.
- Successful insertion **moves** owned native geometry, arrays, and strings
  into the document. It frees the builder and sets the caller's pointer to
  `NULL`. It does not serialize or deep-clone the configured geometry.
- A failed validation or destination lookup preserves the builder so the
  caller can correct it and retry. It leaves the drawing unchanged.
- Named block/layout placement preserves block ownership and viewport lists.
  Structural markers and attribute subentities cannot be placed independently
  through these destination helpers.
- A successful insertion invalidates all existing document views.

## Typed option records

The smaller input-record constructors remain useful when the target entity is
known statically. The original LINE/CIRCLE/ARC/POINT/LWPOLYLINE APIs remain
available; new option records additionally cover TEXT, MTEXT, ELLIPSE, SPLINE,
INSERT, FACE3D, SOLID, RAY, and XLINE. These are conveniences alongside the
complete named-factory API, not the limit of entity coverage.

```c
TextOptions text;
Status status = ar_text_options_default(&text);
if (status == AR_OK) {
    text.text = "Assembly A";
    text.height = 2.5;
    status = ar_text_add(doc, &text, &handle);
}
```

Initialize records with their corresponding `ar_*_options_default` function.
The input records are borrowed only during the call. See
[acadrust_constructors.h](../include/acadrust_constructors.h) for exact fields
and validation, including spline knot/weight consistency and INSERT block
references.
