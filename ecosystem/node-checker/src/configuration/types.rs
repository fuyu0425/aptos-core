// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{metric_evaluator::StateSyncMetricsEvaluatorArgs, runner::BlockingRunnerArgs};
use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;
use poem_openapi::{types::Example, Object as PoemObject};
use serde::{Deserialize, Serialize};
use url::Url;

pub const DEFAULT_METRICS_PORT: u16 = 9101;
pub const DEFAULT_API_PORT: u16 = 8080;
pub const DEFAULT_NOISE_PORT: u16 = 6180;

pub static DEFAULT_METRICS_PORT_STR: Lazy<String> =
    Lazy::new(|| format!("{}", DEFAULT_METRICS_PORT));
pub static DEFAULT_API_PORT_STR: Lazy<String> = Lazy::new(|| format!("{}", DEFAULT_API_PORT));
pub static DEFAULT_NOISE_PORT_STR: Lazy<String> = Lazy::new(|| format!("{}", DEFAULT_NOISE_PORT));

// Todo write about why these structs derive a billion different things:
// - clap: to allow users to generate configs easily using nhc configuration create
// - serde: so we can read / write configs from / to disk
// - poemobject: so we can return the configuration over the API

#[derive(Clone, Debug, Deserialize, Parser, PoemObject, Serialize)]
#[clap(author, version, about, long_about = None)]
pub struct NodeConfiguration {
    #[clap(flatten)]
    pub node_address: NodeAddress,

    /// This is the name we will show for this configuration to users.
    /// For example, if someone opens the NHC frontend, they will see this name
    /// in a dropdown list of configurations they can test their node against.
    /// e.g. "Devnet Full Node", "Testnet Validator Node", etc.
    #[clap(long)]
    pub configuration_name: String,

    /// The chain ID we expect to find when we speak to the node.
    /// If not given, we will just assume the value we find is correct.
    /// If given, we will check that the value is correct, exiting if not.
    chain_id: Option<u16>,

    /// The role type we expect to find when we speak to the node.
    /// If not given, we will just assume the value we find is correct.
    /// If given, we will check that the value is correct, exiting if not.
    /// e.g. "full_node", "validator_node", etc.
    role_type: Option<String>,

    /// The (metric) evaluators to use, e.g. state_sync, api, etc.
    #[clap(long, min_values = 1, use_value_delimiter = true)]
    pub evaluators: Vec<String>,

    #[clap(flatten)]
    pub evaluator_args: EvaluatorArgs,

    #[clap(flatten)]
    pub runner_args: RunnerArgs,
}

impl NodeConfiguration {
    /// Only call this after fetch_additional_configuration has been called.
    pub fn get_chain_id(&self) -> u16 {
        self.chain_id
            .expect("get_chain_id called before fetch_additional_configuration")
    }

    /// Only call this after fetch_additional_configuration has been called.
    pub fn get_role_type(&self) -> &str {
        self.role_type
            .as_ref()
            .expect("get_role_type called before fetch_additional_configuration")
    }

    /// In this function we fetch the chain ID and role type from the node.
    /// If chain_id and role_type are already set, we validate that the values
    /// match up. If they're not set, we set them using the values we find.
    pub async fn fetch_additional_configuration(&mut self) -> Result<()> {
        // TODO: Dummy code while I wait for Josh to implement the /configuration endpoint.
        self.chain_id = Some(16);
        self.role_type = Some("full_node".to_string());
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Parser, PoemObject, Serialize)]
pub struct EvaluatorArgs {
    #[clap(flatten)]
    pub state_sync_evaluator_args: StateSyncMetricsEvaluatorArgs,
}

#[derive(Clone, Debug, Deserialize, Parser, PoemObject, Serialize)]
pub struct RunnerArgs {
    #[clap(flatten)]
    pub blocking_runner_args: BlockingRunnerArgs,
}

/*
/// The URL of the baseline node, e.g. http://fullnode.devnet.aptoslabs.com
/// We assume this is just the base and will add ports and paths to this.
#[clap(long)]
pub baseline_node_url: Url,

/// The metrics port for the baseline node.
#[clap(long, default_value = &DEFAULT_METRICS_PORT_STR)]
pub baseline_metrics_port: u16,

/// The API port for the baseline node.
#[clap(long, default_value = &DEFAULT_API_PORT_STR)]
pub baseline_api_port: u16,

/// The port over which validator nodes can talk to the baseline node.
#[clap(long, default_value = &DEFAULT_NOISE_PORT_STR)]
pub baseline_noise_port: u16,

/// If this is given, the user will be able to call the check_preconfigured_node
/// endpoint, which takes no target, instead using this as the target. If
/// allow_test_node_only is set, only the todo endpoint will work,
/// the node will not respond to health check requests for other nodes.
#[clap(long)]
pub target_node_url: Option<Url>,

// The following 3 arguments are only relevant if the user sets test_node_url.
/// The metrics port for the target node.
#[clap(long, default_value = &DEFAULT_METRICS_PORT_STR)]
pub target_metrics_port: u16,

/// The API port for the target node.
#[clap(long, default_value = &DEFAULT_API_PORT_STR)]
pub target_api_port: u16,

/// The port over which validator nodes can talk to the target node.
#[clap(long, default_value = &DEFAULT_NOISE_PORT_STR)]
pub target_noise_port: u16,

/// See the help message of target_node_url.
#[clap(long)]
pub allow_preconfigured_test_node_only: bool,
*/

#[derive(Clone, Debug, Deserialize, Parser, PoemObject, Serialize)]
#[oai(example)]
pub struct NodeAddress {
    /// Target URL. This should include a scheme (e.g. http://). If there is
    /// no scheme, we will prepend http://.
    pub url: Url,

    /// Metrics port.
    #[oai(default = "Self::default_metrics_port")]
    pub metrics_port: u16,

    /// API port.
    #[oai(default = "Self::default_api_port")]
    pub api_port: u16,

    /// Validator communication port.
    #[oai(default = "Self::default_noise_port")]
    pub noise_port: u16,
}

impl NodeAddress {
    fn default_metrics_port() -> u16 {
        DEFAULT_METRICS_PORT
    }

    fn default_api_port() -> u16 {
        DEFAULT_API_PORT
    }

    fn default_noise_port() -> u16 {
        DEFAULT_NOISE_PORT
    }
}

impl Example for NodeAddress {
    fn example() -> Self {
        Self {
            url: Url::parse("http://mynode.mysite.com").unwrap(),
            metrics_port: Self::default_metrics_port(),
            api_port: Self::default_api_port(),
            noise_port: Self::default_noise_port(),
        }
    }
}
