# Changelog

List of notable changes to the custom-derive macro for the `Endian` trait.

## 0.3.2

### Added

- This changelog.

### Changed

- Made the tests pass correctly

    The `Endian` trait is in the parent crate, but the macro that derives it is
    in this crate, which makes the test libraries very confused when they try to
    pull in both from this crate, which is a dependency of the parent and thus
    cannot depend on the parent, except when building the *test executable*
    which is a standalone crate that pulls in both of them.

## 0.3.1

### Changed

- Increased `syn` and `quote` versions from `0.11` and `0.3` to `0.12` and
    `0.4`, respectively

## 0.3.0

### Changed

- Refactored the code generator to reduce duplication
- Minimum compiler version is now 1.20

    1.19 allows unit structs to be declared as `Type { }` and tuple structs as
    `Type { 0: val, ... }`

    1.20 allows the `unimplemented!` macro to take a descriptive string.

## 0.2.0

### Added

- Implement the derive function for tuple structs and for unit structs.
- Tests

### Changed

- Use stable Rust instead of nightly

## 0.1.1

### Added

- crates.io metadata in `Cargo.toml`

## 0.1.0

### Added

- Initial implementation, capable of deriving only on standard, named-field,
    structs.
