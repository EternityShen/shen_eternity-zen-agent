use std::fmt::Display;

pub enum LogLevel {
    Info,
    Warn,
    Error,
    UnKnown,
}

impl From<String> for LogLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ERROR" => LogLevel::Error,
            "INFO" => LogLevel::Info,
            "WORN" => LogLevel::Warn,
            _ => LogLevel::UnKnown,
        }
    }
}

impl Into<String> for LogLevel {
    fn into(self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            LogLevel::UnKnown => "UNKNOWN".to_string(),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            LogLevel::UnKnown => "UNKNOWN".to_string(),
        };
        write!(f, "{}", str)
    }
}
