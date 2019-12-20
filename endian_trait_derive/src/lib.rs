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

```rust,no-run
extern crate endian_trait;
# fn main() {}
```

and this crate will be pulled in transitively.

By itself, this crate provides a custom-derive macro to emit a trait impl block
that is syntactically valid but will fail to compile without the `endian_trait`
crate and `Endian` trait in scope.

```rust
extern crate endian_trait;
# // This is needed because of the fact that the test is executed from within
# // the context of the derive crate. The tests in the trait crate demonstrate
# // that the macro is correctly re-exported.
# # extern crate endian_trait_derive;

use endian_trait::Endian;

#[derive(Endian)]
struct Foo<A: Endian, B: Endian> {
    bar: A,
    baz: B,
}
# fn main() {}
```
!*/

extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;
extern crate quote;
extern crate syn;

use quote::{
	ToTokens,
	quote,
};

use std::iter;

use syn::{
	Attribute,
	Data,
	DataEnum,
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
	Path,
	Variant,
	spanned::Spanned,
};

/// Hook for receiving `#[derive(Endian)]` code
#[proc_macro_derive(Endian)]
pub fn derive(source: pm::TokenStream) -> pm::TokenStream {
	derive2(source.into()).unwrap_or_else(|err| err.to_compile_error()).into()
}

fn derive2(tokens: pm2::TokenStream) -> syn::Result<pm2::TokenStream> {
	let ast = syn::parse2::<DeriveInput>(tokens)?;
	//  Get the name of the typedef on which `Endian` is to be implemented.
	let name = &ast.ident;
	//  Get any generics from the typedef.
	let generics = &ast.generics;
	match ast.data {
		//  Attempt to derive for an integer-repr enum.
		Data::Enum(DataEnum { variants, .. }) => gen_enum(
			name,
			&ast.attrs,
			variants,
		),
		Data::Struct(DataStruct { fields, .. }) => match fields {
			//  Derive for a record struct
			Fields::Named(FieldsNamed { ref named, .. }) => gen_struct(
				name,
				generics,
				named.iter()
					.map(|f| f.ident.clone().ok_or_else(|| syn::Error::new(
						f.span(),
						"All fields in a struct must be named",
					)))
					.collect::<syn::Result<Vec<_>>>()?,
			),
			//  Derive for a tuple struct
			Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => gen_struct(
				name,
				generics,
				(0 .. unnamed.len()).map(|n| Index {
					index: n as u32,
					span: fields.span(),
				}),
			),
			//  Derive for a zero-sized struct
			Fields::Unit => gen_struct(name, generics, iter::empty::<Ident>()),
		},
		Data::Union(..) => return Err(syn::Error::new(
			name.span(),
			"Rust does not currently permit `#[derive(Trait)]` attributes on \
			`union` types",
		)),
	}
}

