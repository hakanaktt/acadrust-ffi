#ifndef AR_ACIS_H
#define AR_ACIS_H
#include "acadrust.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef struct AcisDocument AcisDocument;
#define AR_ACIS_BOX INT32_C(0)
#define AR_ACIS_WEDGE INT32_C(1)
#define AR_ACIS_PYRAMID INT32_C(2)
#define AR_ACIS_CYLINDER INT32_C(3)
#define AR_ACIS_CONE INT32_C(4)
#define AR_ACIS_SPHERE INT32_C(5)
#define AR_ACIS_TORUS INT32_C(6)
/* Parsed/built models are owned; their views borrow existing model storage.
 * SAT/SAB parsing and writing are CAD encoding operations, not view access.
 * Release returned strings and buffers with the standard free functions. */
AR_API void ar_acis_free(AcisDocument *value);
AR_API Status ar_acis_view(const AcisDocument *value, ValueView *out);
AR_API Status ar_acis_parse_sat(const char *text, AcisDocument **out);
AR_API Status ar_acis_parse_sab(const uint8_t *bytes, size_t len, AcisDocument **out);
AR_API Status ar_acis_write_sat(const AcisDocument *value, char **out);
AR_API Status ar_acis_write_sab(const AcisDocument *value, Buffer *out);
/* Upstream origins: BOX/PYRAMID/SPHERE/TORUS center; WEDGE corner;
 * CYLINDER/CONE base center. BOX/WEDGE size=(length,width,height).
 * PYRAMID size.x=base side, size.z=height. CYLINDER/CONE size.x=radius,
 * size.z=height. SPHERE size.x=radius. TORUS size=(major,minor,0).
 * Unused size components may be zero; all values must be finite. */
AR_API Status ar_acis_primitive(int32_t kind, Vector3 origin, Vector3 size, AcisDocument **out);
AR_API Status ar_entity_parse_acis(const Document *doc, uint64_t handle, AcisDocument **out);
AR_API Status ar_entity_set_acis(Document *doc, uint64_t handle, const AcisDocument *value);
#ifdef __cplusplus
}
#endif
#endif
