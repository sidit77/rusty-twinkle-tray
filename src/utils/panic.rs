use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt::{Arguments, Write};
//use std::panic::take_hook as take_panic_hook;
use std::panic::set_hook as set_panic_hook;

use windows::core::w;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_TASKMODAL};

use crate::utils::string::U16TextBuffer;

pub fn set_hook() {
    //let hook = take_panic_hook();
    set_panic_hook(Box::new(move |info| {
        let capture = Backtrace::capture();
        match capture.status() {
            BacktraceStatus::Captured => log::error!("{info}\n{capture}"),
            _ => log::error!("{info}")
        }
        //hook(info);
        show_msg(format_args!("{info}"));
    }))
}

//pub fn abort_on_panic<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> R {
//    match catch_unwind(f) {
//        Ok(r) => r,
//        Err(panic) => {
//            let msg = panic
//                .downcast_ref::<&'static str>()
//                .unwrap_or("")
//        }
//    }
//}

pub fn show_msg(args: Arguments<'_>) {
    if cfg!(debug_assertions) {
        return;
    }
    let mut buffer = U16TextBuffer::default();
    buffer.write_fmt(args).unwrap_or_else(|_| {
        buffer.clear();
        buffer.write("Failed to format message!");
    });
    unsafe {
        MessageBoxW(None, buffer.finish(), w!("Panic"), MB_OK | MB_ICONERROR | MB_TASKMODAL);
    }
}
