extern crate packbytes;
use core::convert::Infallible;
use packbytes::*;

#[derive(Debug, FromBytes, ToBytes, Eq, PartialEq)]
#[packbytes(be)]
struct Test {
    foo: u32,
    bar: u16,
}

#[derive(Debug, FromBytes, ToBytes, Eq, PartialEq)]
struct Nameless(u16, i32);

#[derive(Debug, FromBytes, ToBytes)]
struct Unit;

#[derive(Debug, Eq, PartialEq, ToBytes, TryFromBytes)]
#[packbytes_error(CustomError)]
#[packbytes_error_exp(CustomError::WrongTestEnum)]
enum TestEnum {
    Foo = 0x2,
    Bar = 0x12,
}

#[derive(Debug, Eq, PartialEq, ToBytes, TryFromBytes)]
#[repr(u32)]
enum FatTestEnum {
    Foo = 0x2,
    Bar = 0x12,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum CustomError {
    WrongTestEnum,
    SomethingElse,
}

impl From<Infallible> for CustomError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Debug, TryFromBytes, ToBytes, Eq, PartialEq)]
#[packbytes_error(CustomError)]
struct ComplexStruct {
    foo: u32,
    e: TestEnum,
}

#[derive(Debug, FromBytes, ToBytes, Eq, PartialEq)]
struct WithArray {
    foo: u8,
    arr: [i16; 3],
    bar: u8,
}

#[test]
fn struct_test() {
    let bytes = [0x3, 0, 0, 0, 0x42, 0];
    let bebytes = [0, 0, 0, 0x3, 0, 0x42];
    assert_eq!(
        Test::from_le_bytes(bytes),
        Test {
            foo: 0x3,
            bar: 0x42
        }
    );
    assert_eq!(
        Test::from_bytes(bytes),
        Test {
            foo: 0x3000000,
            bar: 0x4200
        }
    );
    assert_eq!(
        Test {
            foo: 0x3,
            bar: 0x42
        }
        .to_le_bytes(),
        bytes
    );
    assert_eq!(
        Test {
            foo: 0x3,
            bar: 0x42
        }
        .to_bytes(),
        bebytes
    );
    assert_eq!(Nameless::from_le_bytes(bytes), Nameless(0x3, 0x420000));
    assert_eq!(bytes, Nameless(0x3, 0x420000).to_le_bytes());
    assert_eq!(<Unit as FromBytes>::Bytes::SIZE, 0);
}

#[test]
fn enum_test() {
    assert_eq!(TestEnum::Foo.to_le_bytes(), [0x2]);
    assert_eq!(TestEnum::Bar.to_be_bytes(), [0x12]);
    assert_eq!(FatTestEnum::Foo.to_le_bytes(), [0x2, 0, 0, 0]);
    assert_eq!(FatTestEnum::Bar.to_be_bytes(), [0, 0, 0, 0x12]);
    assert_eq!(
        Ok(FatTestEnum::Foo),
        FatTestEnum::try_from_le_bytes([0x2, 0, 0, 0])
    );
    assert_eq!(
        Err(error::InvalidData),
        FatTestEnum::try_from_le_bytes([0x3, 0, 0, 0])
    );
}

#[test]
fn custom_error() {
    assert_eq!(
        Err(CustomError::WrongTestEnum),
        TestEnum::try_from_le_bytes([0x3])
    );
}

#[test]
fn array_test() {
    let bytes = [0x3, 0x5, 0, 0x7, 0, 0x11, 0, 0x42];
    assert_eq!(
        WithArray::from_le_bytes(bytes),
        WithArray {
            foo: 0x3,
            arr: [0x5, 0x7, 0x11],
            bar: 0x42
        }
    );
    assert_eq!(
        bytes,
        WithArray {
            foo: 0x3,
            arr: [0x5, 0x7, 0x11],
            bar: 0x42
        }
        .to_le_bytes()
    );
}