/// Generate the Endian impl for an enum with an integer repr and no data body.
fn gen_enum<'a>(
	name: &Ident,
	attrs: impl IntoIterator<Item = &'a Attribute>,
	variants: impl IntoIterator<Item = Variant>,
) -> syn::Result<pm2::TokenStream> {
	/* There is only one valid path through the syntax tree for working with
	enums. Rather than drift rightward at each layer, the processing code
	produces a `syn::Result` at each step, and bubbles errors.

	This could be replaced with a series of `.and_then` calls.

	First, find the attribute that is `#[repr]`. This must be present in order
	to make the enum `Endian`, as otherwise it has no fixed memory type.
	*/
	match match match attrs.into_iter().find(|a| a.path
		.segments
		.iter()
		.last()
		.expect("Attribute paths will always have a final segment")
		.ident == "repr"
	).ok_or_else(|| syn::Error::new(
		name.span(),
		"`#[derive(Endian)]` requires a `#[repr]` of `C` or some integer for \
		enums",
	))?
	//  Once a `#[repr]` attribute is discovered, parse it as an attribute and
	//  query its internals.
	.parse_meta()? {
		/* `#[repr(_)]` produces `Meta::List` as its arg type. Anything else is
		invalid. Strictly speaking, this check allows `#[repr(one, two)]`, but
		it is up to the compiler to reject that for us.

		Extract the interior nested item.
		*/
		Meta::List(MetaList { ref nested, .. }) => nested.into_iter()
			.next()
			.ok_or_else(|| syn::Error::new(
				nested.span(),
				"`#[repr(_)]` attributes must contain `C` or an an integer type",
			)),
		other => Err(syn::Error::new(
			other.span(),
			"`#[repr(_)]` attributes must only contain `C` or an integer type",
		)),
	}? {
		/* The inner item must be a `Path`-type object, which must have one
		component. Types other than the Rust fundamental integers are not
		supported.
		*/
		NestedMeta::Meta(Meta::Path(Path { ref segments, .. })) => segments
			.into_iter()
			.last()
			.map(|ps| &ps.ident)
			.ok_or_else(|| syn::Error::new(
				segments.span(),
				"`#[repr()]` attributes must contain a representation type",
			)),
		other => Err(syn::Error::new(
			other.span(),
			"Invalid contents of `#[repr()]`",
		)),
	}? {
		//  Test if the representation is one of the named symbols, which are
		//  not supported.
		kind if kind == "C" || kind == "packed" => Err(syn::Error::new(
			kind.span(),
			"`#[repr(C)]` and `#[repr(packed)]` enums cannot currently \
			implement `Endian`",
		)),
		//  Test if the representation is a Rust fundamental integer.
		kind if kind == "i8" || kind == "i16" || kind == "i32" || kind == "i64"
		|| kind == "u8" || kind == "u16" || kind == "u32" || kind == "u64"
		|| kind == "i128" || kind == "u128" => {
			//  Check that each variant of the enum has no data.
			for var in variants {
				if let Fields::Unit = var.fields {}
				else {
					return Err(syn::Error::new(
						var.fields.span(),
						"`Endian` cannot be derived on enums with data fields",
					));
				}
			}
			/* To implement `Endian`, each function casts a pointer to `Self`
			into a pointer to the `repr` type, then runs the `Endian` function
			on the pointed-to value, and stores the transformed value back in
			the `self` slot. This introduces undefined behavior, in that the
			enum now contains a value not enumerated in the symbol list;
			however, in practice, this appears to not matter as long as the enum
			value is not used as its own type until transformed back into a
			valid variant.

			The expression `enum_value.to_be().from_be()` operates correctly as
			of 1.31.
			*/
			Ok(quote! {
				impl Endian for #name {
					fn from_be(mut self) -> Self {
						let ptr = &mut self as *mut Self as *mut #kind;
						unsafe { ptr.write(Endian::from_be(ptr.read())) }
						self
					}
					fn from_le(mut self) -> Self {
						let ptr = &mut self as *mut Self as *mut #kind;
						unsafe { ptr.write(Endian::from_le(ptr.read())) }
						self
					}
					fn to_be(mut self) -> Self {
						let ptr = &mut self as *mut Self as *mut #kind;
						unsafe { ptr.write(Endian::to_be(ptr.read())) }
						self
					}
					fn to_le(mut self) -> Self {
						let ptr = &mut self as *mut Self as *mut #kind;
						unsafe { ptr.write(Endian::to_le(ptr.read())) }
						self
					}
				}
			})
		},
		kind => Err(syn::Error::new(
			kind.span(),
			"`#[repr(T)]` must have `T` be one of the Rust fundamental integer \
			types",
		))
	}
}

/// Generate the Endian impl for a struct type.
fn gen_struct(
	name: &Ident,
	generics: &Generics,
	fields: impl IntoIterator<Item = impl Clone + ToTokens>,
) -> syn::Result<pm2::TokenStream> {
	/* Due to how `quote!` handles iterators, the sequence of field names must
	be collected into a `Vec` and then cloned, so that each name can be used
	twice in the `quote!` body. Otherwise, `quote!` would advance the iterator
	twice per expansion, which is very incorrect.
	*/
	let l = fields.into_iter().collect::<Vec<_>>();
	let r = l.clone();
	//  Generics must be split into appropriate forms for the impl block.
	let (g_impl, g_ty, g_where) = generics.split_for_impl();
	/* Structs are recursively Endian: the conversion is just a conversion of
	each member, as the tree eventually ends in leaves of Rust fundamentals with
	provided implementations.

	RFC #1506 introduced a universal struct syntax to unify the three struct
	types: all structs can be described with `Name { fields… }` syntax. Record
	structs are unchanged, tuple structs are `Name { 0: val, …}`, and zero-size
	structs can be written as `Name {}` (empty records).
	*/
	Ok(quote! {
		impl #g_impl Endian for #name #g_ty #g_where {
			fn from_be(self) -> Self {
				Self {
					#( #l: Endian::from_be(self.#r), )*
				}
			}
			fn from_le(self) -> Self {
				Self {
					#( #l: Endian::from_le(self.#r), )*
				}
			}
			fn to_be(self) -> Self {
				Self {
					#( #l: Endian::to_be(self.#r), )*
				}
			}
			fn to_le(self) -> Self {
				Self {
					#( #l: Endian::to_le(self.#r), )*
				}
			}
		}
	})
}
