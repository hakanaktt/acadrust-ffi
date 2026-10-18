#include "acadrust.h"

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void check(Status status)
{
    if (status != AR_OK) {
        fprintf(stderr, "acadrust error %" PRId32 ": %s\n", status, ar_last_error());
        exit(EXIT_FAILURE);
    }
}

int main(int argc, char **argv)
{
    const char *path = argc > 1 ? argv[1] : "example.dxf";
    Document *doc = NULL;
    Document *loaded = NULL;
    Line line = {{0.0, 0.0, 0.0}, {100.0, 50.0, 0.0}};
    Circle circle = {{50.0, 25.0, 0.0}, 10.0};
    uint64_t line_handle = 0;
    uint64_t circle_handle = 0;
    size_t count = 0;
    char *json = NULL;

    check(ar_new(AR_VERSION_2018, &doc));
    check(ar_layer_add(doc, "Geometry", 3));
    check(ar_line_add(doc, &line, &line_handle));
    check(ar_circle_add(doc, &circle, &circle_handle));
    check(ar_entity_set_layer(doc, line_handle, "Geometry"));
    check(ar_entity_set_layer(doc, circle_handle, "Geometry"));
    check(ar_write_file(doc, path, AR_FORMAT_DXF_ASCII));
    ar_free(doc);

    check(ar_read_file(path, &loaded));
    check(ar_entity_count(loaded, &count));
    check(ar_entity_get_json(loaded, line_handle, &json));
    printf("%s: %zu entities\nLine %" PRIu64 ": %s\n", path, count, line_handle, json);
    ar_string_free(json);
    ar_free(loaded);
    return EXIT_SUCCESS;
}
