use crate::features::log_viewer::entry::{LogLevel, LogEntry};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// 로그 디렉토리에서 특정 날짜의 로그 파일을 찾아 줄을 반환합니다.
pub fn read_log_lines(date: &str) -> io::Result<Vec<String>> {
    let log_dir = PathBuf::from("logs");
    let log_file = log_dir.join(format!("app.log.{}", date));

    if !log_file.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(&log_file)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().collect::<Result<_, _>>().unwrap_or_default())
}

/// 로그 디렉토리에서 사용 가능한 날짜 목록을 반환합니다.
pub fn list_log_dates() -> io::Result<Vec<String>> {
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        return Ok(Vec::new());
    }

    let mut dates: Vec<String> = Vec::new();
    for entry in fs::read_dir(&log_dir).expect("failed to read logs dir") {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                // app.log.YYYY-MM-DD 패턴 매칭
                if name.starts_with("app.log.") {
                    if let Some(date_part) = name.strip_prefix("app.log.") {
                        dates.push(date_part.to_string());
                    }
                }
            }
        }
    }
    dates.sort();
    Ok(dates)
}

/// 로그 줄을 파싱하여 LogEntry를 생성합니다.
pub fn parse_log_line(line: &str, _date: &str) -> Option<LogEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // 형식: "2026-05-27T02:01:16.059671Z  INFO  gpuishowcase::shared::logger scope=\"app.lifecycle\" logging initialized..."
    let trimmed = line.trim_start();

    // 타임스탬프 파싱: YYYY-MM-DDTHHH:MM:SS.sssZ
    let timestamp_end = find_timestamp_end(trimmed)?;
    let timestamp = &trimmed[..timestamp_end];

    // 날짜와 시간 분리
    if let Some(t) = timestamp.find('T') {
        let date_part = &timestamp[..t];
        let time_part = extract_time(&timestamp[t + 1..]);

        // 레벨 파싱 (타임스탬프 다음 공백 뒤)
        let after_ts = trimmed[timestamp_end..].trim_start();
        let level = parse_level(after_ts);

        // 메시지 파싱: "LEVEL  module scope=\"...\" message" 또는 "LEVEL | module | message"
        let message = extract_message(after_ts, level);

        Some(LogEntry {
            date: date_part.to_string(),
            time: time_part,
            level,
            message,
            raw_line: line.to_string(),
        })
    } else {
        None
    }
}

/// 타임스탬프의 끝 위치를 찾습니다.
fn find_timestamp_end(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut in_bracket = false;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'[' {
            in_bracket = true;
        } else if b == b']' && in_bracket {
            return Some(i + 1);
        } else if b == b' ' && !in_bracket && i > 20 {
            // "Z " 또는 "] "에서 타임스탬프 종료
            if s.as_bytes().get(i - 1).copied() == Some(b'Z') || in_bracket {
                return Some(i);
            }
        }
    }
    // 브래킷 없는 경우: "Z " 찾기
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'Z' && i > 19 {
            return Some(i + 1);
        }
    }
    None
}

/// 시간 부분만 추출합니다.
fn extract_time(ts: &str) -> String {
    // "HH:MM:SS.sss" 형식
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() >= 2 {
        format!("{}:{}:{}", parts[0], parts[1], parts[2].split('.').next().unwrap_or(""))
    } else {
        ts.to_string()
    }
}

