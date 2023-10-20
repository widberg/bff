use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr};

pub fn derive_named_class(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let class_name = LitStr::new(format!("{}_Z", name).as_str(), name.span());
    let class_name_legacy = LitStr::new(&name.to_string().to_uppercase(), name.span());

    // This mess can go away once https://github.com/rust-lang/rust/issues/76001 is stabilized
    quote! {
        impl crate::traits::NamedClass<crate::names::NameAsobo32> for #name {
            const NAME: crate::names::NameAsobo32 = crate::names::NameAsobo32::new(crate::crc::asobo32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameAsobo32 = crate::names::NameAsobo32::new(crate::crc::asobo32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameAsoboAlternate32> for #name {
            const NAME: crate::names::NameAsoboAlternate32 = crate::names::NameAsoboAlternate32::new(crate::crc::asobo_alternate32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameAsoboAlternate32 = crate::names::NameAsoboAlternate32::new(crate::crc::asobo_alternate32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameKalisto32> for #name {
            const NAME: crate::names::NameKalisto32 = crate::names::NameKalisto32::new(crate::crc::kalisto32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameKalisto32 = crate::names::NameKalisto32::new(crate::crc::kalisto32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameBlackSheep32> for #name {
            const NAME: crate::names::NameBlackSheep32 = crate::names::NameBlackSheep32::new(crate::crc::blacksheep32(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameBlackSheep32 = crate::names::NameBlackSheep32::new(crate::crc::blacksheep32(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<crate::names::NameAsobo64> for #name {
            const NAME: crate::names::NameAsobo64 = crate::names::NameAsobo64::new(crate::crc::asobo64(#class_name.as_bytes()));
            const NAME_LEGACY: crate::names::NameAsobo64 = crate::names::NameAsobo64::new(crate::crc::asobo64(#class_name_legacy.as_bytes()));
        }

        impl crate::traits::NamedClass<&'static str> for #name {
            const NAME: &'static str = #class_name;
            const NAME_LEGACY: &'static str = #class_name_legacy;
        }
    }
}
