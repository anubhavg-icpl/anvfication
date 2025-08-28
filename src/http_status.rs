use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task::JoinSet;

async fn check_url(url: String) -> String {
    match reqwest::Client::new()
        .get(&url)
        .send()
        .await
    {
        Ok(response) => format!("{} - {}", url, response.status().as_u16()),
        Err(e) => format!("{} - Error: {}", url, e),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <base_url> <paths_file>", args[0]);
        std::process::exit(1);
    }
    
    let base_url = &args[1];
    let file_path = &args[2];
    
    // Read paths from file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let paths: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    
    // Create concurrent tasks
    let mut tasks = JoinSet::new();
    
    for path in paths {
        let url = format!("{}/{}", base_url.trim_end_matches('/'), path);
        tasks.spawn(check_url(url));
    }
    
    // Collect results
    while let Some(result) = tasks.join_next().await {
        if let Ok(status) = result {
            println!("{}", status);
        }
    }
}