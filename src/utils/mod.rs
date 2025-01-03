//! Utility functions and helpers for the Coral trading agent

pub mod solana;
pub mod metrics;
pub mod logger;

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn parse_pubkey(key: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(key).map_err(|e| e.to_string())
}

pub fn format_amount(amount: u64, decimals: u8) -> String {
    let amount_f64 = amount as f64 / 10_f64.powi(decimals as i32);
    format!("{:.precision$}", amount_f64, precision = decimals as usize)
}

pub fn calculate_percentage_change(old_value: f64, new_value: f64) -> f64 {
    ((new_value - old_value) / old_value) * 100.0
}

pub fn format_duration(seconds: i64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    
    match (days, hours, minutes) {
        (0, 0, m) => format!("{}m", m),
        (0, h, m) => format!("{}h {}m", h, m),
        (d, h, _) => format!("{}d {}h", d, h),
    }
}
