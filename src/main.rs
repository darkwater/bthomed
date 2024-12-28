use std::sync::Arc;

use anyhow::Context as _;
use axum::{Router, extract::State, routing::get};

use clap::Parser;
use tokio::sync::RwLock;

use self::registry::Registry;

mod bthome;
mod registry;
mod scanner;

#[derive(Debug, Parser)]
struct Args {
    /// Port for the HTTP server to listen on
    #[clap(short, long, default_value = "9556")]
    port: u16,

    /// Expiry time for devices in seconds
    #[clap(short, long, default_value = "90")]
    expiry: f32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = Args::parse();

    let registry = Arc::new(RwLock::new(Registry::default()));

    tokio::spawn(scanner::scan(registry.clone()));

    tokio::spawn({
        let registry = registry.clone();

        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs_f32(config.expiry / 10.)).await;
                if registry.read().await.needs_pruning(config.expiry) {
                    registry.write().await.prune(config.expiry);
                }
            }
        }
    });

    let app = Router::new()
        .route("/metrics", get(metrics))
        .with_state(registry);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.context("server error")
}

async fn metrics(State(registry): State<Arc<RwLock<Registry>>>) -> String {
    let registry = registry.read().await;

    registry
        .devices
        .iter()
        .flat_map(|(dev_name, device)| {
            device
                .stats()
                .iter()
                .map(|(obj_name, value)| {
                    format!("bthome_{obj_name}{{device=\"{dev_name}\"}} {value}\n")
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .join("")
}
