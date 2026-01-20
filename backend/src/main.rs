use axum::{
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod api;
mod converter;
mod models;

use api::{convert, get_rates, detect_currency};
use models::RatesResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let state = Arc::new(RwLock::new(None::<RatesResponse>));

    {
        let mut guard = state.write().await;
        *guard = Some(fetch_rates().await?);
    }

    let app = Router::new()
        .route("/api/rates", get(get_rates))
        .route("/api/convert", get(convert))
        .route("/api/detect-currency", get(detect_currency))
        .layer(CorsLayer::permissive()) 
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app,
    )
    .await?;

    Ok(())
}

async fn fetch_rates() -> anyhow::Result<RatesResponse> {
    let url = "https://api.frankfurter.app/latest?base=MYR";
    let resp: serde_json::Value = reqwest::get(url).await?.json().await?;

    let base = resp["base"]
        .as_str()
        .ok_or(anyhow::anyhow!("Missing 'base' field"))?
        .to_string();

    let date = resp["date"]
        .as_str()
        .ok_or(anyhow::anyhow!("Missing 'date' field"))?
        .to_string();

    let rates = resp["rates"]
        .as_object()
        .ok_or(anyhow::anyhow!("Missing 'rates' object"))?
        .iter()
        .filter_map(|(k, v)| v.as_f64().map(|v| (k.clone(), v)))
        .collect::<HashMap<String, f64>>();

    let mut all_rates = rates.clone();
    all_rates.insert("MYR".to_string(), 1.0);

    Ok(RatesResponse { date, base, rates: all_rates })
}