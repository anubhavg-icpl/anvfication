use clap::Parser;
use colored::Colorize;
use serde_json::{Value, from_str, to_string_pretty};
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(author, version, about = "Format and validate JSON", long_about = None)]
struct Args {
    #[arg(help = "JSON file (use '-' for stdin)")]
    file: String,
    
    #[arg(short, long, help = "Minify instead of prettify")]
    minify: bool,
    
    #[arg(short, long, help = "Validate only, no output")]
    validate: bool,
    
    #[arg(short, long, help = "Colorize output")]
    color: bool,
}

fn colorize_json(json_str: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escape = false;
    
    for ch in json_str.chars() {
        if escape {
            result.push(ch);
            escape = false;
            continue;
        }
        
        match ch {
            '"' if !in_string => {
                in_string = true;
                result.push_str(&ch.to_string().yellow().to_string());
            }
            '"' if in_string => {
                result.push_str(&ch.to_string().yellow().to_string());
                in_string = false;
            }
            '{' | '}' | '[' | ']' if !in_string => {
                result.push_str(&ch.to_string().bright_white().to_string());
            }
            ':' if !in_string => {
                result.push_str(&ch.to_string().cyan().to_string());
            }
            ',' if !in_string => {
                result.push_str(&ch.to_string().white().to_string());
            }
            '\\' if in_string => {
                escape = true;
                result.push(ch);
            }
            _ if in_string => {
                result.push_str(&ch.to_string().green().to_string());
            }
            _ => result.push(ch),
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let input = if args.file == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(&args.file)?
    };
    
    let parsed: Value = match from_str(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} Invalid JSON: {}", "✗".red(), e);
            std::process::exit(1);
        }
    };
    
    if args.validate {
        println!("{} Valid JSON", "✓".green());
        return Ok(());
    }
    
    let output = if args.minify {
        parsed.to_string()
    } else {
        to_string_pretty(&parsed).unwrap()
    };
    
    if args.color {
        println!("{}", colorize_json(&output));
    } else {
        println!("{}", output);
    }
    
    Ok(())
}