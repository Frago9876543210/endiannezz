use proc_macro2::{Literal, TokenStream};
use syn::{Data, DeriveInput, Error, Result};

use quote::quote;

use crate::{attr, fields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let name = &input.ident;

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let default = attr::parse(&input.attrs)?
		.ok_or(Error::new_spanned(&input, "please specify default endian"))?;

	let imports = quote! {
		#[allow(unused_imports)]
		use ::endiannezz::internal::{HackedIo, HackedPrimitive};
	};

	let (write, read) = match &input.data {
		Data::Struct(data) => (
			fields::write(
				&data.fields,
				|ident| quote! { self.#ident },
				|i, _| {
					let i = Literal::usize_unsuffixed(i);
					quote!(self.#i)
				},
				&default,
			)?,
			fields::read(&data.fields, &default)?,
		),
		_ => return Err(Error::new_spanned(
			input,
			"CanIo can be derived only for structures",
		))
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
				Ok(Self #read)
			}
		}
	})
}
