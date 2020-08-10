use proc_macro2::{Ident, Literal, TokenStream};
use syn::{Data, DeriveInput, Error, Result};

use quote::quote;

use crate::{attr, fields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let default = attr::endian::parse(&input.attrs)?
        .ok_or_else(|| Error::new_spanned(&input, "please specify default endian"))?;

    let imports = quote! {
        #[allow(unused_imports)]
        use ::endiannezz::internal::{HackedIo, HackedPrimitive};
    };

    let (write, read) = match &input.data {
        Data::Struct(data) => {
            let write = fields::write(
                &data.fields,
                |ident| quote!(self.#ident),
                |i| {
                    let i = Literal::usize_unsuffixed(i);
                    quote!(self.#i)
                },
                &default,
            )?;
            let read = fields::read(&data.fields, &default)?;

            (write, quote!(Self #read))
        }
        Data::Enum(data) => {
            let repr_attr = attr::find(&input.attrs, "repr")
                .ok_or_else(|| Error::new_spanned(&input, "Enums must declare #[repr]"))?;
            let repr_ty = repr_attr.parse_args::<Ident>()?;

            if !repr_ty.to_string().starts_with(|c| matches!(c, 'u' | 'i')) {
                return Err(Error::new_spanned(&input, "Unsupported repr type"));
            }

            let capacity = data.variants.len();

            let (mut write_vars, mut read_vars) =
                (Vec::with_capacity(capacity), Vec::with_capacity(capacity));

            let (repr_write, repr_read) = (
                quote!(::endiannezz::#default::write::<#repr_ty, _>),
                quote!(::endiannezz::#default::read::<#repr_ty, _>),
            );

            for variant in &data.variants {
                let variant_name = &variant.ident;

                let (_, discriminant) = variant.discriminant.as_ref().ok_or_else(|| {
                    Error::new_spanned(
                        variant,
                        "All enum variants must have explicit discriminants",
                    )
                })?;

                let fields_patterns = fields::make_patterns(&variant.fields);
                let fields_write = fields::write(
                    &variant.fields,
                    |ident| quote!(#ident),
                    |i| {
                        let ident = fields::generate_pattern(i);
                        quote!(#ident)
                    },
                    &default,
                )?;
                let fields_read = fields::read(&variant.fields, &default)?;

                write_vars.push(quote!(Self::#variant_name #fields_patterns => {
                    use ::endiannezz::Endian;

                    #repr_write(#discriminant, &mut w)?;
                    #fields_write
                }));
                read_vars.push(quote!(#discriminant => Self::#variant_name #fields_read));
            }

            let write = quote! {
                match self {
                    #(#write_vars),*
                }
            };
            let read = quote! {{
                use ::endiannezz::Endian;

                match #repr_read(r)? {
                    #(#read_vars,)*
                    _ => Err(::std::io::Error::from(::std::io::ErrorKind::InvalidData))?,
                }
            }};

            (write, read)
        }
        _ => {
            return Err(Error::new_spanned(
                input,
                "Io can be derived only for structures and enums (in nightly version)",
            ));
        }
    };

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::endiannezz::Io for #name #ty_generics #where_clause {
            fn write<W: ::std::io::Write>(&self, mut w: W) -> ::std::io::Result<()> {
                #imports
                #write
                Ok(())
            }

            fn read<R: ::std::io::Read>(mut r: R) -> ::std::io::Result<Self> {
                #imports
                Ok(#read)
            }
        }
    })
}
