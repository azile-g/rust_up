// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error, 
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let loglevel = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG"
    };
    format!("[{0}]: {1}", loglevel, message)
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {0}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {0}", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {0}", message)
}

