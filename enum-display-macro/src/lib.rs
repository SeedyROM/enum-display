use convert_case::{Case, Casing};
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Attribute, DeriveInput, FieldsNamed, FieldsUnnamed, Ident, Variant};

// Enum attributes
struct EnumAttrs {
    case_transform: Option<Case>,
}

impl EnumAttrs {
    fn from_attrs(attrs: Vec<Attribute>) -> Self {
        let mut case_transform: Option<Case> = None;

        for attr in attrs.into_iter() {
            if attr.path.is_ident("enum_display") {
                let meta = attr.parse_meta().unwrap();
                if let syn::Meta::List(list) = meta {
                    for nested in list.nested {
                        if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
                            if name_value.path.is_ident("case") {
                                if let syn::Lit::Str(lit_str) = name_value.lit {
                                    case_transform =
                                        Some(Self::parse_case_name(lit_str.value().as_str()));
                                }
                            }
                        }
                    }
                }
            }
        }

        Self { case_transform }
    }

    fn parse_case_name(case_name: &str) -> Case {
        match case_name {
            "Upper" => Case::Upper,
            "Lower" => Case::Lower,
            "Title" => Case::Title,
            "Toggle" => Case::Toggle,
            "Camel" => Case::Camel,
            "Pascal" => Case::Pascal,
            "UpperCamel" => Case::UpperCamel,
            "Snake" => Case::Snake,
            "UpperSnake" => Case::UpperSnake,
            "ScreamingSnake" => Case::ScreamingSnake,
            "Kebab" => Case::Kebab,
            "Cobol" => Case::Cobol,
            "UpperKebab" => Case::UpperKebab,
            "Train" => Case::Train,
            "Flat" => Case::Flat,
            "UpperFlat" => Case::UpperFlat,
            "Alternating" => Case::Alternating,
            _ => panic!("Unrecognized case name: {case_name}"),
        }
    }

    fn transform_case(&self, ident: String) -> String {
        if let Some(case) = self.case_transform {
            ident.to_case(case)
        } else {
            ident
        }
    }
}

// Variant attributes
struct VariantAttrs {
    format: Option<String>,
}

