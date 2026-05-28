<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Migrating from `rand` 0.10 to `vrd`

Drop-in mapping for the most common rand patterns. vrd
implements `rand_core::TryRng`, `Rng`, and `SeedableRng`, so
most code that *takes* an rng will work unchanged - only the
constructor lines need to change.

## Quick translation

| `rand` 0.10 | `vrd` equivalent | Notes |
| :-- | :-- | :-- |
| `use rand::prelude::*;` | `use vrd::Random;` | Plus `use rand::rand_core::TryRng;` if you use it |
| `let mut rng = rand::rng();` | `let mut rng = Random::new();` | OS-seeded |
| `let mut rng = rand::thread_rng();` | `let mut rng = Random::new();` | rand 0.9 form |
| `rng.random::<u32>()` | `rng.rand()` | |
| `rng.random::<u64>()` | `rng.u64()` | |
| `rng.random::<f64>()` | `rng.double()` | |
| `rng.random_range(0..n)` | `rng.uint(0, n - 1)` | both unbiased |
| `rng.random_range(0.0..1.0)` | `rng.uniform(0.0, 1.0)` | |
| `rng.gen_bool(0.5)` | `rng.bool(0.5)` | rand 0.9 form |
| `rng.fill_bytes(&mut buf)` | `rng.try_fill_bytes(&mut buf).unwrap()` | `try_fill_bytes` is from `rand_core::TryRng` |
| `rng.fill_array::<32>()` | `rng.fill_array::<32>()` | identical |
| `slice.choose(&mut rng)` | `rng.choose(slice)` | |
| `slice.shuffle(&mut rng)` | `rng.shuffle(slice)` | `alloc` |
| `let mut rng = StdRng::seed_from_u64(s);` | `let mut rng = Random::from_u64_seed(s);` | |
| `let mut rng = StdRng::from_seed([0; 32]);` | `let mut rng = Random::from_seed([0; 32]);` | |
| `let mut rng = SmallRng::seed_from_u64(s);` | `let mut rng = Random::from_u64_seed(s);` | vrd default is Xoshiro256++ |
| `rand_chacha::ChaCha20Rng::from_seed(s)` | `Random::from_secure_seed(s)` | needs `crypto` feature |
| `rand_chacha::ChaCha20Rng::from_os_rng()` | `Random::new_secure()` | needs `crypto + std` |

## Distributions

vrd ships four distributions inline; `rand_distr`'s 20+
catalogue is wider. For the four vrd covers:

| `rand_distr` | `vrd` equivalent |
| :-- | :-- |
| `Uniform::new(low, high).sample(&mut rng)` | `rng.uniform(low, high)` |
| `Normal::new(mu, sigma).unwrap().sample(&mut rng)` | `rng.normal(mu, sigma)` |
| `Exp::new(lambda).unwrap().sample(&mut rng)` | `rng.exponential(lambda)` |
| `Poisson::new(lambda).unwrap().sample(&mut rng) as u64` | `rng.poisson(lambda)` |

For distributions vrd doesn't ship, you can either:

1. Continue using `rand_distr` alongside vrd - vrd's `Random`
   implements `rand_core::TryRng`, so any `rand_distr`
   distribution will sample from it. Pass `&mut rng` directly.

2. Implement them yourself via vrd's `Distribution<T>` trait:

```rust
use vrd::{Distribution, Random};

struct Bernoulli { p: f64 }
impl Distribution<bool> for Bernoulli {
    fn sample(&self, rng: &mut Random) -> bool {
        rng.double() < self.p
    }
}
```

## Tokens / UUIDs

If you're using `uuid` + `rand` together:

| `uuid` + `rand` | `vrd` equivalent |
| :-- | :-- |
| `uuid::Uuid::new_v4().to_string()` | `rng.uuid_v4()` (`alloc`) |
| `uuid::Uuid::from_bytes(rng.random())` | `rng.uuid_v4_bytes()` (no_std) |

If you're using `hex::encode` for log IDs:

```rust
// rand + hex:
let mut buf = [0u8; 16];
rng.fill_bytes(&mut buf);
let token = hex::encode(buf);

// vrd:
let token = rng.hex_token(16);              // alloc-only
```

If you're using `base64::URL_SAFE_NO_PAD`:

```rust
// rand + base64:
let mut buf = [0u8; 15];
rng.fill_bytes(&mut buf);
let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf);

// vrd:
let token = rng.base64_token(15);           // alloc-only
```

## Dependency reduction

A typical "rand + companion crates" pull becomes a single vrd
dependency:

```toml
# Before
[dependencies]
rand = "0.10"
rand_chacha = "0.10"
rand_distr = "0.5"
uuid = { version = "1", features = ["v4"] }
hex = "0.4"
base64 = "0.22"

# After
[dependencies]
vrd = { version = "0.0.12", features = ["crypto"] }
```

The `crypto` feature pulls `rand_chacha` transitively; that's
the only extra dependency vrd adds for security-grade output.
For non-crypto use, vrd has no runtime dependencies beyond
`rand_core` (for trait compat) and `libm` (for `no_std`).

## When NOT to migrate

- You're using ≥5 distributions from `rand_distr` that vrd
  doesn't ship (binomial, gamma, log-normal, Cauchy, ...).
  Keep using `rand_distr`; you can pass vrd's `Random` to its
  `sample` methods anyway.
- You're using one of rand's specialised RNGs vrd doesn't
  carry (`ReseedingRng`, `IsaacRng`, ...).
- Your codebase has hundreds of call sites referencing
  `rand::random::<T>()` and the rewrite cost outweighs the
  dep-shrinking benefit. The two crates interoperate cleanly
  via `rand_core` - there's no need to flip the whole codebase.

## Stable-output caveat

The `rand` crate explicitly does **not** guarantee output
stability across versions. vrd does (across patch releases;
algorithm swaps are flagged in minor releases). If your test
suite has golden-file assertions that pin RNG output, vrd's
commitment can simplify your version-pinning logic.
