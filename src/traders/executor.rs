use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::core::types::{CoralError, TradeResult};
use super::{Trader, TradeOrder, TradeRoute};

pub struct TradeExecutor {
    traders: Vec<Box<dyn Trader>>,
    metrics: Arc<MetricsCollector>,
    logger: TradeLogger,
}

impl TradeExecutor {
    pub fn new(metrics: Arc<MetricsCollector>) -> Self {
        Self {
            traders: Vec::new(),
            metrics,
            logger: TradeLogger::new("trade_executor".to_string()),
        }
    }

    pub fn register_trader(&mut self, trader: Box<dyn Trader>) {
        self.traders.push(trader);
    }

    async fn find_best_execution_route(&self, order: &TradeOrder) -> Result<(Box<&dyn Trader>, TradeRoute), CoralError> {
        let mut best_route = None;
        let mut best_price = 0.0;
        let mut selected_trader = None;

        for trader in &self.traders {
            match trader.get_available_routes(&order.from_token, &order.to_token).await {
                Ok(routes) => {
                    for route in routes {
                        let quote = trader.get_price_quote(&order.from_token, order.amount).await?;
                        if quote.output_amount as f64 > best_price {
                            best_price = quote.output_amount as f64;
                            best_route = Some(route);
                            selected_trader = Some(trader);
                        }
                    }
                }
                Err(e) => {
                    self.logger.log_error(&format!("Route finding error: {}", e));
                    continue;
                }
            }
        }

        match (selected_trader, best_route) {
            (Some(trader), Some(route)) => Ok((Box::new(trader), route)),
            _ => Err(CoralError::TradingFailed("No valid trading route found".to_string())),
        }
    }

    pub async fn execute_trade(&self, order: TradeOrder) -> Result<TradeResult, CoralError> {
        self.logger.log_trade_start(&format!("Executing trade for {} tokens", order.amount));
        self.metrics.increment_trade_count();

        let start_time = std::time::Instant::now();

        // Find the best execution route
        let (trader, route) = self.find_best_execution_route(&order).await?;

        // Validate the route and order
        self.validate_execution(&order, &route)?;

        // Execute the trade
        let result = trader.execute(order).await;

        // Record metrics
        let execution_time = start_time.elapsed().as_millis() as u64;
        match &result {
            Ok(trade_result) => {
                self.metrics.record_trade_execution(TradeExecutionData {
                    token: order.from_token.to_string(),
                    timestamp: chrono::Utc::now(),
                    amount: order.amount as f64,
                    price: trade_result.executed_price,
                    success: true,
                    profit_loss: 0.0, // Calculate actual P&L
                    execution_time_ms: execution_time,
                }).await;
                self.logger.log_trade_completion(true, &format!(
                    "Trade executed successfully. Signature: {}",
                    trade_result.transaction_signature
                ));
            }
            Err(e) => {
                self.logger.log_trade_completion(false, &format!("Trade failed: {}", e));
                self.metrics.record_error();
            }
        }

        result
    }

    fn validate_execution(&self, order: &TradeOrder, route: &TradeRoute) -> Result<(), CoralError> {
        // Implement validation logic
        if route.price_impact > order.slippage_tolerance {
            return Err(CoralError::TradingFailed(
                format!("Price impact too high: {}", route.price_impact)
            ));
        }

        if let Some(min_received) = order.minimum_received {
            if route.expected_output < min_received {
                return Err(CoralError::TradingFailed(
                    "Expected output below minimum threshold".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::MetricsCollector;

    #[tokio::test]
    async fn test_trade_executor() {
        let metrics = Arc::new(MetricsCollector::new());
        let mut executor = TradeExecutor::new(metrics.clone());
        
        // Add mock trader
        executor.register_trader(Box::new(MockTrader::new()));

        let order = TradeOrder {
            from_token: Pubkey::new_unique(),
            to_token: Pubkey::new_unique(),
            amount: 1000000,
            slippage_tolerance: 0.01,
            minimum_received: None,
            route_preference: super::RoutePreference::BestPrice,
        };

        let result = executor.execute_trade(order).await;
        assert!(result.is_ok());
    }

    struct MockTrader;

    impl MockTrader {
        fn new() -> Self {
            Self
        }
    }

    #[async_trait]
    impl Trader for MockTrader {
        async fn execute(&self, _order: TradeOrder) -> Result<TradeResult, CoralError> {
            Ok(TradeResult {
                transaction_signature: "mock_signature".to_string(),
                executed_price: 1.0,
                amount: 1000000,
                fee: 1000,
                timestamp: chrono::Utc::now().timestamp(),
            })
        }

        async fn get_price_quote(&self, _token: &Pubkey, _amount: u64) -> Result<super::PriceQuote, CoralError> {
            Ok(super::PriceQuote {
                input_amount: 1000000,
                output_amount: 1000000,
                price_impact: 0.001,
                fee_amount: 1000,
                route: TradeRoute {
                    hops: vec![],
                    total_fee: 1000,
                    expected_output: 990000,
                    price_impact: 0.001,
                },
            })
        }

        async fn get_available_routes(&self, _from: &Pubkey, _to: &Pubkey) -> Result<Vec<TradeRoute>, CoralError> {
            Ok(vec![TradeRoute {
                hops: vec![],
                total_fee: 1000,
                expected_output: 990000,
                price_impact: 0.001,
            }])
        }
    }
}
