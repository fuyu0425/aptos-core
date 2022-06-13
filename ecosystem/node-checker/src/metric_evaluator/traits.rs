use anyhow::{Error, Result};
use async_trait::async_trait;
use prometheus_parse::Scrape as PrometheusScrape;
use std::fmt::{Display, Formatter, Result as FmtResult};

// TODO: Consider using thiserror.

#[derive(Debug)]
pub enum MetricsEvaluatorError {
    /// The metric we're evaluating is missing from the baseline. Args:
    ///   - The metric name.
    ///   - Explanation.
    /// When the target node is missing a metric, we return an Evaluation
    /// indiating that something is wrong with the target node, but if the
    /// baseline node is missing a metric, it implies that something is wrong
    /// without our node checker configuration, so we return an error here.
    MissingBaselineMetric(String, String),

    UnknownError(Error),
}

impl Display for MetricsEvaluatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MetricsEvaluatorError {}

// TODO: Should I find a way to have typed actual + expected fields?
#[derive(Clone, Debug)]
pub struct Evaluation {
    /// Headline of the evaluation, e.g. "Healthy!" or "Metrics missing!".
    pub headline: String,

    /// Score out of 100.
    pub score: u8,

    /// Explanation of the evaluation.
    pub explanation: String,
}

/// todo describe the trait
/// todo assert these trait constraints are necessary
///
/// This is only for metrics evaluation, we will need a different
/// more permissive trait for other evaluation types. ideally we will still be able
/// to return Evaluation from those too (in which case we lift that type up), but
/// if not, we can use a trait instead.
///
/// Note:
///  - Clone is required because multiple calls to spawn need to be static but also share
///      the same todo instance (mostly for the in-memory versions).
///
///  - Sync + Send is required because this will be a member of the todo which needs
///      to be used across async boundaries
///
///  - 'static is required because this will be stored on the todo which needs to be 'static
#[async_trait]
pub trait MetricsEvaluator: Clone + Sync + Send + 'static {
    fn evaluate_metrics(
        &self,
        previous_baseline_metrics: &PrometheusScrape,
        previous_target_metrics: &PrometheusScrape,
        latest_baseline_metrics: &PrometheusScrape,
        latest_target_metrics: &PrometheusScrape,
    ) -> Result<Vec<Evaluation>, MetricsEvaluatorError>;

    /// todo
    fn get_name() -> String;
}
