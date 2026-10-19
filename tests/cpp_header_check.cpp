#include "acadrust.h"
#include <cstddef>
#include <type_traits>

static_assert(sizeof(Status) == 4, "status width");
static_assert(sizeof(Vector3) == 3 * sizeof(double), "vector ABI");
static_assert(sizeof(Line) == 6 * sizeof(double), "line ABI");
static_assert(sizeof(LwVertex) == 5 * sizeof(double), "vertex ABI");
static_assert(offsetof(Buffer, len) == sizeof(void *), "buffer ABI");
static_assert(std::is_standard_layout<Arc>::value, "POD layout");
static_assert(sizeof(ValueView) == 2 * sizeof(void *), "native view ABI");
static_assert(std::is_standard_layout<ValueView>::value, "native view layout");
static_assert(std::is_standard_layout<ReadOptions>::value, "reader options layout");
static_assert(std::is_standard_layout<WritableSpan>::value, "writable span layout");
static_assert(offsetof(WritableSpan, len) == sizeof(void *), "writable span ABI");

int main()
{
    Document *doc = nullptr;
    const Status status = ar_document_new(AR_VERSION_2018, &doc);
    ar_document_free(doc);
    return status == AR_OK && ar_abi_version() == 1 ? 0 : 1;
}
