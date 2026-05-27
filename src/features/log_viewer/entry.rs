use std::fmt;

/// 로그 레벨을 나타내는 열거형.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Unknown,
}

impl LogLevel {
    /// 레벨 이름을 반환합니다.
    pub fn name(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Unknown => "?????",
        }
    }

    /// 레벨에서 색상(R, G, B)을 반환합니다.
    pub fn color(self) -> (u8, u8, u8) {
        match self {
            Self::Trace => (107, 107, 107),
            Self::Debug => (143, 143, 241),
            Self::Info => (91, 165, 91),
            Self::Warn => (210, 180, 46),
            Self::Error => (216, 76, 90),
            Self::Unknown => (150, 150, 150),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// 파싱된 로그 엔트리.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntry {
    /// 날짜 (YYYY-MM-DD 형식).
    pub date: String,
    /// 시간 (HH:MM:SS.sss 형식).
    pub time: String,
    /// 로그 레벨.
    pub level: LogLevel,
    /// 메시지 (스코프 포함).
    pub message: String,
    /// 원문 줄 전체.
    pub raw_line: String,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} {}", self.level, self.date, self.message)
    }
}