use crate::attr::endian;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Fields, Result, Type};

pub fn write<Named, Unnamed>(
    fields: &Fields,
    access_named: Named,
    access_unnamed: Unnamed,
    default_endian: &Ident,
) -> Result<TokenStream>
where
    Named: Fn(Option<&Ident>) -> TokenStream,
    Unnamed: Fn(usize) -> TokenStream,
{
    let mut derived = Vec::new();

    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let accessor = access_named(field.ident.as_ref());
                let attribute = endian::parse(&field.attrs)?;
                let endian =
                    endian::choice(field.attrs.first(), attribute.as_ref(), default_endian)?;

                derived.push(write_field(&accessor, &endian));
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let accessor = access_unnamed(i);
                let attribute = endian::parse(&field.attrs)?;
                let endian =
                    endian::choice(field.attrs.first(), attribute.as_ref(), default_endian)?;

                derived.push(write_field(&accessor, &endian));
            }
        }
        Fields::Unit => {}
    }

    Ok(quote!(#(#derived)*))
}

fn write_field(name: &TokenStream, endian: &Ident) -> TokenStream {
    quote! {
        #name.write_hacked::<::endiannezz::#endian, _>(&mut w)?;
    }
}

pub fn read(fields: &Fields, default_endian: &Ident) -> Result<TokenStream> {
    let mut derived = Vec::new();

    Ok(match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let ident = field.ident.as_ref();
                let attribute = endian::parse(&field.attrs)?;
                let endian =
                    endian::choice(field.attrs.first(), attribute.as_ref(), default_endian)?;

                let read = read_field(&field.ty, &endian);

                derived.push(quote!(#ident: #read));
            }
            quote!({ #(#derived),* })
        }
        Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                let attribute = endian::parse(&field.attrs)?;
                let endian =
                    endian::choice(field.attrs.first(), attribute.as_ref(), default_endian)?;

                derived.push(read_field(&field.ty, &endian));
            }
            quote!(( #(#derived),* ))
        }
        Fields::Unit => quote!(),
    })
}

fn read_field(ty: &Type, endian: &Ident) -> TokenStream {
    quote! {
        #ty::read_hacked::<::endiannezz::#endian, _>(&mut r)?
    }
}

pub fn generate_pattern(i: usize) -> Ident {
    format_ident!("variant_{}", i)
}

pub fn make_patterns(fields: &Fields) -> TokenStream {
    match fields {
        #[rustfmt::skip]
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .filter_map(|field| field.ident.as_ref());
            quote!({ #(#fields),* })
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| generate_pattern(i));
            quote!(( #(#fields),* ))
        }
        Fields::Unit => quote!(),
    }
}
