use clap::Parser;
use md5::Md5;
use sha2::{Sha256, Sha512, Digest};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use hex;

#[derive(Parser, Debug)]
#[command(author, version, about = "Calculate file hashes", long_about = None)]
struct Args {
    #[arg(help = "File to hash")]
    file: String,
    
    #[arg(short, long, value_enum, default_value = "all", help = "Hash algorithm")]
    algorithm: HashAlgorithm,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum HashAlgorithm {
    Md5,
    Sha256,
    Sha512,
    All,
}

fn calculate_md5(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn calculate_sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    if !Path::new(&args.file).exists() {
        eprintln!("Error: File '{}' not found", args.file);
        std::process::exit(1);
    }
    
    let mut file = File::open(&args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let file_size = buffer.len();
    println!("File: {}", args.file);
    println!("Size: {} bytes", file_size);
    println!("---");
    
    match args.algorithm {
        HashAlgorithm::Md5 => {
            println!("MD5:    {}", calculate_md5(&buffer));
        }
        HashAlgorithm::Sha256 => {
            println!("SHA256: {}", calculate_sha256(&buffer));
        }
        HashAlgorithm::Sha512 => {
            println!("SHA512: {}", calculate_sha512(&buffer));
        }
        HashAlgorithm::All => {
            println!("MD5:    {}", calculate_md5(&buffer));
            println!("SHA256: {}", calculate_sha256(&buffer));
            println!("SHA512: {}", calculate_sha512(&buffer));
        }
    }
    
    Ok(())
}