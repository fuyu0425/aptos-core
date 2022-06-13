use std::net::{IpAddr, Ipv4Addr};

use super::traits::{MetricCollector, MetricCollectorError};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use log::debug;
use reqwest::Client as ReqwestClient;
use reqwest::Url;
use url::Host;

// TODO Make it possible to reject nodes unless they are a specific type.
#[derive(Clone, Debug)]
pub struct ReqwestMetricCollector {
    client: ReqwestClient,

    /// This should be an address that points at the metrics port of the node.
    /// It should not point to the /metrics endpoint, just the root, we will
    /// add the /metrics endpoint ourselves.
    node_url: Url,
}

impl ReqwestMetricCollector {
    pub fn new(node_url: Url) -> Self {
        let mut client_builder = ReqwestClient::builder();
        let mut is_localhost = false;
        if let Some(host) = node_url.host() {
            match host {
                Host::Domain(s) => {
                    if s.contains("localhost") {
                        is_localhost = true;
                    }
                }
                Host::Ipv4(ip) => {
                    if ip == Ipv4Addr::LOCALHOST {
                        is_localhost = true;
                    }
                }
                _ => {}
            }
            if is_localhost {
                client_builder = client_builder.local_address(IpAddr::from([127, 0, 0, 1]));
            }
        }
        ReqwestMetricCollector {
            client: client_builder.build().unwrap(),
            node_url,
        }
    }
}

#[async_trait]
impl MetricCollector for ReqwestMetricCollector {
    async fn collect_metrics(&self) -> Result<Vec<String>, MetricCollectorError> {
        debug!("Connecting to {} to collect metrics", self.node_url);
        let response = self
            .client
            .get(self.node_url.clone())
            .send()
            .await
            .with_context(|| format!("Failed to get data from {}", self.node_url))
            .map_err(|e| MetricCollectorError::GetDataError(anyhow!(e)))?;
        let body = response
            .text()
            .await
            .with_context(|| format!("Failed to process response body from {}", self.node_url))
            .map_err(|e| MetricCollectorError::ResponseParseError(anyhow!(e)))?;
        Ok(body.lines().map(|line| line.to_owned()).collect())
    }
}
