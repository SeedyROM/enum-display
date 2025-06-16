use convert_case::{Case, Casing};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
        _ => panic!("Unrecognized case name: {}", case_name),
    }
}

#[proc_macro_derive(EnumDisplay, attributes(enum_display))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let DeriveInput {
        ident, data, attrs, generics, ..
    } = parse_macro_input!(input);

    // Should we transform the case of the enum variants?
    let mut case_transform: Option<Case> = None;

    // Find the enum_display attribute
    for attr in attrs.into_iter() {
        if attr.path.is_ident("enum_display") {
            let meta = attr.parse_meta().unwrap();
            if let syn::Meta::List(list) = meta {
                for nested in list.nested {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested {
                        if name_value.path.is_ident("case") {
                            if let syn::Lit::Str(lit_str) = name_value.lit {
                                // Set the case transform
                                case_transform = Some(parse_case_name(lit_str.value().as_str()));
                            }
                        }
                    }
                }
            }
        }
    }

    // Build the match arms
    let variants = match data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => panic!("EnumDisplay can only be derived for enums"),
    }
    .into_iter()
    .map(|variant| {
        let ident = variant.ident;
        let ident_str = if case_transform.is_some() {
            ident.to_string().to_case(case_transform.unwrap())
        } else {
            ident.to_string()
        };

        match variant.fields {
            syn::Fields::Named(_) => quote! {
                #ident { .. } => #ident_str,
            },
            syn::Fields::Unnamed(_) => quote! {
                #ident(..) => #ident_str,
            },
            syn::Fields::Unit => quote! {
                #ident => #ident_str,
            },
        }
    });

    // Copy generics and bounds
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
                        #(#ident::#variants)*
                    },
                )
            }
        }
    };
    output.into()
}
