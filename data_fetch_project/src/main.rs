use ureq;
use serde::Deserialize;
use std::error::Error;


struct Bitcoin {
    api_address: String,
    file_name: String,
}

struct Ethereum {
    api_address: String,
    file_name: String,
}

struct SnP500 {
    api_address: String,
    file_name: String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32 {
        let final_result = match ureq::get(&self.api_address).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<Crypto>() {
                        Ok(result) => ApiResult::Success(result),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => {
                let error_details = format!("Request failed: {}", e);
                ApiResult::NetworkError(error_details)
            },
        };
    
        match final_result {
            ApiResult::Success(crypto) => {
                println!("✅ Success!");
                println!("Bitcoin in USD", );

                crypto.bitcoin.usd
            },
            ApiResult::ApiError(e) => {
                println!("❌ API Error: {}", e);

                -1.0
            },
            ApiResult::NetworkError(e) => {
                println!("❌ Network Error: {}", e);

                -1.0
            },
        }
    }

    fn save_to_file(&self) {

    }
}

#[derive(Debug, Deserialize)]
struct Price {
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct Crypto {
    bitcoin: Price,
}

#[derive(Debug)]
enum ApiResult {
    Success(Crypto),
    ApiError(String),
    NetworkError(String),
}

fn main() -> Result<(), Box<dyn Error>> {
    let bitcoin_url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();
    let bitcoin_file = "bitcoin_prices.json".to_string();

    let ethereum_url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let ethereum_file = "ethereum_prices.json".to_string();

    let snp500_url = "".to_string();
    let snp500_file = "".to_string();

    let btc = Bitcoin {
        api_address: bitcoin_url,
        file_name: bitcoin_file,
    };

    let eth = Ethereum {
        api_address: ethereum_url,
        file_name: ethereum_file,
    };

    let snp = SnP500 {
        api_address: snp500_url,
        file_name: snp500_file,
    };

    Ok(())
}