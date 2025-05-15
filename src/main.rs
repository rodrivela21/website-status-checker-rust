use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    collections::HashSet,
};

use chrono::Utc;
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    url: String,
    status: String,
    elapsed_ms: u128,
    timestamp: String,
    error: Option<String>,
}

fn read_urls(file_path: Option<&str>, cli_urls: Vec<String>) -> Vec<String> {
    let mut urls = vec![];

    if let Some(path) = file_path {
        if let Ok(file) = File::open(path) {
            for line in io::BufReader::new(file).lines().flatten() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    urls.push(trimmed.to_string());
                }
            }
        } else {
            eprintln!("Warning: Could not open file '{}'", path);
        }
    }

    urls.extend(cli_urls);
    urls
}

fn check_website(url: &str) -> Status {
    let start = std::time::Instant::now();
    let timestamp = Utc::now().to_rfc3339();
    match reqwest::blocking::get(url) {
        Ok(resp) => Status {
            url: url.to_string(),
            status: resp.status().to_string(),
            elapsed_ms: start.elapsed().as_millis(),
            timestamp,
            error: None,
        },
        Err(e) => Status {
            url: url.to_string(),
            status: "FAILURE".to_string(),
            elapsed_ms: start.elapsed().as_millis(),
            timestamp,
            error: Some(e.to_string()),
        },
    }
}

fn print_usage() {
    eprintln!("Usage: website_checker [--file sites.txt] [URL ...]");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut file_path: Option<&str> = None;
    let mut cli_urls = Vec::new();

    // Parse CLI arguments for --file and URLs
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--file" && i + 1 < args.len() {
            file_path = Some(&args[i + 1]);
            i += 2;
        } else if args[i].starts_with("--") {
            // skip unknown flags for now
            i += 1;
        } else {
            cli_urls.push(args[i].clone());
            i += 1;
        }
    }

    if file_path.is_none() && cli_urls.is_empty() {
        print_usage();
        std::process::exit(2);
    }

    let urls = read_urls(file_path, cli_urls);
    let mut seen = HashSet::new();
    let mut results = Vec::new();

    for url in urls {
        let url = url.trim();
        if url.is_empty() || !seen.insert(url.to_string()) {
            continue; // skip empty or duplicate URLs
        }
        let status = check_website(url);
        if status.error.is_none() {
            println!("[{}] {} OK in {}ms", status.timestamp, status.url, status.elapsed_ms);
        } else {
            println!("[{}] {} FAILURE: {} in {}ms", status.timestamp, status.url, status.error.as_ref().unwrap(), status.elapsed_ms);
        }
        results.push(status);
    }

    let json = serde_json::to_string_pretty(&results).unwrap();
    std::fs::write("status.json", json).expect("Unable to write status.json");
}
