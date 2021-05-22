use std::fmt;
use std::ops::Deref;
use std::convert::TryFrom;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FixedLenStr<const MAX_LEN: usize> {
    /// The number of bytes in `bytes` that are used (up to MAX_LEN)
    len: usize,
    /// The bytes stored in this string, with `bytes[..len]` guaranteed to be valid UTF-8
    bytes: [u8; MAX_LEN],
}

impl<const MAX_LEN: usize> fmt::Debug for FixedLenStr<MAX_LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<const MAX_LEN: usize> fmt::Display for FixedLenStr<MAX_LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<const MAX_LEN: usize> Deref for FixedLenStr<MAX_LEN> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety: `bytes[..len]` is guaranteed to be valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len]) }
    }
}

impl<'a, const MAX_LEN: usize> TryFrom<&'a str> for FixedLenStr<MAX_LEN> {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let value_bytes = value.as_bytes();
        let len = value_bytes.len();
        if len <= MAX_LEN {
            let mut bytes = [0; MAX_LEN];
            bytes[..len].copy_from_slice(value_bytes);
            Ok(Self {len, bytes})
        } else {
            Err(value)
        }
    }
}
