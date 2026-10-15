# .bin policy file, v1 (all little-endian)

| Offset          | Field             | Notes                                                   |
|-----------------|-------------------|---------------------------------------------------------|
| 0x00..0x3F      | AegisPolicyHeader | 64 bytes                                                |
| 0x40..0x40+P    | ProjectionWeights | int8[P], P = header.reserved[0..4] (u32 LE), P % 64 == 0 |
| 0x40+P..end     | ThreatCentroids   | float32[K*D], L2-normalised rows                        |

Validation (Rust, before mmap swap): file.len() == 64 + P + K*D*4, magic == 0x41454753,
version == 1, P % 64 == 0.

## Contract gap found in Week 1
The charter's header has no field for the size of ProjectionWeights, so a reader cannot
locate the centroids or check file.len(). v1 fixes this WITHOUT changing the 64-byte
header by defining reserved[0..4] = P (header byte offset 24). Decide: keep this, or add a
real proj_bytes field and bump version to 2.

## Socket direction (Go <-> Rust)
The charter doesn't say who listens. v1: the Go agent LISTENS on /var/run/aegis.sock and
Rust CONNECTS. Frames are [u32 big-endian length][protobuf]. Rust->Go carries
SecurityAuditEvent; Go->Rust on the same connection carries PolicyReloadCommand.
