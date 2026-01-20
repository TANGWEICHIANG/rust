use std::collections::HashMap;
use std::io::{self, Write};
use anyhow::{Context, Result};
use serde::Deserialize;
use cashify::convert;

#[derive(Deserialize)]
struct ApiResponse {
    base: String,
    date: String,              
    rates: HashMap<String, f64>,
}

fn main() -> Result<()> {
    println!("Fetching latest exchange rates with MYR as base...");

    let url = "https://api.frankfurter.app/latest?base=MYR";
    let api_response: ApiResponse = reqwest::blocking::get(url)?
        .json()
        .context("Failed to parse JSON from Frankfurter API")?;

    let mut rates_owned = api_response.rates;
    rates_owned.insert(api_response.base.clone(), 1.0);

    let rates: HashMap<&str, f64> = rates_owned
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

    let mut currencies: Vec<&str> = rates.keys().cloned().collect();
    currencies.sort();
    println!("\nAvailable currencies (base: MYR):");
    for chunk in currencies.chunks(8) {
        println!("  {}", chunk.join("  "));
    }

    println!("\nRates as of: {}", api_response.date);
    println!("\nNote: Enter currencies in uppercase (e.g. USD, EUR, SGD)");
    println!("      Type 'quit' or 'exit' or Ctrl+C to quit\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let from = "MYR";  
    let base = api_response.base.as_str();

    loop {
        print!("Enter amount in MYR (e.g. 1000): ");
        stdout.flush()?;
        let mut amount_str = String::new();
        stdin.read_line(&mut amount_str)?;
        let amount_str = amount_str.trim();

        if amount_str.is_empty() {
            continue;
        }

        if amount_str.eq_ignore_ascii_case("quit") || amount_str.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let amount: f64 = match amount_str.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid amount. Please enter a number.");
                continue;
            }
        };

        print!("To currency (e.g. USD): ");
        stdout.flush()?;
        let mut to = String::new();
        stdin.read_line(&mut to)?;
        let to = to.trim().to_uppercase();

        if to.is_empty() {
            continue;
        }

        if !rates.contains_key(to.as_str()) {
            println!("Unknown currency: {}. Try again.", to);
            continue;
        }

        let result = convert(amount, from, &to, base, rates.clone());

        println!("→ {:.4} MYR = {:.4} {}", amount, result, to);
        println!(); 
    }

    Ok(())
}