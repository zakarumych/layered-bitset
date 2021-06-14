# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Traits for bitsets: `BitSet` and `BitSetMut` which allows mutation.
- Implementation of the traits above for primitive types.
- Implementation of the traits for various indirection types.
- Implementation of the traits for an `Option` type wrapper.
- Implementation for a layered bitset using top layer bitset as a marker to quickly which elements in bottom array contain at least one bit. Layers can be stacked indefinitely.
- Bunch of type aliases for bitsets of various sizes.
