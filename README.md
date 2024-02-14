[crates.io](https://crates.io/crates/packbytes) | [docs](https://docs.rs/packbytes)

Convert structures to and from packed representavises - byte arrays of fixed size that live on stack.

You can use them to read the structures from a `std::io::Reader` or write them to a `std::io::Writer`
with a single `read` or `write` call.

# Motivation
When reading structured data into a Rust struct (such as a header from a file), one might be tempted
to simply do the following:
```rust
use std::io;

struct MyStruct {
    version: u8,
    kind: u16,
    matrix: [i32; 16]
}

// Do not do this
fn read_my_struct<R: io::Read>(reader: &mut R) -> io::Result<MyStruct> {
    let mut bytes = [0; std::mem::size_of::<MyStruct>()];
    reader.read_exact(&mut bytes)?;
    Ok(unsafe { std::mem::transmute(bytes) })
}
```
The `unsafe` is not OK!
The memory alignment of the Rust struct typically contains some padding after fields,
whereas data in the file is tightly packed, one piece after another. Therefore, it is safe
only if the struct is [`repr(packed)`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprpacked),
which appart from being suboptimal for performance can [cause undefined behaviour](https://github.com/rust-lang/rust/issues/27060)
on some platforms. Moreover, [endianness](https://en.wikipedia.org/wiki/Endianness) of the data
must match the endianness of the platform.

This crate essentially allows doing the preceding, but safely (no `unsafe` used!), by converting structs
to and from packed byte representatives. This is done via the `ToBytes` and `FromBytes` traits,
which can be automatically derived for most structs. Compare:
```rust
use std::io;
use packbytes::{FromBytes, ByteArray};

#[derive(FromBytes)]
struct MyStruct {
    version: u8,
    kind: u16,
    matrix: [i32; 16]
}

fn read_my_struct<R: io::Read>(reader: &mut R) -> io::Result<MyStruct> {
    let mut bytes = [0; <MyStruct as FromBytes>::Bytes::SIZE];
    reader.read_exact(&mut bytes)?;
    Ok(MyStruct::from_bytes(bytes))
}
```
The `from_bytes` and `to_bytes` methods can be considered as unpacking and packing the structure
to and from its native in memory representation.

For convenience, to read and write like this, the methods `read_packed` and `write_packed`
are provided.

When not every sequence of bytes represents valid data (such as when a field can attain
just a small set of values), the trait `TryFromBytes` may be used.

# Endianness
By default, the `FromBytes` and `ToBytes` derive macros assume that the data is prefered to be stored
in the little endian order, and this is what is used by the `from_bytes` and `to_bytes` methods.
You can change this by setting the attribute `#[packbytes(be)]` for big endian or `#[packbytes(ne)]`
for the platform native endian.

# `no_std` support
Appart from the convenience methods, everything in this crate does not require `std`. The `std` feature
can be turned off. In fact, as everything happens on the stack, not even `alloc` is required.
