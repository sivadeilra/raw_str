//! Defines the `RawStr` type, which represents a string that is expected to contain UTF-8 string
//! data, but which has not yet been validated.
//!
#![doc = include_str!("../README.md")]

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(all(not(feature = "std"), test), no_std)]

use core::fmt::{self, Debug, Display};
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(test)]
mod tests;

/// A reference to a string which is believed to contain valid UTF-8 string data, but which has
/// not yet been verified to contain UTF-8 data.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[repr(transparent)]
pub struct RawStr<'a>(pub &'a [u8]);

impl<'a> RawStr<'a> {
    /// Converts this value to a string.
    ///
    /// If the string is valid UTF-8, then this returns a borrowed `&str` reference. If the string
    /// does invalid UTF-8 sequences, then this will allocate a new `String` and convert the
    /// character data to UTF-8, replacing invalid sequences with the Unicode replacement character.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn to_str_lossy(&self) -> Cow<'a, str> {
        String::from_utf8_lossy(self.0)
    }

    /// Validates that the string contains valid UTF-8, and returns `Some(s)` if so.
    /// If the string contains invalid UTF-8 sequences, this returns `None`.
    #[inline(always)]
    pub fn to_str(&self) -> Option<&'a str> {
        core::str::from_utf8(self.0).ok()
    }

    /// Wraps a byte array as `RawStr`.
    #[inline(always)]
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    /// Wraps a string slice as `RawStr`.
    #[inline(always)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &'a str) -> Self {
        Self(s.as_bytes())
    }

    /// Gets the raw bytes
    #[inline(always)]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Tests if the string is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Length of the string in bytes
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Equivalent to `str::eq_ignore_ascii_case`
    #[inline(always)]
    pub fn eq_ignore_ascii_case(&self, other: RawStr<'_>) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }

    /// Equivalent to `str::ends_with`
    #[inline(always)]
    pub fn ends_with(&self, other: RawStr<'_>) -> bool {
        let other_bytes: &[u8] = other.0;
        if other_bytes.len() <= self.0.len() {
            self.0[self.0.len() - other_bytes.len()..self.0.len()] == *other_bytes
        } else {
            false
        }
    }
}

impl<'a> PartialEq<str> for RawStr<'a> {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl<'a> PartialEq<&str> for RawStr<'a> {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl<'a> AsRef<[u8]> for RawStr<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> Debug for RawStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            let s = self.to_str_lossy();
            Debug::fmt(&*s, f)
        }

        #[cfg(not(feature = "std"))]
        {
            if let Ok(s) = core::str::from_utf8(self.0) {
                Debug::fmt(s, f)
            } else {
                write!(f, "{:x?}", self.0)
            }
        }
    }
}

impl<'a> Display for RawStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            let s = self.to_str_lossy();
            Display::fmt(&*s, f)
        }

        #[cfg(not(feature = "std"))]
        {
            if let Ok(s) = core::str::from_utf8(self.0) {
                Display::fmt(s, f)
            } else {
                write!(f, "{:x?}", self.0)
            }
        }
    }
}

impl<'a, S: 'a + AsRef<str> + ?Sized> From<&'a S> for RawStr<'a> {
    #[inline(always)]
    fn from(s: &'a S) -> Self {
        Self(s.as_ref().as_bytes())
    }
}
