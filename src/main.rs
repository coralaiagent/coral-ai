use coral::{
    core::{CoralAgent, types::*},
    utils::{CoralLogger, MetricsCollector},
};
use log::Level;
use std::sync::Arc;
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() -> Result<(), CoralError> {
    // Initialize logger
    CoralLogger::init(Level::Info);

    // Initialize metrics collector
    let metrics = Arc::new(MetricsCollector::new());

    // Create Coral agent
    let agent = CoralAgent::new(
        std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
        Some(metrics.clone()),
    );

    // Example token address (USDC on Solana mainnet)
    let token_address = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
        .expect("Failed to parse token address");

    // Analyze token
    log::info!("Starting analysis for token: {}", token_address);
    let analysis = agent.analyze_token(&token_address).await?;

    // Print analysis results
    log::info!("Analysis complete:");
    log::info!("Authenticity Score: {}", analysis.authenticity_score);
    log::info!("Risk Level: {:?}", analysis.risk_level);
    log::info!("Recommendation: {:?}", analysis.recommendation);

    // Get metrics summary
    let metrics_summary = metrics.get_metrics_summary().await;
    log::info!("Metrics Summary: {:?}", metrics_summary);

    Ok(())
}
