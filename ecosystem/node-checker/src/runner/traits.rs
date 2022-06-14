// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Error, Result};
use async_trait::async_trait;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    metric_collector::{MetricCollector, MetricCollectorError},
    metric_evaluator::MetricsEvaluatorError,
    public_types::CompleteEvaluation,
};

// TODO: Consider using thiserror.

#[derive(Debug)]
pub enum RunnerError {
    /// We failed to collect metrics for some reason.
    MetricCollectorError(MetricCollectorError),

    /// We couldn't parse the metrics.
    ParseMetricsError(Error),

    /// One of the evaluators failed. This is not the same as a poor score from
    /// an evaluator, this is an actual failure in the evaluation process.
    MetricEvaluatorError(MetricsEvaluatorError),

    UnknownError(Error),
}

impl Display for RunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RunnerError {}

// TODO: When we have the metric_evaluator, include a vec of those here,
// as well as an overall evaluation.
struct RunnerResult {}

// This runner doesn't block in the multithreading sense, but from the user
// perspective. To run the health check, we pull metrics once, wait, and then
// pull the metrics again. It does not support continually running beyond this
// point. You can imagine smarter versions of this where you store the last seen
// set of metrics, then compare against that, or perhaps even multiple previously
// seen sets of metrics and do more complex analysis. Additionally we could leverage
// things like long polling +/ sticky routing to make it that the client request
// doesn't just hang waiting for the run to complete.

/// todo describe the trait
/// todo assert these trait constraints are necessary
/// todo consider whether we need Clone if we need to spawn multiple handlers ourselves.
///
/// Note:
///  - Sync + Send is required because this will be a member of the todo which needs
///      to be used across async boundaries
///
///  - 'static is required because this will be stored on the todo which needs to be 'static
#[async_trait]
pub trait Runner: Sync + Send + 'static {
    // TODO: add proper result type.
    async fn run<M: MetricCollector>(
        &self,
        target_retriever: &M,
    ) -> Result<CompleteEvaluation, RunnerError>;
}
