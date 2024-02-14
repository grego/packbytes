use core::convert::Infallible;
use core::fmt::{Display, Formatter, Result};

/// A general error signaling an attempt to construct an value from an invalid byte representation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InvalidData;

/// An error signaling an attempt to construct a value of the type `char` from an invalid
/// byte representation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InvalidChar;

/// An error signaling an attempt to construct a value of the type `bool` from an invalid
/// byte representation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBool;

impl Display for InvalidData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the bytes did not represent a valid value")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidData {}

impl Display for InvalidChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the bytes did not represent a valid value")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidChar {}

impl Display for InvalidBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the bytes did not represent a valid value")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidBool {}

impl From<Infallible> for InvalidData {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<Infallible> for InvalidChar {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<Infallible> for InvalidBool {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<InvalidChar> for InvalidData {
    fn from(_: InvalidChar) -> Self {
        InvalidData
    }
}

impl From<InvalidBool> for InvalidData {
    fn from(_: InvalidBool) -> Self {
        InvalidData
    }
}
