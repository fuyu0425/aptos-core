use super::{Runner, RunnerError};
use crate::metric_collector::MetricCollector;
use anyhow::{Context, Result};
use async_trait::async_trait;
use log::debug;
use prometheus_parse::Scrape as PrometheusScrape;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct BlockingRunner<M: MetricCollector> {
    baseline_retriever: M,
    metrics_fetch_delay: Duration,
}

impl<M: MetricCollector> BlockingRunner<M> {
    pub fn new(baseline_retriever: M, metrics_fetch_delay: Duration) -> Self {
        Self {
            baseline_retriever,
            metrics_fetch_delay,
        }
    }

    fn parse_response(&self, lines: Vec<String>) -> Result<PrometheusScrape, RunnerError> {
        PrometheusScrape::parse(lines.into_iter().map(|s| Ok(s)))
            .context("Failed to parse metrics response")
            .map_err(|e| RunnerError::ParseMetricsError(e))
    }
}

// todo, we need to collect the target metrics first and then collect the baseline metrics
// because we need to know what kind of node we're talking to. To this end, the metric
// collector should probably take in a map of all the baseline retrievers. There needs to
// be a key construction function, probs just network+node_type.

#[async_trait]
impl<M: MetricCollector> Runner for BlockingRunner<M> {
    async fn run<T: MetricCollector>(&self, target_retriever: T) -> Result<(), RunnerError> {
        debug!("Collecting first round of baseline metrics");
        let first_baseline_metrics = self
            .baseline_retriever
            .collect_metrics()
            .await
            .map_err(|e| RunnerError::MetricCollectorError(e))?;

        debug!("Collecting first round of target metrics");
        let first_target_metrics = self
            .baseline_retriever
            .collect_metrics()
            .await
            .map_err(|e| RunnerError::MetricCollectorError(e))?;

        let first_baseline_metrics = self.parse_response(first_baseline_metrics)?;
        let first_target_metrics = self.parse_response(first_target_metrics)?;

        tokio::time::sleep(self.metrics_fetch_delay).await;

        debug!("Collecting second round of baseline metrics");
        let second_baseline_metrics = self
            .baseline_retriever
            .collect_metrics()
            .await
            .map_err(|e| RunnerError::MetricCollectorError(e))?;

        debug!("Collecting second round of target metrics");
        let second_target_metrics = self
            .baseline_retriever
            .collect_metrics()
            .await
            .map_err(|e| RunnerError::MetricCollectorError(e))?;

        let second_baseline_metrics = self.parse_response(second_baseline_metrics)?;
        let second_target_metrics = self.parse_response(second_target_metrics)?;

        Ok(())
    }
}
