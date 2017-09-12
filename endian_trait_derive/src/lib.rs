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

/// Hook for receiving `#[derive(Endian)]` code
#[proc_macro_derive(Endian)]
pub fn endian_trait(source: TokenStream) -> TokenStream {
	let s = source.to_string();
	//  TODO: Handle parse failure
	let ast = syn::parse_derive_input(&s).unwrap();
	let imp = impl_endian(&ast);
	//  TODO: Handle reparse failure
	imp.parse().unwrap()
}

/// Code generator for Endian implementations
fn impl_endian(ast: &DeriveInput) -> Tokens {
	let name = &ast.ident;
	match ast.body {
		Body::Enum(ref _variants) => {
			unimplemented!(r#"Enum are not integral types.

If enums are present in a type that will be serialized in a way to require
Endian transforms, then the enums in question must implement a conversion to and
from an appropriate integral type, which will then perform the Endian actions."#);
		},
		//  Normal struct with named fields
		Body::Struct(VariantData::Struct(ref fields)) => {
			//  Collect all the names. This should be infallible, hence the
			//  unreachable! in the None branch.
			let names: Vec<_> = fields.iter().map(|f| match f.ident {
				Some(ref n) => n,
				None => unreachable!("Struct fields MUST have idents"),
			}).collect();
			impl_on_idents(&name, names.as_slice())
		},
		Body::Struct(VariantData::Tuple(ref fields)) => {
			//  Tuples don't have field names. For each field, get its index,
			//  and convert from the raw number into an Ident.
			//  This is necessary as usize quotes as VALusize, not VAL.
			let nums: Vec<Ident> = fields.iter()
				.enumerate().map(|(n, _)| n.into()).collect();
			let numr: Vec<&Ident> = nums.iter().collect();
			impl_on_idents(name, numr.as_slice())
		},
		Body::Struct(VariantData::Unit) => {
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

fn impl_on_idents(n: &Ident, i: &[&Ident]) -> Tokens {
	let (l, r) = (i, i);
	quote! {
impl Endian for #n {
	fn from_be(self) -> Self { Self {
		#( #l: Endian::from_be(self.#r), )*
	} }
	fn from_le(self) -> Self { Self {
		#( #l: Endian::from_le(self.#r), )*
	} }
	fn to_be(self) -> Self { Self {
		#( #l: Endian::to_be(self.#r), )*
	} }
	fn to_le(self) -> Self { Self {
		#( #l: Endian::to_le(self.#r), )*
	} }
}
	}
}
