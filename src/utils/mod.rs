pub mod error;
pub mod logger;
pub mod panic;

use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Write};
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use betrayer::{TrayEvent, TrayIcon, TrayIconBuilder, TrayResult};

use windows::core::PCWSTR;
use winit::event_loop::EventLoop;

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
        let end = self.0.iter().position(|c| *c == 0).unwrap_or(self.0.len());
        &self.0[..end]
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

#[derive(Default)]
pub struct U16TextBuffer {
    inner: Vec<u16>
}

impl U16TextBuffer {
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn finish(&mut self) -> PCWSTR {
        self.inner.push(0);
        PCWSTR(self.inner.as_ptr())
    }

    pub fn write<S: AsRef<str>>(&mut self, text: S) {
        self.inner.extend(text.as_ref().encode_utf16());
    }
}

impl Write for U16TextBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write(s);
        Ok(())
    }
}

pub trait TrayIconBuilderExt<T> {
    fn build_event_loop<E, F>(self, event_loop: &EventLoop<E>, map: F) -> TrayResult<TrayIcon<T>>
        where
            F: Fn(TrayEvent<T>) -> Option<E> + Send + 'static,
            E: Send;
}

impl<T: Clone + Send + 'static> TrayIconBuilderExt<T> for TrayIconBuilder<T> {
    fn build_event_loop<E, F>(self, event_loop: &EventLoop<E>, map: F) -> TrayResult<TrayIcon<T>>
        where
            F: Fn(TrayEvent<T>) -> Option<E> + Send + 'static,
            E: Send
    {
        let proxy = event_loop.create_proxy();
        self.build(move |event| {
            if let Some(event) = map(event) {
                proxy
                    .send_event(event)
                    .unwrap_or_else(|err| log::warn!("Failed to forward event: {}", err));
            }
        })
    }
}