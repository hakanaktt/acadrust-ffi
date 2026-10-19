#ifndef AR_IO_H
#define AR_IO_H

#include "acadrust.h"
#include "acadrust_views.h"

#ifdef __cplusplus
extern "C" {
#endif

/* DWG progress callbacks may run concurrently on worker threads. Values are
 * monotonic, in 0..1000. Callbacks and user_data must remain valid until the
 * read returns. Callbacks must be thread-safe, return normally, and must not
 * mutate/free the drawing being read. No callback is retained after return. */
typedef void (*ReadProgressCallback)(uint16_t progress, void *user_data);
/* Resolve an R12 stored font filename to a canonical identity. The argument
 * is borrowed for this invocation. Return a borrowed UTF-8 string that remains
 * valid until the library copies it immediately after return. NULL or invalid
 * UTF-8 uses the stored name. Font resolution reads no font files. */
typedef const char *(*LegacyFontCallback)(const char *font, void *user_data);

typedef struct ReadOptions {
    uint8_t failsafe;
    uint8_t recover;
    const char *dxf_default_encoding;
    const char *dwg_r12_code_page;
    ReadProgressCallback progress;
    LegacyFontCallback legacy_font_resolver;
    void *user_data;
} ReadOptions;

typedef struct ReadReport ReadReport;
typedef struct DwgInspection DwgInspection;

/* Initialize options using the corresponding default function. NULL options
 * also use defaults. All flags must be 0 or 1. Options/strings are borrowed
 * only during the call. Default reading is strict. failsafe permits partial
 * parsing; recover enables failsafe and applies conservative document repair.
 * Recovery fails when no usable drawing records survive.
 *
 * Encoding strings may be NULL, or a supported code page such as ANSI_1252.
 * dxf_default_encoding is used for pre-2007 DXF without a declared encoding.
 * dwg_r12_code_page overrides R12 string decoding while preserving the declared
 * header code page. Progress/font callbacks apply only to DWG input.
 *
 * Extended readers require both outputs and initialize them to NULL before
 * reading. Success returns a new owned document plus an owned ReadReport.
 * Free with ar_document_free and ar_read_report_free, independently.
 * ar_read_report_view provides borrowed fields "stats" and "repair" (Option).
 * Statistics include source/version, section and record
 * counts, completion/recovery flags, output counts, recovered_errors, and
 * diagnostics (code, stage, section, byte/line location, handle/type, message).
 * The repair field exposes the upstream RepairReport structure directly.
 * Views and all descendant strings/data borrow native report storage. They
 * remain valid until the report is freed, independent of document lifetime.
 * No report serialization or copies occur when obtaining/traversing views.
 * AUTO detects content, including binary DXF.
 * Inputs and reports have the same ownership rules as the basic I/O calls. */
AR_API Status ar_read_options_default(ReadOptions *out);
AR_API Status ar_document_read_file_ex(const char *path, Format format,
                                      const ReadOptions *options,
                                      Document **out, ReadReport **out_report);
AR_API Status ar_document_read_buffer_ex(const uint8_t *data, size_t len,
                                        Format format, const ReadOptions *options,
                                        Document **out, ReadReport **out_report);
AR_API Status ar_read_report_view(const ReadReport *report, ValueView *out);
AR_API void ar_read_report_free(ReadReport *report);
/* Native DWG input through an OS read-only memory map. All other reader options,
 * recovery and report ownership rules apply. The input file must not be modified
 * or truncated while this call runs. The map is released before return; returned
 * documents/reports own their native storage. Unavailable on WebAssembly. */
AR_API Status ar_dwg_read_file_mmap(const char *path, const ReadOptions *options,
                                  Document **out, ReadReport **out_report);

/* DWG pass-2 entity visiting reads expose each decoded entity directly before
 * storage. Return 1 to keep it or 0 to omit it from the returned document.
 * The callback receives ReadOptions.user_data and runs serially on the reader
 * thread. The entity view and all descendants expire when the callback returns;
 * never retain them. Progress callbacks may still run concurrently.
 * Upstream also visits decoded block and block-end framing entities.
 * recover must be zero; callers can repair the completed document separately.
 * NULL visitors and return values other than 0/1 are invalid arguments. */
typedef uint8_t (*DwgEntityVisitCallback)(ValueView entity, void *user_data);
AR_API Status ar_dwg_read_file_visiting(const char *path,
                                       const ReadOptions *options,
                                       DwgEntityVisitCallback visitor, Document **out);
AR_API Status ar_dwg_read_buffer_visiting(const uint8_t *data, size_t len,
                                         const ReadOptions *options,
                                         DwgEntityVisitCallback visitor, Document **out);

typedef struct WriteOptions {
    uint32_t version;
    uint8_t dwg_no_lz77;
} WriteOptions;

/* version=0 uses the document's version; otherwise use an AR_VERSION_* code.
 * Upstream promotes R2000/R2004 binary DXF to R2010 for compatibility; explicit
 * R12 DXF export uses the upstream R12 ASCII conversion path.
 * The source document is not changed. dwg_no_lz77 is a diagnostic mode
 * supported only for version 1021 DWG: pages are Reed-Solomon encoded without
 * LZ77 compression. Default is normal compression. Writers validate and
 * serialize before opening the destination; an I/O failure may leave a partial
 * file. Existing files are overwritten. Output requires an explicit format. */
AR_API Status ar_write_options_default(WriteOptions *out);
AR_API Status ar_document_write_file_ex(const Document *doc, const char *path,
                                       Format format, const WriteOptions *options);
AR_API Status ar_document_write_buffer_ex(const Document *doc, Format format,
                                         const WriteOptions *options, Buffer *out);

typedef int32_t ImportFormat;
#define AR_IMPORT_AUTO ((ImportFormat)0)
#define AR_IMPORT_STL ((ImportFormat)1)
#define AR_IMPORT_COLLADA ((ImportFormat)2)
#define AR_IMPORT_OBJ ((ImportFormat)3)
#define AR_IMPORT_GLTF ((ImportFormat)4)
#define AR_IMPORT_GLB ((ImportFormat)5)
#define AR_IMPORT_FBX ((ImportFormat)6)

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

/* Import STL (ASCII/binary), COLLADA, OBJ/MTL, glTF/GLB, or FBX (ASCII/binary)
 * as upstream Mesh entities. File input detects its extension and resolves
 * relative material/buffer resources from the input file directory. Buffer
 * input requires an explicit ImportFormat and uses upstream buffer-import
 * semantics: use embedded resources for self-contained models.
 *
 * Defaults: layer_prefix="imported", default_color.mode=BY_LAYER,
 * merge_vertices=1, merge_tolerance=1e-9, scale_factor=1.0. NULL strings use
 * defaults. Color modes support all upstream Color variants; only the index
 * field is used for INDEX (1..255), and r/g/b are used for RGB (0..255).
 * merge_tolerance must be finite and nonnegative; scale_factor finite and
 * nonzero. out receives a new owned document, or NULL on failure. The source
 * arrays/strings are never retained. */
AR_API Status ar_import_options_default(ImportOptions *out);
AR_API Status ar_document_import_file(const char *path,
                                     const ImportOptions *options, Document **out);
AR_API Status ar_document_import_buffer(const uint8_t *data, size_t len,
                                       ImportFormat format,
                                       const ImportOptions *options, Document **out);

#define AR_IMPORT_COLOR_BY_LAYER INT32_C(0)
#define AR_IMPORT_COLOR_BY_BLOCK INT32_C(1)
#define AR_IMPORT_COLOR_NONE INT32_C(2)
#define AR_IMPORT_COLOR_INDEX INT32_C(3)
#define AR_IMPORT_COLOR_RGB INT32_C(4)

typedef int32_t DwgInspectKind;
#define AR_DWG_INSPECT_HEADER ((DwgInspectKind)0)
#define AR_DWG_INSPECT_CRCS ((DwgInspectKind)1)

/* Read DWG metadata without reconstructing a document. HEADER returns all
 * DwgFileHeaderInfo fields including version, section/page layout and stored
 * CRC metadata. CRCS returns the upstream CrcExtractionReport including
 * per-page checksums and notifications; this extracts stored checksums and
 * does not promise integrity validation. Each call returns an owned native
 * DwgInspection. Its view has a Header or Crcs variant with one native payload.
 * All descendant data borrow the inspection until ar_dwg_inspection_free.
 * Section calls return an owned decompressed section buffer; use names from
 * the header report, for example "AcDb:Header" or "AcDb:AcDbObjects".
 * Release inspections with ar_dwg_inspection_free and buffers with
 * ar_buffer_free. Free functions accept NULL. */
AR_API Status ar_dwg_inspect_file(const char *path, DwgInspectKind kind, DwgInspection **out);
AR_API Status ar_dwg_inspect_buffer(const uint8_t *data, size_t len,
                                   DwgInspectKind kind, DwgInspection **out);
AR_API Status ar_dwg_inspection_view(const DwgInspection *inspection, ValueView *out);
AR_API void ar_dwg_inspection_free(DwgInspection *inspection);
AR_API Status ar_dwg_section_file(const char *path, const char *section_name, Buffer *out);
AR_API Status ar_dwg_section_buffer(const uint8_t *data, size_t len,
                                   const char *section_name, Buffer *out);

#ifdef __cplusplus
}
#endif

#endif
