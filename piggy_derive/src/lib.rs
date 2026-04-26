extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derives the [`Pig`](piggyl::Pig) trait: `pig()` returns `"StructName today is a ChosenPig"`.
///
/// Re-exported from `piggyl`; import via `use piggyl::Pig` instead of depending on this crate directly.
#[proc_macro_derive(Pig)]
pub fn pig_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let r#gen = quote! {
        impl piggyl::Pig for #struct_name {
            fn pig(&self) -> String {
                let struct_name = stringify!(#struct_name);
                let chosen_pig = piggyl::pick_pig_for(struct_name);
                format!("{} today is a {}", struct_name, chosen_pig)
            }
        }
    };
    r#gen.into()
}
