[![Crates.io](https://img.shields.io/crates/v/sphalerite)](https://crates.io/crates/sphalerite)
[![docs.rs](https://img.shields.io/docsrs/sphalerite)](https://docs.rs/sphalerite)
[![Crates.io](https://img.shields.io/crates/l/sphalerite)](https://crates.io/crates/sphalerite)
# Sphalerite
A dependency-less crate for simple binary serialization and deserialization in rust.

## Usage

To use the crate simply install it using cargo
(`cargo add sphalerite`)
and implement the Transcode trait.

The `Transcode` trait implements following functions:
```rust
fn to_bytes(&self,_writer: &mut dyn std::io::Write) -> std::io::Result<usize>
fn from_bytes(&self,_reader: &mut dyn std::io::Read) -> std::io::Result<Self>
```
To serialize a variable call `to_bytes` and for deserialization call `from_bytes`