extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate proc_macro_error;
extern crate convert_case;

use convert_case::{Case, Casing as ConvertCasing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, token, AttrStyle, Attribute, AttributeArgs, Fields, Item, ItemEnum,
    ItemStruct, Meta, NestedMeta, Path, PathSegment,
};

enum Casing {
    Pascal,
    Camel,
    Lower,
    Upper,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

impl FromStr for Casing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PascalCase" => Ok(Self::Pascal),
            "CamelCase" => Ok(Self::Camel),
            "LowerCase" => Ok(Self::Lower),
            "UpperCase" => Ok(Self::Upper),
            "SnakeCase" => Ok(Self::Snake),
            "ScreamingSnakeCase" => Ok(Self::ScreamingSnake),
            "KebabCase" => Ok(Self::Kebab),
            "ScreamingKebabCase" => Ok(Self::ScreamingKebab),
            _ => Err(format!(
                "unknown casing: {} try one of {:?}",
                s,
                Casing::all()
            )),
        }
    }
}

impl ToString for Casing {
    fn to_string(&self) -> String {
        let case = match self {
            Self::Camel => "camelCase",
            Self::Pascal => "PascalCase",
            Self::Lower => "lowercase",
            Self::Upper => "UPPERCASE",
            Self::Snake => "snake_case",
            Self::ScreamingSnake => "SCREAMING_SNAKE_CASE",
            Self::Kebab => "kebab-case",
            Self::ScreamingKebab => "SCREAMING-KEBAB-CASE",
        };
        case.to_string()
    }
}

impl Casing {
    const fn all() -> &'static [&'static str] {
        return &[
            "PascalCase",
            "CamelCase",
            "LowerCase",
            "UpperCase",
            "SnakeCase",
            "ScreamingSnakeCase",
            "KebabCase",
            "ScreamingKebabCase",
        ];
    }
}

#[proc_macro_attribute]
pub fn serde_alias(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut aliases = vec![];

    for arg in args {
        if let NestedMeta::Meta(meta) = arg {
            if let Meta::Path(path) = meta {
                let case_ident = path.get_ident().expect("expected casing");
                let case =
                    Casing::from_str(&case_ident.to_string()).unwrap_or_else(|e| panic!("{}", e));

                aliases.push(case);
            }
        }
    }

    let input = parse_macro_input!(input as Item);

    match input {
        Item::Enum(input) => alias_enum(aliases, input),
        Item::Struct(input) => alias_struct(aliases, input),
        _ => abort!(input, "Only supported on structs or enums"),
    }
}

fn alias_struct(aliases: Vec<Casing>, mut input: ItemStruct) -> TokenStream {
    if let Fields::Named(ref mut named) = input.fields {
        for field in &mut named.named {
            let mut punc_attr = Punctuated::new();

            punc_attr.push_value(PathSegment {
                ident: format_ident!("serde"),
                arguments: Default::default(),
            });

            let mut casings = vec![];
            for case in &aliases {
                let convert_casing = match case {
                    Casing::Pascal => Case::Pascal,
                    Casing::Camel => Case::Camel,
                    Casing::Lower => Case::Lower,
                    Casing::Upper => Case::Upper,
                    Casing::Snake => Case::Snake,
                    Casing::ScreamingSnake => Case::ScreamingSnake,
                    Casing::Kebab => Case::Kebab,
                    Casing::ScreamingKebab => Case::Cobol,
                };

                let converted = field
                    .ident
                    .as_ref()
                    .expect("invalid field")
                    .to_string()
                    .to_case(convert_casing);

                let f = format!(r#"alias = "{}""#, converted);
                casings.push(f);
            }

            let res: String = casings.join(",");

            field.attrs.push(Attribute {
                pound_token: token::Pound::default(),
                style: AttrStyle::Outer,
                bracket_token: Default::default(),
                path: Path {
                    leading_colon: None,
                    segments: punc_attr.clone(),
                },
                tokens: TokenStream2::from_str(&format!("({})", res.as_str()))
                    .unwrap_or_else(|a| abort!(punc_attr, format!("Lex error: {}", a))),
            })
        }

        let tokens = quote! {#input};
        return tokens.into();
    }

    abort!(input, "Tuple structs not supported")
}

fn alias_enum(_aliases: Vec<Casing>, mut _input: ItemEnum) -> TokenStream {
    unimplemented!();
}
