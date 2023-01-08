
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
    let level = format!("[{:?}]", level).to_uppercase();
    return format!("{}: {}", level, message).to_string();
}
pub fn info(message: &str) -> String {
    let level = format!("[{:?}]", LogLevel::Info).to_uppercase();
    return format!("{}: {}", level, message).to_string();
}
pub fn warn(message: &str) -> String {
    let level = format!("[{:?}]", LogLevel::Warning).to_uppercase();
    return format!("{}: {}", level, message).to_string();
}
pub fn error(message: &str) -> String {
    let level = format!("[{:?}]", LogLevel::Error).to_uppercase();
    return format!("{}: {}", level, message).to_string();
}
