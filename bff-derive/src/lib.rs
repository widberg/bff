use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use crate::bff_class::{BffClassMacroInput, derive_bff_class};
use crate::bigfiles::{BffBigFileMacroInput, derive_bigfiles};
use crate::generic_class::derive_generic_class;
use crate::named_class::derive_named_class;
use crate::referenced_names::derive_referenced_names;
use crate::trivial_class::{TrivialClassMacroInput, derive_trivial_class};

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
    derive_generic_class(parse_macro_input!(input as DeriveInput)).into()
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
