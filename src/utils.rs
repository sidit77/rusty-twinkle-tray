use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Write};
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WStr<const N: usize>([u16; N]);

impl<const N: usize> From<[u16; N]> for WStr<N> {
    fn from(value: [u16; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Display for WStr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in char::decode_utf16(self.as_slice().iter().copied()) {
            f.write_char(c.unwrap_or(std::char::REPLACEMENT_CHARACTER))?
        }
        Ok(())
    }
}

impl<const N: usize> Debug for WStr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl<const N: usize> From<WStr<N>> for OsString {
    fn from(value: WStr<N>) -> Self {
        OsString::from_wide(value.as_slice())
    }
}

impl<const N: usize> From<WStr<N>> for PathBuf {
    fn from(value: WStr<N>) -> Self {
        PathBuf::from(OsString::from(value))
    }
}

impl<const N: usize> WStr<N> {
    pub fn as_slice(&self) -> &[u16] {
        let end = self.0
            .iter()
            .position(|c| *c == 0)
            .unwrap_or(self.0.len());
        &self.0[..end]
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}