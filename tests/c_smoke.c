#include "acadrust.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define REQUIRE(condition) do { \
    if (!(condition)) { \
        fprintf(stderr, "line %d: %s (%s)\n", __LINE__, #condition, ar_last_error()); \
        return EXIT_FAILURE; \
    } \
} while (0)

#define OK(call) REQUIRE((call) == AR_OK)

int main(void)
{
    Document *doc = NULL;
    Document *copy = NULL;
    Buffer buffer = {0};
    Line line = {{1.0, 2.0, 3.0}, {4.0, 5.0, 6.0}};
    Line result = {0};
    Circle circle = {{10.0, 20.0, 0.0}, 5.0};
    Arc arc = {{0.0, 0.0, 0.0}, 2.0, 0.0, 1.5};
    Point point = {{8.0, 9.0, 10.0}};
    LwVertex vertices[] = {{0.0, 0.0, 0.5, 1.0, 2.0}, {2.0, 3.0, 0.0, 0.0, 0.0}};
    LwVertex retrieved[2] = {{0}};
    uint64_t handles[5] = {0};
    uint64_t line_handle = 0, circle_handle = 0, arc_handle = 0;
    uint64_t point_handle = 0, poly_handle = 0, json_handle = 0;
    uint32_t version = 0;
    uint32_t flags = 0;
    uint8_t closed = 0;
    int16_t color = 0;
    size_t count = 0;
    double elevation = 0;
    char *text = NULL;
    int format;

    REQUIRE(ar_abi_version() == 1);
    REQUIRE(strcmp(ar_version(), "0.1.0") == 0);
    REQUIRE(strcmp(ar_upstream_version(), "0.6.5") == 0);
    OK(ar_io_set_threads(2));
    {
        size_t workers = 0;
        OK(ar_io_get_threads(&workers));
        REQUIRE(workers == 2);
    }
    OK(ar_documents_read_files(NULL, 0, NULL, NULL));
    OK(ar_documents_write_files(NULL, NULL, 0, AR_FORMAT_DXF_ASCII, NULL));
    REQUIRE(ar_document_new(123, &doc) == AR_INVALID_ARGUMENT);
    REQUIRE(doc == NULL);
    REQUIRE(strlen(ar_last_error()) > 0);
    OK(ar_new(AR_VERSION_2018, &doc));
    REQUIRE(strlen(ar_last_error()) == 0);
    OK(ar_document_get_version(doc, &version));
    REQUIRE(version == AR_VERSION_2018);
    OK(ar_document_set_version(doc, AR_VERSION_2013));
    OK(ar_document_set_version(doc, AR_VERSION_2018));
    OK(ar_layer_add(doc, "Geometry", 3));
    OK(ar_layer_count(doc, &count));
    REQUIRE(count == 2);
    OK(ar_layer_name(doc, 1, &text));
    REQUIRE(strcmp(text, "Geometry") == 0);
    ar_string_free(text);
    text = NULL;
    OK(ar_layer_set_color_index(doc, "Geometry", 4));
    OK(ar_layer_get_color_index(doc, "Geometry", &color));
    REQUIRE(color == 4);
    OK(ar_layer_set_flags(doc, "Geometry", AR_LAYER_LOCKED | AR_LAYER_OFF));
    OK(ar_layer_get_flags(doc, "Geometry", &flags));
    REQUIRE(flags == (AR_LAYER_LOCKED | AR_LAYER_OFF));

    OK(ar_line_add(doc, &line, &line_handle));
    OK(ar_line_get(doc, line_handle, &result));
    REQUIRE(result.end.z == 6.0);
    line.end.z = 7.0;
    OK(ar_line_set(doc, line_handle, &line));
    OK(ar_circle_add(doc, &circle, &circle_handle));
    OK(ar_circle_get(doc, circle_handle, &circle));
    OK(ar_circle_set(doc, circle_handle, &circle));
    OK(ar_arc_add(doc, &arc, &arc_handle));
    OK(ar_arc_get(doc, arc_handle, &arc));
    OK(ar_arc_set(doc, arc_handle, &arc));
    OK(ar_point_add(doc, &point, &point_handle));
    OK(ar_point_get(doc, point_handle, &point));
    OK(ar_point_set(doc, point_handle, &point));
    REQUIRE(ar_line_get(doc, circle_handle, &result) == AR_TYPE_MISMATCH);
    OK(ar_lwpolyline_add(doc, vertices, 2, 1, 4.0, &poly_handle));
    OK(ar_lwpolyline_get(doc, poly_handle, NULL, 0, &count, &closed, &elevation));
    REQUIRE(count == 2 && closed == 1 && elevation == 4.0);
    OK(ar_lwpolyline_get(doc, poly_handle, retrieved, 2, &count, &closed, &elevation));
    REQUIRE(retrieved[0].bulge == 0.5 && retrieved[1].y == 3.0);
    OK(ar_lwpolyline_set(doc, poly_handle, vertices, 2, 0, 2.0));
    OK(ar_entity_set_layer(doc, line_handle, "Geometry"));
    OK(ar_entity_get_layer(doc, line_handle, &text));
    REQUIRE(strcmp(text, "Geometry") == 0);
    ar_string_free(text);
    text = NULL;
    OK(ar_entity_set_color_index(doc, line_handle, 1));
    OK(ar_entity_get_color_index(doc, line_handle, &color));
    REQUIRE(color == 1);
    OK(ar_entity_type(doc, line_handle, &text));
    REQUIRE(strcmp(text, "LINE") == 0);
    ar_string_free(text);
    text = NULL;
    OK(ar_entity_get_json(doc, line_handle, &text));
    OK(ar_entity_set_json(doc, line_handle, text));
    OK(ar_entity_add_json(doc, text, &json_handle));
    REQUIRE(json_handle != line_handle);
    ar_string_free(text);
    text = NULL;
    OK(ar_entity_remove(doc, json_handle));
    OK(ar_entity_count(doc, &count));
    REQUIRE(count == 5);
    OK(ar_entity_handles(doc, NULL, 0, &count));
    REQUIRE(count == 5);
    REQUIRE(ar_entity_handles(doc, handles, 1, &count) == AR_BUFFER_TOO_SMALL);
    OK(ar_entity_handles(doc, handles, 5, &count));
    REQUIRE(handles[0] == line_handle);
    OK(ar_document_clone(doc, &copy));
    ar_document_free(copy);
    copy = NULL;

    for (format = AR_FORMAT_DXF_ASCII; format <= AR_FORMAT_DWG; ++format) {
        OK(ar_document_write_buffer(doc, format, &buffer));
        REQUIRE(buffer.data != NULL && buffer.len > 100);
        OK(ar_document_read_buffer(buffer.data, buffer.len, AR_FORMAT_AUTO, &copy));
        OK(ar_line_get(copy, line_handle, &result));
        REQUIRE(result.start.y == 2.0 && result.end.z == 7.0);
        ar_document_free(copy);
        copy = NULL;
        ar_buffer_free(&buffer);
        REQUIRE(buffer.data == NULL && buffer.len == 0);
        ar_buffer_free(&buffer);
    }
    ar_free(doc);
    ar_document_free(NULL);
    ar_string_free(NULL);
    ar_buffer_free(NULL);
    puts("C ABI smoke test passed");
    return EXIT_SUCCESS;
}
