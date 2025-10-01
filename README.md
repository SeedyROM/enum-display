# `enum-display`

[![GitHub](https://img.shields.io/badge/github-enum--display-8da0cb?logo=github)](https://github.com/SeedyROM/enum-display)
[![crates.io version](https://img.shields.io/crates/v/enum-display.svg)](https://crates.io/crates/enum-display)
[![docs.rs docs](https://docs.rs/enum-display/badge.svg)](https://docs.rs/enum-display)
[![crates.io version](https://img.shields.io/crates/l/enum-display.svg)](https://github.com/SeedyROM/enum-display/blob/main/LICENSE)
[![CI build](https://github.com/SeedyROM/enum-display/actions/workflows/rust.yml/badge.svg)](https://github.com/SeedyROM/enum-display/actions)

`enum-display` is a crate for implementing `std::fmt::Display` on enum variants with macros.

## Features

- **`std` (default)**: Enables standard library support for convenience methods like `to_string()`
- **`no_std`**: Core functionality that works in `no_std` environments without allocation

### Using in `no_std` environments

To use in `no_std` mode, disable default features:

```toml
[dependencies]
enum-display = { version = "0.2.1", default-features = false }
```

The crate works without allocation by writing directly to the formatter:

```rust,no_run
#![no_std]
extern crate alloc;
use alloc::string::ToString;
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum Status {
    Ready,
    #[display("Error: {code}")]
    Error { code: u32 },
}

fn main() {
    // Works in no_std!
    assert_eq!(Status::Ready.to_string(), "Ready");
}
```

## Simple Example

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

## Example With Custom Case Transform

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

## Custom Variant Formatting with `#[display]`

The `#[display]` attribute allows you to customize how individual enum variants are formatted. This attribute accepts a format string that follows Rust's standard formatting syntax.

### Basic Usage

```rust
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum Status {
    // Unit variant with custom text
    #[display("System is ready")]
    Ready,
    
    // Using the variant name with {variant}
    #[display("{variant}: Operation completed")]
    Success,
}

assert_eq!(Status::Ready.to_string(), "System is ready");
assert_eq!(Status::Success.to_string(), "Success: Operation completed");
```

### Field Access Patterns

The `#[display]` attribute provides different ways to access variant data:

| Variant Type  | Access Pattern | Example |
|---------------|----------------|---------|
| **Unit**      | `{variant}` only | `#[display("{variant} occurred")]` |
| **Named {...}** | Field names | `#[display("Error: {message} (code: {code})")]` |
| **Tuple (...)** | Positional indices | `#[display("Processing {0} of {1}")]` |

### Named Fields Example

```rust
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum Response {
    #[display("Success: {message}")]
    Success { message: String },
    
    #[display("Error {code}: {description}")]
    Error { code: u32, description: String },
}

let success = Response::Success { 
    message: "Data saved".to_string() 
};
assert_eq!(success.to_string(), "Success: Data saved");
```

### Tuple Fields Example

```rust
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum Progress {
    #[display("Loading... {0}%")]
    Loading(u8),
    
    #[display("Processing item {0} of {1}")]
    Processing(usize, usize),
}

assert_eq!(Progress::Loading(75).to_string(), "Loading... 75%");
assert_eq!(Progress::Processing(3, 10).to_string(), "Processing item 3 of 10");
```

## Advanced Formatting

The `#[display]` attribute supports all of Rust's format string features:

```rust
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum Metrics {
    #[display("CPU: {usage:.1}%")]
    CpuUsage { usage: f64 },
    
    #[display("Memory: {used:>8} / {total:<8} bytes")]
    Memory { used: usize, total: usize },
    
    #[display("Temperature: {0:3}°C")]
    Temperature(i32),
}
```
