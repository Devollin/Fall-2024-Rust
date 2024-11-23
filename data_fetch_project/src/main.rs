use ureq;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{Write};


struct Bitcoin {
    api_address: String,
    file_name: String,
    price: f32,
}

struct Ethereum {
    api_address: String,
    file_name: String,
    price: f32,
}

struct SnP500 {
    api_address: String,
    file_name: String,
    price: f32,
}

#[derive(Debug)]
pub enum ApiResult {
    Success(f32),
    ApiError(String),
    NetworkError(String),
}


#[derive(Debug, Deserialize)]
struct Price {
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct BitcoinPrice {
    bitcoin: Price,
}

#[derive(Debug, Deserialize)]
struct EthereumPrice {
    ethereum: Price,
}

#[derive(Debug, Deserialize)]
struct SnPClose {
    close: [f32; 1],
}

#[derive(Debug, Deserialize)]
struct SnPIndicators {
    quote: Vec<SnPClose>,
}

#[derive(Debug, Deserialize)]
struct SnPResult {
    indicators: SnPIndicators,
}

#[derive(Debug, Deserialize)]
struct SnPChart {
    result: Vec<SnPResult>,
}

#[derive(Debug, Deserialize)]
struct SnPAPI {
    chart: SnPChart,
}

pub trait Pricing {
    fn fetch_price(&self) -> ApiResult;
    fn save_to_file(&self);
}

fn get_pricing_for_snp(result: SnPAPI) -> f32 {
    let mut arr = result.chart.result[0].indicators.quote[0].close;
    arr.rotate_right(1); // Need to rotate since we can't use negative indexes

    arr[0]
}


impl Pricing for Bitcoin {
    fn fetch_price(&self) -> ApiResult {
        match ureq::get(&self.api_address).call() {
            Ok(response) => match response.status() {
                200 => match response.into_json::<BitcoinPrice>() {
                    Ok(result) => ApiResult::Success(result.bitcoin.usd),
                    Err(e) => ApiResult::ApiError(format!("[BTC] Failed to parse JSON: {}", e)),
                },
                _ => ApiResult::ApiError(format!("[BTC] HTTP error: {}", response.status())),
            },
            Err(e) => ApiResult::NetworkError(format!("[BTC] Request failed: {}", e)),
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create(self.file_name.clone()).unwrap();

        write!(file, "{}\n", &self.price).unwrap();
    }
}


impl Pricing for Ethereum {
    fn fetch_price(&self) -> ApiResult {
        match ureq::get(&self.api_address).call() {
            Ok(response) => match response.status() {
                200 => match response.into_json::<EthereumPrice>() {
                    Ok(result) => ApiResult::Success(result.ethereum.usd),
                    Err(e) => ApiResult::ApiError(format!("[ETH] Failed to parse JSON: {}", e)),
                },
                _ => ApiResult::ApiError(format!("[ETH] HTTP error: {}", response.status())),
            },
            Err(e) => ApiResult::NetworkError(format!("[ETH] Request failed: {}", e)),
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create(self.file_name.clone()).unwrap();
        
        write!(file, "{}\n", &self.price).unwrap();
    }
}

impl Pricing for SnP500 {
    fn fetch_price(&self) -> ApiResult {
        match ureq::get(&self.api_address).call() {
            Ok(response) => match response.status() {
                200 => match response.into_json::<SnPAPI>() {
                    Ok(result) => ApiResult::Success(get_pricing_for_snp(result)),
                    Err(e) => ApiResult::ApiError(format!("[S&P] Failed to parse JSON: {}", e)),
                },
                _ => ApiResult::ApiError(format!("[S&P] HTTP error: {}", response.status())),
            },
            Err(e) => ApiResult::NetworkError(format!("[S&P] Request failed: {}", e)),
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create(self.file_name.clone()).unwrap();
        
        write!(file, "{}\n", &self.price).unwrap();
    }
}


fn get_price_from_result(result: ApiResult) -> f32 {
    match result {
        ApiResult::Success(price) => price,
        ApiResult::NetworkError(e) => {
            println!("{}", e);

            -1.0
        },
        ApiResult::ApiError(e) => {
            println!("{}", e);

            -1.0
        }, 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let bitcoin_url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();
    let bitcoin_file = "bitcoin_prices.json".to_string();

    let ethereum_url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let ethereum_file = "ethereum_prices.json".to_string();

    let snp500_url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string();
    let snp500_file = "snp_500.json".to_string();

    let mut btc = Bitcoin {
        api_address: bitcoin_url,
        file_name: bitcoin_file,
        price: -1.0,
    };

    let mut eth = Ethereum {
        api_address: ethereum_url,
        file_name: ethereum_file,
        price: -1.0,
    };

    let mut snp = SnP500 {
        api_address: snp500_url,
        file_name: snp500_file,
        price: -1.0,
    };

    println!("Starting loop, to exit, press Ctrl+C.\n");

    loop {
        let btc_price = get_price_from_result(btc.fetch_price());
        let eth_price = get_price_from_result(eth.fetch_price());
        let snp_price = get_price_from_result(snp.fetch_price());
        
        if btc_price != -1.0 {
            println!("Bitcoin Price: {}", btc_price);
            btc.price = btc_price;

            btc.save_to_file();
        };

        if eth_price != -1.0 {
            println!("Ethereum Price: {}", eth_price);
            eth.price = eth_price;

            eth.save_to_file();
        };

        if snp_price != -1.0 {
            println!("S&P Index: {}", snp_price);
            snp.price = snp_price;

            snp.save_to_file();
        };

        // Commented out the snp price, since the JSON is not able to be decoded.
        if btc_price == -1.0 || eth_price == -1.0 /* || snp_price == -1.0 */ {
            break;
        }

        println!("Waiting...\n");

        thread::sleep(Duration::from_secs(10));
    }

    Ok(())
}