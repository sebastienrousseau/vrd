<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/vrd/images/logos/vrd.svg"
alt="Random (VRD) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Random (VRD) v0.0.3 🦀

A Rust library for generating high-quality random numbers based on the
Mersenne Twister algorithm.

*Part of the [Mini Functions][0] family of libraries.*

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Random (VRD) Banner][banner]

[![Made With Rust][made-with-rust-badge]][14] [![Crates.io][crates-badge]][8] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][9] [![License][license-badge]][2]

• [Website][1] • [Documentation][9] • [Report Bug][4] • [Request Feature][4] • [Contributing Guidelines][5]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

`Random (VRD)` is a Rust library for generating high-quality random numbers based on the Mersenne Twister algorithm.

The Mersenne Twister is a pseudorandom number generator (PRNG) that is often used in computer simulations and games. It is a fast and reliable PRNG.

The Random (VRD) is used to generate random numbers using the Mersenne Twister algorithm. It generates pseudorandom integers uniformly distributed in 0..(2^32 - 1) starting from any odd seed in 0..(2^32 - 1).

The index is incremented after each random number is generated. When the index reaches 624, the array is reinitialized and the index is reset to 0.

## Features ✨

- Create new random number generator and seed it with a value.
- Design for speed and efficiency in mind.
- Generate random 32-bit unsigned integer within a given range.
- Provide random numbers of different types, including booleans, bytes, chars, doubles, floats, integers, and unsigned integers.
- Mutate the state of the random number generator.
- Produce pseudo-random number sequences that are different from each other.
- Regulate the randomness of the generated numbers, including the seed value and the number of bits used.
- Select a random element from a slice of values.

## Changelog 📚

[0]: https://minifunctions.com/ "MiniFunctions"
[1]: https://vrdlib.one "Random (VRD)"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[4]: https://github.com/sebastienrousseau/vrd/issues "Issues"
[5]: https://github.com/sebastienrousseau/vrd/blob/main/CONTRIBUTING.md "Contributing Instructions"
[8]: https://crates.io/crates/vrd "Crates.io"
[9]: https://docs.rs/vrd "Docs.rs"
[10]: https://lib.rs/crates/vrd "Lib.rs"
[14]: https://www.rust-lang.org "The Rust Programming Language"

[banner]: https://kura.pro/vrd/images/titles/title-vrd.svg 'Random (VRD) Banner'
[crates-badge]: https://img.shields.io/crates/v/vrd.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/vrd.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.3-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/vrd.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
