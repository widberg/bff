use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parenthesized, DataStruct, DeriveInput, Ident, PathArguments, Type};

const PRIMITIVES: &[&str] = &[
    "bool",
    "char",
    "f32",
    "f64",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "Name",
    "PascalString",
    "Vec2",
    "Vec2f",
    "Vec2i16",
    "Vec3",
    "Vec3f",
    "Vec4",
    "Vec4f",
    "Vec4i16",
    "Quat",
    "RGB",
    "RGBA",
    "Mat",
    "Mat3f",
    "Mat4f",
    "Mat3x4f",
    "KeyframerBezierRot",
    "KeyframerFloatComp",
    "KeyframerMessage",
    "KeyframerRot",
    "KeyframerVec3fComp",
];

struct SpecificClass<'a> {
    name: &'a Ident,
    generic_name: Ident,
    data: &'a DataStruct,
    is_complete: bool,
    // is_link_header: LitBool,
}

fn simple_parse(input: &DeriveInput) -> SpecificClass {
    let attrs = &input.attrs;
    let name = &input.ident;
    let mut custom_name: Option<Ident> = None;
    let mut is_complete = false;
    for attr in attrs {
        if attr.path().is_ident("generic") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let content;
                    parenthesized!(content in meta.input);
                    let name: Ident = content.parse()?;
                    custom_name = Some(name);
                    return Ok(());
                }
                if meta.path.is_ident("complete") {
                    is_complete = true;
                    return Ok(());
                }
                Err(meta.error(format!("unknown attribute {:?}", meta.path)))
            })
            .unwrap();
            break;
        }
    }
    let generic_name = match custom_name {
        Some(name) => name,
        None => {
            let gen_name = format!(
                "{}Generic",
                &input
                    .ident
                    .to_string()
                    .split_inclusive("Body")
                    .next()
                    .unwrap()
            );
            Ident::new(&gen_name, gen_name.span())
        }
    };
    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("Not a struct"),
    };
    SpecificClass {
        name,
        generic_name,
        data,
        is_complete,
    }
}

fn attrs(attrs: &Vec<syn::Attribute>) -> bool {
    let mut into = true;
    for attr in attrs {
        if attr.path().is_ident("generic") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("no_convert") {
                    into = false;
                }
                Ok(())
            });
        }
    }
    into
}

pub fn derive_generic_class(input: DeriveInput) -> TokenStream {
    let from_specific_to_generic = impl_from_specific_to_generic(&input);
    let from_generic_substitute = impl_from_generic_substitute(&input);
    quote! {
        #from_specific_to_generic
        #from_generic_substitute
    }
}

fn impl_from_specific_to_generic(input: &DeriveInput) -> TokenStream {
    let class = simple_parse(input);
    let (name, generic_name, complete) = (class.name, class.generic_name, class.is_complete);
    let generic_intos = class
        .data
        .fields
        .iter()
        .filter(|f| {
            complete
                || !f
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("generic"))
                    .collect::<Vec<_>>()
                    .is_empty()
        })
        .map(|f| {
            let field_ident = f.ident.as_ref().expect("Only named structs are supported");
            let field_type = &f.ty;

            let into = attrs(&f.attrs);

            if into {
                if let Type::Path(p) = field_type {
                    if p.path.get_ident().is_none() {
                        let first = p.path.segments.first().unwrap();
                        if first.ident.to_string() == "DynArray" {
                            return quote! { #field_ident: object.#field_ident.inner.into_iter().map(|x| x.into()).collect::<Vec<_>>().into() };
                        } else if first.ident.to_string() == "Vec" {
                            let item_type = match first.arguments {
                                PathArguments::AngleBracketed(ref args) => args.args.first().unwrap(),
                                _ => panic!("Invalid type"),
                            };
                            if PRIMITIVES.contains(&item_type.to_token_stream().to_string().as_str()) {
                                return quote! { #field_ident: object.#field_ident };
                            } else {
                                return quote! { #field_ident: object.#field_ident.into_iter().map(|x| x.into()).collect::<Vec<_>>() };
                            }
                        }
                    }
                }
                quote! { #field_ident: object.#field_ident.into() }
            } else {
                quote! { #field_ident: object.#field_ident }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl From<#name> for super::generic::#generic_name {
            fn from(object: #name) -> Self {
                super::generic::#generic_name {
                    #(#generic_intos),*
                }
            }
        }
    }
}

fn impl_from_generic_substitute(input: &DeriveInput) -> TokenStream {
    let class = simple_parse(input);
    let (name, generic_name, complete) = (class.name, class.generic_name, class.is_complete);
    let fields = class.data.fields.iter()
        .map(|f| {
            let replace_generic =
                complete || !f.attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("generic"))
                    .collect::<Vec<_>>()
                    .is_empty();

            let field_ident = f.ident.as_ref().expect("Only named structs are supported");
            let field_type = &f.ty;

            if !replace_generic {
                return quote! { #field_ident: substitute.#field_ident };
            }

            let into = attrs(&f.attrs);

            if into {
                if let Type::Path(p) = field_type {
                    if let Some(id) = p.path.get_ident() {
                        if PRIMITIVES.contains(&id.to_string().as_str()) {
                            return quote! { #field_ident: substitute.#field_ident.into() };
                        }
                    } else {
                        let first = p.path.segments.first().unwrap();
                        if first.ident.to_string() == "DynArray" {
                            return quote! { #field_ident: std::iter::zip(generic.#field_ident.inner, substitute.#field_ident.inner)
                                .map(|(gen, sub)| <_>::try_from_generic_substitute(gen, sub).unwrap()).collect::<Vec<_>>().into() };
                        }
                        else if first.ident.to_string() == "Vec" {
                            let item_type = match first.arguments {
                                PathArguments::AngleBracketed(ref args) => args.args.first().unwrap(),
                                _ => panic!("Invalid type"),
                            };
                            if PRIMITIVES.contains(&item_type.to_token_stream().to_string().as_str()) {
                                return quote! { #field_ident: generic.#field_ident };
                            } else {
                                return quote! { #field_ident: std::iter::zip(generic.#field_ident, substitute.#field_ident)
                                    .map(|(gen, sub)| <_>::try_from_generic_substitute(gen, sub).unwrap()).collect::<Vec<_>>() };
                            }
                        }
                    }
                }
                quote! { #field_ident: #field_type::try_from_generic_substitute(generic.#field_ident, substitute.#field_ident)? }
            } else {
                quote! { #field_ident: generic.#field_ident }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        impl crate::traits::TryFromGenericSubstitute<super::generic::#generic_name, #name> for #name {
            type Error = crate::error::Error;
            fn try_from_generic_substitute(generic: super::generic::#generic_name, substitute: #name) -> crate::BffResult<Self> {
                Ok(#name {
                    #(#fields),*,
                })
            }
        }
    }
}
