use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    add: Option<String>,
    
    #[arg(short, long, default_value = "your@email.com")]
    email: String,
}

fn main() {
    let args = Args::parse();
    println!("Email: {}", args.email);
}