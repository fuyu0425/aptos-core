use crate::metric_evaluator::StateSyncMetricsEvaluatorArgs;
use crate::runner::BlockingRunnerArgs;
use clap::Parser;
use lazy_static::lazy_static;
use url::Url;

const DEFAULT_METRICS_PORT: u16 = 9101;
const DEFAULT_API_PORT: u16 = 8080;
const DEFAULT_NOISE_PORT: u16 = 6180;

lazy_static! {
    static ref DEFAULT_METRICS_PORT_STR: String = format!("{}", DEFAULT_METRICS_PORT);
    static ref DEFAULT_API_PORT_STR: String = format!("{}", DEFAULT_API_PORT);
    static ref DEFAULT_NOISE_PORT_STR: String = format!("{}", DEFAULT_NOISE_PORT);
}

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Whether to enable debug logging or not. This is a shortcut for the
    /// standard env logger configuration via env vars
    #[clap(short, long)]
    pub debug: bool,

    /// What address to listen on.
    #[clap(long, default_value = "0.0.0.0")]
    pub listen_address: String,

    /// What port to listen on.
    #[clap(long, default_value = "20121")]
    pub listen_port: u16,

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

    #[clap(flatten)]
    pub blocking_runner_args: BlockingRunnerArgs,

    /// The (metric) evaluators to use, e.g. state_sync, api, etc.
    #[clap(long, min_values = 1, use_value_delimiter = true)]
    pub evaluators: Vec<String>,

    #[clap(flatten)]
    pub evaluator_args: EvaluatorArgs,
}

#[derive(Clone, Debug, Parser)]
pub struct EvaluatorArgs {
    #[clap(flatten)]
    pub state_sync_evaluator_args: StateSyncMetricsEvaluatorArgs,
}
