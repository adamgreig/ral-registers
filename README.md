# ral-registers

[![Version](https://img.shields.io/crates/v/ral-registers.svg)](https://crates.io/crates/ral-registers)
[![Documentation](https://docs.rs/ral-registers/badge.svg)](https://docs.rs/ral-registers)
![CI](https://github.com/adamgreig/ral-registers/workflows/CI/badge.svg)
![License](https://img.shields.io/crates/l/ral-registers.svg)

This crate contains an MMIO abstraction that uses macros to read, modify,
and write fields in registers.

For example, several fields on a register can be updated (without changing the
other fields) using:

```rust
// Modify some fields on GPIOA.MODER without changing others.
modify_reg!(gpio, GPIOA, MODER, MODER1: Input, MODER2: Output, MODER3: Input);

// Check a condition on a field.
while read_reg!(gpio, GPIOA, IDR, IDR3 == High) {}

// Read and write the entire register word value.
let port = read_reg!(gpio, GPIOA, IDR);
write_reg!(gpio, GPIOA, port);
```

This crate contains register code originally written in
[stm32ral](https://github.com/adamgreig/stm32ral), extracted
for easier use in other projects.

## Licence

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
