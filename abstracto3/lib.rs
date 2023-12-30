#![doc = include_str!("../README.md")]
#![deny(missing_docs, unused, clippy::all)]

use proc_macro::TokenStream;

/// AbstractO3
#[proc_macro_attribute]
pub fn abstracto3(cfg: TokenStream, input: TokenStream) -> TokenStream {
    ::macros::abstracto3(input, cfg).into()
}
