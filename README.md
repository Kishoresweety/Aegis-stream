# AegisStream: Week 1 (ABI Foundation & Pass-Through Proxy)

## Exit criterion
Rust proxy compiles, links libmojoguard, calls `mojo_inspect_window` on a live SSE stream, logs VERDICT_PASS.

## Run order
```bash
# 1. Dummy policy (stdlib Python only)
cd python-compiler && python3 -m aegis_policy.cli dummy-bin /tmp/active.bin && python3 -m unittest discover tests

# 2. Mock upstream
python3 tools/mock_sse.py 8000 &

# 3. Rust proxy (uses the C stub unless AEGIS_MOJO_LIB_DIR points at a real libmojoguard)
cd rust-proxy && cargo test
AEGIS_POLICY=/tmp/active.bin AEGIS_UPSTREAM=http://127.0.0.1:8000 RUST_LOG=info cargo run --release
curl -N -X POST localhost:8080/v1/chat/completions -d '{}'    # expect VERDICT_PASS log lines

# 4. Go agent (needs protoc + protoc-gen-go for `make proto`)
cd go-control-plane && make test && AEGIS_SOCK=/tmp/aegis.sock go run ./cmd/aegis-node-agent
```

## Status
| Piece | State |
|---|---|
| contracts/ (header, proto, BIN_FORMAT.md) | done; two gaps flagged in BIN_FORMAT.md |
| python-compiler dummy .bin + verifier + tests | done, tested |
| mojo-engine C stub + smoke test | done, tested |
| mojo-engine/src/mojoguard.mojo | written, NOT compiled (no Mojo toolchain checked) |
| rust-proxy | written, NOT compiled |
| go-control-plane | written, NOT compiled; Rust does not connect to the socket until Week 3/4 |
