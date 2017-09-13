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
	Generics,
	Ident,
	VariantData,
};

/// Hook for receiving `#[derive(Endian)]` code
#[proc_macro_derive(Endian)]
pub fn endian_trait(source: TokenStream) -> TokenStream {
	let s = source.to_string();
	//  A parse failure means that the input was invalid Rust. This is not our
	//  problem, so a panic is permissible.
	let ast = syn::parse_derive_input(&s).unwrap();
	//  Take the parsed AST and pass it into the actual worker function.
	let imp = impl_endian(ast);
	//  The worker function will only ever generate valid Rust code, so this can
	//  also be glibly unwrapped.
	imp.parse().unwrap()
}

/// Code generator for Endian implementations
fn impl_endian(ast: DeriveInput) -> Tokens {
	//  Get the name of the struct on which Endian is to be implemented
	let name: &Ident = &ast.ident;
	//  Get any generics from the struct
	let generics: &Generics = &ast.generics;
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
			codegen(name, generics, names.as_slice())
		},
		//  Tuple struct with unnamed fields
		Body::Struct(VariantData::Tuple(ref fields)) => {
			//  Tuples don't have field names. For each field, get its index,
			//  and convert from the raw number into an Ident.
			//  This is necessary as usize quotes as VALusize, not VAL.
			let nums: Vec<Ident> = (0 .. fields.len()).map(|n| n.into()).collect();
			//  Get a collection of references to each Ident for the implementor
			let numr: Vec<&Ident> = nums.iter().collect();
			//  Tuple structs are able to use Name { 0: val, ... } since 1.19
			//  See RFC #1506
			codegen(name, generics, numr.as_slice())
		},
		//  Unit struct: no fields
		//  This is simple: All Endian functions are just the identity function.
		//  Note that Unit is a special subset of ZST: Unit has no fields at all
		//  while ZST can be any tuple or struct such that all its fields are
		//  zero sized. Unit is specifically `()` or `struct Name;` or
		//  `struct Name();`
		//  All ZST structs or tuples have fields which eventually bottom out
		//  here; their construction can be handled by the Struct and Enum arms.
		Body::Struct(VariantData::Unit) => {
			//  Also in RFC #1506 (implemented 1.19), unit structs can be
			//  declared as Name{}. Coincidentally, Name{} is also what will be
			//  emitted by the codegen function when given an empty list of
			//  fields.
			//  Call codegen on the name and generics (which for Unit will be
			//  empty) and an empty array.
			codegen(name, generics, &[])
		},
	}
}

/// Generates Rust code to impl Endian on the given struct declaration.
///
/// This requires the struct's name, its generic information (if any), and its
/// fields (if any).
///
/// Thanks to RFC #1506 (stabilized in 1.19), this can be a single code path for
/// all three types of struct definition: standard, tuple, and unit.
fn codegen(name: &Ident, generics: &Generics, fields: &[&Ident]) -> Tokens {
	//  We need left and right access to each name so that the stepper can draw
	//  from each once, rather than advancing the name iterator twice per step.
	//  Without this, the stepper would have incorrect behavior consuming the
	//  old field and inserting it into the new.
	let (l, r) = (fields, fields);
	//  Split the type's generics into appropriate forms for the impl block.
	let (g_impl, g_ty, g_where) = generics.split_for_impl();
	//  Build Rust code that is the impl block and each function, consuming self
	//  and building a new Self instance that is the result of applying the
	//  Endian conversion to each field.
	quote! {
		impl #g_impl Endian for #name #g_ty #g_where {
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
