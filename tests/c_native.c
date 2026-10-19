#include "acadrust_views.h"
#include "acadrust_entities.h"
#include "acadrust_io.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <string.h>

int main(void) {
    Document *doc = NULL;
    ValueView entity = {0}, payload = {0}, field = {0}, header = {0};
    StringView name = {0}, variant = {0};
    const void *first = NULL, *second = NULL;
    size_t count = 0, len = 0, stride = 0;
    uint64_t handle = 0;
    NativeResult *report = NULL;
    ReadOptions read_options;
    ImportOptions import_options;
    WritableSpan writable = {0};
    NativeWrite input;

    assert(ar_read_options_default(&read_options) == AR_OK);
    assert(read_options.failsafe == 0);
    assert(ar_import_options_default(&import_options) == AR_OK);
    assert(ar_document_new(AR_VERSION_2018, &doc) == AR_OK);
    assert(ar_entity_kind_count(&count) == AR_OK && count >= 48);
    assert(ar_entity_add_default(doc, "TEXT", &handle) == AR_OK);
    assert(ar_entity_view(doc, handle, &entity) == AR_OK);
    assert(ar_value_variant(entity, &variant) == AR_OK);
    assert(variant.len == 4 && memcmp(variant.data, "Text", 4) == 0);
    assert(ar_value_child(entity, 0, &name, &payload) == AR_OK);
    assert(ar_value_field(payload, "value", &field) == AR_OK);
    assert(ar_value_data(field, &first, &len, &stride) == AR_OK);
    assert(stride == 1);
    assert(ar_value_data(field, &second, &len, &stride) == AR_OK);
    assert(first == second);
    input.kind = AR_VALUE_STRING;
    input.data = "hello";
    input.len = 5;
    assert(ar_entity_write(doc, handle, "value/value", &input) == AR_OK);
    assert(ar_entity_borrow_mut(doc, handle, "value/value", &writable) == AR_OK);
    assert(writable.kind == AR_VALUE_STRING && writable.len == 5);
    memcpy(writable.data, "world", 5);
    assert(ar_entity_count(doc, &count) == AR_INVALID_ARGUMENT);
    assert(ar_writable_release(&writable) == AR_OK);
    assert(writable.token == NULL && writable.data == NULL);
    assert(ar_document_view(doc, &header) == AR_OK);
    assert(ar_value_field(header, "header", &payload) == AR_OK);
    assert(ar_value_field(payload, "text_height", &field) == AR_OK);
    assert(ar_value_data(field, &first, &len, &stride) == AR_OK);
    assert(len == 1 && stride == sizeof(double));
    assert(*(const double *)first > 0.0);
    assert(ar_document_audit(doc, &report) == AR_OK);
    assert(ar_result_view(report, &payload) == AR_OK);
    assert(ar_value_field(payload, "issues", &field) == AR_OK);
    ar_result_free(report);
    ar_document_free(doc);
    return 0;
}
