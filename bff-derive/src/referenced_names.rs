use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident};

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
                    .map(|field| {
                        let name = field.ident.as_ref().unwrap();
                        quote! {
                            names.extend(self.#name.names());
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
                    .map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote! {
                            names.extend(self.#i.names());
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
                    match &variant.fields {
                        Fields::Named(named) => {
                            let (names, fields): (Vec<_>, Vec<_>) = named
                                .named
                                .iter()
                                .map(|field| {
                                    let name = field.ident.as_ref().unwrap();
                                    (
                                        name,
                                        quote! {
                                                names.extend(#name.names());
                                        },
                                    )
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
                                .map(|(i, _)| {
                                    let ident =
                                        Ident::new(format!("field_{}", i).as_str(), variant.span());
                                    (
                                        quote! {
                                            names.extend(#ident.names());
                                        },
                                        ident,
                                    )
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
                    }
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
            fn names(&self) -> std::collections::HashSet<crate::names::Name> {
                let mut names = std::collections::HashSet::new();
                #body
                names
            }
        }
    }
}
