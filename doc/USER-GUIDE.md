<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# User guide

A walkthrough of vrd's surface for first-time users. Read
top-to-bottom for the full tour; jump to a section if you know
what you need.

## Picking a constructor

```rust
use vrd::Random;

// 1. Default - Xoshiro256++, entropy-seeded from the OS.
//    Requires `std`. Fastest non-crypto path.
# #[cfg(feature = "std")]
let mut rng = Random::new();

// 2. Deterministic - same seed → same sequence forever.
//    Works in pure no_std.
let mut rng = Random::from_u64_seed(42);
let mut rng = Random::from_seed([0x42u8; 32]);

// 3. Legacy reproducibility - MT19937 for matching old test
//    vectors. Requires `alloc + std` for the entropy variant.
# #[cfg(all(feature = "alloc", feature = "std"))]
let mut rng = Random::new_mersenne_twister();
# #[cfg(feature = "alloc")]
let mut rng = Random::new_mersenne_twister_with_seed(42);

// 4. Crypto-quality - ChaCha20. Requires `crypto` feature.
# #[cfg(all(feature = "crypto", feature = "std"))]
let mut rng = Random::new_secure();
# #[cfg(feature = "crypto")]
let mut rng = Random::from_secure_seed([0u8; 32]);

// 5. PCG - small-state, fastest through the facade.
//    Requires `pcg` feature.
# #[cfg(all(feature = "pcg", feature = "std"))]
let mut rng = Random::new_pcg32();          // 16-byte state
# #[cfg(feature = "pcg")]
let mut rng = Random::new_pcg32_with_seed(42);
# #[cfg(all(feature = "pcg", feature = "std"))]
let mut rng = Random::new_pcg64();          // 32-byte state
# #[cfg(feature = "pcg")]
let mut rng = Random::new_pcg64_with_seed(42u128);
```

## Numbers

```rust
use vrd::Random;
# let mut rng = Random::from_u64_seed(1);

let n: u32 = rng.rand();        // any u32
let n: u64 = rng.u64();         // any u64
let n: i64 = rng.i64();         // any i64

let n = rng.int(1, 100);        // i32 in [1, 100], unbiased
let n = rng.uint(1, 100);       // u32 in [1, 100], unbiased
let n = rng.random_range(0, 50); // u32 in [0, 50), unbiased
let n = rng.bounded(50);        // u32 in [0, 50), unbiased
```

All bounded paths use Lemire's nearly-divisionless method -
no modulo bias even when the requested range doesn't divide
2³² cleanly.

## Floats

```rust
use vrd::Random;
# let mut rng = Random::from_u64_seed(1);

let x = rng.float();            // f32 in [0, 1), 24 mantissa bits
let x = rng.double();           // f64 in [0, 1), 53 mantissa bits
let x = rng.f64();              // alias for double()
let x = rng.uniform(-5.0, 5.0); // f64 in [-5.0, 5.0)
```

`float()` and `double()` use the maximum mantissa bits the
type allows; both are guaranteed to land in `[0, 1)` exclusive
of 1.0.

## Distributions

```rust
use vrd::Random;
# let mut rng = Random::from_u64_seed(1);

let z = rng.normal(0.0, 1.0);     // Ziggurat sampler, ~3.7 ns
let t = rng.exponential(1.5);     // mean = 1/lambda
let k = rng.poisson(3.0);         // u64; mean = lambda
let x = rng.uniform(0.0, 1.0);    // continuous uniform
```

For user-defined distributions, implement the `Distribution<T>`
trait:

```rust
use vrd::{Distribution, Random};

struct Bernoulli { p: f64 }
impl Distribution<bool> for Bernoulli {
    fn sample(&self, rng: &mut Random) -> bool { rng.double() < self.p }
}

let mut rng = Random::from_u64_seed(1);
let coin = Bernoulli { p: 0.5 }.sample(&mut rng);

// or batch-sample as an iterator:
let many: Vec<bool> = Bernoulli { p: 0.3 }
    .samples(&mut rng)
    .take(100)
    .collect();
# let _ = coin; let _ = many;
```

## Bytes

```rust
use vrd::Random;
# let mut rng = Random::from_u64_seed(1);

// Stack-allocated, allocation-free:
let buf: [u8; 32] = rng.fill_array();

// Mutable slice, any length:
use rand::rand_core::TryRng;
let mut buf = [0u8; 1024];
rng.try_fill_bytes(&mut buf).unwrap();

// Heap-allocated Vec<u8> (alloc-only):
# #[cfg(feature = "alloc")]
let buf = rng.bytes(32);
```

For bulk byte generation enable the `simd` feature for the
2–3× SIMD-batched path. The same seed produces a different
byte stream under `simd` vs. scalar - see
[`xoshiro_simd`](../src/xoshiro_simd.rs) for the contract.

## Slice operations

