use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct RatesResponse {
    pub date: String,
    pub base: String,
    pub rates: HashMap<String, f64>,
}