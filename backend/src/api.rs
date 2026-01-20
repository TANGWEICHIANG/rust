use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::RwLock;

use crate::converter::perform_conversion;
use crate::models::RatesResponse;

#[derive(Deserialize)]
pub struct ConvertQuery {
    amount: f64,
    from: String,
    to: String,
}

pub type AppState = Arc<RwLock<Option<RatesResponse>>>;

pub async fn get_rates(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.read().await;
    match &*guard {
        Some(rates) => Json(rates.clone()).into_response(),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Rates not available yet".to_string(),
        )
            .into_response(),
    }
}

pub async fn convert(
    State(state): State<AppState>,
    Query(query): Query<ConvertQuery>,
) -> impl IntoResponse {
    let guard = state.read().await;
    if let Some(rates) = &*guard {
        // Validate both currencies exist
        if !rates.rates.contains_key(&query.from.to_uppercase()) {
            return (StatusCode::BAD_REQUEST, format!("Unknown source currency: {}", query.from)).into_response();
        }
        
        if !rates.rates.contains_key(&query.to.to_uppercase()) {
            return (StatusCode::BAD_REQUEST, format!("Unknown target currency: {}", query.to)).into_response();
        }
        
        match perform_conversion(
            query.amount,
            &query.from.to_uppercase(),
            &query.to.to_uppercase(),
            &rates.base,
            &rates.rates,
        ) {
            Ok(result) => Json(serde_json::json!({
                "from": query.from.to_uppercase(),
                "amount": query.amount,
                "to": query.to.to_uppercase(),
                "result": result,
                "date": rates.date,
            }))
            .into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Rates not loaded").into_response()
    }
}

pub async fn detect_currency(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let guard = state.read().await;
    
    let country_code = match get_country_from_ip(addr.ip()).await {
        Ok(code) => code,
        Err(_) => "US".to_string(), 
    };
    
    let currency = match country_code.as_str() {
        "MY" => "MYR",
        "US" => "USD",
        "GB" => "GBP",
        "EU" | "DE" | "FR" | "IT" | "ES" => "EUR",
        "JP" => "JPY",
        "CN" => "CNY",
        "SG" => "SGD",
        "AU" => "AUD",
        "CA" => "CAD",
        "IN" => "INR",
        _ => "USD", // Default to USD
    };
    
    let response = match &*guard {
        Some(rates) => {
            let is_available = rates.rates.contains_key(currency);
            Json(serde_json::json!({
                "detected_currency": currency,
                "available": is_available,
                "suggested_target": "USD", 
                "all_currencies": rates.rates.keys().collect::<Vec<&String>>()
            }))
        }
        None => Json(serde_json::json!({
            "detected_currency": currency,
            "available": false,
            "error": "Rates not loaded"
        })),
    };
    
    response.into_response()
}

async fn get_country_from_ip(ip: std::net::IpAddr) -> Result<String, reqwest::Error> {
    let url = format!("http://ip-api.com/json/{}", ip);
    let resp: Value = reqwest::get(&url).await?.json().await?;
    
    Ok(resp["countryCode"]
        .as_str()
        .unwrap_or("US")
        .to_string())
}