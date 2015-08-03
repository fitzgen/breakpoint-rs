# `breakpoint!()`

[![Build Status](https://travis-ci.org/fitzgen/breakpoint-rs.png?branch=master)](https://travis-ci.org/fitzgen/breakpoint-rs)

[![crates.io](http://meritbadge.herokuapp.com/breakpoint)](https://crates.io/crates/breakpoint)

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
breakpoint = "0.1.0"
```

Import the macro into your crate:

```rust
#![feature(asm)]
#[macro_use] extern crate breakpoint;
```

Set breakpoints!

```rust
breakpoint!();
```

Set breakpoints with conditions!

```rust
breakpoint!(ref_count == 1);
```

## Documentation

[Read the docs!](https://fitzgen.github.io/breakpoint-rs/breakpoint/index.html)
