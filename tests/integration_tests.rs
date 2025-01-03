use coral::{
    analyzers::{Analyzer, AuthenticityAnalyzer, WalletAnalyzer, DeveloperAnalyzer},
    core::{CoralAgent, types::*},
    traders::JupiterTrader,
    utils::{MetricsCollector, SolanaUtils},
};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Mock RPC responses
mod mock {
    use super::*;
    use mock_it::Mock;

    pub struct MockRpcClient {
        mock: Mock,
    }

    impl MockRpcClient {
        pub fn new() -> Self {
            let mut mock = Mock::new();
            mock.when("get_token_account_balance")
                .then_return(Ok(1000_u64));
            Self { mock }
        }
    }
}

#[tokio::test]
async fn test_token_analysis() {
    // Setup test environment
    let test_token = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        .expect("Failed to parse token address");
    
    let agent = CoralAgent::new(
        "https://api.testnet.solana.com".to_string(),
        None,
    );

    // Perform analysis
    let result = agent.analyze_token(&test_token).await;
    
    assert!(result.is_ok());
    let analysis = result.unwrap();
    
    // Verify analysis results
    assert!(analysis.authenticity_score >= 0.0 && analysis.authenticity_score <= 1.0);
    assert!(matches!(analysis.risk_level, RiskLevel::Low | RiskLevel::Medium | RiskLevel::High));
}

#[tokio::test]
async fn test_trade_execution() {
    let test_token = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        .expect("Failed to parse token address");
    
    let trader = JupiterTrader::new(
        "test_api_key",
        Default::default(),
    );

    let order = TradeOrder {
        from_token: test_token,
        to_token: Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
            .expect("Failed to parse USDC address"),
        amount: 1000000,
        slippage_tolerance: 0.01,
        minimum_received: None,
        route_preference: RoutePreference::BestPrice,
    };

    let result = trader.execute(order).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_metrics_collection() {
    let metrics = MetricsCollector::new();
    
    // Record some test metrics
    metrics.increment_trade_count();
    metrics.increment_analysis_count();
    
    metrics.record_analysis_time(100).await;
    
    let trade_data = TradeExecutionData {
        token: "TEST".to_string(),
        timestamp: chrono::Utc::now(),
        amount: 1000.0,
        price: 1.5,
        success: true,
        profit_loss: 50.0,
        execution_time_ms: 150,
    };
    
    metrics.record_trade_execution(trade_data).await;
    
    // Verify metrics
    let summary = metrics.get_metrics_summary().await;
    assert_eq!(summary.total_trades, 1);
    assert_eq!(summary.total_analyses, 1);
    assert!(summary.success_rate > 0.0);
}

#[tokio::test]
async fn test_error_handling() {
    let invalid_token = Pubkey::from_str("InvalidTokenAddress")
        .expect_err("Should fail with invalid address");
    
    let agent = CoralAgent::new(
        "https://api.testnet.solana.com".to_string(),
        None,
    );

    let result = agent.analyze_token(&Pubkey::new_unique()).await;
    assert!(matches!(result, Err(CoralError::AnalysisFailed(_))));
}
