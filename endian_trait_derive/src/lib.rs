extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{
	Body,
	DeriveInput,
	VariantData,
};

#[proc_macro_derive(Endian)]
pub fn endian_trait(source: TokenStream) -> TokenStream {
	let s = source.to_string();
	let ast = syn::parse_derive_input(&s).unwrap();
	let imp = impl_endian(&ast);
	imp.parse().unwrap()
}

fn impl_endian(ast: &DeriveInput) -> Tokens {
	let name = &ast.ident;
	match &ast.body {
		&Body::Enum(ref _variants) => {
			unimplemented!();
		},
		&Body::Struct(VariantData::Struct(ref fields)) => {
			//  I know that this is a named-field struct, so unwrap is fine.
			let names: Vec<_> = fields.iter().map(|f| match f.ident {
				Some(ref n) => n,
				None => panic!(),
			}).collect();
			let (nl, nr) = (&names, &names);
			quote! {
impl Endian for #name {
	fn from_be(self) -> Self { Self {
		#( #nl: Endian::from_be(self.#nr), )*
	} }
	fn from_le(self) -> Self { Self {
		#( #nl: Endian::from_le(self.#nr), )*
	} }
	fn to_be(self) -> Self { Self {
		#( #nl: Endian::to_be(self.#nr), )*
	} }
	fn to_le(self) -> Self { Self {
		#( #nl: Endian::to_le(self.#nr), )*
	} }
}
			}
		},
		&Body::Struct(VariantData::Tuple(ref _fields)) => {
			unimplemented!();
		},
		&Body::Struct(VariantData::Unit) => {
			panic!("You can't implement Endian conversions on a ZST");
		},
	}
}
