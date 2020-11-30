use yahoo_finance_api as yahoo;
//use std::time::{Duration, UNIX_EPOCH};
use chrono::offset;
use chrono::{Utc,TimeZone};
use clap::{App, Arg};

fn main() {
    let args = App::new("rust-stock-cli")
        .version("0.1")
        .about("Assignment 1")
        .arg(Arg::with_name("ticker")
             .help("Stock ticker: MSFT,GOOG,AAPL,UBER,IBM")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("from")
             .help("Get quotes from date")
             .takes_value(true)
             .required(false))
        .get_matches();
    let ticker = args.value_of("ticker").unwrap();
    let from = args.value_of("from").unwrap_or("-");

    let split: Vec<&str> = from.split('/').collect();
    let year = split[0].parse::<i32>().unwrap();
    let month = split[1].parse::<u32>().unwrap();
    let day = split[2].parse::<u32>().unwrap();

    let start = Utc.ymd(year, month, day).and_hms_milli(0, 0, 0, 0);
    let end = offset::Utc::now();

    println!("Start date: {}", start);
    println!("End date: {}", end);

    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history(ticker, start, end).unwrap();
    let quotes = response.quotes().unwrap();
    println!("{}'s quotes: {:?}", ticker, quotes);
}

