// Week-1 stand-in for libmojoguard implementing the exact mojoguard.h ABI, so the Rust
// proxy builds and runs without the Mojo toolchain. Swap in the real lib via AEGIS_MOJO_LIB_DIR.
#include "mojoguard.h"
#include <string.h>

int32_t mojo_inspect_window(uint8_t* window_ptr, size_t window_len, size_t inspect_offset,
                            const uint8_t* policy_mmap_ptr, float* out_threat_score,
                            uint32_t* out_rule_id) {
    if (!window_ptr || !policy_mmap_ptr || !out_threat_score || !out_rule_id) return VERDICT_ERROR;
    if (inspect_offset > window_len) return VERDICT_ERROR;
    uint32_t magic;
    memcpy(&magic, policy_mmap_ptr, 4);
    if (magic != 0x41454753u) return VERDICT_ERROR;
    *out_threat_score = 0.0f;
    *out_rule_id = 0;
    return VERDICT_PASS;
}
