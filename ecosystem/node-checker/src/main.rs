// For use while we're developing.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod metric_collector;
mod runner;

use anyhow::Result;
use clap::Parser;
use log::{debug, info};
use poem::{handler, listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use url::Url;
use std::path::PathBuf;
use reqwest::Url;
use runner::BlockingRunner;

// TODO: Replace this with the real frontend, or perhaps an error handler if we
// decide to route the frontend to just a static hoster such as nginx.
#[handler]
fn root() -> String {
    "Hello World!".to_string()
}

struct Api;

// TODO: Should we host an endpoint that says what node types
// the user can work with? Derived from the keys of baseline_node_addresses.
//
// TODO: There should be an endpoint that doesn't take in a target, but
// it returns an error if the user hasn't specified a default target.
#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/check_node", method = "get")]
    async fn run_check(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
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

    /// The URL of the baseline node.
    #[clap(long)]
    baseline_node_url: Url,

    /// The (metric) evaluators to use.
    #[clap(long)]
    evaluators: Vec<String>,

    /// If this is given, the user will be able to call the todo endpoint,
    /// which takes no target, instead using this as the target. If
    /// allow_test_node_only is set, only the todo endpoint will work,
    /// the node will not respond to health check requests for other nodes.
    #[clap(long)]
    test_node_url: Option<Url>,

    /// See the help message of test_node_address,
    #[clap(long)]
    allow_test_node_only: bool,
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

    let api = Api;
    let api_service =
        OpenApiService::new(api, "Aptos Node Checker", version).server("http://localhost:3000/api");
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
