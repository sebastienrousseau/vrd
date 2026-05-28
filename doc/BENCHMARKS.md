# vrd benchmarks

Reference results for vrd's performance and statistical-quality claims.
The numbers below are recorded on a fixed machine and updated on each
release; the *methodology* is reproducible from this repository — re-run
`cargo bench` and `cargo run --release --example crush --features crush`
on your own hardware to verify locally.

Last updated: 2026-05-27 (vrd v0.0.11).
Reference machine: Apple Silicon (M-series), macOS 25.5, rustc stable.
Profiles: `--release` with `opt-level = 3`, `lto = true`,
`codegen-units = 1`.

---

## Performance — `cargo bench`

Numbers from `cargo bench --features simd,pcg,quasirandom --bench benchmark`,
recorded on the reference machine on 2026-05-27. Times are the
criterion median; ±1 ns variance is normal.

| Operation                       | Path                       | Time          | Notes |
| :---                            | :---                       | ---:          | :--- |
| `Random::rand()` (`u32`)        | Xoshiro256++ (default)     | 3.09 ns       | Facade dispatch + Xoshiro inner. |
|                                 | MT19937                    | 3.12 ns       |  |
|                                 | **PCG32**                  | **2.72 ns**   | Fastest backend through the facade. |
|                                 | PCG64                      | 4.26 ns       | u64 internal, high 32 bits used. |
|                                 | `fastrand` (reference)     | 0.60 ns       | Direct calls, no trait dispatch. |
|                                 | `rand::rng()` (reference)  | 4.05 ns       |  |
| `Random::u64()`                 | Xoshiro256++ (default)     | 3.10 ns       | Native 64-bit output. |
|                                 | MT19937                    | 4.68 ns       | Two `u32`s concatenated. |
|                                 | PCG32                      | 3.72 ns       | Two `u32`s concatenated. |
|                                 | PCG64                      | 4.21 ns       | Native 64-bit output. |
| `Random::normal(0, 1)`          | Ziggurat (Marsaglia 2000)  | **3.71 ns**   | Was 14.7 ns under the polar method (4× faster). |
| `Random::try_fill_bytes` (1 KiB)| Scalar                     | 138 ns        | 7.4 GB/s. |
|                                 | **SIMD** (`--features simd`)| **63.3 ns**  | 16.2 GB/s; **2.18× over scalar**. |
| `Random::try_fill_bytes` (16 KiB)| Scalar                    | 2188 ns       | 7.5 GB/s. |
|                                 | **SIMD** (`--features simd`)| **736 ns**   | 22.3 GB/s; **2.97× over scalar**. |
| `Random::split()`               | Xoshiro `jump()` + clone   | 326 ns        | One-time setup per parallel split (≈256 scalar `next_u64`s). |
| Quasi-random `Halton(d=2)`      | `HaltonSequence::new(2)`   | 20.7 ns/pt    | Two radical-inverse passes. |
| Quasi-random `Halton(d=4)`      | `HaltonSequence::new(4)`   | 30.6 ns/pt    | |
| Quasi-random `Sobol(d=2)`       | `SobolSequence::new(2)`    | **1.07 ns/pt**| Trailing-zero + XOR per dim. |
| Quasi-random `Sobol(d=4)`       | `SobolSequence::new(4)`    | **2.06 ns/pt**| |

Reproduce:

```bash
cargo bench --features simd,pcg,quasirandom --bench benchmark
```

---

## Statistical validation — PractRand

[PractRand](http://pracrand.sourceforge.net/) is the de-facto modern
statistical test battery for non-cryptographic PRNGs. We pipe 256 MiB
per backend through `RNG_test stdin64` and record pass / anomaly /
failure counts. ChaCha20 (CSPRNG) is included for completeness but its
crypto-quality output is expected to clear all PractRand sub-tests by
construction.

Reproduce: `cargo run --release --example crush --features crush,pcg,crypto`.

| Backend         | Tests run | Anomalies | Failures | Verdict |
| :---            | ---:      | ---:      | ---:     | :---:   |
| Xoshiro256++    | _(record here after running on the reference machine)_ | | | |
| MersenneTwister | | | | |
| PCG32           | | | | |
| PCG64           | | | | |
| ChaCha20        | | | | |

> Maintainer note: install PractRand 0.94+ (build the `RNG_test` binary
> per its README), put it in `PATH`, and run the command above. Paste
> the resulting markdown row into this file before tagging the
> release.

---

## Distribution shape — empirical moments

`Random::normal(0, 1)` (Ziggurat) over 200 000 samples — checked in
`src/ziggurat.rs::tests::moments_match_standard_normal`:

| Statistic         | Target | Tolerance | Observed |
| :---              | ---:   | ---:      | ---:     |
| Mean              | 0.0    | ±0.02     | pass     |
| Standard deviation| 1.0    | ±0.02     | pass     |
| Skewness          | 0.0    | ±0.05     | pass     |
| Excess kurtosis   | 0.0    | ±0.10     | pass     |

These tests run in every CI build and pin the sampler's correctness
even if future table-generation refactors change individual `K`/`W`/`F`
entries.
