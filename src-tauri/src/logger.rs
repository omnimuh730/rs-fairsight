use std::sync::Mutex;
use std::collections::VecDeque;
use chrono::Local;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Activity,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Activity => write!(f, "ACTIVITY"),
        }
    }
}

const MAX_LOG_ENTRIES: usize = 1000; // Keep last 1000 log entries

static GLOBAL_LOGGER: Lazy<Mutex<VecDeque<LogEntry>>> = Lazy::new(|| {
    Mutex::new(VecDeque::with_capacity(MAX_LOG_ENTRIES))
});

pub fn log_message(level: LogLevel, module: &str, message: &str) {
    let entry = LogEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
        level,
        module: module.to_string(),
        message: message.to_string(),
    };

    // Print to console as well
    println!("[{}] [{}] {}: {}", entry.timestamp, entry.level, entry.module, entry.message);

    let mut logger = GLOBAL_LOGGER.lock().unwrap();
    
    // Remove oldest entry if we've reached capacity
    if logger.len() >= MAX_LOG_ENTRIES {
        logger.pop_front();
    }
    
    logger.push_back(entry);
}

pub fn get_logs() -> Vec<LogEntry> {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.iter().cloned().collect()
}

pub fn get_recent_logs(count: usize) -> Vec<LogEntry> {
    let logger = GLOBAL_LOGGER.lock().unwrap();
    logger.iter().rev().take(count).cloned().collect::<Vec<_>>().into_iter().rev().collect()
}

pub fn clear_logs() {
    let mut logger = GLOBAL_LOGGER.lock().unwrap();
    logger.clear();
}

// Convenience macros
#[macro_export]
macro_rules! log_info {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::log_message($crate::logger::LogLevel::Info, $module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::log_message($crate::logger::LogLevel::Warning, $module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::log_message($crate::logger::LogLevel::Error, $module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::log_message($crate::logger::LogLevel::Debug, $module, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_activity {
    ($module:expr, $($arg:tt)*) => {
        $crate::logger::log_message($crate::logger::LogLevel::Activity, $module, &format!($($arg)*))
    };
}
