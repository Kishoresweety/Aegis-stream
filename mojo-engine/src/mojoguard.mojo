# Week-1 skeleton of libmojoguard. UNVERIFIED against a Mojo toolchain: @export and
# UnsafePointer syntax shift between Mojo releases, so check against the SDK you pin.
# Typical build: mojo build --emit shared-lib src/mojoguard.mojo -o libmojoguard.so
from memory import UnsafePointer

alias VERDICT_PASS: Int32 = 0
alias VERDICT_REDACTED: Int32 = 1
alias VERDICT_BLOCK: Int32 = 2
alias VERDICT_ERROR: Int32 = -1
alias AEGIS_MAGIC: UInt32 = 0x41454753

@export(ABI="C")
fn mojo_inspect_window(
    window_ptr: UnsafePointer[UInt8],
    window_len: Int,
    inspect_offset: Int,
    policy_ptr: UnsafePointer[UInt8],
    out_threat_score: UnsafePointer[Float32],
    out_rule_id: UnsafePointer[UInt32],
) -> Int32:
    if (not window_ptr) or (not policy_ptr) or (not out_threat_score) or (not out_rule_id):
        return VERDICT_ERROR
    if inspect_offset > window_len:
        return VERDICT_ERROR
    var magic = policy_ptr.bitcast[UInt32]()[0]
    if magic != AEGIS_MAGIC:
        return VERDICT_ERROR
    out_threat_score[0] = 0.0
    out_rule_id[0] = 0
    return VERDICT_PASS  # Week 2: SIMD PII pass. Week 3: centroid similarity.
