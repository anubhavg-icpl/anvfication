use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <base_url> <paths_file>", args[0]);
        std::process::exit(1);
    }
    
    let base_url = &args[1];
    let file_path = &args[2];
    
    println!("Starting HTTP status checker...");
}