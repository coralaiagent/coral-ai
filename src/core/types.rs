use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub token_address: Pubkey,
    pub authenticity_score: f64,
    pub risk_level: RiskLevel,
    pub recommendation: TradeRecommendation,
    pub analysis_timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeRecommendation {
    Buy {
        confidence: f64,
        suggested_amount: u64,
        entry_price: f64,
        stop_loss: f64,
        take_profit: f64,
    },
    Sell {
        confidence: f64,
        percentage: f64,
        reason: String,
    },
    Hold {
        duration: String,
        reevaluation_price: f64,
    },
    Avoid {
        reasons: Vec<String>,
        risk_factors: Vec<String>,
    },
}

#[derive(Error, Debug)]
pub enum CoralError {
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("Trading execution failed: {0}")]
    TradingFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub transaction_signature: String,
    pub executed_price: f64,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: i64,
}
