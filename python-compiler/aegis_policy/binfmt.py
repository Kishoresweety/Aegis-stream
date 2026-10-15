"""Deterministic writer/verifier for the AegisStream v1 .bin policy format.
Layout (little-endian): 64B header | int8 projection (P bytes, P%64==0) | float32 K*D centroids.
header = magic,version,K,D (4xu32) | block_threshold f32 | enable_pii_mask u32 | reserved[40]
reserved[0..4] = P (see contracts/BIN_FORMAT.md)."""
import math
import random
import struct

MAGIC = 0x41454753
VERSION = 1
HEADER_LEN = 64
_HDR = "<IIIIfI40s"
assert struct.calcsize(_HDR) == HEADER_LEN


def _pad64(n: int) -> int:
    return (n + 63) // 64 * 64


def write_bin(path, centroids, proj, block_threshold=0.85, enable_pii_mask=0xF):
    """centroids: list of K lists of D floats (re-normalised here). proj: list of ints in [-128,127]."""
    k = len(centroids)
    d = len(centroids[0])
    if any(len(c) != d for c in centroids):
        raise ValueError("ragged centroids")
    if any(not -128 <= x <= 127 for x in proj):
        raise ValueError("projection values must fit int8")
    p = _pad64(len(proj))
    reserved = struct.pack("<I", p) + b"\x00" * 36
    out = bytearray(struct.pack(_HDR, MAGIC, VERSION, k, d, block_threshold, enable_pii_mask, reserved))
    out += struct.pack(f"<{len(proj)}b", *proj) + b"\x00" * (p - len(proj))
    for c in centroids:
        n = math.sqrt(sum(x * x for x in c)) or 1.0
        out += struct.pack(f"<{d}f", *[x / n for x in c])
    with open(path, "wb") as f:
        f.write(out)
    return bytes(out)


def verify_bin(data: bytes) -> dict:
    """Same rules the Rust proxy enforces. Raises ValueError on any violation."""
    if len(data) < HEADER_LEN:
        raise ValueError("file too small")
    magic, ver, k, d, thr, mask, reserved = struct.unpack(_HDR, data[:HEADER_LEN])
    if magic != MAGIC:
        raise ValueError("bad magic")
    if ver != VERSION:
        raise ValueError(f"unsupported version {ver}")
    (p,) = struct.unpack("<I", reserved[:4])
    if p % 64:
        raise ValueError("proj_bytes not 64-aligned")
    if len(data) != HEADER_LEN + p + k * d * 4:
        raise ValueError(f"length {len(data)} != expected {HEADER_LEN + p + k * d * 4}")
    return {"k": k, "d": d, "proj_bytes": p, "threshold": thr, "pii_mask": mask, "size": len(data)}


def build_dummy(path, k=4, d=64, buckets=1024, seed=1234):
    """Deterministic dummy policy for pipeline bring-up (not a real model)."""
    rng = random.Random(seed)
    proj = [rng.randint(-127, 127) for _ in range(buckets * d)]
    cents = [[rng.gauss(0, 1) for _ in range(d)] for _ in range(k)]
    return write_bin(path, cents, proj)