impl VariantAttrs {
    fn from_attrs(attrs: Vec<Attribute>) -> Self {
        let mut format = None;

        // Find the display attribute
        for attr in attrs.into_iter() {
            if attr.path.is_ident("display") {
                let meta = attr.parse_meta().unwrap();
                if let syn::Meta::List(list) = meta {
                    if let Some(first_nested) = list.nested.first() {
                        match first_nested {
                            // Handle literal string: #[display("format string")]
                            syn::NestedMeta::Lit(syn::Lit::Str(lit_str)) => {
                                format =
                                    Some(Self::translate_numeric_placeholders(&lit_str.value()));
                            }
                            // Handle named value: #[display(format = "format string")]
                            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                                if let syn::Lit::Str(lit_str) = &name_value.lit {
                                    format = Some(Self::translate_numeric_placeholders(
                                        &lit_str.value(),
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Self { format }
    }

    // Translates {123:?} to {_unnamed_123:?} for safer format arg usage
    fn translate_numeric_placeholders(fmt: &str) -> String {
        let re = Regex::new(r"\{\s*(\d+)\s*([^}]*)\}").unwrap();
        re.replace_all(fmt, |caps: &regex::Captures| {
            let idx = &caps[1];
            let fmt_spec = &caps[2];
            format!("{{_unnamed_{idx}{fmt_spec}}}")
        })
        .to_string()
    }
}

// Shared intermediate variant info
struct VariantInfo {
    ident: Ident,
    ident_transformed: String,
    attrs: VariantAttrs,
}

// Intermediate Named variant info
struct NamedVariantIR {
    info: VariantInfo,
    fields: Vec<Ident>,
}

impl NamedVariantIR {
    fn from_fields_named(fields_named: FieldsNamed, info: VariantInfo) -> Self {
        let fields = fields_named
            .named
            .into_iter()
            .filter_map(|field| field.ident)
            .collect();
        Self { info, fields }
    }

    fn generate(self, any_has_format: bool) -> proc_macro2::TokenStream {
        let VariantInfo {
            ident,
            ident_transformed,
            attrs,
        } = self.info;
        let fields = self.fields;
        match (any_has_format, attrs.format) {
            (true, Some(fmt)) => {
                quote! { #ident { #(#fields),* } => { let variant = #ident_transformed; format!(#fmt) } }
            }
            (true, None) => quote! { #ident { .. } => String::from(#ident_transformed), },
            (false, None) => quote! { #ident { .. } => #ident_transformed, },
            _ => unreachable!(
                "`any_has_format` should never be false when a variant has format string"
            ),
        }
    }
}

// Intermediate Unnamed variant info
struct UnnamedVariantIR {
    info: VariantInfo,
    fields: Vec<Ident>,
}

impl UnnamedVariantIR {
    fn from_fields_unnamed(fields_unnamed: FieldsUnnamed, info: VariantInfo) -> Self {
        let fields: Vec<Ident> = fields_unnamed
            .unnamed
            .into_iter()
            .enumerate()
            .map(|(i, _)| Ident::new(format!("_unnamed_{i}").as_str(), Span::call_site()))
            .collect();
        Self { info, fields }
    }

    fn generate(self, any_has_format: bool) -> proc_macro2::TokenStream {
        let VariantInfo {
            ident,
            ident_transformed,
            attrs,
        } = self.info;
        let fields = self.fields;
        match (any_has_format, attrs.format) {
            (true, Some(fmt)) => {
                quote! { #ident(#(#fields),*) => { let variant = #ident_transformed; format!(#fmt) } }
            }
            (true, None) => quote! { #ident(..) => String::from(#ident_transformed), },
            (false, None) => quote! { #ident(..) => #ident_transformed, },
            _ => unreachable!(
                "`any_has_format` should never be false when a variant has format string"
            ),
        }
    }
}

// Intermediate Unit variant info
struct UnitVariantIR {
    info: VariantInfo,
}

impl UnitVariantIR {
    fn new(info: VariantInfo) -> Self {
        Self { info }
    }

    fn generate(self, any_has_format: bool) -> proc_macro2::TokenStream {
        let VariantInfo {
            ident,
            ident_transformed,
            attrs,
        } = self.info;
        match (any_has_format, attrs.format) {
            (true, Some(fmt)) => {
                quote! { #ident => { let variant = #ident_transformed; format!(#fmt) } }
            }
            (true, None) => quote! { #ident => String::from(#ident_transformed), },
            (false, None) => quote! { #ident => #ident_transformed, },
            _ => unreachable!(
                "`any_has_format` should never be false when a variant has format string"
            ),
        }
    }
}

// Intermediate version of Variant
enum VariantIR {
    Named(NamedVariantIR),
    Unnamed(UnnamedVariantIR),
    Unit(UnitVariantIR),
}

impl VariantIR {
    fn from_variant(variant: Variant, enum_attrs: &EnumAttrs) -> Self {
        let ident_str = variant.ident.to_string();
        let info = VariantInfo {
            ident: variant.ident,
            ident_transformed: enum_attrs.transform_case(ident_str),
            attrs: VariantAttrs::from_attrs(variant.attrs),
        };
        match variant.fields {
            syn::Fields::Named(fields_named) => {
                Self::Named(NamedVariantIR::from_fields_named(fields_named, info))
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                Self::Unnamed(UnnamedVariantIR::from_fields_unnamed(fields_unnamed, info))
            }
            syn::Fields::Unit => Self::Unit(UnitVariantIR::new(info)),
        }
    }

    fn generate(self, any_has_format: bool) -> proc_macro2::TokenStream {
        match self {
            VariantIR::Named(named_variant) => named_variant.generate(any_has_format),
            VariantIR::Unnamed(unnamed_variant) => unnamed_variant.generate(any_has_format),
            VariantIR::Unit(unit_variant) => unit_variant.generate(any_has_format),
        }
    }

    fn has_format(&self) -> bool {
        match self {
            VariantIR::Named(named_variant) => &named_variant.info,
            VariantIR::Unnamed(unnamed_variant) => &unnamed_variant.info,
            VariantIR::Unit(unit_variant) => &unit_variant.info,
        }
        .attrs
        .format
        .is_some()
    }
}

#[proc_macro_derive(EnumDisplay, attributes(enum_display, display))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let DeriveInput {
        ident,
        data,
        attrs,
        generics,
        ..
    } = parse_macro_input!(input);

    // Copy generics and bounds
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Read enum attrs
    let enum_attrs = EnumAttrs::from_attrs(attrs);

    // Read variants and variant attrs into an intermediate format
    let intermediate_variants: Vec<VariantIR> = match data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => panic!("EnumDisplay can only be derived for enums"),
    }
    .into_iter()
    .map(|variant| VariantIR::from_variant(variant, &enum_attrs))
    .collect();

    // If any variants have a format string, the output of all match arms must be String instead of &str
    // This is because we can't return a reference to the temporary output of format!()
    let any_has_format = intermediate_variants.iter().any(|v| v.has_format());
    let post_fix = if any_has_format {
        quote! { .as_str() }
    } else {
        quote! {}
    };

    // Build the match arms
    let variants = intermediate_variants
        .into_iter()
        .map(|v| v.generate(any_has_format));

    // #[allow(unused_qualifications)] is needed
    // due to https://github.com/SeedyROM/enum-display/issues/1
    // Possibly related to https://github.com/rust-lang/rust/issues/96698
    let output = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #impl_generics ::core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        #(Self::#variants)*
                    }#post_fix
                )
            }
        }
    };
    output.into()
}
