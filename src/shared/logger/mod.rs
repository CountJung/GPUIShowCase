use std::{
    fs, io,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use tracing::{debug, error, info, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    Registry, filter::LevelFilter, fmt, layer::SubscriberExt, reload, util::SubscriberInitExt,
};

type FilterHandle = reload::Handle<LevelFilter, Registry>;

static FILTER_HANDLE: OnceLock<Mutex<Option<FilterHandle>>> = OnceLock::new();
static LOG_GUARD: OnceLock<Mutex<Option<WorkerGuard>>> = OnceLock::new();

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppLogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
}

impl AppLogLevel {
    pub const ALL: [Self; 4] = [Self::Error, Self::Warn, Self::Info, Self::Debug];

    pub fn title(self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warn => "Warn",
            Self::Info => "Info",
            Self::Debug => "Debug",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Error => "치명적 오류만 파일에 기록합니다.",
            Self::Warn => "경고와 오류를 파일에 기록합니다.",
            Self::Info => "일반 사용자 이벤트와 상태 전이를 파일에 기록합니다.",
            Self::Debug => "상세 진단 이벤트까지 파일에 기록합니다.",
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    fn as_level_filter(self) -> LevelFilter {
        match self {
            Self::Error => LevelFilter::ERROR,
            Self::Warn => LevelFilter::WARN,
            Self::Info => LevelFilter::INFO,
            Self::Debug => LevelFilter::DEBUG,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerRuntime {
    pub log_directory: String,
    pub rolling_strategy: String,
}

pub fn init_logging(initial_level: AppLogLevel) -> io::Result<LoggerRuntime> {
    let log_directory = PathBuf::from("logs");
    fs::create_dir_all(&log_directory)?;

    let file_appender = tracing_appender::rolling::daily(&log_directory, "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let (filter_layer, handle) = reload::Layer::new(initial_level.as_level_filter());

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_writer(non_blocking),
        )
        .try_init()
        .map_err(|error| io::Error::other(error.to_string()))?;

    FILTER_HANDLE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .expect("filter handle mutex poisoned")
        .replace(handle);
    LOG_GUARD
        .get_or_init(|| Mutex::new(None))
        .lock()
        .expect("log guard mutex poisoned")
        .replace(guard);

    emit_event(
        AppLogLevel::Info,
        "app.lifecycle",
        format!(
            "logging initialized: dir={} strategy=daily-app.log",
            log_directory.display()
        ),
    );

    Ok(LoggerRuntime {
        log_directory: log_directory.display().to_string(),
        rolling_strategy: "daily app.log rotation".to_string(),
    })
}

pub fn set_log_level(level: AppLogLevel) {
    if let Some(mutex) = FILTER_HANDLE.get()
        && let Some(handle) = mutex.lock().expect("filter handle mutex poisoned").as_mut()
    {
        let _ = handle.modify(|current| *current = level.as_level_filter());
    }
}

pub fn emit_event(level: AppLogLevel, scope: &str, message: impl AsRef<str>) {
    let message = message.as_ref();

    match level {
        AppLogLevel::Error => error!(scope = scope, message = %message),
        AppLogLevel::Warn => warn!(scope = scope, message = %message),
        AppLogLevel::Info => info!(scope = scope, message = %message),
        AppLogLevel::Debug => debug!(scope = scope, message = %message),
    }
}
