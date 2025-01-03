use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::core::types::{AnalysisResult, CoralError, TradeResult};

#[async_trait]
pub trait Agent {
    async fn analyze_token(&self, token: &Pubkey) -> Result<AnalysisResult, CoralError>;
    async fn execute_trade(&self, order: TradeOrder) -> Result<TradeResult, CoralError>;
    async fn monitor_position(&self, position: Position) -> Result<(), CoralError>;
}

pub struct CoralAgent {
    config: AgentConfig,
    analyzers: Vec<Box<dyn Analyzer>>,
    trader: Box<dyn Trader>,
}

impl CoralAgent {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            analyzers: vec![
                Box::new(AuthenticityAnalyzer::new()),
                Box::new(WalletAnalyzer::new()),
                Box::new(DeveloperAnalyzer::new()),
            ],
            trader: Box::new(JupiterTrader::new()),
        }
    }

    async fn aggregate_analysis(&self, token: &Pubkey) -> Result<AnalysisResult, CoralError> {
        let mut results = Vec::new();
        
        for analyzer in &self.analyzers {
            let result = analyzer.analyze(token).await?;
            results.push(result);
        }

        self.combine_analysis_results(results)
    }
}

#[async_trait]
impl Agent for CoralAgent {
    async fn analyze_token(&self, token: &Pubkey) -> Result<AnalysisResult, CoralError> {
        self.aggregate_analysis(token).await
    }

    async fn execute_trade(&self, order: TradeOrder) -> Result<TradeResult, CoralError> {
        self.trader.execute(order).await
    }

    async fn monitor_position(&self, position: Position) -> Result<(), CoralError> {
        // Implementation for position monitoring
        todo!()
    }
}
