use super::{Runner, RunnerError};
use crate::metric_collector::MetricCollector;
use async_trait::async_trait;
use log::debug;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct BlockingRunner<M: MetricCollector> {
    baseline_retriever: M,
    target_retriever: M,
    metrics_fetch_delay: Duration,
}

impl<M: MetricCollector> BlockingRunner<M> {
    pub fn new(baseline_retriever: M, target_retriever: M, metrics_fetch_delay: Duration) -> Self {
        Self {
            baseline_retriever,
            target_retriever,
            metrics_fetch_delay,
        }
    }
}

#[async_trait]

impl<M: MetricCollector> Runner for BlockingRunner<M> {
    async fn run(&self) -> Result<(), RunnerError> {
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

        Ok(())
    }
}
