use std::sync::Once;
use log::{Level, LevelFilter, Metadata, Record};
use chrono::Local;

static INIT: Once = Once::new();

pub struct CoralLogger {
    level: Level,
}

impl CoralLogger {
    pub fn init(level: Level) {
        INIT.call_once(|| {
            let logger = CoralLogger { level };
            log::set_boxed_logger(Box::new(logger))
                .map(|()| log::set_max_level(LevelFilter::Trace))
                .expect("Failed to initialize logger");
        });
    }

    fn format_log(&self, record: &Record) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level = record.level();
        let target = record.target();
        let args = record.args();

        match level {
            Level::Error => format!("[{} ERROR {} ] {}", timestamp, target, args),
            Level::Warn => format!("[{} WARN {} ] {}", timestamp, target, args),
            Level::Info => format!("[{} INFO {} ] {}", timestamp, target, args),
            Level::Debug => format!("[{} DEBUG {} ] {}", timestamp, target, args),
            Level::Trace => format!("[{} TRACE {} ] {}", timestamp, target, args),
        }
    }
}

impl log::Log for CoralLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_message = self.format_log(record);
            println!("{}", log_message);
        }
    }

    fn flush(&self) {}
}

#[derive(Debug)]
pub struct TradeLogger {
    trade_id: String,
    start_time: chrono::DateTime<chrono::Utc>,
}

impl TradeLogger {
    pub fn new(trade_id: String) -> Self {
        Self {
            trade_id,
            start_time: chrono::Utc::now(),
        }
    }

    pub fn log_trade_start(&self, details: &str) {
        log::info!(
            "Trade {} started at {} - {}",
            self.trade_id,
            self.start_time,
            details
        );
    }

    pub fn log_trade_step(&self, step: &str, details: &str) {
        log::debug!(
            "Trade {} - Step: {} - Details: {}",
            self.trade_id,
            step,
            details
        );
    }

    pub fn log_trade_completion(&self, success: bool, details: &str) {
        let duration = chrono::Utc::now() - self.start_time;
        if success {
            log::info!(
                "Trade {} completed successfully after {}ms - {}",
                self.trade_id,
                duration.num_milliseconds(),
                details
            );
        } else {
            log::error!(
                "Trade {} failed after {}ms - {}",
                self.trade_id,
                duration.num_milliseconds(),
                details
            );
        }
    }

    pub fn log_error(&self, error: &str) {
        log::error!("Trade {} error: {}", self.trade_id, error);
    }
}
