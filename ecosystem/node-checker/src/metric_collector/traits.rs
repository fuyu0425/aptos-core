// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Error, Result};
use async_trait::async_trait;
use std::fmt::{Display, Formatter, Result as FmtResult};

// TODO: Consider using thiserror.

#[derive(Debug)]
pub enum MetricCollectorError {
    /// We were unable to get data from the node.
    GetDataError(Error),

    /// We could perform basic parsing on the response.
    ResponseParseError(Error),
}

impl Display for MetricCollectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MetricCollectorError {}

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
pub trait MetricCollector: Sync + Send + 'static {
    async fn collect_metrics(&self) -> Result<Vec<String>, MetricCollectorError>;
}
