// mojoguard.h - Strictly enforced ABI between Rust and Mojo
#ifndef MOJOGUARD_H
#define MOJOGUARD_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#define VERDICT_PASS     0
#define VERDICT_REDACTED 1
#define VERDICT_BLOCK    2
#define VERDICT_ERROR   -1

typedef struct __attribute__((packed, aligned(64))) {
    uint32_t magic;              // 0x41454753 ("AEGS"), little-endian
    uint32_t version;            // 1
    uint32_t num_centroids;      // K
    uint32_t embed_dim;          // D
    float    block_threshold;
    uint32_t enable_pii_mask;    // 1=CC, 2=Aadhaar, 4=PAN, 8=HighEntropy
    uint8_t  reserved[40];       // v1: reserved[0..4] = proj_bytes (uint32 LE), see BIN_FORMAT.md
} AegisPolicyHeader;

int32_t mojo_inspect_window(
    uint8_t*       window_ptr,
    size_t         window_len,
    size_t         inspect_offset,
    const uint8_t* policy_mmap_ptr,
    float*         out_threat_score,
    uint32_t*      out_rule_id
);

#ifdef __cplusplus
}
#endif
#endif // MOJOGUARD_H
