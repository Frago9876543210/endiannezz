use crate::attr;
use proc_macro2::{Ident, Span};
use syn::{Attribute, Error, Meta, NestedMeta, Result};

macro_rules! ident {
    ($t:tt) => {
        Ident::new(stringify!($t), Span::call_site())
    };
}

fn determine_endian(ident: &Ident) -> Result<Ident> {
    match ident.to_string().as_str() {
        "_" | "ne" | "native" => Ok(ident!(NativeEndian)),
        "le" | "little" => Ok(ident!(LittleEndian)),
        "be" | "big" => Ok(ident!(BigEndian)),
        _ => Err(Error::new_spanned(ident, "failed to determine endian")),
    }
}

fn parse_endian_attr(attr: &Attribute) -> Result<Ident> {
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

pub fn parse(attrs: &[Attribute]) -> Result<Option<Ident>> {
    attr::find(attrs, "endian")
        .map(parse_endian_attr)
        .transpose()
}

pub fn choice<'a>(
    first: Option<&Attribute>,
    attribute: Option<&'a Ident>,
    default: &'a Ident,
) -> Result<&'a Ident> {
    match attribute {
        Some(attr) if attr == default => Err(Error::new_spanned(
            first,
            "this attribute does not make sense",
        )),
        Some(attr) => Ok(attr),
        None => Ok(default),
    }
}
