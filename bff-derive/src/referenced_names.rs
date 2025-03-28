use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields, Ident};

fn does_not_have_skip_attr(attrs: &Vec<Attribute>) -> bool {
    let mut skip = false;
    for attr in attrs {
        if !attr.path().is_ident("referenced_names") {
            continue;
        }
        if let syn::Meta::List(meta) = &attr.meta {
            if meta.tokens.is_empty() {
                continue;
            }
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                skip = true;
            }
            Ok(())
        })
        .unwrap();
        if skip {
            break;
        }
    }
    !skip
}

pub fn derive_referenced_names(mut input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    for param in &mut input.generics.params {
        if let syn::GenericParam::Type(ty) = param {
            ty.bounds
                .push(syn::parse_quote!(crate::traits::ReferencedNames));
        }
    }
    let generics = &mut input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named) => {
                let fields = named
                    .named
                    .iter()
                    .filter(|field| does_not_have_skip_attr(&field.attrs))
                    .map(|field| {
                        let name = field.ident.as_ref().unwrap();
                        quote! {
                            self.#name.extend_referenced_names(names);
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    #(#fields)*
                }
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .filter(|(_, field)| does_not_have_skip_attr(&field.attrs))
                    .map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote! {
                            self.#i.extend_referenced_names(names);
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    #(#fields)*
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        Data::Enum(data) => {
            let variants = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    does_not_have_skip_attr(&variant.attrs)
                        .then(|| match &variant.fields {
                            Fields::Named(named) => {
                                let (names, fields): (Vec<_>, Vec<_>) = named
                                    .named
                                    .iter()
                                    .map(|field| {
                                        does_not_have_skip_attr(&field.attrs)
                                            .then(|| {
                                                let name = field.ident.as_ref().unwrap();
                                                (
                                                    name,
                                                    quote! {
                                                        #name.extend_referenced_names(names);
                                                    },
                                                )
                                            })
                                            .unwrap_or_else(|| {
                                                let name = field.ident.as_ref().unwrap();
                                                (name, quote! {})
                                            })
                                    })
                                    .unzip();

                                quote! {
                                    #name::#variant_name { #(#names,)* } => {
                                        #(#fields)*
                                    }
                                }
                            }
                            Fields::Unnamed(unnamed) => {
                                let (fields, names): (Vec<_>, Vec<_>) = unnamed
                                    .unnamed
                                    .iter()
                                    .enumerate()
                                    .map(|(i, field)| {
                                        does_not_have_skip_attr(&field.attrs)
                                            .then(|| {
                                                let ident = Ident::new(
                                                    format!("field_{}", i).as_str(),
                                                    variant.span(),
                                                );
                                                (
                                                    quote! {
                                                        #ident.extend_referenced_names(names);
                                                    },
                                                    ident,
                                                )
                                            })
                                            .unwrap_or_else(|| {
                                                let ident = Ident::new(
                                                    format!("field_{}", i).as_str(),
                                                    variant.span(),
                                                );
                                                (quote! {}, ident)
                                            })
                                    })
                                    .unzip();

                                quote! {
                                    #name::#variant_name(#(#names,)*) => {
                                        #(#fields)*
                                    }
                                }
                            }
                            Fields::Unit => {
                                quote! { #name::#variant_name => {} }
                            }
                        })
                        .unwrap_or_else(|| {
                            quote! { #name::#variant_name => {} }
                        })
                })
                .collect::<Vec<_>>();

            quote! {
                match self {
                    #(#variants)*
                }
            }
        }
        Data::Union(_) => {
            unimplemented!()
        }
    };

    quote! {
        impl #impl_generics crate::traits::ReferencedNames for #name #ty_generics #where_clause {
            #[allow(unused_variables)]
            #[inline]
            fn extend_referenced_names(&self, names: &mut std::collections::HashSet<crate::names::Name>) {
                #body
            }
        }
    }
}
