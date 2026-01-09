use {
    async_trait::async_trait,
    carbon_core::{error::CarbonResult, metrics::Metrics},
    std::{collections::HashMap, time::Instant},
    tokio::sync::RwLock,
};

pub struct LogMetrics {
    pub updates_received: RwLock<u64>,
    pub updates_processed: RwLock<u64>,
    pub updates_successful: RwLock<u64>,
    pub updates_failed: RwLock<u64>,
    pub updates_queued: RwLock<u64>,
    pub updates_processing_times: RwLock<Vec<u64>>,

    pub account_updates_processed: RwLock<u64>,
    pub transaction_updates_processed: RwLock<u64>,
    pub account_deletions_processed: RwLock<u64>,

    pub counters: RwLock<HashMap<String, u64>>,
    pub gauges: RwLock<HashMap<String, f64>>,
    pub histograms: RwLock<HashMap<String, Vec<f64>>>,

    pub start: RwLock<Instant>,
    pub last_flush: RwLock<Instant>,
}

impl Default for LogMetrics {
    fn default() -> Self {
        Self {
            updates_received: RwLock::new(0),
            updates_processed: RwLock::new(0),
            updates_successful: RwLock::new(0),
            updates_failed: RwLock::new(0),
            updates_queued: RwLock::new(0),
            updates_processing_times: RwLock::new(Vec::new()),
            account_updates_processed: RwLock::new(0),
            transaction_updates_processed: RwLock::new(0),
            account_deletions_processed: RwLock::new(0),
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
            start: RwLock::new(Instant::now()),
            last_flush: RwLock::new(Instant::now()),
        }
    }
}

impl LogMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Metrics for LogMetrics {
    async fn initialize(&self) -> CarbonResult<()> {
        let mut start = self.start.write().await;
        *start = Instant::now();

        let mut last_flush = self.last_flush.write().await;
        *last_flush = *start;

        Ok(())
    }

    async fn flush(&self) -> CarbonResult<()> {
        self.histograms.write().await.clear();
        self.updates_processing_times.write().await.clear();
        *self.last_flush.write().await = Instant::now();
        Ok(())
    }

    async fn shutdown(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()> {
        match name {
            "updates_received" => {
                let mut updates_received = self.updates_received.write().await;
                *updates_received += value;
            }
            "updates_processed" => {
                let mut updates_processed = self.updates_processed.write().await;
                *updates_processed += value;
            }
            "updates_successful" => {
                let mut updates_successful = self.updates_successful.write().await;
                *updates_successful += value;
            }
            "updates_failed" => {
                let mut updates_failed = self.updates_failed.write().await;
                *updates_failed += value;
            }
            "account_updates_processed" => {
                let mut account_updates_processed = self.account_updates_processed.write().await;
                *account_updates_processed += value;
            }
            "transaction_updates_processed" => {
                let mut transaction_updates_processed =
                    self.transaction_updates_processed.write().await;
                *transaction_updates_processed += value;
            }
            "account_deletions_processed" => {
                let mut account_deletions_processed =
                    self.account_deletions_processed.write().await;
                *account_deletions_processed += value;
            }
            _ => {
                let mut counters = self.counters.write().await;
                let counter = counters.entry(name.to_string()).or_insert(0);
                *counter += value;
            }
        };

        Ok(())
    }

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()> {
        match name {
            "updates_queued" => {
                let mut updates_queued = self.updates_queued.write().await;
                *updates_queued = value as u64;
            }
            _ => {
                let mut gauges = self.gauges.write().await;
                let gauge = gauges.entry(name.to_string()).or_insert(0.0);
                *gauge = value;
            }
        };

        Ok(())
    }

    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()> {
        match name {
            "updates_processing_times" => {
                let mut updates_processing_times = self.updates_processing_times.write().await;
                updates_processing_times.push(value as u64);
            }
            _ => {
                let mut histograms = self.histograms.write().await;
                let histogram = histograms.entry(name.to_string()).or_insert(Vec::new());
                histogram.push(value);
            }
        };

        Ok(())
    }

    async fn get_gauge(&self, name: &str) -> Option<f64> {
        match name {
            "updates_queued" => Some(*self.updates_queued.read().await as f64),
            _ => self.gauges.read().await.get(name).copied(),
        }
    }

    async fn get_gauges_by_prefix(&self, prefix: &str) -> HashMap<String, f64> {
        self.gauges
            .read()
            .await
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }
}
