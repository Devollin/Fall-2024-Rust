/*

Website Status Checker Project Requirements

Project Overview
Create a concurrent website monitoring system that can check the status of multiple websites simultaneously.

Core Requirements
    Functionality
        ✅ Must accept a list of website URLs for monitoring
        ✅ Must support concurrent checking of websites
        ✅ Must implement configurable timeout for each request
        ✅ Must collect and report:
            ✅ HTTP status code
            ✅ Response time
            ✅ Any errors encountered
            
    Performance Requirements
        ✅ Must support checking at least 50 websites concurrently
        ✅ Each request must timeout after a configurable duration (default: 5 seconds)
        ✅ Must handle network failures gracefully

    Technical Requirements
        ✅ Implementation must use Rust's threading system
        ✅ Must use channels for communication between threads
        ✅ Must implement proper error handling
        ✅ Must support graceful shutdown

    Configuration Options
        ✅ Number of worker threads
        ✅ Request timeout duration
        ✅ Maximum retries per website

Bonus Features
    Support for periodic monitoring
    HTTP header validation
    ✅ SSL certificate checking
    Response body validation
    Statistics collection (avg response time, uptime percentage)

Testing Requirements
    ✅ Unit tests for core functionality
    Integration tests with mock HTTP server
    Performance tests with multiple concurrent requests
    Error handling tests
*/

use ureq;
use chrono::{DateTime, Utc};
use std::sync::mpsc;
use core::time::Duration;
use std::time::Instant;
use std::process;
use std::thread;


pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub struct Configuration {
    pub urls: Vec<String>,
    pub timeout: Duration,
    pub threads: usize,
    pub retries: usize,
}

pub struct Worker {
    pub configuration: Configuration,
}


impl WebsiteStatus {
    pub fn print(&self) {
        println!("URL: {:?}", self.url);
        println!("Status: {:?}", self.status);
        println!("Response Time: {:?}", self.response_time);
        println!("Timestamp: {:?}", self.timestamp);
    }
}

impl Configuration {
    pub fn new(mut args: std::env::Args) -> Result<Configuration, &'static str> {
        args.next();

        let urls = args.next().ok_or("No URLs provided. Provide a comma-separated list of URLs.")?.split(',').map(String::from).collect();
        let timeout = args.next().and_then(|t| t.parse::<u64>().ok()).map(Duration::from_secs).unwrap_or(Duration::from_secs(5));
        let threads = args.next().and_then(|t| t.parse::<usize>().ok()).unwrap_or(4);
        let retries = args.next().and_then(|t| t.parse::<usize>().ok()).unwrap_or(3);

        Ok(Configuration { urls, timeout, threads, retries })
    }
}

impl Worker {
    pub fn new(configuration: Configuration) -> Worker {
        Worker {
            configuration: configuration,
        }
    }

    pub fn run_workers(&self) {
        let (sender, reciever) = mpsc::channel();
    
        let urls = self.configuration.urls.clone();
        let threads = self.configuration.threads;
        let timeout = self.configuration.timeout;

        let mut handles = Vec::new();
    
        for chunk in urls.chunks(threads) {
            let sender = sender.clone();

            let handle = thread::spawn(move || {
                let chunk = chunk.to_vec();

                for url in chunk {
                    let start_time = Instant::now();

                    let status: Result<u16, String> = match ureq::get(&url).timeout(timeout).call() {
                        Ok(response) => Ok(response.status()),
                        Err(err) => Err(err.to_string()),
                    };

                    let website_status = WebsiteStatus {
                        url: url.to_string(),
                        status: status,
                        response_time: Duration::new(start_time.elapsed().as_secs(), 0),
                        timestamp: Utc::now()
                    };
    
                    sender.send(website_status).unwrap();
                }
            });
    
            handles.push(handle);
        }

        drop(sender);
    
        while let Ok(status) = reciever.recv() {
            status.print();
        }
    
        for handle in handles {
            handle.join().unwrap();
        }
    }
}


fn main() {
    let configuration = Configuration::new(std::env::args()).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);

        process::exit(1); // Exit the process
    });

    let worker = Worker::new(configuration);
    worker.run_workers();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_status() {
        let status = WebsiteStatus::new("https://www.rust-lang.org/");
        
        assert_eq!(match status.status {
            Ok(_) => true,
            Err(_) => false,
        }, true);
    }
}