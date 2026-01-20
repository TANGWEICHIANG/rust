use cashify::convert;
use std::collections::HashMap;

pub fn perform_conversion(
    amount: f64,
    from: &str,
    to: &str,
    base: &str,
    rates: &HashMap<String, f64>,  
) -> Result<f64, String> {
    if !rates.contains_key(from) {
        return Err(format!("Unknown source currency: {}", from));
    }
    
    if !rates.contains_key(to) {
        return Err(format!("Unknown target currency: {}", to));
    }

    let borrowed_rates: HashMap<&str, f64> = rates
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

    Ok(convert(amount, from, to, base, borrowed_rates))
}