```rust
use vrd::Random;
# let mut rng = Random::from_u64_seed(1);

// Pick one (returns Option<&T>):
let pick = rng.choose(&[10, 20, 30]);

// Sample without replacement (alloc-only):
# #[cfg(feature = "alloc")]
let picks = rng.sample(&[1, 2, 3, 4, 5], 3);

// Sample with replacement:
# #[cfg(feature = "alloc")]
let picks = rng.sample_with_replacement(&[1, 2, 3, 4, 5], 10);

// Shuffle in place (alloc-only):
# #[cfg(feature = "alloc")]
{
    let mut deck: Vec<u32> = (0..52).collect();
    rng.shuffle(&mut deck);
}

// Weighted pick (Option<&T>):
let pick = rng.weighted(&[10, 20, 30], &[1.0, 5.0, 1.0]);
```

## Tokens

```rust
# #[cfg(all(feature = "alloc", feature = "std"))]
# {
use vrd::Random;
let mut rng = Random::new();

let id = rng.uuid_v4_bytes();        // [u8; 16], no_std
let id = rng.uuid_v4();              // "8400a91d-..." RFC 4122 hyphenated
let log_id = rng.hex_token(16);      // 32 lowercase hex chars
let csrf = rng.base64_token(15);     // 20 URL-safe base64 chars
# }
```

For **security-sensitive** tokens (API keys, session IDs,
password-reset links, CSRF), enable the `crypto` feature and
generate via `Random::new_secure()`:

```rust
# #[cfg(all(feature = "crypto", feature = "std"))]
# {
use vrd::Random;
let mut rng = Random::new_secure();              // ChaCha20 CSPRNG

let session_id = rng.base64_token(32);           // 256 bits crypto-grade
let api_key = rng.hex_token(32);                 // 64 hex chars crypto-grade
# }
```

## Parallel streams

For fan-out work where each worker needs its own deterministic
stream, use `Random::split()` on the default Xoshiro backend:

```rust
use vrd::Random;

let mut parent = Random::from_u64_seed(0);
let mut child = parent.split().expect("Xoshiro supports split");
// parent and child produce non-overlapping subsequences.
// Each can be sent to a different thread.
assert_ne!(parent.u64(), child.u64());
```

`split()` returns `None` on MT, PCG, and ChaCha20 backends -
none have an analogous fixed-distance jump. For those backends,
derive distinct seeds per worker manually.

## Iterators

For taking `n` samples without manual loops:

```rust
use vrd::Random;
# let mut rng = Random::from_u64_seed(1);

let xs: Vec<u32> = rng.iter_u32().take(10).collect();
let xs: Vec<u64> = rng.iter_u64().take(10).collect();

# #[cfg(feature = "alloc")]
let bytes: Vec<u8> = rng.iter_bytes().take(1024).collect();
```

## Save & restore state

With the `serde` feature:

```toml
vrd = { version = "0.0.12", features = ["serde"] }
```

```rust,ignore
use vrd::Random;

let mut rng = Random::from_u64_seed(42);
let snap = serde_json::to_string(&rng).unwrap();

let mut restored: Random = serde_json::from_str(&snap).unwrap();
assert_eq!(rng.rand(), restored.rand());
```

`Random`, `Xoshiro256PlusPlus`, `Pcg32`, `Pcg64`, `ChaChaRng`,
`MersenneTwisterParams`, and `MersenneTwisterConfig` all derive
`Serialize` / `Deserialize` under the `serde` feature.

## Quasi-random for Monte Carlo

```rust
# #[cfg(feature = "quasirandom")]
# {
use vrd::quasirandom::SobolSequence;

let mut s = SobolSequence::new(2);
let mut inside = 0;
for _ in 0..4096 {
    let p = s.next_point::<2>();
    if p[0] * p[0] + p[1] * p[1] < 1.0 { inside += 1; }
}
let pi_estimate = 4.0 * (inside as f64) / 4096.0;
// ~3.143 - closer to π than a uniform PRNG of the same N.
# }
```

See [`examples/halton.rs`](../examples/halton.rs) and
[`examples/sobol.rs`](../examples/sobol.rs) for side-by-side
convergence demos against the default PRNG.

## Where to look next

- Architecture: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- Testing strategy: [`TESTING.md`](TESTING.md)
- Comparison vs other crates: [`COMPARISON.md`](COMPARISON.md)
- Migration from `rand`: [`MIGRATION-FROM-RAND.md`](MIGRATION-FROM-RAND.md)
- Migration from `fastrand`: [`MIGRATION-FROM-FASTRAND.md`](MIGRATION-FROM-FASTRAND.md)
- Engineering policies: [`POLICIES.md`](POLICIES.md)
- Benchmarks: [`BENCHMARKS.md`](BENCHMARKS.md)
- Every public method's worked example: [docs.rs/vrd](https://docs.rs/vrd)
