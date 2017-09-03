/*! Custom Derive for Endian Trait

This crate provides a custom-derive procedural macro for the Endian trait. This
macro simply takes the component fields of the type on which it is annotated and
attempts to call the Endian conversion on each field. This means that Endian can
only be derived on types composed of types that are all themselves Endian.
Attempting to derive Endian on a type with a non-Endian type in it will result
in a compilation error.

# Use Case

Network serialization of complex types. That's pretty much it. This trait does
not require explicitly, but in practice basically does need, that the types on
which it is implemented are `repr(C)` or `repr(packed)`. You can get away with
using it on types that do not have that representation if you can guarantee that
you know how to properly handle the actual byte representation and the generator
and consumer are the same Rust version.

# Usage

Import the crate and its macros. You can't make use of this crate without the
`endian_trait` crate, and that's where fuller documentation can be found. You
can then add a `#[derive(Endian)]` marker on structs (regular, tuple, or void).
Enums are not yet supported. All elements of the struct must themselves be
Endian.

```rust,ignore
extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

#[derive(Endian)]
struct Foo<A: Endian, B: Endian> {
    bar: A,
    baz: B,
}
```
!*/

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{
	Body,
	DeriveInput,
	Ident,
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
			unimplemented!("I have not yet learned enough `quote` to do this");
		},
		&Body::Struct(VariantData::Struct(ref fields)) => {
			//  I know that this is a named-field struct, so unwrap is fine.
			let names: Vec<_> = fields.iter().map(|f| match f.ident {
				Some(ref n) => n,
				None => unreachable!("Struct fields MUST have idents"),
			}).collect();
			//  This is necessary to convince the macro system that we can bind
			//  on the names twice in the loop without causing issues.
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
		&Body::Struct(VariantData::Tuple(ref fields)) => {
			//  Tuples don't have field names. For each field, get its index,
			//  and convert from the raw number into an Ident.
			//  This is necessary as usize serializes as VALusize, not VAL.
			let nums: Vec<Ident> = fields.iter()
				.enumerate().map(|(n, _)| n.into()).collect();
			let nums: &Vec<_> = &nums;
			quote! {
impl Endian for #name {
	fn from_be(self) -> Self { #name(
		#( Endian::from_be(self.#nums), )*
	)}
	fn from_le(self) -> Self { #name(
		#( Endian::from_le(self.#nums), )*
	)}
	fn to_be(self) -> Self { #name(
		#( Endian::to_be(self.#nums), )*
	)}
	fn to_le(self) -> Self { #name(
		#( Endian::to_le(self.#nums), )*
	)}
}
			}
		},
		&Body::Struct(VariantData::Unit) => {
			quote! {
impl Endian for #name {
	fn from_be(self) -> Self { self }
	fn from_le(self) -> Self { self }
	fn to_be(self) -> Self { self }
	fn to_le(self) -> Self { self }
}
			}
		},
	}
}
