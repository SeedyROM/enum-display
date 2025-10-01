#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub use enum_display_macro::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    use std::string::{String, ToString};

    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    use alloc::string::{String, ToString};

    #[allow(dead_code)]
    #[derive(EnumDisplay)]
    enum TestEnum {
        Name,

        #[display("Overridden Name")]
        OverriddenName,

        #[display("Unit: {variant}")]
        NameFullFormat,

        Address {
            street: String,
            city: String,
            state: String,
            zip: String,
        },

        #[display("Named: {variant} {{{street}, {zip}}}")]
        AddressPartialFormat {
            street: String,
            city: String,
            state: String,
            zip: String,
        },

        #[display("Named: {variant} {{{street}, {city}, {state}, {zip}}}")]
        AddressFullFormat {
            street: String,
            city: String,
            state: String,
            zip: String,
        },

        DateOfBirth(u32, u32, u32),

        #[display("Unnamed: {variant}({2})")]
        DateOfBirthPartialFormat(u32, u32, u32),

        #[display("Unnamed: {variant}({0}, {1}, {2})")]
        DateOfBirthFullFormat(u32, u32, u32),
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

    #[allow(dead_code)]
    #[derive(EnumDisplay)]
    enum TestEnumWithLifetimeAndGenerics<'a, T: Clone>
    where
        T: core::fmt::Display,
    {
        Name,
        Address {
            street: &'a T,
            city: &'a T,
            state: &'a T,
            zip: &'a T,
        },
        DateOfBirth(u32, u32, u32),
    }

    #[test]
    fn test_unit_field_variant() {
        assert_eq!(TestEnum::Name.to_string(), "Name");
        assert_eq!(TestEnum::OverriddenName.to_string(), "Overridden Name");
        assert_eq!(TestEnum::NameFullFormat.to_string(), "Unit: NameFullFormat");
    }

    #[test]
    fn test_named_fields_variant() {
        assert_eq!(
            TestEnum::Address {
                street: "123 Main St".to_string(),
                city: "Any Town".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string()
            }
            .to_string(),
            "Address"
        );
        assert_eq!(
            TestEnum::AddressPartialFormat {
                street: "123 Main St".to_string(),
                city: "Any Town".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string()
            }
            .to_string(),
            "Named: AddressPartialFormat {123 Main St, 12345}"
        );
        assert_eq!(
            TestEnum::AddressFullFormat {
                street: "123 Main St".to_string(),
                city: "Any Town".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string()
            }
            .to_string(),
            "Named: AddressFullFormat {123 Main St, Any Town, CA, 12345}"
        );
    }

    #[test]
    fn test_unnamed_fields_variant() {
        assert_eq!(TestEnum::DateOfBirth(1, 2, 1999).to_string(), "DateOfBirth");
        assert_eq!(
            TestEnum::DateOfBirthPartialFormat(1, 2, 1999).to_string(),
            "Unnamed: DateOfBirthPartialFormat(1999)"
        );
        assert_eq!(
            TestEnum::DateOfBirthFullFormat(1, 2, 1999).to_string(),
            "Unnamed: DateOfBirthFullFormat(1, 2, 1999)"
        );
    }

    #[test]
    fn test_unit_field_variant_case_transform() {
        assert_eq!(TestEnumWithAttribute::Name.to_string(), "name");
    }

    #[test]
    fn test_named_fields_variant_case_transform() {
        assert_eq!(
            TestEnumWithAttribute::Address {
                street: "123 Main St".to_string(),
                city: "Any Town".to_string(),
                state: "CA".to_string(),
                zip: "12345".to_string()
            }
            .to_string(),
            "address"
        );
    }

    #[test]
    fn test_unnamed_fields_variant_case_transform() {
        assert_eq!(
            TestEnumWithAttribute::DateOfBirth(1, 1, 2000).to_string(),
            "date-of-birth"
        );
    }

    #[test]
    fn test_unit_field_variant_with_lifetime_and_generics() {
        assert_eq!(
            TestEnumWithLifetimeAndGenerics::<'_, String>::Name.to_string(),
            "Name"
        );
    }

    #[test]
    fn test_named_fields_variant_with_lifetime_and_generics() {
        assert_eq!(
            TestEnumWithLifetimeAndGenerics::Address {
                street: &"123 Main St".to_string(),
                city: &"Any Town".to_string(),
                state: &"CA".to_string(),
                zip: &"12345".to_string()
            }
            .to_string(),
            "Address"
        );
    }

    #[test]
    fn test_unnamed_fields_variant_with_lifetime_and_generics() {
        assert_eq!(
            TestEnumWithLifetimeAndGenerics::<'_, String>::DateOfBirth(1, 1, 2000).to_string(),
            "DateOfBirth"
        );
    }
}
