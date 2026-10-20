#include "acadrust_builder.h"
#include "acadrust_constructors.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stdio.h>

typedef Status (*NewBuilder)(EntityBuilder **);
typedef Status (*CreateEntity)(Document *, const EntityEdit *, size_t, uint64_t *);
typedef struct Factory { const char *kind; NewBuilder builder; CreateEntity create; } Factory;
static const Factory factories[] = {
#define AR_ENTITY_FACTORY(name, kind) {kind, ar_##name##_builder_new, ar_##name##_create},
#include "acadrust_entity_catalog.inc"
#undef AR_ENTITY_FACTORY
};

int main(void) {
    Document *doc = NULL;
    size_t count = 0, index;
    EntityBuilder *builder = NULL;
    uint64_t handle = 0;
    TextOptions text;
    assert(ar_document_new(AR_VERSION_2018, &doc) == AR_OK);
    assert(ar_entity_kind_count(&count) == AR_OK);
    assert(count == sizeof(factories) / sizeof(factories[0]));
    for (index = 0; index < count; ++index) {
        assert(factories[index].builder(&builder) == AR_OK);
        assert(ar_entity_builder_add(doc, &builder, &handle) == AR_OK);
        assert(builder == NULL && handle != 0);
        assert(factories[index].create(doc, NULL, 0, &handle) == AR_OK);
    }
    assert(ar_text_options_default(&text) == AR_OK);
    text.text = "Typed constructor";
    assert(ar_text_add(doc, &text, &handle) == AR_OK);
    assert(ar_multileader_builder_new(&builder) == AR_OK);
    ar_entity_builder_free(builder);
    ar_document_free(doc);
    printf("Validated %zu named native entity factories\n", count);
    return 0;
}
