# `enum-display`

[![GitHub](https://img.shields.io/badge/github-enum--display-8da0cb?logo=github)](https://github.com/SeedyROM/enum-display)
[![crates.io version](https://img.shields.io/crates/v/enum-display.svg)](https://crates.io/crates/enum-display)
[![docs.rs docs](https://docs.rs/enum-display/badge.svg)](https://docs.rs/enum-display)
[![crates.io version](https://img.shields.io/crates/l/enum-display.svg)](https://github.com/SeedyROM/enum-display/blob/main/LICENSE-APACHE)
[![CI build](https://github.com/SeedyROM/enum-display/actions/workflows/rust.yml/badge.svg)](https://github.com/SeedyROM/enum-display/actions)

`enum-display` is a crate for implementing `std::fmt::Display` on enum variants with macros.

# Simple Example

```rust
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum Color {
  Red,
  Green,
  Blue,
}

assert_eq!(Color::Red.to_string(), "Red");
assert_eq!(Color::Green.to_string(), "Green");
assert_eq!(Color::Blue.to_string(), "Blue");
```

# Example With Custom Case Transform

Any case from [convert_case](https://docs.rs/convert_case/latest/convert_case/) is supported.

```rust
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
#[enum_display(case = "Kebab")]
enum Message {
    HelloGreeting { name: String },
}

assert_eq!(Message::HelloGreeting { name: "Alice".to_string() }.to_string(), "hello-greeting");
```
