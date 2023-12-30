#![doc = include_str!("../README.md")]
#![deny(missing_docs, unused, clippy::all)]

use proc_macro::TokenStream;

/// AbstractO3
#[proc_macro]
pub fn abstracto3(input: TokenStream) -> TokenStream {
    ::macros::abstracto3(input).into()
}
