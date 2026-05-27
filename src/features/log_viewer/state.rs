use crate::features::log_viewer::entry::{LogLevel, LogEntry};
use crate::features::log_viewer::parser;
use std::time::{SystemTime, UNIX_EPOCH};

/// 로그 뷰어의 상태 관리.
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogViewerState {
    /// 사용 가능한 날짜 목록.
    pub dates: Vec<String>,
    /// 선택된 날짜.
    pub selected_date: String,
    /// 파싱된 모든 로그 엔트리.
    pub all_entries: Vec<LogEntry>,
    /// 필터링된 로그 엔트리.
    pub filtered_entries: Vec<LogEntry>,
    /// 현재 선택된 레벨 필터.
    pub level_filter: Option<LogLevel>,
    /// 검색 키워드.
    pub search_keyword: String,
    /// 로딩 중 여부.
    pub is_loading: bool,
}

#[allow(dead_code)]
impl LogViewerState {
    pub fn new() -> Self {
        let dates = parser::list_log_dates().unwrap_or_default();
        let today = Self::get_today_str();

        Self {
            dates: dates.clone(),
            selected_date: today.clone(),
            all_entries: Vec::new(),
            filtered_entries: Vec::new(),
            level_filter: None,
            search_keyword: String::new(),
            is_loading: false,
        }
    }

    /// 선택된 날짜의 로그를 로드하고 파싱합니다.
    pub fn load_log(&mut self) {
        self.is_loading = true;

        if let Ok(lines) = parser::read_log_lines(&self.selected_date) {
            self.all_entries = parser::parse_log_lines(&lines, &self.selected_date);
            self.apply_filters();
        } else {
            self.all_entries.clear();
            self.filtered_entries.clear();
        }

        self.is_loading = false;
    }

    /// 레벨 필터를 설정하고 적용합니다.
    pub fn set_level_filter(&mut self, level: Option<LogLevel>) {
        self.level_filter = level;
        self.apply_filters();
    }

    /// 검색 키워드를 설정하고 적용합니다.
    pub fn set_search_keyword(&mut self, keyword: String) {
        self.search_keyword = keyword;
        self.apply_filters();
    }

    /// 현재 필터를 적용합니다.
    fn apply_filters(&mut self) {
        self.filtered_entries = parser::filter_entries(
            &self.all_entries,
            self.level_filter,
            &self.search_keyword,
        );
    }

    /// 날짜 목록을 새로고침합니다.
    pub fn refresh_dates(&mut self) {
        self.dates = parser::list_log_dates().unwrap_or_default();
    }

    /// 날짜를 선택하고 로그를 로드합니다.
    pub fn select_date(&mut self, date: String) {
        self.selected_date = date.clone();
        self.load_log();
    }

    /// 총 로그 수 (필터링 전).
    pub fn total_count(&self) -> usize {
        self.all_entries.len()
    }

    /// 필터링된 로그 수.
    pub fn filtered_count(&self) -> usize {
        self.filtered_entries.len()
    }

    /// 레벨별 카운트.
    pub fn count_by_level(&self, level: LogLevel) -> usize {
        self.all_entries.iter().filter(|e| e.level == level).count()
    }

    /// 현재 날짜를 YYYY-MM-DD 형식으로 반환합니다.
    fn get_today_str() -> String {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let total_seconds = duration.as_secs();
        
        // Unix epoch (1970-01-01)부터의 일을 계산
        let days_since_epoch = total_seconds / 86400;
        
        // 간단한 날짜 계산 (epoch부터 현재까지)
        Self::days_to_date(days_since_epoch as i64)
    }

    /// Unix epoch 이후 일수에서 YYYY-MM-DD 문자열로 변환
    fn days_to_date(days: i64) -> String {
        // 1970년 1월 1일을 기준으로 날짜 계산
        let mut year = 1970i64;
        let mut remaining_days = days;
        
        loop {
            let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
            if remaining_days < days_in_year {
                break;
            }
            remaining_days -= days_in_year;
            year += 1;
        }
        
        let month_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap = Self::is_leap_year(year);
        
        let mut month = 1;
        for i in 1..=12 {
            let days_in_month = if i == 2 && is_leap { 29 } else { month_days[i] };
            if remaining_days < days_in_month {
                month = i;
                break;
            }
            remaining_days -= days_in_month;
        }
        
        let day = remaining_days + 1;
        format!("{:04}-{:02}-{:02}", year, month, day)
    }

    /// 윤년인지 확인합니다.
    fn is_leap_year(year: i64) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}