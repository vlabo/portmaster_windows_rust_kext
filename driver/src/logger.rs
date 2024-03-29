use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use protocol::info::{LogLine, Severity};

#[cfg(not(debug_assertions))]
pub const LOG_LEVEL: u8 = Severity::Error as u8;

#[cfg(debug_assertions)]
pub const LOG_LEVEL: u8 = Severity::Debug as u8;

pub(crate) struct Logger {
    log_lines: Option<Vec<AtomicPtr<LogLine>>>,
    start_index: usize,
    end_index: AtomicUsize,
}

impl Logger {
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(1024);
        for _ in 0..vec.capacity() {
            vec.push(AtomicPtr::default());
        }

        Self {
            log_lines: Some(vec),
            start_index: 0,
            end_index: AtomicUsize::new(0),
        }
    }

    pub fn add_line(&mut self, severity: Severity, prefix: String, line_str: String) {
        if let Some(log_lines) = &mut self.log_lines {
            let mut index = self.end_index.fetch_add(1, Ordering::Acquire);
            index %= log_lines.len();
            let ptr = &mut log_lines[index];
            let line = Box::new(LogLine::new(severity, prefix, line_str));
            let old = ptr.swap(Box::into_raw(line), Ordering::Release);
            if !old.is_null() {
                unsafe {
                    _ = Box::from_raw(old);
                }
            }
        }
    }

    #[allow(clippy::vec_box)] // Box<LogLine> is used without the Vec later.
    pub fn flush(&mut self) -> Vec<Box<LogLine>> {
        let mut vec = Vec::new();
        let end_index = self.end_index.load(Ordering::Acquire);
        if end_index <= self.start_index {
            return vec;
        }
        let start_index = self.start_index;
        let count = end_index - start_index;
        if let Some(log_lines) = &mut self.log_lines {
            for i in start_index..start_index + count {
                let index = i % log_lines.len();
                let ptr = log_lines[index].swap(core::ptr::null_mut(), Ordering::Acquire);
                unsafe {
                    if !ptr.is_null() {
                        vec.push(Box::from_raw(ptr));
                    }
                }
            }
        }

        self.start_index = end_index;
        vec
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        if let Some(log_lines) = &mut self.log_lines {
            for line in log_lines {
                let ptr = line.load(Ordering::Relaxed);
                unsafe {
                    if !ptr.is_null() {
                        _ = Box::from_raw(ptr);
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! crit {
    ($log:expr, $($arg:tt)*) => ({
        if protocol::info::Severity::Error as u8 >= $crate::logger::LOG_LEVEL {
            let message = alloc::format!($($arg)*);
            $log.add_line(protocol::info::Severity::Critical, alloc::format!("{}:{} ", file!(), line!()), message)
        }
    });
}

#[macro_export]
macro_rules! err {
    ($log:expr, $($arg:tt)*) => ({
        if protocol::info::Severity::Error as u8 >= $crate::logger::LOG_LEVEL {
            let message = alloc::format!($($arg)*);
            $log.add_line(protocol::info::Severity::Error, alloc::format!("{}:{} ", file!(), line!()), message)
        }
    });
}

#[macro_export]
macro_rules! dbg {
    ($log:expr, $($arg:tt)*) => ({
        if protocol::info::Severity::Debug as u8 >= $crate::logger::LOG_LEVEL {
            let message = alloc::format!($($arg)*);
            $log.add_line(protocol::info::Severity::Debug, alloc::format!("{}:{} ", file!(), line!()), message)
        }
    });
}

#[macro_export]
macro_rules! warn {
    ($log:expr, $($arg:tt)*) => ({
        if protocol::info::Severity::Warning as u8 >= $crate::logger::LOG_LEVEL {
            let message = alloc::format!($($arg)*);
            $log.add_line(protocol::info::Severity::Warning, alloc::format!("{}:{} ", file!(), line!()), message)
        }
    });
}

#[macro_export]
macro_rules! info {
    ($log:expr, $($arg:tt)*) => ({
        if protocol::info::Severity::Info as u8 >= $crate::logger::LOG_LEVEL {
            let message = alloc::format!($($arg)*);
            $log.add_line(protocol::info::Severity::Info, alloc::format!("{}:{} ", file!(), line!()), message)
        }
    });
}
