// For use while we're developing.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod metric_collector;
mod metric_evaluator;
mod runner;

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use lazy_static::lazy_static;
use log::{debug, info};
use metric_collector::{MetricCollector, ReqwestMetricCollector};
use poem::http::StatusCode;
use poem::{
    handler, listener::TcpListener, Error as PoemError, Result as PoemResult, Route, Server,
};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use reqwest::Client as ReqwestClient;
use runner::BlockingRunner;
use std::path::PathBuf;
use url::Url;

const DEFAULT_METRICS_PORT: u16 = 9091;
const DEFAULT_API_PORT: u16 = 8080;
const DEFAULT_NOISE_PORT: u16 = 6180;

lazy_static! {
    static ref DEFAULT_METRICS_PORT_STR: String = format!("{}", DEFAULT_METRICS_PORT);
    static ref DEFAULT_API_PORT_STR: String = format!("{}", DEFAULT_API_PORT);
    static ref DEFAULT_NOISE_PORT_STR: String = format!("{}", DEFAULT_NOISE_PORT);
}

// TODO: Replace this with the real frontend, or perhaps an error handler if we
// decide to route the frontend to just a static hoster such as nginx.
#[handler]
fn root() -> String {
    "Hello World!".to_string()
}

struct Api<M: MetricCollector> {
    pub baseline_metric_collector: M,
    pub target_metric_collector: Option<M>,
    pub allow_preconfigured_test_node_only: bool,
}

#[OpenApi]
impl<M: MetricCollector> Api<M> {
    /// Hello world
    #[oai(path = "/check_node", method = "get")]
    async fn check_node(&self) -> PoemResult<PlainText<&'static str>> {
        if self.allow_preconfigured_test_node_only {
            return Err(PoemError::from((
                StatusCode::METHOD_NOT_ALLOWED,
                anyhow!(
                "This node health checker is configured to only check its preconfigured test node"),
            )));
        }
        Ok(PlainText("Hello World"))
    }

    #[oai(path = "/check_preconfigured_node", method = "get")]
    async fn check_preconfigured_node(&self) -> PoemResult<PlainText<&'static str>> {
        if self.target_metric_collector.is_none() {
            return Err(PoemError::from((
                StatusCode::METHOD_NOT_ALLOWED,
                anyhow!(
                    "This node health checker has not been set up with a preconfigured test node"
                ),
            )));
        }
        Ok(PlainText("Hello World"))
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to enable debug logging or not. This is a shortcut for the
    /// standard env logger configuration via env vars
    #[clap(short, long)]
    debug: bool,

    /// What address to listen on.
    #[clap(long, default_value = "0.0.0.0")]
    listen_address: String,

    /// What port to listen on.
    #[clap(long, default_value = "20121")]
    listen_port: u16,

    /// The URL of the baseline node, e.g. http://fullnode.devnet.aptoslabs.com
    /// We assume this is just the base and will add ports and paths to this.
    #[clap(long)]
    baseline_node_url: Url,

    /// The metrics port for the baseline node.
    #[clap(long, default_value = &DEFAULT_METRICS_PORT_STR)]
    baseline_metrics_port: u16,

    /// The API port for the baseline node.
    #[clap(long, default_value = &DEFAULT_API_PORT_STR)]
    baseline_api_port: u16,

    /// The port over which validator nodes can talk to the baseline node.
    #[clap(long, default_value = &DEFAULT_NOISE_PORT_STR)]
    baseline_noise_port: u16,

    /// The (metric) evaluators to use, e.g. state_sync, api, etc.
    #[clap(long)]
    evaluators: Vec<String>,

    /// If this is given, the user will be able to call the check_preconfigured_node
    /// endpoint, which takes no target, instead using this as the target. If
    /// allow_test_node_only is set, only the todo endpoint will work,
    /// the node will not respond to health check requests for other nodes.
    #[clap(long)]
    target_node_url: Option<Url>,

    // The following 3 arguments are only relevant if the user sets test_node_url.
    /// The metrics port for the target node.
    #[clap(long, default_value = &DEFAULT_METRICS_PORT_STR)]
    target_metrics_port: u16,

    /// The API port for the target node.
    #[clap(long, default_value = &DEFAULT_API_PORT_STR)]
    target_api_port: u16,

    /// The port over which validator nodes can talk to the target node.
    #[clap(long, default_value = &DEFAULT_NOISE_PORT_STR)]
    target_noise_port: u16,

    /// See the help message of target_node_url.
    #[clap(long)]
    allow_preconfigured_test_node_only: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.debug {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
        debug!("Logging at debug level");
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
        info!("Logging at info level");
    }

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());

    let baseline_metric_collector =
        ReqwestMetricCollector::new(args.baseline_node_url.clone(), args.baseline_metrics_port);

    let target_metric_collector = match args.target_node_url {
        Some(url) => Some(ReqwestMetricCollector::new(url, args.target_metrics_port)),
        None => None,
    };

    let api = Api {
        baseline_metric_collector,
        target_metric_collector,
        allow_preconfigured_test_node_only: args.allow_preconfigured_test_node_only,
    };

    let api_service = OpenApiService::new(api, "Aptos Node Checker", version)
        .server(format!("{}:{}/api", args.listen_address, args.listen_port));
    let ui = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();
    let spec_yaml = api_service.spec_endpoint_yaml();

    Server::new(TcpListener::bind((args.listen_address, args.listen_port)))
        .run(
            Route::new()
                .nest("/", root)
                .nest("/api", api_service)
                .nest("/docs", ui)
                .at("/spec_json", spec_json)
                .at("/spec_yaml", spec_yaml),
        )
        .await
        .map_err(anyhow::Error::msg)
}
