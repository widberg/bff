use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use crate::generic_class::derive_generic_class;
use crate::referenced_names::derive_referenced_names;

mod generic_class;
mod referenced_names;

#[proc_macro_derive(GenericClass, attributes(generic))]
pub fn generic_class(input: TokenStream) -> TokenStream {
    derive_generic_class(parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(ReferencedNames, attributes(referenced_names))]
pub fn referenced_names(input: TokenStream) -> TokenStream {
    derive_referenced_names(parse_macro_input!(input as DeriveInput)).into()
}
