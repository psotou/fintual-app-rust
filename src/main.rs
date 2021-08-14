use serde::Deserialize;
use reqwest::Error;
use std::env;
use colored::*;

#[derive(Deserialize, Debug)]
struct Data {
    data: Vec<DataFintual>,
}

#[derive(Deserialize, Debug)]
struct DataFintual {
    attributes: Attributes,
}

#[derive(Deserialize, Debug)]
struct Attributes {
    name: String,
    nav: f64,
    deposited: f64,
    profit: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://fintual.cl/api/goals?user_email=pascualsu%40gmail.com&user_token={fintual_token}",
        fintual_token = env::var("FINTUAL_TOKEN").unwrap_or("none".to_string())
    );
    let response = reqwest::get(&request_url).await?;
    let users: Data = response.json().await?;

    println!("{:-^56}", "-");
    println!(
        "{:24} {:12} {:10} {:8}",
        "GOAL".bold(),
        "DEPOSITED".bold(),
        "BALANCE".bold(),
        "PROFIT".bold()
    );
    println!("{:-^56}", "-");

    let mut deposited_acum: f64 = 0.0;
    let mut balance_acum: f64 = 0.0;
    let mut profit_acum: f64 = 0.0;

    for attr in users.data {
        let goal_name = attr.attributes.name;
        let deposited = attr.attributes.deposited;
        let balance = attr.attributes.nav;
        let profit = attr.attributes.profit;

        deposited_acum += deposited;
        balance_acum += balance;
        profit_acum += profit;

        println!("{:24} {:>9} {:>10.0} {:>9.0}", goal_name, deposited, balance, profit);
    }

    println!(
        "{:24} {:>9} {:>10} {:>9}",
        "Total".cyan().bold(),
        deposited_acum.to_string().cyan().bold(),
        balance_acum.trunc().to_string().cyan().bold(),
        profit_acum.trunc().to_string().cyan().bold()
    );
    println!("{:-^56}", "-");

    Ok(())
}
