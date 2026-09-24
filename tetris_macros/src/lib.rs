use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(EnumAdvance)]
pub fn derive_enum_advance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl crate::utility::enum_advance::EnumAdvance for #name {
            fn enum_prev(&self) -> Option<Self> {
                let value = *self as usize;
                if value == 0 {
                    None
                } else {
                    Self::from_repr(value - 1)
                }
            }

            fn enum_next(&self) -> Option<Self> {
                let value = *self as usize;
                if value == Self::COUNT - 1 {
                    None
                } else {
                    Self::from_repr(value + 1)
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(EnumAdvanceCycle)]
pub fn derive_enum_advance_cycle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl crate::utility::enum_advance_cycle::EnumAdvanceCycle for #name {
            fn enum_prev_cycle(&self) -> Self {
                crate::utility::enum_advance::EnumAdvance::enum_prev(self)
                    .unwrap_or_else(|| Self::from_repr(Self::COUNT - 1).unwrap())
            }

            fn enum_next_cycle(&self) -> Self {
                crate::utility::enum_advance::EnumAdvance::enum_next(self)
                    .unwrap_or_else(|| Self::from_repr(0).unwrap())
            }
        }
    }
    .into()
}
