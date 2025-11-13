# Rust vs C++ Bench — Repro Harness (Starter)

This is a **starter** repo you can publish now and extend later. It includes:
- a Rust JSON-lines generator and parser
- a minimal C++ JSON-lines parser (no external libs)
- scripts, configs, and placeholder results so readers can run *something* immediately

> Tag **`v1.0`** matches this starter harness. Push more complete versions as `v1.1+`.

## Quick Start (5 steps)

```bash
# 1) clone
git clone https://github.com/YOUR_USER/rust-cpp-bench-starter.git
cd rust-cpp-bench-starter

# 2) generate 5M log lines (~300MB) as JSONL
cargo run --bin gen --release -- 5000000 data/logs.jsonl

# 3) run the Rust parser
cargo run --bin parse --release -- data/logs.jsonl

# 4) build C++ and run the C++ parser
make -C cpp
./cpp/bin/parse data/logs.jsonl

# 5) compare timings printed by each
```

## What’s here

- `rust/` — Cargo workspace with two binaries:
  - `gen` — generates newline-delimited JSON logs
  - `parse` — parses the logs with `serde_json` and prints timing
- `cpp/` — tiny JSON-lines parser using basic string scanning (no external deps)
- `results/` — placeholder CSVs you can replace with your real runs
- `configs/` — sample compiler versions and CPU info to keep runs reproducible

## Notes

- Allocator choice matters for alloc-heavy tests. This starter uses default allocators.
- The C++ parser intentionally avoids third-party JSON libs so it builds anywhere.
- For full apples-to-apples with your article, replace the parsers with your real implementations and update `/results`.

## License

MIT — see `LICENSE`.
