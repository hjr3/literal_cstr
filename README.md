# Literal C String

[![Build Status](https://travis-ci.com/hjr3/literal_cstr.svg?branch=master)](https://travis-ci.com/hjr3/literal_cstr)

A procedural macro for making literal C strings.

## Problem

I was tired of typing the following:

```rust
let s = CString::new("Hello, world!").unwrap();
```

Ideally, I would be able to do something similar to the syntax of a byte string `c"Hello, world!"` but that requires a language change.

## Solution

I decided to create a macro that is close enough.

```rust
let s = c!("Hello, world!");
```

## Installation

**Currently requires a nightly build of rust.** Use `rustc 1.28.0 (9634041f0 2018-07-30)` or newer.

```toml
[dependencies]
quote = "0.6"
```

```rust
#![feature(proc_macro_non_items)]

extern crate literal_cstr;

use literal_cstr::c;
```

Procedural macros are stable, but `#![feature(proc_macro_non_items)]` is required in order to use the macro with a literal string expression.

## Usage

```rust
let s = c!("Hello, world!");
```

Check out [/examples/show.rs](examples/show.rs) for a full example.

### Tests

```bash
$ cargo test
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
