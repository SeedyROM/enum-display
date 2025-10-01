#![no_std]

extern crate alloc;
use alloc::string::ToString;
use enum_display::EnumDisplay;

#[derive(EnumDisplay)]
enum SimpleEnum {
    Red,
    Green,
    Blue,
}

#[derive(EnumDisplay)]
enum FormattedEnum {
    #[display("Custom: {variant}")]
    Format,

    #[display("Value is {value}")]
    Data { value: u32 },

    #[display("Tuple: ({0}, {1})")]
    Tuple(i32, i32),
}

#[derive(EnumDisplay)]
#[enum_display(case = "Snake")]
enum CaseTransformEnum {
    CamelCase,
    AnotherExample,
    XmlHttpRequest,
}

#[derive(EnumDisplay)]
enum ComplexEnum {
    Unit,

    Named {
        _field1: u32,
        _field2: alloc::string::String,
    },

    #[display("Complex: {variant} with {field1}")]
    NamedFormat {
        field1: u32,
        field2: alloc::string::String,
    },

    #[allow(dead_code)]
    Tuple(u32, alloc::string::String),

    #[display("Tuple: ({0})")]
    TupleFormat(u32),
}

#[test]
fn test_simple_enum() {
    assert_eq!(SimpleEnum::Red.to_string(), "Red");
    assert_eq!(SimpleEnum::Green.to_string(), "Green");
    assert_eq!(SimpleEnum::Blue.to_string(), "Blue");
}

#[test]
fn test_formatted_enum() {
    assert_eq!(FormattedEnum::Format.to_string(), "Custom: Format");
    assert_eq!(FormattedEnum::Data { value: 42 }.to_string(), "Value is 42");
    assert_eq!(FormattedEnum::Tuple(10, 20).to_string(), "Tuple: (10, 20)");
}

#[test]
fn test_case_transform() {
    assert_eq!(CaseTransformEnum::CamelCase.to_string(), "camel_case");
    assert_eq!(
        CaseTransformEnum::AnotherExample.to_string(),
        "another_example"
    );
    assert_eq!(
        CaseTransformEnum::XmlHttpRequest.to_string(),
        "xml_http_request"
    );
}

#[test]
fn test_complex_enum() {
    assert_eq!(ComplexEnum::Unit.to_string(), "Unit");

    assert_eq!(
        ComplexEnum::Named {
            _field1: 123,
            _field2: "test".into()
        }
        .to_string(),
        "Named"
    );

    assert_eq!(
        ComplexEnum::NamedFormat {
            field1: 456,
            field2: "ignored".into()
        }
        .to_string(),
        "Complex: NamedFormat with 456"
    );

    assert_eq!(ComplexEnum::Tuple(789, "value".into()).to_string(), "Tuple");

    assert_eq!(ComplexEnum::TupleFormat(999).to_string(), "Tuple: (999)");
}

#[test]
fn test_core_fmt_usage() {
    use core::fmt::Write;

    let mut buffer = alloc::string::String::new();

    // Test simple enum
    write!(&mut buffer, "{}", SimpleEnum::Red).unwrap();
    assert_eq!(buffer, "Red");

    buffer.clear();

    // Test formatted enum
    write!(&mut buffer, "{}", FormattedEnum::Format).unwrap();
    assert_eq!(buffer, "Custom: Format");

    buffer.clear();

    // Test case transform
    write!(&mut buffer, "{}", CaseTransformEnum::CamelCase).unwrap();
    assert_eq!(buffer, "camel_case");
}

// Test that Display trait is properly implemented
#[test]
fn test_display_trait() {
    fn accepts_display<T: core::fmt::Display>(item: T) -> alloc::string::String {
        alloc::format!("{item}")
    }

    assert_eq!(accepts_display(SimpleEnum::Red), "Red");
    assert_eq!(accepts_display(FormattedEnum::Format), "Custom: Format");
    assert_eq!(accepts_display(CaseTransformEnum::CamelCase), "camel_case");
}

// Test with generics
#[derive(EnumDisplay)]
enum GenericEnum<T: core::fmt::Display> {
    Value(T),

    #[display("Generic: {0}")]
    FormattedValue(T),
}

#[test]
fn test_generic_enum() {
    assert_eq!(GenericEnum::Value(42u32).to_string(), "Value");
    assert_eq!(
        GenericEnum::FormattedValue(42u32).to_string(),
        "Generic: 42"
    );
}
