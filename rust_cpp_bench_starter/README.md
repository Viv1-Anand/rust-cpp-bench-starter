# Rust vs C++ Bench — Repro Harness (Starter)

This is a **starter** repo you can publish now and extend later. It includes:
- a Rust JSON-lines generator and parser
- a minimal C++ JSON-lines parser (no external libs)
- scripts, configs, and placeholder results so readers can run *something* immediately

> Tag **`v1.0`** matches this starter harness. Push more complete versions as `v1.1+`.

## Quick Start (5 steps)

# 1) clone repo, here we clone viv1-anand use appropriate repo
```
git clone https://github.com/viv1-anand/rust-cpp-bench-starter.git
cd rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench
```

# 2) generate 5M log lines (~300MB) as JSONL into ../../data/logs.jsonl
```
mkdir -p ../../data
cargo run --bin gen --release -- 5000000 ../../data/logs.jsonl
ls -l ../../data/logs.jsonl
```

# 3) run the Rust parser
```
cargo run --bin parse --release -- ../../data/logs.jsonl
```

# 4) build C++ and run the C++ parser
```
cd ../../cpp
make
bin/parse ../data/logs.jsonl
```

# Example session

```
wink@3900x 25-11-15T17:26:22.793Z:~/data/prgs/benchmarks
$ git clone https://github.com/viv1-anand/rust-cpp-bench-starter.git
cd rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench
Cloning into 'rust-cpp-bench-starter'...
remote: Enumerating objects: 20, done.
remote: Counting objects: 100% (20/20), done.
remote: Compressing objects: 100% (16/16), done.
remote: Total 20 (delta 0), reused 20 (delta 0), pack-reused 0 (from 0)
Receiving objects: 100% (20/20), 4.67 KiB | 4.67 MiB/s, done.
wink@3900x 25-11-15T17:26:39.560Z:~/data/prgs/benchmarks/rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench (main)
$ mkdir -p ../../data
cargo run --bin gen --release -- 5000000 ../../data/logs.jsonl
ls -l ../../data/logs.jsonl
    Updating crates.io index
     Locking 26 packages to latest compatible versions
      Adding rand v0.8.5 (available: v0.9.2)
   Compiling proc-macro2 v1.0.103
   Compiling libc v0.2.177
   Compiling quote v1.0.42
   Compiling unicode-ident v1.0.22
   Compiling zerocopy v0.8.27
   Compiling serde_core v1.0.228
   Compiling cfg-if v1.0.4
   Compiling serde v1.0.228
   Compiling powerfmt v0.2.0
   Compiling serde_json v1.0.145
   Compiling memchr v2.7.6
   Compiling ryu v1.0.20
   Compiling itoa v1.0.15
   Compiling num-conv v0.1.0
   Compiling time-core v0.1.6
   Compiling deranged v0.5.5
   Compiling syn v2.0.110
   Compiling getrandom v0.2.16
   Compiling rand_core v0.6.4
   Compiling time v0.3.44
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling serde_derive v1.0.228
   Compiling json_bench v0.1.0 (/home/wink/data/prgs/benchmarks/rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench)
    Finished `release` profile [optimized] target(s) in 3.28s
     Running `target/release/gen 5000000 ../../data/logs.jsonl`
-rw-r--r-- 1 wink users 433888890 Nov 15 09:27 ../../data/logs.jsonl
wink@3900x 25-11-15T17:27:01.504Z:~/data/prgs/benchmarks/rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench (main)
$ cargo run --bin parse --release -- ../../data/logs.jsonl
   Compiling json_bench v0.1.0 (/home/wink/data/prgs/benchmarks/rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench)
    Finished `release` profile [optimized] target(s) in 0.23s
     Running `target/release/parse ../../data/logs.jsonl`
rust_parse lines=5000000 duration_ms=2039.53
wink@3900x 25-11-15T17:27:48.347Z:~/data/prgs/benchmarks/rust-cpp-bench-starter/rust_cpp_bench_starter/rust/json_bench (main)
$ cd ../../cpp
make
bin/parse ../data/logs.jsonl
g++ -O3 -std=c++20 -march=native -Wall -Wextra src/parse.cpp -o bin/parse 
cpp_parse lines=5000000 ok=5000000 bad=0 duration_ms=169.27
wink@3900x 25-11-15T17:29:27.088Z:~/data/prgs/benchmarks/rust-cpp-bench-starter/rust_cpp_bench_starter/cpp (main)
```

# 5) compare timings printed by each


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
