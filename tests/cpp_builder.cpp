#include "acadrust.hpp"

#ifdef NDEBUG
#undef NDEBUG
#endif
#include <cassert>
#include <cstring>
#include <memory>
#include <string>
#include <type_traits>
#include <utility>

static_assert(!std::is_copy_constructible<acadrust::EntityBuilder>::value,
              "builders must have unique ownership");
static_assert(!std::is_copy_assignable<acadrust::EntityBuilder>::value,
              "builders must have unique ownership");
static_assert(std::is_nothrow_move_constructible<acadrust::EntityBuilder>::value,
              "builder moves must not throw");
static_assert(std::is_nothrow_move_assignable<acadrust::EntityBuilder>::value,
              "builder moves must not throw");

static ValueView child(ValueView value, size_t index) {
    StringView name = {};
    ValueView result = {};
    acadrust::check(ar_value_child(value, index, &name, &result));
    return result;
}

static ValueView field(ValueView value, const char *name) {
    ValueView result = {};
    acadrust::check(ar_value_field(value, name, &result));
    return result;
}

static ValueView entity(Document *document, uint64_t handle) {
    ValueView result = {};
    acadrust::check(ar_entity_view(document, handle, &result));
    return child(result, 0);
}

static int64_t integer(ValueView value) {
    const void *data = nullptr;
    size_t len = 0, stride = 0;
    if (ar_value_data(value, &data, &len, &stride) != AR_OK) {
        int64_t result = 0;
        acadrust::check(ar_value_read_integer(value, &result));
        return result;
    }
    ValueKind kind = AR_VALUE_OPAQUE;
    acadrust::check(ar_value_kind(value, &kind));
    assert(len == 1);
    if (kind == AR_VALUE_I32) return *static_cast<const int32_t *>(data);
    if (kind == AR_VALUE_U8) return *static_cast<const uint8_t *>(data);
    throw std::runtime_error("unexpected native integer kind");
}

static double real(ValueView value) {
    const void *data = nullptr;
    size_t count = 0;
    size_t stride = 0;
    acadrust::check(ar_value_data(value, &data, &count, &stride));
    assert(count == 1 && stride == sizeof(double));
    return *static_cast<const double *>(data);
}

static std::string text(ValueView value) {
    const void *data = nullptr;
    size_t count = 0;
    size_t stride = 0;
    acadrust::check(ar_value_data(value, &data, &count, &stride));
    assert(stride == 1);
    return std::string(static_cast<const char *>(data), count);
}

