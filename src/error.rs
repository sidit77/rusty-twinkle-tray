use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt::{Debug, Display, Formatter};
use std::panic::Location;
use windows::core::Error;
use windows::Win32::Foundation::NO_ERROR;

pub type Result<T> = std::result::Result<T, TracedError>;

pub enum Trace {
    Backtrace(Box<Backtrace>),
    Location(&'static Location<'static>)
}

impl Trace {

    #[track_caller]
    pub fn capture() -> Self {
        let capture = Backtrace::capture();
        match capture.status() {
            BacktraceStatus::Captured => Self::Backtrace(Box::new(capture)),
            _ => Self::Location(Location::caller())
        }
    }

    pub fn is_backtrace(&self) -> bool {
        matches!(self, Self::Backtrace(_))
    }

}

impl Debug for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Trace::Backtrace(capture) => Debug::fmt(capture, f),
            Trace::Location(location) => Debug::fmt(location, f)
        }
    }
}

impl Display for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Trace::Backtrace(capture) => Display::fmt(capture, f),
            Trace::Location(location) => Display::fmt(location, f)
        }
    }
}

pub struct TracedError {
    inner: Error,
    backtrace: Trace
}

impl Debug for TracedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.backtrace.is_backtrace() {
            true => write!(f, "{:?}\n{}", self.inner, self.backtrace),
            false => f
                .debug_struct("Error")
                .field("code", &self.inner.code())
                .field("message", &self.inner.message())
                .field("location", &FromDisplay(&self.backtrace))
                .finish()
        }
    }
}

impl Display for TracedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl std::error::Error for TracedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl<T: Into<Error>> From<T> for TracedError {
    #[track_caller]
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
            backtrace: Trace::capture(),
        }
    }
}

struct FromDisplay<T>(pub T);

impl<T: Display> Debug for FromDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub trait OptionExt<T> {
    fn some(self) -> windows::core::Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn some(self) -> windows::core::Result<T> {
        self.ok_or(Error::from(NO_ERROR))
    }
}

#[macro_export]
macro_rules! win_assert {
    ($cond:expr) => {
        if !($cond) {
            return Err(windows::core::Error::from(windows::Win32::Foundation::ERROR_ASSERTION_FAILURE).into());
        }
    };
}