//! Analysis modules for token evaluation
//! Includes authenticity, wallet behavior, developer activity, and market analysis

pub mod authenticity;
pub mod wallet;
pub mod developer;
pub mod market;

use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::core::types::{AnalysisResult, CoralError};

#[async_trait]
pub trait Analyzer {
    async fn analyze(&self, token: &Pubkey) -> Result<AnalysisResult, CoralError>;
}

/// Common analysis metrics and thresholds
pub struct AnalysisMetrics {
    pub confidence_score: f64,
    pub risk_score: f64,
    pub reliability_score: f64,
}

impl AnalysisMetrics {
    pub fn calculate_weighted_score(&self, weights: &AnalysisWeights) -> f64 {
        self.confidence_score * weights.confidence
            + self.risk_score * weights.risk
            + self.reliability_score * weights.reliability
    }
}

pub struct AnalysisWeights {
    pub confidence: f64,
    pub risk: f64,
    pub reliability: f64,
}

impl Default for AnalysisWeights {
    fn default() -> Self {
        Self {
            confidence: 0.4,
            risk: 0.4,
            reliability: 0.2,
        }
    }
}
