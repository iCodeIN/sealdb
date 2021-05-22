use std::fmt;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct FixedLenStr<const LEN: usize> {
    /// The number of bytes in `bytes` that are used (up to LEN)
    len: usize,
    /// The bytes stored in this string, with `bytes[..len]` guaranteed to be valid UTF-8
    bytes: [u8; LEN],
}

impl<const LEN: usize> fmt::Debug for FixedLenStr<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<const LEN: usize> fmt::Display for FixedLenStr<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<const LEN: usize> Deref for FixedLenStr<LEN> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety: `bytes[..len]` is guaranteed to be valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len]) }
    }
}
