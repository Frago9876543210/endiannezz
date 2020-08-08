use syn::Attribute;

pub mod endian;

fn only_one<I: Iterator<Item = T>, T>(mut it: I) -> Option<T> {
    match (it.next(), it.next()) {
        (Some(first), None) => Some(first),
        _ => None,
    }
}

pub fn find<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    only_one(attrs.iter().filter(|attr| attr.path.is_ident(name)))
}
