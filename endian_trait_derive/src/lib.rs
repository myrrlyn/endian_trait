/*! Custom Derive for the Endian Trait

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

This crate shouldn't be used directly; use

```rust,ignore
#[macro_use]
extern crate endian_trait;
# fn main() {}
```

and this crate will be pulled in transitively.

By itself, this crate provides a custom-derive macro to emit a trait impl block
that is syntactically valid but will fail to compile without the `endian_trait`
crate and `Endian` trait in scope.

```rust,ignore
#[macro_use]
extern crate endian_trait;
# // This is needed because of the fact that the test is executed from within
# // the context of the derive crate. The tests in the trait crate demonstrate
# // that the macro is correctly re-exported.
# #[macro_use]
# extern crate endian_trait_derive;

use endian_trait::Endian;

#[derive(Endian)]
struct Foo<A: Endian, B: Endian> {
    bar: A,
    baz: B,
}
# fn main() {}
```
!*/

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{
	Tokens,
	ToTokens,
};
use syn::{
	Attribute,
	Data,
	DataStruct,
	DeriveInput,
	Fields,
	FieldsNamed,
	FieldsUnnamed,
	Generics,
	Ident,
	Index,
	Meta,
	MetaList,
	NestedMeta,
};

/// Hook for receiving `#[derive(Endian)]` code
#[proc_macro_derive(Endian)]
pub fn endian_trait(source: TokenStream) -> TokenStream {
	//  A parse failure means that the input was invalid Rust. This is not our
	//  problem, so a panic is permissible.
	let ast = syn::parse(source).unwrap();
	//  Take the parsed AST and pass it into the actual worker function, then
	//  convert the results back into Rust tokens.
	impl_endian(ast).into()
}

/// Code generator for Endian implementations
fn impl_endian(ast: DeriveInput) -> Tokens {
	//  Get the name of the struct on which Endian is to be implemented
	let name: &Ident = &ast.ident;
	//  Get any generics from the struct
	let generics: &Generics = &ast.generics;
	match ast.data {
		//  Attempt to derive Endian for an integer-repr enum
		Data::Enum(_) => codegen_enum(name, &ast.attrs),
		Data::Struct(DataStruct { ref fields, .. }) =>  match *fields {
			//  Normal struct: named fields
			Fields::Named(FieldsNamed { ref named, .. }) => {
				//  Collect references to all the field names. A precondition of
				//  reaching this code path is that all fields HAVE names, so it
				//  is safe to have an unreachable trap in the None condition.
				let names: Vec<&Ident> = named.iter().map(|f| match f.ident {
					Some(ref n) => n,
					None => unreachable!("All fields in a struct must be named"),
				}).collect();
				codegen_struct(name, generics, &names)
			},
			//  Tuple struct: unnamed fields
			Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
				let nums: Vec<Index> = (0 .. unnamed.len()).map(|n| n.into())
					.collect();
				codegen_struct(name, generics, &nums)
			},
			//  Unit struct: no fields
			//  This is simple: all Endian functions are just the identity
			//  function. Note that Unit structs are a subset of zero-sized
			//  types: specifically, unit has zero fields, while ZST is the set
			//  of all structs with zero or more fields which are all of zero
			//  size. Unit structs are specifically:
			//
			//  - ()
			//  - struct Foo;
			//  - struct Foo();
			//  - struct Foo{};
			//
			//  All ZST types eventually bottom out here; the interim can be
			//  handled by the other branches.
			//
			//  RFC #1506 (stabilized in 1.19) permits unit structs to be
			//  declared as `Foo {}`. This allows for a deduplication of the
			//  code generation path: the codegen function emits
			//
			//  fn func(self) -> Self { Self { /* fields */ } }
			//
			//  which, when `fields` is empty, leaves `Self { }` as the output
			//  token. Since this is legal, there is no special case for
			//  handling empty field sets.
			Fields::Unit => codegen_struct::<Index>(name, generics, &[]),
		},
		Data::Union(_) => {
			unimplemented!("Proc-macro derives are not yet allowed on unions");
		},
	}
}

/// Generate the Endian impl for an enum with an integer repr and no data body.
fn codegen_enum(name: &Ident, attrs: &[Attribute]) -> Tokens {
	//  Find the attr that is #[repr(_)]. We need to build one for comparison.
	let repr_path: syn::Path = Ident::from("repr").into();
	//  Seek for a #[repr(_)] attribute
	let repr: &Meta = &attrs.iter().find(|ref a| &a.path == &repr_path)
	//  Unwrap, and panic if this returned None instead of Some, because repr is
	//  the bare minimum of required attributes
	.expect("Endian can only be derived on enums with #[repr()] attributes")
	//  Take a reference to the actual value of the attribute, which is
	//  "repr(_)" from the source.
	.interpret_meta()
	.expect("#[repr(_)] cannot fail to be interpreted");
	//  Now figure out what the repr *is*. The format is #[repr(Name)] where
	//  Name is one of {i,u}{8,16,32,64}. This comes out to be a
	//  List(MetaList(Vec<Meta>, ..)) in syn structures. We want the one-element
	//  Vec's one element. Anything else is broken.
	let kind: &Ident = match *repr {
		Meta::List(MetaList { ref nested, .. }) => {
			if nested.len() != 1 {
				panic!("The #[repr()] attribute must be a single primitive integer type");
			}
			match nested[0] {
				NestedMeta::Meta(Meta::Word(ref ty)) => ty,
				_ => panic!("The #[repr()] interior must be a primitive integer type"),
			}
		},
		_ => unreachable!("The #[repr()] interior must be a primitive integer type"),
	};
	if kind == &Ident::from("C") {
		panic!("#[repr(C)] enums cannot implement Endian");
	}
	else if kind == &Ident::from("packed") {
		panic!("#[repr(packed)] enums cannot implement Endian");
	}
	quote! {
		impl Endian for #name {
			fn from_be(self) -> Self { unsafe {
				use std::mem::transmute;
				let raw: #kind = transmute(self);
				transmute(raw.from_be())
			} }
			fn from_le(self) -> Self { unsafe {
				use std::mem::transmute;
				let raw: #kind = transmute(self);
				transmute(raw.from_le())
			} }
			fn to_be(self) -> Self { unsafe {
				use std::mem::transmute;
				let raw: #kind = transmute(self);
				transmute(raw.to_be())
			} }
			fn to_le(self) -> Self { unsafe {
				use std::mem::transmute;
				let raw: #kind = transmute(self);
				transmute(raw.to_le())
			} }
		}
	}
}

/// Generates Rust code to impl Endian on the given struct declaration.
///
/// This requires the struct's name, its generic information (if any), and its
/// fields (if any).
///
/// Thanks to RFC #1506 (stabilized in 1.19), this can be a single code path for
/// all three types of struct definition: standard, tuple, and unit.
fn codegen_struct<T: ToTokens>(name: &Ident, generics: &Generics, fields: &[T]) -> Tokens {
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
