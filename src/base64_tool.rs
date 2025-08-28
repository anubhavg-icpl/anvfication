use base64::{engine::general_purpose, Engine};
use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about = "Base64 encode/decode tool", long_about = None)]
struct Args {
    #[arg(help = "Input file (use '-' for stdin)")]
    input: String,
    
    #[arg(short, long, help = "Decode instead of encode")]
    decode: bool,
    
    #[arg(short, long, help = "Output file (default: stdout)")]
    output: Option<String>,
    
    #[arg(short, long, help = "URL-safe encoding/decoding")]
    url_safe: bool,
    
    #[arg(short, long, help = "Wrap encoded output at column N", default_value = "76")]
    wrap: usize,
}

fn wrap_string(s: &str, width: usize) -> String {
    if width == 0 {
        return s.to_string();
    }
    
    s.chars()
        .collect::<Vec<char>>()
        .chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // Read input
    let input_data = if args.input == "-" {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        fs::read(&args.input)?
    };
    
    // Choose encoder
    let engine = if args.url_safe {
        &general_purpose::URL_SAFE_NO_PAD
    } else {
        &general_purpose::STANDARD
    };
    
    // Process data
    let output_data = if args.decode {
        // For decoding, remove whitespace first
        let input_str = String::from_utf8_lossy(&input_data);
        let cleaned = input_str.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        
        match engine.decode(cleaned.as_bytes()) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!("Error decoding: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let encoded = engine.encode(&input_data);
        let wrapped = if args.wrap > 0 {
            wrap_string(&encoded, args.wrap)
        } else {
            encoded
        };
        wrapped.into_bytes()
    };
    
    // Write output
    match args.output {
        Some(path) => {
            fs::write(path, &output_data)?;
        }
        None => {
            io::stdout().write_all(&output_data)?;
            if !args.decode {
                println!(); // Add newline for encoded output
            }
        }
    }
    
    Ok(())
}