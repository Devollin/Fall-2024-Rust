/*
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
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
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}*/

use ureq;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Price {
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct Crypto {
    bitcoin: Price,
    ethereum: Price,
}

#[derive(Debug)]
enum ApiResult {
    Success(Crypto),
    ApiError(String),
    NetworkError(String),
}

fn fetch_crypto_price() -> ApiResult {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";
    
    match ureq::get(url).call() {
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
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Crypto Price Fetcher");
    println!("=================\n");

    match fetch_crypto_price() {
        ApiResult::Success(crypto) => {
            println!("✅ Success!");
            println!("Bitcoin in USD: {}", crypto.bitcoin.usd);
            println!("Ethereum in USD: {}", crypto.ethereum.usd);
        },
        ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
        ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
    }
    println!();

    Ok(())
}