//! Trading execution modules
//! Provides interfaces and implementations for executing trades across different DEXs

use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::core::types::{CoralError, TradeResult};

#[async_trait]
pub trait Trader {
    async fn execute(&self, order: TradeOrder) -> Result<TradeResult, CoralError>;
    async fn get_price_quote(&self, token: &Pubkey, amount: u64) -> Result<PriceQuote, CoralError>;
    async fn get_available_routes(&self, from: &Pubkey, to: &Pubkey) -> Result<Vec<TradeRoute>, CoralError>;
}

#[derive(Debug, Clone)]
pub struct TradeOrder {
    pub from_token: Pubkey,
    pub to_token: Pubkey,
    pub amount: u64,
    pub slippage_tolerance: f64,
    pub minimum_received: Option<u64>,
    pub route_preference: RoutePreference,
}

#[derive(Debug, Clone)]
pub struct PriceQuote {
    pub input_amount: u64,
    pub output_amount: u64,
    pub price_impact: f64,
    pub fee_amount: u64,
    pub route: TradeRoute,
}

#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub hops: Vec<RouteHop>,
    pub total_fee: u64,
    pub expected_output: u64,
    pub price_impact: f64,
}

#[derive(Debug, Clone)]
pub struct RouteHop {
    pub dex: DexType,
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub pool_address: Pubkey,
    pub fee_tier: u32,
}

#[derive(Debug, Clone)]
pub enum RoutePreference {
    BestPrice,
    MinimumHops,
    PreferredDex(DexType),
    Custom(Box<dyn RouteStrategy>),
}

#[derive(Debug, Clone)]
pub enum DexType {
    Jupiter,
    Orca,
    Raydium,
    Serum,
    Custom(String),
}

#[async_trait]
pub trait RouteStrategy: Send + Sync {
    async fn evaluate_route(&self, route: &TradeRoute) -> f64;
}
