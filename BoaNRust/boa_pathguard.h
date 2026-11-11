#ifndef BOA_PATHGUARD_H
#define BOA_PATHGUARD_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 0 = accepted; negative values = rejected/error. */
int boa_validate_path(const char *document_root,
                      const char *candidate_path,
                      char *out,
                      size_t out_len);

#ifdef __cplusplus
}
#endif

#endif
