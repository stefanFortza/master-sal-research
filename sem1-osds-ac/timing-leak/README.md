# Side-Channel Timing Leak Detector (Rust)

```markdown
# Side-Channel Timing Leak Detector (Rust)

A statistical verification tool designed to detect timing discrepancies in cryptographic routines and comparison operations on Linux x86_64 architectures.

## 1. Overview
This project implements leakage detection inspired by the **dudect** methodology (Welch's t-test) to verify whether critical functions adhere to the **constant-time execution** paradigm. It compares a vulnerable memory comparison against a constant-time bitwise implementation.

## 2. Theoretical Background
- **The Threat:** Non-constant-time comparisons (e.g., standard `memcmp` or `==`) abort execution at the first mismatching byte. An adversary observing CPU cycle counts can deduce secrets byte-by-byte.
- **CPU Serialization:** Modern out-of-order execution engines reorder instructions. To measure isolated routines accurately, memory barriers and serializing instructions (`lfence`, `rdtscp`) are mandatory.
- **Statistical Significance:** Welch's t-test is evaluated across two interleaved measurement vectors ($D_0$ and $D_1$). A threshold of $\vert{}t\vert{} > 4.5$ indicates statistically significant evidence of timing leakage.

## 3. Architecture & Edge Cases
- **Compiler Optimizations:** Uses `std::hint::black_box` to prevent LLVM from eliding comparison loops or vectorizing branching logic at `-O3`.
- **System Jitter & Context Switches:** Execution must be pinned to an isolated core to eliminate scheduler pollution:
  ```bash
  taskset -c 2 cargo run --release

```

* **Dynamic Frequency Scaling:** CPU frequency scaling (governor) alters cycle metrics dynamically. Benchmarks must run under a fixed frequency governor (`performance`).

## 4. Project Structure

```text
.
├── Cargo.toml
├── src/
│   ├── main.rs            # CLI entrypoint, argument parsing, output reporting
│   ├── cycle_counter.rs   # Serialized rdtsc/rdtscp measurement harness
│   ├── victims.rs         # Target functions: naive comparison vs constant-time
│   └── statistics.rs      # Online Welch's t-test calculation (Welford's algorithm)
└── tests/
    └── synthetic_leaks.rs # Sanity regression tests on known leaky functions

```

## 5. Getting Started

### Prerequisites

* Linux x86_64
* Rust toolchain (`stable` or `nightly`)

### Build & Run

```bash
# Compile with maximum optimizations
cargo build --release

# Run with CPU affinity to eliminate OS thread migration
taskset -c 1 ./target/release/timing-detector --samples 1000000 --target naive

```

## 6. Results & Interpretation

| Implementation | Sample Size | Max $t$-statistic | Verdict |
| --- | --- | --- | --- |
| `naive_memcmp` | $10^6$ | $> 15.0$ | **LEAK DETECTED** |
| `constant_time_eq` | $10^6$ | $< 2.1$ | **CONSTANT TIME** |

```

```
