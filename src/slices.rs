/*! Implement `Endian` on mutable slices.
!*/

use super::Endian;

/// Traverse a slice, performing the `Endian` method on each item in place.
impl<'a, T: Endian> Endian for &'a mut [T] {
	fn from_be(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			let ptr = elt as *mut T;
			ptr.write(ptr.read().from_be());
		} }
		self
	}
	fn from_le(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			let ptr = elt as *mut T;
			ptr.write(ptr.read().from_le());
		} }
		self
	}
	fn to_be(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			let ptr = elt as *mut T;
			ptr.write(ptr.read().to_be());
		} }
		self
	}
	fn to_le(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			let ptr = elt as *mut T;
			ptr.write(ptr.read().to_le());
		} }
		self
	}
}
