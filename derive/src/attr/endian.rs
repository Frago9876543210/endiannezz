use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Error, Meta, NestedMeta, Result};

use quote::quote;

use crate::attr;

fn determine_endian(ident: &Ident) -> Result<TokenStream> {
    match ident.to_string().as_str() {
        "_" | "ne" | "native" => Ok(quote!(NativeEndian)),
        "le" | "little" => Ok(quote!(LittleEndian)),
        "be" | "big" => Ok(quote!(BigEndian)),
        _ => Err(Error::new_spanned(ident, "failed to determine endian")),
    }
}

fn parse_endian_attr(attr: &Attribute) -> Result<TokenStream> {
    let list = match attr.parse_meta()? {
        Meta::List(list) => list,
        other => return Err(Error::new_spanned(other, "expected attribute arguments")),
    };

    let ident = attr::only_one(list.nested.iter())
        .and_then(|meta| match meta {
            NestedMeta::Meta(Meta::Path(path)) => path.get_ident(),
            _ => None,
        })
        .ok_or_else(|| Error::new_spanned(attr, "excepted endian identifier"))?;

    Ok(determine_endian(ident)?)
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
