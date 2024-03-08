#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

/// Errors signaling bytes that don't represent valid data.
pub mod error;
mod primitives;

#[cfg(feature = "packbytes-derive")]
pub use packbytes_derive::{FromBytes, ToBytes, TryFromBytes};

use core::convert::Infallible;
use core::ops;

/// A private module to disallow implementing ByteArray on other types than byte arrays.
mod private {
    pub trait ByteArray {}

    impl<const N: usize> ByteArray for [u8; N] {}
}

/// A helper trait that is implemented only for byte arrays.
///
/// It is necessary since associated constants cannot be used in function signatures so far.
pub trait ByteArray:
    private::ByteArray
    + ops::IndexMut<usize, Output = u8>
    + ops::IndexMut<ops::Range<usize>, Output = [u8]>
    + AsRef<[u8]>
    + AsMut<[u8]>
{
    /// The length of this byte array.
    const SIZE: usize;

    /// Return the array with all zeros.
    /// Cannot use `Default` as it is not implemented for all array sizes.
    fn zeroed() -> Self;
}

impl<const N: usize> ByteArray for [u8; N] {
    const SIZE: usize = N;

    fn zeroed() -> Self {
        [0; N]
    }
}

/// Create a value from its representation as a packed stack byte array of a fixed size.
///
/// Most times, the method `from_bytes` should be used, as it ensures consistency by respecting
/// the byte order set by the `PREFERS_LE` associated constant.
pub trait FromBytes: Sized {
    /// A byte array which can store a packed representation of this type.
    type Bytes: ByteArray;

    /// Is it preferred to represent this type as bytes in the little endian order?
    const PREFERS_LE: bool = true;

    /// Create a value of this type from its representation as a byte array in little endian.
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
    /// Create a value of this type from its representation as a byte array in big endian.
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Create a value of this type from its representation as a byte array in the preferred
    /// byte order, set in the associated constant `PREFERS_LE`.
    #[inline]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        if Self::PREFERS_LE {
            Self::from_le_bytes(bytes)
        } else {
            Self::from_be_bytes(bytes)
        }
    }

    /// Create a value of this type from its representation as a byte array in native endian.
    ///
    /// As the target platform’s native endianness is used, portable code likely wants to use
    /// `from_le_bytes` or `from_be_bytes`, as appropriate instead.
    #[inline]
    fn from_ne_bytes(bytes: Self::Bytes) -> Self {
        if cfg!(target_endian = "little") {
            Self::from_le_bytes(bytes)
        } else {
            Self::from_be_bytes(bytes)
        }
    }

    /// Read a byte representation of this type in the preferred byte order (set in the associated
    /// constant `PREFERS_LE`) and create a value of this type from it.
    #[cfg(feature = "std")]
    #[inline]
    fn read_packed<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut bytes = Self::Bytes::zeroed();
        reader.read_exact(bytes.as_mut())?;
        Ok(Self::from_bytes(bytes))
    }
}

/// Try to create a value from its representation as a packed stack byte array of a fixed size.
///
/// Most times, the method `try_from_bytes` should be used, as it ensures consistency by respecting
/// the byte order set by the `PREFERS_LE` associated constant.
pub trait TryFromBytes: Sized {
    /// A byte array which can store a packed representation of this type.
    type Bytes: ByteArray;
    /// A type containing the failure of creating a value of the type from bytes.
    type Error;

    /// Is it preferred to represent this type as bytes in the little endian order?
    const PREFERS_LE: bool = true;

    /// Try to create a value of this type from its representation as a byte array in little endian.
    fn try_from_le_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error>;
    /// Try to create a value of this type from its representation as a byte array in big endian.
    fn try_from_be_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error>;

    /// Try to create a value of this type from its representation as a byte array in the preferred
    /// byte order, set in the associated constant `PREFERS_LE`.
    #[inline]
    fn try_from_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error> {
        if Self::PREFERS_LE {
            Self::try_from_le_bytes(bytes)
        } else {
            Self::try_from_be_bytes(bytes)
        }
    }

    /// Try to create a value of this type from its representation as a byte array in native endian.
    ///
    /// As the target platform’s native endianness is used, portable code likely wants to use
    /// `try_from_le_bytes` or `try_from_be_bytes`, as appropriate instead.
    #[inline]
    fn try_from_ne_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error> {
        if cfg!(target_endian = "little") {
            Self::try_from_le_bytes(bytes)
        } else {
            Self::try_from_be_bytes(bytes)
        }
    }
}

/// Pack this type into a stack byte array of a fixed size.
///
/// Most times, the method `to_bytes` should be used, as it ensures consistency by respecting
/// the byte order set by the `PREFERS_LE` associated constant.
pub trait ToBytes: Sized {
    /// A byte array which can store a packed representation of this type.
    type Bytes: ByteArray;

    /// Is it preferred to represent this type as bytes in the little endian order?
    const PREFERS_LE: bool = true;

    /// Return the memory representation of this type as a byte array in little endian byte order.
    fn to_le_bytes(self) -> Self::Bytes;
    /// Return the memory representation of this type as a byte array in big endian byte order.
    fn to_be_bytes(self) -> Self::Bytes;

    /// Return the memory representation of this type as a byte array in the preferred
    /// byte order, set in the associated constant `PREFERS_LE`.
    #[inline]
    fn to_bytes(self) -> Self::Bytes {
        if Self::PREFERS_LE {
            self.to_le_bytes()
        } else {
            self.to_be_bytes()
        }
    }

    /// Return the memory representation of this type as a byte array in native endian byte order.
    ///
    /// As the target platform’s native endianness is used, portable code likely wants to use
    /// `to_le_bytes` or `to_be_bytes`, as appropriate instead.
    #[inline]
    fn to_ne_bytes(self) -> Self::Bytes {
        if cfg!(target_endian = "little") {
            self.to_le_bytes()
        } else {
            self.to_be_bytes()
        }
    }

    /// Write the value of this type to a writer in preferred byte order, set by the associated
    /// constant `PREFERS_LE`.
    #[cfg(feature = "std")]
    #[inline]
    fn write_packed<W: std::io::Write>(self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.to_bytes().as_ref())
    }
}

impl<B: ByteArray, T: FromBytes<Bytes = B>> TryFromBytes for T {
    /// A byte array which can store a packed representation of this type.
    type Bytes = B;
    /// A type containing the failure of creating a value of the type from bytes.
    type Error = Infallible;

    /// Create a value of this type from its representation as a byte array in little endian.
    #[inline]
    fn try_from_le_bytes(bytes: B) -> Result<Self, Self::Error> {
        Ok(Self::from_le_bytes(bytes))
    }

    /// Create a value of this type from its representation as a byte array in big endian.
    #[inline]
    fn try_from_be_bytes(bytes: B) -> Result<Self, Self::Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}

impl<const N: usize> FromBytes for [u8; N] {
    type Bytes = Self;

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        bytes
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        bytes
    }
}

impl<const N: usize> ToBytes for [u8; N] {
    type Bytes = Self;

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        self
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        self
    }
}
