# Changelog

## 0.4.0

### Added

- re-export the `#[derive(Endian)]` macro from `endian_trait_derive`
- CI and tests
- This changelog
- `Endian` can now derive on discriminant-only (C-like) enums with integer
    representation. It cannot derive on `#[repr(C)]` enums or data-carrying
    enums.

### Changed

- pinned Rust version to 1.20.0

### Removed

- implementations of `Endian` for `isize` and `usize`.

## 0.3.0

### Added

- implement `Endian` on `i128`, `u128`, and `[T; N]` for arrays of zero through
    256

## 0.2.0

### Added

- implement `Endian` on `bool`, `char`, `f32`, `f64`

### Changed

- use stable Rust instead of nightly

## 0.1.1

### Added

- crates.io metadata in `Cargo.toml`

## 0.1.0

### Added

- `Endian` trait, implemented on the integer primitives
