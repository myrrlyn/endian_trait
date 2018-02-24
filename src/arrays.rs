/*! Implement `Endian` on mutable slices and standard arrays.

I want this library to remain zero-allocation, so I can't return a new Vec that
has executed the conversion, and without type-level integers I can't accept
arbitrary slices.

This mutates a slice or array in place, replacing each element with its
converted form.
!*/

use super::Endian;
use std::ptr;

/// This is gonna get ... weird.
///
/// I really should just make Endian require Copy, and there's still time to do
/// that as I'm in the 0.* series still, but that's not STRICTLY SPEAKING
/// necessary. I haven't yet done anything truly stupid like impl Endian for *T,
/// since pointers should never ever leave their host context.
///
/// Anyway. Implementing Endian on slices of Endian items, where I did not
/// require that Endian: Copy + !Drop.
///
/// The Endian trait consumes its input and returns a new value for output; it
/// does not take references. However, we cannot move out of a borrowed slice.
/// The solution to this is, unfortunately, unsafe C-style code. We loop across
/// the slice, using ptr::read to bitwise-copy the value out of the slice and
/// into local context, where we then invoke its Endian implementation, and then
/// use ptr::write to bitwise-copy the new value back into the slice. The ptr
/// read and write methods do not invoke Drop on the values they move, so from
/// the type's perspective, nothing happens; this operation purely affects the
/// bitwise memory repr of the instance.
macro_rules! flip_collection {
	() => {
		#[allow(unused_mut)]
		fn from_be(mut self) -> Self {
			for el in self.iter_mut() { unsafe {
				ptr::write(el, ptr::read(el).from_be());
			} }
			self
		}
		#[allow(unused_mut)]
		fn from_le(mut self) -> Self {
			for el in self.iter_mut() { unsafe {
				ptr::write(el, ptr::read(el).from_le());
			} }
			self
		}
		#[allow(unused_mut)]
		fn to_be(mut self) -> Self {
			for el in self.iter_mut() { unsafe {
				ptr::write(el, ptr::read(el).to_be());
			} }
			self
		}
		#[allow(unused_mut)]
		fn to_le(mut self) -> Self {
			for el in self.iter_mut() { unsafe {
				ptr::write(el, ptr::read(el).to_le());
			} }
			self
		}
	};
}

//  Implement across any slice
impl<'a, T> Endian for &'a mut [T] where T: Endian {
	flip_collection!();
}

//  Implement on specific array length types
macro_rules! implendian_a {
	( $( $n:expr, )* ) => { $(
		impl<T> Endian for [T; $n] where T: Endian {
			flip_collection!();
		}
	)* };
}

implendian_a![
	0,
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
	17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
	33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
	49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
	65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
	81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
	97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
	113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
	129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144,
	145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
	161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
	177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
	193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208,
	209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224,
	225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240,
	241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256,
];

#[cfg(test)]
mod tests {
	use Endian;

	#[test]
	fn slices() {
		let src = [
			1, 2, 3, 4, 5, 6, 7, 8,
		];
		let flip = src.clone();
		let mut comp = src.clone();

		//  Flip one slice via the Endian trait method
		let rflip: &[i32] = &flip.to_be();
		//  Flip the other by looping over it and using the inherent method
		for e in comp.iter_mut() {
			*e = e.to_be();
		}

		assert_eq!(rflip, comp);
	}

	#[test]
	fn arrays() {
		let src = [
			1, 2, 3, 4, 5, 6, 7, 8,
		];
		let flip = src.clone().to_be();
		let mut comp = src.clone();
		for e in comp.iter_mut() {
			*e = e.to_be();
		}

		assert_eq!(flip, comp);
	}
}
