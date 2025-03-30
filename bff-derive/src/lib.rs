use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::bff_class::{derive_bff_class, BffClassMacroInput};
use crate::bigfiles::{derive_bigfiles, BffBigFileMacroInput};
use crate::generic_class::derive_generic_class;
use crate::named_class::derive_named_class;
use crate::referenced_names::derive_referenced_names;
use crate::trivial_class::{derive_trivial_class, TrivialClassMacroInput};

mod bff_class;
mod bigfiles;
mod generic_class;
mod named_class;
mod referenced_names;
mod trivial_class;

#[proc_macro_derive(NamedClass)]
pub fn named_class(input: TokenStream) -> TokenStream {
    derive_named_class(parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(GenericClass, attributes(generic))]
pub fn generic_class(input: TokenStream) -> TokenStream {
    derive_generic_class(parse_macro_input!(input as DeriveInput), false).into()
}

#[proc_macro_derive(GenericClassComplete, attributes(generic))]
pub fn generic_class_complete(input: TokenStream) -> TokenStream {
    derive_generic_class(parse_macro_input!(input as DeriveInput), true).into()
}

#[proc_macro]
pub fn bff_class(input: TokenStream) -> TokenStream {
    derive_bff_class(parse_macro_input!(input as BffClassMacroInput)).into()
}

#[proc_macro]
pub fn bigfiles(input: TokenStream) -> TokenStream {
    derive_bigfiles(parse_macro_input!(input as BffBigFileMacroInput)).into()
}

#[proc_macro]
pub fn trivial_class(input: TokenStream) -> TokenStream {
    derive_trivial_class(parse_macro_input!(input as TrivialClassMacroInput)).into()
}

#[proc_macro_derive(ReferencedNames, attributes(referenced_names))]
pub fn referenced_names(input: TokenStream) -> TokenStream {
    derive_referenced_names(parse_macro_input!(input as DeriveInput)).into()
}
