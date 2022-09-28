//!
//! EnumDisplay is a crate for implementing [`std::fmt::Display`] on enum variants with macros.
//!
//! # Simple Example
//!
//! ```rust
//! use enum_display_derive::EnumDisplay;
//!
//! #[derive(EnumDisplay)]
//! enum Color {
//!    Red,
//!    Green,
//!    Blue,
//! }
//!
//! assert_eq!(Color::Red.to_string(), "Red");
//! assert_eq!(Color::Green.to_string(), "Green");
//! assert_eq!(Color::Blue.to_string(), "Blue");
//! ```
//!
//! # Example With Custom Case Transform
//!
//! Any case from [convert_case](https://docs.rs/convert_case/latest/convert_case/) is supported.
//!
//! ```rust
//! use enum_display_derive::EnumDisplay;
//!
//! #[derive(EnumDisplay)]
//! #[enum_display(case = "Kebab")]
//! enum Message {
//!     HelloGreeting { name: String },
//! }
//!
//! assert_eq!(Message::Hello { name: "Alice".to_string() }.to_string(), "hello-greeting");
//!

pub use enum_display_derive::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(EnumDisplay)]
    enum TestEnum {
        Name,
        Address {
            street: String,
            city: String,
            state: String,
            zip: String,
        },
        DateOfBirth(u32, u32, u32),
    }

    #[allow(dead_code)]
    #[derive(EnumDisplay)]
    #[enum_display(case = "Kebab")]
    enum TestEnumWithAttribute {
        Name,
        Address {
            street: String,
            city: String,
            state: String,
            zip: String,
        },
        DateOfBirth(u32, u32, u32),
    }

    #[test]
    fn test_unit_field_variant() {
        assert!(TestEnum::Name.to_string() == "Name");
    }

    #[test]
    fn test_named_fields_variant() {
        assert!(
            TestEnum::Address {
                street: "123 Main St".to_string(),
                city: "Any Town".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string()
            }
            .to_string()
                == "Address"
        );
    }

    #[test]
    fn test_unnamed_fields_variant() {
        assert!(TestEnum::DateOfBirth(1, 1, 2000).to_string() == "DateOfBirth");
    }

    #[test]
    fn test_unit_field_variant_case_transform() {
        assert!(TestEnumWithAttribute::Name.to_string() == "name");
    }

    #[test]
    fn test_named_fields_variant_case_transform() {
        assert!(
            TestEnumWithAttribute::Address {
                street: "123 Main St".to_string(),
                city: "Any Town".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string()
            }
            .to_string()
                == "address"
        );
    }

    #[test]
    fn test_unnamed_fields_variant_case_transform() {
        assert!(TestEnumWithAttribute::DateOfBirth(1, 1, 2000).to_string() == "date-of-birth");
    }
}
