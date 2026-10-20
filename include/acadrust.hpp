#ifndef ACADRUST_HPP
#define ACADRUST_HPP

#include "acadrust.h"
#include "acadrust_builder.h"

#include <initializer_list>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace acadrust {

class Error : public std::runtime_error {
public:
    explicit Error(Status status)
        : std::runtime_error(message()), status_(status) {}

    Status status() const noexcept { return status_; }

private:
    static std::string message() {
        const char *text = ar_last_error();
        return text && *text ? text : "acadrust operation failed";
    }

    Status status_;
};

inline void check(Status status) {
    if (status != AR_OK) {
        throw Error(status);
    }
}

// The builder owns a detached native entity until a successful add consumes it.
class EntityBuilder {
public:
#define AR_ENTITY_FACTORY(name, kind) \
    static EntityBuilder name() { return EntityBuilder(kind); }
#include "acadrust_entity_catalog.inc"
#undef AR_ENTITY_FACTORY

    explicit EntityBuilder(const char *kind) : value_(nullptr) {
        const Status status = ar_entity_builder_new(kind, &value_);
        if (status != AR_OK) {
            const Error error(status);
            ar_entity_builder_free(value_);
            throw error;
        }
    }

    explicit EntityBuilder(const std::string &kind) : EntityBuilder(kind.c_str()) {}

    ~EntityBuilder() { ar_entity_builder_free(value_); }

    EntityBuilder(const EntityBuilder &) = delete;
    EntityBuilder &operator=(const EntityBuilder &) = delete;

    EntityBuilder(EntityBuilder &&other) noexcept : value_(other.value_) {
        other.value_ = nullptr;
    }

    EntityBuilder &operator=(EntityBuilder &&other) noexcept {
        if (this != &other) {
            ar_entity_builder_free(value_);
            value_ = other.value_;
            other.value_ = nullptr;
        }
        return *this;
    }

    explicit operator bool() const noexcept { return value_ != nullptr; }
    ::EntityBuilder *get() const noexcept { return value_; }

    // All builder views expire before the next edit, move, add, or destruction.
    ValueView view() const {
        ValueView result = {};
        check(ar_entity_builder_view(value_, &result));
        return result;
    }

    EntityBuilder &write(const char *path, const NativeWrite &input) {
        check(ar_entity_builder_write(value_, path, &input));
        return *this;
    }

    EntityBuilder &real(const char *path, double value) {
        check(ar_entity_builder_set_f64(value_, path, value));
        return *this;
    }

    EntityBuilder &integer(const char *path, int64_t value) {
        check(ar_entity_builder_set_i64(value_, path, value));
        return *this;
    }

    EntityBuilder &uinteger(const char *path, uint64_t value) {
        check(ar_entity_builder_set_u64(value_, path, value));
        return *this;
    }

    EntityBuilder &boolean(const char *path, bool value) {
        check(ar_entity_builder_set_bool(value_, path, value ? 1 : 0));
        return *this;
    }

    EntityBuilder &text(const char *path, const char *value) {
        check(ar_entity_builder_set_text(value_, path, value));
        return *this;
    }

    EntityBuilder &text(const char *path, const std::string &value) {
        const NativeWrite input = {AR_VALUE_STRING, value.data(), value.size()};
        return write(path, input);
    }

    EntityBuilder &point(const char *path, Vector3 value) {
        check(ar_entity_builder_set_vec3(value_, path, value));
        return *this;
    }

    EntityBuilder &point(const char *path, double x, double y, double z = 0.0) {
        const Vector3 value = {x, y, z};
        return point(path, value);
    }

    EntityBuilder &points(const char *path, const Vector3 *values, size_t count) {
        check(ar_entity_builder_set_vec3s(value_, path, values, count));
        return *this;
    }

    EntityBuilder &points(const char *path, const std::vector<Vector3> &values) {
        return points(path, values.data(), values.size());
    }

    EntityBuilder &points(const char *path, std::initializer_list<Vector3> values) {
        return points(path, values.begin(), values.size());
    }

    EntityBuilder &reals(const char *path, const double *values, size_t count) {
        const NativeWrite input = {AR_VALUE_F64, values, count};
        return write(path, input);
    }

    EntityBuilder &reals(const char *path, const std::vector<double> &values) {
        return reals(path, values.data(), values.size());
    }

    EntityBuilder &reals(const char *path, std::initializer_list<double> values) {
        return reals(path, values.begin(), values.size());
    }

    EntityBuilder &resize(const char *path, size_t count) {
        check(ar_entity_builder_resize(value_, path, count));
        return *this;
    }

    EntityBuilder &select(const char *path, const char *variant) {
        check(ar_entity_builder_select(value_, path, variant));
        return *this;
    }

    EntityBuilder &option(const char *path, bool enabled = true) {
        check(ar_entity_builder_option(value_, path, enabled ? 1 : 0));
        return *this;
    }

    EntityBuilder &layer(const char *name) { return text("common/layer", name); }
    EntityBuilder &layer(const std::string &name) { return text("common/layer", name); }
    EntityBuilder &by_layer() { return select("common/color", "ByLayer"); }
    EntityBuilder &by_block() { return select("common/color", "ByBlock"); }

    EntityBuilder &color_index(uint8_t index) {
        if (index == 0) {
            return by_block();
        }
        return select("common/color", "Index").uinteger("common/color/value", index);
    }

    EntityBuilder &rgb(uint8_t red, uint8_t green, uint8_t blue) {
        return select("common/color", "Rgb")
            .uinteger("common/color/r", red)
            .uinteger("common/color/g", green)
            .uinteger("common/color/b", blue);
    }

    uint64_t add(Document *document) {
        uint64_t handle = 0;
        check(ar_entity_builder_add(document, &value_, &handle));
        return handle;
    }

    uint64_t add_to_block(Document *document, const char *block) {
        uint64_t handle = 0;
        check(ar_entity_builder_add_to_block(document, &value_, block, &handle));
        return handle;
    }

    uint64_t add_to_layout(Document *document, const char *layout) {
        uint64_t handle = 0;
        check(ar_entity_builder_add_to_layout(document, &value_, layout, &handle));
        return handle;
    }

private:
    ::EntityBuilder *value_;
};

} // namespace acadrust

#endif
