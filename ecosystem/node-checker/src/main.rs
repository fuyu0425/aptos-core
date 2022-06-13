use anyhow::Result;
use clap::Parser;
use log::{debug, info};
use poem::{handler, listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

// TODO: Replace this with the real frontend, or perhaps an error handler if we
// decide to route the frontend to just a static hoster such as nginx.
#[handler]
fn root() -> String {
    "Hello World!".to_string()
}

struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
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
    #[clap(short, long, default_value = "20121")]
    port: u16,
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

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "1.0.0".to_string());

    let api = Api;
    let api_service =
        OpenApiService::new(api, "Aptos Node Checker", version).server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();
    let spec_yaml = api_service.spec_endpoint_yaml();


    Server::new(TcpListener::bind((args.listen_address, args.port)))
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
