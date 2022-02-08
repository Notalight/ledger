use std::{fs::File, io::Read};

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
//use serde_with::{serde_as, DurationSeconds};

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    id: String,
    pub name: String,
    pub accounts: Vec<Account>,
    pub assets: Vec<Asset>,
    //#[serde_as(as = "serde_with::DurationSeconds<i64>")]
    //pub last_datetime: Option<NaiveDateTime>,
    pub last_update: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: String,
    name: String,
    account_number: String,
    type_: String,
    bank_name: String,
    stocks: Option<Vec<Stock>>,
    loans: Option<Vec<Loan>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    name: String,
    pub description: String,
    type_: String,
    acquisition_price: f64,
    acquisition_date: Option<NaiveDate>,
    pub estimation_price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Loan {
    loan_number: String,
    name: String,
    description: String,
    pv: f64,
    rate: f64,
    nper: f64,
    start_date: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stock {
    order_number: String,
    name: String,
    symbol: String,
    market: String,
    qty: f64,
    purchase_date: NaiveDate,
    unit_purchase_price: f64,
    purchase_change_rate: Option<f64>,
    currency: String,
    fees: f64,
    currency_fees: Option<String>,
}

pub fn load_portfolio(path: String) -> Portfolio {
    let mut file = File::open(path.to_owned()).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();
    let deserialized: Portfolio = serde_json::from_str(&buffer).unwrap();

    let mut total_loans = 0;
    let mut total_stocks = 0;
    let total_accounts = deserialized.accounts.len();
    let total_assets = deserialized.assets.len();

    for account in deserialized.accounts {
        total_loans = total_loans + account.loans.unwrap().len();
        total_stocks = total_stocks + account.stocks.unwrap().len();
    } ;

    println!(
        "> {} accounts / {} loans / {} stocks / {} assets loaded.",
        total_accounts,
        total_loans,
        total_stocks,
        total_assets
    );
    return deserialized;
}
