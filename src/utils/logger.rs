use std::sync::OnceLock;

use log::{LevelFilter, Log, Metadata, Record};

struct ConsoleLogger(LevelFilter);

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{:<5}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

struct DebuggerLogger(LevelFilter);

impl Log for DebuggerLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            debugger::print(format_args!("[{:<5}] {}", record.level(), record.args()))
        }
    }

    fn flush(&self) {}
}

struct CombinedLogger {
    console: ConsoleLogger,
    debugger: DebuggerLogger
}

impl CombinedLogger {
    pub const fn new(console_level: LevelFilter, debugger_level: LevelFilter) -> Self {
        Self {
            console: ConsoleLogger(console_level),
            debugger: DebuggerLogger(debugger_level)
        }
    }

    pub fn max_level(&self) -> LevelFilter {
        self.console.0.max(self.debugger.0)
    }
}

impl Log for CombinedLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.console.enabled(metadata) || self.debugger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        self.console.log(record);
        self.debugger.log(record);
    }

    fn flush(&self) {
        self.console.flush();
        self.debugger.flush();
    }
}

pub fn init(console_level: LevelFilter, debugger_level: LevelFilter) -> bool {
    static LOGGER: OnceLock<CombinedLogger> = OnceLock::new();
    if LOGGER.get().is_some() {
        return false;
    }
    let logger = LOGGER.get_or_init(|| CombinedLogger::new(console_level, debugger_level));
    log::set_logger(logger).expect("Failed to set logger");
    log::set_max_level(logger.max_level());
    true
}

mod debugger {
    use std::fmt::{Arguments, Write};

    use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;

    use crate::utils::string::U16TextBuffer;

    pub fn print(args: Arguments<'_>) {
        U16TextBuffer::with_local(move |buffer| {
            buffer.write_fmt(args).expect("Failed to format log string");
            unsafe { OutputDebugStringW(buffer.finish()) }
        })
    }
}
