use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue,
    parse_macro_input,
};

#[proc_macro_derive(PhotonPacket, attributes(photon))]
pub fn derive_photon_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("PhotonPacket only supports named fields"),
        },
        _ => panic!("PhotonPacket only supports structs"),
    };

    let decode_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            let photon_attr = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("photon"));

            let Some(photon_attr) = photon_attr else {
                panic!(
                    "Field {} is missing #[photon(...)] attribute",
                    field_name.as_ref().unwrap()
                );
            };

            let parsed = parse_photon_attr(photon_attr);

            match parsed.kind {
                PhotonFieldKind::Simple { index, default } => {
                    let unwrap_expr = if let Some(default_val) = default {
                        quote! { .unwrap_or(#default_val) }
                    } else {
                        quote! { .unwrap_or_default() }
                    };

                    quote! {
                        #field_name: crate::ops::get_param::<#field_type>(&params, #index)
                            #unwrap_expr
                    }
                }
                PhotonFieldKind::DictKey {
                    index,
                    dict_key,
                    default,
                } => {
                    let unwrap_expr = if let Some(default_val) = default {
                        quote! { #default_val }
                    } else {
                        quote! { 0 }
                    };

                    quote! {
                        #field_name: match params.get(&#index) {
                            Some(::photon_decoder::PhotonValue::Dictionary(dict)) => {
                                match dict.get(&::photon_decoder::PhotonValue::Int(#dict_key)) {
                                    Some(::photon_decoder::PhotonValue::Long(v)) => *v as u64,
                                    Some(::photon_decoder::PhotonValue::Int(v)) => *v as u64,
                                    Some(::photon_decoder::PhotonValue::Short(v)) => *v as u64,
                                    _ => #unwrap_expr,
                                }
                            }
                            _ => #unwrap_expr,
                        }
                    }
                }
                PhotonFieldKind::DecodeWith { index, decoder } => {
                    quote! {
                        #field_name: crate::ops::get_param::<::std::vec::Vec<u8>>(&params, #index)
                            .map(|bytes| #decoder(&bytes))
                            .unwrap_or_default()
                    }
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #struct_name {
            pub fn decode(params: &std::collections::HashMap<u8, ::photon_decoder::PhotonValue>) -> Option<Self> {
                Some(Self {
                    #(#decode_fields,)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

enum PhotonFieldKind {
    Simple {
        index: u8,
        default: Option<Expr>,
    },
    DictKey {
        index: u8,
        dict_key: i32,
        default: Option<Expr>,
    },
    DecodeWith {
        index: u8,
        decoder: syn::Path,
    },
}

struct ParsedPhotonAttr {
    kind: PhotonFieldKind,
}

fn parse_photon_attr(attr: &Attribute) -> ParsedPhotonAttr {
    let mut index: Option<u8> = None;
    let mut dict_key: Option<i32> = None;
    let mut default: Option<Expr> = None;
    let mut decode_with: Option<syn::Path> = None;

    let nested = attr
        .parse_args_with(|input: syn::parse::ParseStream| {
            input.parse_terminated(Meta::parse, syn::Token![,])
        })
        .expect("Failed to parse #[photon(...)] attributes");

    for meta in nested {
        match meta {
            Meta::NameValue(MetaNameValue { path, value, .. }) => {
                if path.is_ident("index") {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Int(lit_int),
                        ..
                    }) = value
                    else {
                        panic!("photon index must be an integer literal");
                    };
                    index = Some(lit_int.base10_parse().expect("Invalid index value"));
                } else if path.is_ident("dict_key") {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Int(lit_int),
                        ..
                    }) = value
                    else {
                        panic!("photon dict_key must be an integer literal");
                    };
                    dict_key = Some(lit_int.base10_parse().expect("Invalid dict_key value"));
                } else if path.is_ident("default") {
                    default = Some(value);
                } else if path.is_ident("decode_with") {
                    let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }) = value
                    else {
                        panic!("photon decode_with must be a string literal path");
                    };
                    decode_with = Some(lit_str.parse().expect("Invalid decode_with path"));
                }
            }
            _ => panic!("Unsupported #[photon(...)] syntax"),
        }
    }

    let index = index.expect("#[photon(...)] must specify index = N");

    let kind = if let Some(decoder) = decode_with {
        PhotonFieldKind::DecodeWith { index, decoder }
    } else if let Some(dict_key) = dict_key {
        PhotonFieldKind::DictKey {
            index,
            dict_key,
            default,
        }
    } else {
        PhotonFieldKind::Simple { index, default }
    };

    ParsedPhotonAttr { kind }
}
