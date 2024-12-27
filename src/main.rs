use std::{collections::HashMap, sync::Arc};

use anyhow::Context as _;
use axum::{Router, extract::State, routing::get};

use clap::Parser;
use tokio::sync::RwLock;

mod bthome;
mod scanner;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value = "9556")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = Args::parse();

    let registry = Arc::new(RwLock::new(Registry::default()));

    tokio::spawn(scanner::scan(registry.clone()));

    let app = Router::new()
        .route("/metrics", get(metrics))
        .with_state(registry);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.context("server error")
}

#[derive(Debug, Default)]
pub struct Registry {
    devices: HashMap<String, Device>,
}

#[derive(Debug, Default)]
pub struct Device {
    stats: HashMap<&'static str, f32>,
}

async fn metrics(State(registry): State<Arc<RwLock<Registry>>>) -> String {
    let registry = registry.read().await;

    registry
        .devices
        .iter()
        .flat_map(|(dev_name, device)| {
            device
                .stats
                .iter()
                .map(|(obj_name, value)| {
                    format!("bthome_{obj_name}{{device=\"{dev_name}\"}} {value}\n")
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .join("")
}
