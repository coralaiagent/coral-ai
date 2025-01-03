use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Performance and operational metrics tracker
pub struct MetricsCollector {
    trade_count: AtomicU64,
    analysis_count: AtomicU64,
    error_count: AtomicU64,
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    trade_metrics: Arc<RwLock<TradeMetrics>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            trade_count: AtomicU64::new(0),
            analysis_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            performance_metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            trade_metrics: Arc::new(RwLock::new(TradeMetrics::default())),
        }
    }

    pub fn increment_trade_count(&self) {
        self.trade_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_analysis_count(&self) {
        self.analysis_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::SeqCst);
    }

    pub async fn record_analysis_time(&self, duration_ms: u64) {
        let mut metrics = self.performance_metrics.write().await;
        metrics.record_analysis_time(duration_ms);
    }

    pub async fn record_trade_execution(&self, trade_data: TradeExecutionData) {
        let mut metrics = self.trade_metrics.write().await;
        metrics.record_trade(trade_data);
    }

    pub async fn get_metrics_summary(&self) -> MetricsSummary {
        let performance = self.performance_metrics.read().await;
        let trades = self.trade_metrics.read().await;

        MetricsSummary {
            total_trades: self.trade_count.load(Ordering::SeqCst),
            total_analyses: self.analysis_count.load(Ordering::SeqCst),
            total_errors: self.error_count.load(Ordering::SeqCst),
            average_analysis_time: performance.average_analysis_time(),
            success_rate: trades.calculate_success_rate(),
            profit_loss: trades.calculate_total_pnl(),
        }
    }
}

#[derive(Default)]
struct PerformanceMetrics {
    analysis_times: Vec<u64>,
    execution_times: Vec<u64>,
    last_update: DateTime<Utc>,
}

impl PerformanceMetrics {
    fn record_analysis_time(&mut self, duration_ms: u64) {
        self.analysis_times.push(duration_ms);
        self.last_update = Utc::now();
    }

    fn average_analysis_time(&self) -> f64 {
        if self.analysis_times.is_empty() {
            return 0.0;
        }
        self.analysis_times.iter().sum::<u64>() as f64 / self.analysis_times.len() as f64
    }
}

#[derive(Default)]
struct TradeMetrics {
    trades: Vec<TradeExecutionData>,
    token_performance: HashMap<String, TokenPerformance>,
}

impl TradeMetrics {
    fn record_trade(&mut self, trade: TradeExecutionData) {
        self.update_token_performance(&trade);
        self.trades.push(trade);
    }

    fn calculate_success_rate(&self) -> f64 {
        if self.trades.is_empty() {
            return 0.0;
        }
        let successful = self.trades.iter().filter(|t| t.success).count();
        successful as f64 / self.trades.len() as f64
    }

    fn calculate_total_pnl(&self) -> f64 {
        self.trades.iter().map(|t| t.profit_loss).sum()
    }

    fn update_token_performance(&mut self, trade: &TradeExecutionData) {
        let entry = self.token_performance
            .entry(trade.token.clone())
            .or_insert_with(TokenPerformance::default);
        
        entry.update(trade);
    }
}

#[derive(Debug)]
pub struct TradeExecutionData {
    pub token: String,
    pub timestamp: DateTime<Utc>,
    pub amount: f64,
    pub price: f64,
    pub success: bool,
    pub profit_loss: f64,
    pub execution_time_ms: u64,
}

#[derive(Debug, Default)]
struct TokenPerformance {
    total_trades: u64,
    successful_trades: u64,
    total_volume: f64,
    total_profit_loss: f64,
    best_trade_profit: f64,
    worst_trade_loss: f64,
}

impl TokenPerformance {
    fn update(&mut self, trade: &TradeExecutionData) {
        self.total_trades