int main() {
    Document *raw = nullptr;
    acadrust::check(ar_document_new(AR_VERSION_2018, &raw));
    const std::unique_ptr<Document, decltype(&ar_document_free)> document(raw, ar_document_free);
#define AR_ENTITY_FACTORY(name, kind) { acadrust::EntityBuilder named = acadrust::EntityBuilder::name(); assert(named); }
#include "acadrust_entity_catalog.inc"
#undef AR_ENTITY_FACTORY
    acadrust::check(ar_layer_add(raw, "Geometry", 3));

    acadrust::EntityBuilder label("TEXT");
    label.text("value", "Native builder")
        .point("insertion_point", 2.0, 3.0)
        .real("height", 2.5)
        .layer("Geometry")
        .rgb(10, 20, 30);
    assert(text(field(child(label.view(), 0), "value")) == "Native builder");
    const uint64_t label_handle = label.add(raw);
    assert(label_handle != 0 && !label);
    ValueView payload = entity(raw, label_handle);
    assert(text(field(payload, "value")) == "Native builder");
    assert(real(field(field(payload, "insertion_point"), "x")) == 2.0);
    assert(text(field(field(payload, "common"), "layer")) == "Geometry");
    assert(integer(field(field(field(payload, "common"), "color"), "r")) == 10);

    const uint64_t spline_handle = acadrust::EntityBuilder("SPLINE")
        .integer("degree", 2)
        .points("control_points", {{0.0, 0.0, 0.0}, {2.0, 3.0, 0.0}, {4.0, 0.0, 0.0}})
        .reals("knots", {0.0, 0.0, 0.0, 1.0, 1.0, 1.0})
        .reals("weights", {1.0, 1.0, 1.0})
        .option("dwg_scenario")
        .integer("dwg_scenario/value", 2)
        .add(raw);
    payload = entity(raw, spline_handle);
    assert(integer(field(payload, "degree")) == 2);
    assert(real(field(child(field(payload, "control_points"), 1), "y")) == 3.0);
    assert(integer(child(field(payload, "dwg_scenario"), 0)) == 2);

    const uint64_t hatch_handle = acadrust::EntityBuilder::hatch()
        .resize("paths", 1)
        .uinteger("paths/0/flags", 3)
        .resize("paths/0/edges", 1)
        .select("paths/0/edges/0", "Polyline")
        .points("paths/0/edges/0/vertices", {
            {0.0, 0.0, 0.0}, {4.0, 0.0, 0.0}, {4.0, 3.0, 0.0}, {0.0, 3.0, 0.0}
        })
        .boolean("paths/0/edges/0/is_closed", true)
        .add(raw);
    payload = entity(raw, hatch_handle);
    ValueView boundary = child(field(payload, "paths"), 0);
    ValueView edge = child(field(boundary, "edges"), 0);
    size_t vertex_count = 0;
    acadrust::check(ar_value_len(field(child(edge, 0), "vertices"), &vertex_count));
    assert(vertex_count == 4);

    const uint64_t dimension_handle = acadrust::EntityBuilder::dimension_aligned()
        .point("first_point", 0.0, 0.0)
        .point("second_point", 4.0, 0.0)
        .point("definition_point", 0.0, 2.0)
        .real("base/actual_measurement", 4.0)
        .add(raw);
    ValueView dimension = child(entity(raw, dimension_handle), 0);
    assert(real(field(field(dimension, "second_point"), "x")) == 4.0);

    acadrust::EntityBuilder retry("LINE");
    retry.point("end", 1.0, 1.0).layer("MissingLayer");
    std::string saved_error;
    try {
        retry.add(raw);
        assert(false);
    } catch (const acadrust::Error &error) {
        assert(error.status() != AR_OK);
        assert(std::strlen(error.what()) != 0);
        saved_error = error.what();
    }
    assert(retry);
    acadrust::check(ar_layer_add(raw, "MissingLayer", 4));
    assert(!saved_error.empty());
    assert(retry.add(raw) != 0 && !retry);

    acadrust::EntityBuilder original("CIRCLE");
    original.point("center", 5.0, 6.0).real("radius", 4.0);
    acadrust::EntityBuilder moved(std::move(original));
    assert(!original && moved);
    acadrust::EntityBuilder assigned("POINT");
    assigned = std::move(moved);
    assert(!moved && assigned);
    const uint64_t circle_handle = assigned.add(raw);
    Circle circle = {};
    acadrust::check(ar_circle_get(raw, circle_handle, &circle));
    assert(circle.radius == 4.0 && circle.center.x == 5.0);

    uint64_t block_handle = 0;
    acadrust::check(ar_table_add(raw, AR_TABLE_BLOCK_RECORD, "Part", &block_handle));
    const uint64_t block_entity = acadrust::EntityBuilder("LINE")
        .point("start", 0.0, 1.0)
        .point("end", 2.0, 1.0)
        .color_index(5)
        .add_to_block(raw, "Part");
    payload = entity(raw, block_entity);
    assert(static_cast<uint64_t>(integer(field(field(payload, "common"), "owner_handle")))
           == block_handle);

    uint64_t layout_handle = 0;
    acadrust::check(ar_layout_add(raw, "Sheet2", &layout_handle));
    ValueView layout = {};
    acadrust::check(ar_object_view(raw, layout_handle, &layout));
    const uint64_t layout_block = static_cast<uint64_t>(integer(field(child(layout, 0), "block_record")));
    const uint64_t layout_entity = acadrust::EntityBuilder("POINT")
        .point("location", 10.0, 20.0)
        .add_to_layout(raw, "Sheet2");
    payload = entity(raw, layout_entity);
    assert(static_cast<uint64_t>(integer(field(field(payload, "common"), "owner_handle")))
           == layout_block);

    try {
        acadrust::EntityBuilder unknown("NO_SUCH_ENTITY");
        assert(false);
    } catch (const acadrust::Error &error) {
        const std::string message = error.what();
        size_t count = 0;
        acadrust::check(ar_entity_count(raw, &count));
        assert(!message.empty() && message == error.what());
    }

    return 0;
}
