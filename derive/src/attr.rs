use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Error, Meta, NestedMeta, Result};

use quote::quote;

fn only_one<I: Iterator<Item = T>, T>(mut it: I) -> Option<T> {
    match (it.next(), it.next()) {
        (Some(first), None) => Some(first),
        _ => None,
    }
}

fn determine_endian(ident: &Ident) -> Result<TokenStream> {
    match ident.to_string().as_str() {
        "_" | "ne" | "native" => Ok(quote! { NativeEndian }),
        "le" | "little" => Ok(quote! { LittleEndian }),
        "be" | "big" => Ok(quote! { BigEndian }),
        _ => Err(Error::new_spanned(ident, "failed to determine endian")),
    }
}

fn parse_attr(attr: &Attribute) -> Result<TokenStream> {
    let list = match attr.parse_meta()? {
        Meta::List(list) => list,
        other => return Err(Error::new_spanned(other, "unsupported attribute")),
    };

    let ident = only_one(list.nested.iter())
        .and_then(|meta| match meta {
            NestedMeta::Meta(Meta::Path(path)) => path.get_ident(),
            _ => None,
        })
        .ok_or_else(|| Error::new_spanned(attr, "excepted endian"))?;

    Ok(determine_endian(ident)?)
}

pub fn find<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    only_one(attrs.iter().filter(|attr| attr.path.is_ident(name)))
}

pub fn parse(attrs: &[Attribute]) -> Result<Option<TokenStream>> {
    find(attrs, "endian").map(parse_attr).transpose()
}

pub fn get_endian(attrs: &[Attribute], default: TokenStream) -> Result<TokenStream> {
    match parse(attrs)? {
        Some(attr) if default.to_string() == attr.to_string() => Err(Error::new_spanned(
            attrs.first(),
            "this attribute does not make sense",
        )),
        Some(attr) => Ok(attr),
        None => Ok(default),
    }
}
