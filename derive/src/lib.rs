use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attr;
mod fields;
mod io;

#[proc_macro_derive(Io, attributes(endian))]
pub fn derive_io(input: TokenStream) -> TokenStream {
    io::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
