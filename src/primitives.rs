use crate::error::{InvalidBool, InvalidChar};
use crate::{FromBytes, ToBytes, TryFromBytes};

// 8-bit

impl FromBytes for u8 {
    type Bytes = [u8; 1];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u8::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        u8::from_be_bytes(bytes)
    }
}

impl ToBytes for u8 {
    type Bytes = [u8; 1];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u8::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u8::to_be_bytes(self)
    }
}

impl FromBytes for i8 {
    type Bytes = [u8; 1];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        i8::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        i8::from_be_bytes(bytes)
    }
}

impl ToBytes for i8 {
    type Bytes = [u8; 1];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        i8::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        i8::to_be_bytes(self)
    }
}

// 16-bit

impl FromBytes for u16 {
    type Bytes = [u8; 2];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u16::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        u16::from_be_bytes(bytes)
    }
}

impl ToBytes for u16 {
    type Bytes = [u8; 2];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u16::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u16::to_be_bytes(self)
    }
}

impl FromBytes for i16 {
    type Bytes = [u8; 2];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        i16::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        i16::from_be_bytes(bytes)
    }
}

impl ToBytes for i16 {
    type Bytes = [u8; 2];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        i16::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        i16::to_be_bytes(self)
    }
}

// 32-bit

impl FromBytes for u32 {
    type Bytes = [u8; 4];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u32::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        u32::from_be_bytes(bytes)
    }
}

impl ToBytes for u32 {
    type Bytes = [u8; 4];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u32::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u32::to_be_bytes(self)
    }
}

impl FromBytes for i32 {
    type Bytes = [u8; 4];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        i32::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        i32::from_be_bytes(bytes)
    }
}

impl ToBytes for i32 {
    type Bytes = [u8; 4];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        i32::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        i32::to_be_bytes(self)
    }
}

// 64-bit

impl FromBytes for u64 {
    type Bytes = [u8; 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u64::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        u64::from_be_bytes(bytes)
    }
}

impl ToBytes for u64 {
    type Bytes = [u8; 8];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u64::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u64::to_be_bytes(self)
    }
}

impl FromBytes for i64 {
    type Bytes = [u8; 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        i64::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        i64::from_be_bytes(bytes)
    }
}

impl ToBytes for i64 {
    type Bytes = [u8; 8];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        i64::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        i64::to_be_bytes(self)
    }
}

// 128-bit

impl FromBytes for u128 {
    type Bytes = [u8; 16];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        u128::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        u128::from_be_bytes(bytes)
    }
}

impl ToBytes for u128 {
    type Bytes = [u8; 16];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u128::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u128::to_be_bytes(self)
    }
}

impl FromBytes for i128 {
    type Bytes = [u8; 16];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        i128::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        i128::from_be_bytes(bytes)
    }
}

impl ToBytes for i128 {
    type Bytes = [u8; 16];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        i128::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        i128::to_be_bytes(self)
    }
}

// Size

impl FromBytes for usize {
    type Bytes = [u8; usize::BITS as usize / 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        usize::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        usize::from_be_bytes(bytes)
    }
}

impl ToBytes for usize {
    type Bytes = [u8; usize::BITS as usize / 8];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        usize::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        usize::to_be_bytes(self)
    }
}

impl FromBytes for isize {
    type Bytes = [u8; isize::BITS as usize / 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        isize::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        isize::from_be_bytes(bytes)
    }
}

impl ToBytes for isize {
    type Bytes = [u8; isize::BITS as usize / 8];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        isize::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        isize::to_be_bytes(self)
    }
}

// Floats

impl FromBytes for f32 {
    type Bytes = [u8; 4];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        f32::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        f32::from_be_bytes(bytes)
    }
}

impl ToBytes for f32 {
    type Bytes = [u8; 4];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        f32::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        f32::to_be_bytes(self)
    }
}

impl FromBytes for f64 {
    type Bytes = [u8; 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        f64::from_le_bytes(bytes)
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        f64::from_be_bytes(bytes)
    }
}

impl ToBytes for f64 {
    type Bytes = [u8; 8];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        f64::to_le_bytes(self)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        f64::to_be_bytes(self)
    }
}

// Char

impl TryFromBytes for char {
    type Bytes = [u8; 4];
    type Error = InvalidChar;

    #[inline]
    fn try_from_le_bytes(bytes: Self::Bytes) -> Result<Self, InvalidChar> {
        char::from_u32(u32::from_le_bytes(bytes)).ok_or(InvalidChar)
    }

    #[inline]
    fn try_from_be_bytes(bytes: Self::Bytes) -> Result<Self, InvalidChar> {
        char::from_u32(u32::from_be_bytes(bytes)).ok_or(InvalidChar)
    }
}

impl ToBytes for char {
    type Bytes = [u8; 4];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u32::to_le_bytes(self as u32)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u32::to_be_bytes(self as u32)
    }
}

// Bool

impl TryFromBytes for bool {
    type Bytes = [u8; 1];
    type Error = InvalidBool;

    #[inline]
    fn try_from_le_bytes(bytes: Self::Bytes) -> Result<Self, InvalidBool> {
        match u8::from_le_bytes(bytes) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(InvalidBool),
        }
    }

    #[inline]
    fn try_from_be_bytes(bytes: Self::Bytes) -> Result<Self, InvalidBool> {
        match u8::from_be_bytes(bytes) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(InvalidBool),
        }
    }
}

impl ToBytes for bool {
    type Bytes = [u8; 1];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        u8::to_le_bytes(self as u8)
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        u8::to_be_bytes(self as u8)
    }
}

// ()

impl FromBytes for () {
    type Bytes = [u8; 0];

    #[inline]
    fn from_le_bytes(_: Self::Bytes) -> Self {}

    #[inline]
    fn from_be_bytes(_: Self::Bytes) -> Self {}
}

impl ToBytes for () {
    type Bytes = [u8; 0];

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        []
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        []
    }
}
