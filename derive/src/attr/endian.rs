use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Error, Result};

use quote::quote;

use crate::attr;

fn parse_endian_attr(attr: &Attribute) -> Result<TokenStream> {
    let ident = attr.parse_args::<Ident>()?;

    match ident.to_string().as_str() {
        "_" | "ne" | "native" => Ok(quote! { NativeEndian }),
        "le" | "little" => Ok(quote! { LittleEndian }),
        "be" | "big" => Ok(quote! { BigEndian }),
        _ => Err(Error::new_spanned(ident, "failed to determine endian")),
    }
}

pub fn parse(attrs: &[Attribute]) -> Result<Option<TokenStream>> {
    attr::find(attrs, "endian")
        .map(parse_endian_attr)
        .transpose()
}

pub fn choice(attrs: &[Attribute], default: TokenStream) -> Result<TokenStream> {
    match parse(attrs)? {
        Some(attr) if default.to_string() == attr.to_string() => Err(Error::new_spanned(
            attrs.first(),
            "this attribute does not make sense",
        )),
        Some(attr) => Ok(attr),
        None => Ok(default),
    }
}
