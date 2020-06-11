extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};

mod io;
mod attr;
mod fields;

#[proc_macro_derive(Io, attributes(endian))]
pub fn derive_io(input: TokenStream) -> TokenStream {
	io::derive(parse_macro_input!(input as DeriveInput))
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}
