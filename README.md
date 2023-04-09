# Archway
[![License](https://img.shields.io/crates/l/archway?style=flat-square)][LICENSE]
[![Crates.io](https://img.shields.io/crates/v/archway?style=flat-square)][CratesIO]
[![docs.rs](https://img.shields.io/docsrs/archway?style=flat-square)][DocsRS]

Rust traits for `Rc` and `Arc` interoperation.

## Requirements
- There are no external dependencies for this crate.
- This crate is `no-std`, but requires `alloc`. This should not
  be a problem. If you are writing allocation-free code, how could
  you make use of shared pointers?

[CratesIO]: https://crates.io/crates/archway
[DocsRS]: https://docs.rs/archway/latest/archway/
[LICENSE]: https://github.com/zfzackfrost/archway/blob/main/LICENSE