/// 로그 레벨을 파싱합니다.
fn parse_level(s: &str) -> LogLevel {
    let upper = s.to_uppercase();
    if let Some(pos) = upper.find("ERROR") {
        // "ERROR" 앞에 공백이 있고 뒤에 공백이나 따옴표가 있으면 매칭
        if pos == 0 || s.as_bytes()[pos - 1] == b' ' {
            if let Some(b) = upper.as_bytes().get(pos + 5) {
                if *b == b' ' || *b == b'"' || *b == b'|' {
                    return LogLevel::Error;
                }
            }
        }
    }
    if let Some(pos) = upper.find("WARN") {
        if pos == 0 || s.as_bytes()[pos - 1] == b' ' {
            if let Some(b) = upper.as_bytes().get(pos + 4) {
                if *b == b' ' || *b == b'"' || *b == b'|' {
                    return LogLevel::Warn;
                }
            }
        }
    }
    if let Some(pos) = upper.find("INFO") {
        if pos == 0 || s.as_bytes()[pos - 1] == b' ' {
            if let Some(b) = upper.as_bytes().get(pos + 4) {
                if *b == b' ' || *b == b'"' || *b == b'|' {
                    return LogLevel::Info;
                }
            }
        }
    }
    if let Some(pos) = upper.find("DEBUG") {
        if pos == 0 || s.as_bytes()[pos - 1] == b' ' {
            if let Some(b) = upper.as_bytes().get(pos + 5) {
                if *b == b' ' || *b == b'"' || *b == b'|' {
                    return LogLevel::Debug;
                }
            }
        }
    }
    if let Some(pos) = upper.find("TRACE") {
        if pos == 0 || s.as_bytes()[pos - 1] == b' ' {
            if let Some(b) = upper.as_bytes().get(pos + 5) {
                if *b == b' ' || *b == b'"' || *b == b'|' {
                    return LogLevel::Trace;
                }
            }
        }
    }
    LogLevel::Unknown
}

/// 메시지 부분을 추출합니다.
fn extract_message(after_ts: &str, level: LogLevel) -> String {
    let level_str = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warn => "WARN",
        LogLevel::Info => "INFO",
        LogLevel::Debug => "DEBUG",
        LogLevel::Trace => "TRACE",
        LogLevel::Unknown => "",
    };

    // 레벨 키워드 찾기
    let upper = after_ts.to_uppercase();
    let mut msg_start = after_ts.len();

    for (i, c) in upper.char_indices() {
        if c == level_str.chars().next().unwrap() {
            if &upper[i..i + level_str.len()] == level_str {
                // 레벨 앞에 공백 확인
                if i == 0 || after_ts.as_bytes()[i - 1] == b' ' {
                    msg_start = i + level_str.len();
                    break;
                }
            }
        }
    }

    let rest = after_ts[msg_start..].trim_start();

    // " | module | message" 형식 처리
    if rest.starts_with("|") {
        let parts: Vec<&str> = rest.split('|').collect();
        if parts.len() >= 3 {
            return parts[2].trim().to_string();
        }
        return rest.trim().to_string();
    }

    // "scope=\"...\" message" 형식 처리
    let mut msg = rest.to_string();
    if let Some(pos) = msg.find("scope=") {
        let before_scope = &msg[..pos].trim();
        // 모듈 경로 제거 (스コープ 전까지)
        if !before_scope.is_empty() && *before_scope != "gpuishowcase" {
            msg = msg[pos + 6..].to_string();
        } else {
            msg = msg[pos + 6..].to_string();
        }
    }

    // scope="..." 부분 제거
    if let Some(start) = msg.find("scope=\"") {
        if let Some(end) = msg[start..].find("\" ") {
            let after_scope = &msg[start + end + 2..];
            return after_scope.trim().to_string();
        } else if let Some(end) = msg[start..].find("\",") {
            let after_scope = &msg[start + end + 1..];
            return after_scope.trim().to_string();
        }
    }

    msg.trim().to_string()
}

/// 로그 줄을 파싱하여 LogEntry 벡터를 생성합니다.
pub fn parse_log_lines(lines: &[String], date: &str) -> Vec<LogEntry> {
    lines
        .iter()
        .filter_map(|line| parse_log_line(line, date))
        .collect()
}

/// 로그 줄을 레벨과 키워드로 필터링합니다.
pub fn filter_entries(
    entries: &[LogEntry],
    level_filter: Option<LogLevel>,
    keyword: &str,
) -> Vec<LogEntry> {
    entries
        .iter()
        .filter(|entry| {
            // 레벨 필터 적용
            if let Some(level) = level_filter {
                if entry.level != level {
                    return false;
                }
            }

            // 키워드 필터 적용
            if !keyword.is_empty() {
                let kw = keyword.to_lowercase();
                let in_raw = entry.raw_line.to_lowercase().contains(&kw);
                let in_msg = entry.message.to_lowercase().contains(&kw);
                if !in_raw && !in_msg {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}