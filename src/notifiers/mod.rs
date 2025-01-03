use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn send_notification(&self, message: &NotificationMessage) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub title: String,
    pub content: String,
    pub priority: NotificationPriority,
    pub category: NotificationCategory,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationCategory {
    TokenAnalysis,
    TradingSignal,
    RiskAlert,
    PriceMovement,
    SystemStatus,
}

pub mod telegram;
pub mod twitter;
