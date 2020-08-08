use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Error, Result};

use quote::quote;

fn only_one<I: Iterator<Item = T>, T>(mut it: I) -> Option<T> {
    match (it.next(), it.next()) {
        (Some(first), None) => Some(first),
        _ => None,
    }
}

fn parse_endian_attr(attr: &Attribute) -> Result<TokenStream> {
    let ident = attr.parse_args::<Ident>()?;

    match ident.to_string().as_str() {
        "_" | "ne" | "native" => Ok(quote! { NativeEndian }),
        "le" | "little" => Ok(quote! { LittleEndian }),
        "be" | "big" => Ok(quote! { BigEndian }),
        _ => Err(Error::new_spanned(ident, "failed to determine endian")),
    }
}

pub fn find<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    only_one(attrs.iter().filter(|attr| attr.path.is_ident(name)))
}

pub fn parse_endian(attrs: &[Attribute]) -> Result<Option<TokenStream>> {
    find(attrs, "endian").map(parse_endian_attr).transpose()
}

pub fn get_endian(attrs: &[Attribute], default: TokenStream) -> Result<TokenStream> {
    match parse_endian(attrs)? {
        Some(attr) if default.to_string() == attr.to_string() => Err(Error::new_spanned(
            attrs.first(),
            "this attribute does not make sense",
        )),
        Some(attr) => Ok(attr),
        None => Ok(default),
    }
}
