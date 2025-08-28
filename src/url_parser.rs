use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about = "Parse and analyze URLs", long_about = None)]
struct Args {
    #[arg(help = "URL to parse")]
    url: String,
    
    #[arg(short, long, help = "Show query parameters as JSON")]
    json: bool,
    
    #[arg(short, long, help = "Decode URL-encoded components")]
    decode: bool,
    
    #[arg(short, long, help = "Validate URL only")]
    validate: bool,
}

fn decode_component(s: &str) -> String {
    urlencoding::decode(s)
        .unwrap_or_else(|_| std::borrow::Cow::Borrowed(s))
        .into_owned()
}

fn parse_query_params(query: &str) -> HashMap<String, Vec<String>> {
    let mut params: HashMap<String, Vec<String>> = HashMap::new();
    
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        let key = parts[0];
        let value = if parts.len() > 1 { parts[1] } else { "" };
        
        params.entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value.to_string());
    }
    
    params
}

fn main() {
    let args = Args::parse();
    
    // Parse URL
    let parsed = match Url::parse(&args.url) {
        Ok(url) => url,
        Err(e) => {
            // Try adding scheme if missing
            match Url::parse(&format!("http://{}", args.url)) {
                Ok(url) => url,
                Err(_) => {
                    eprintln!("{} Invalid URL: {}", "Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }
    };
    
    if args.validate {
        println!("{} Valid URL", "✓".green());
        return;
    }
    
    // Display parsed components
    println!("{}", "URL Components:".bold());
    println!("─────────────────────");
    
    println!("{:15} {}", "Original:".cyan(), args.url);
    println!("{:15} {}", "Normalized:".cyan(), parsed.to_string());
    
    println!();
    println!("{:15} {}", "Scheme:".green(), parsed.scheme());
    
    if let Some(host) = parsed.host_str() {
        let host_display = if args.decode {
            decode_component(host)
        } else {
            host.to_string()
        };
        println!("{:15} {}", "Host:".green(), host_display);
    }
    
    if let Some(port) = parsed.port() {
        println!("{:15} {}", "Port:".green(), port);
    } else {
        let default_port = match parsed.scheme() {
            "http" => "80",
            "https" => "443",
            "ftp" => "21",
            "ssh" => "22",
            _ => "none",
        };
        println!("{:15} {} (default)", "Port:".green(), default_port);
    }
    
    let path = parsed.path();
    if !path.is_empty() && path != "/" {
        let path_display = if args.decode {
            decode_component(path)
        } else {
            path.to_string()
        };
        println!("{:15} {}", "Path:".green(), path_display);
        
        // Show path segments
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if !segments.is_empty() {
            println!("{:15} {:?}", "Path segments:".yellow(), segments);
        }
    }
    
    if let Some(query) = parsed.query() {
        println!();
        println!("{:15} {}", "Query:".blue(), query);
        
        let params = parse_query_params(query);
        
        if args.json {
            let json_params: HashMap<String, Vec<String>> = if args.decode {
                params.iter()
                    .map(|(k, v)| {
                        (decode_component(k), v.iter().map(|s| decode_component(s)).collect())
                    })
                    .collect()
            } else {
                params
            };
            
            println!("{}", serde_json::to_string_pretty(&json_params).unwrap());
        } else {
            println!("{}", "Query Parameters:".yellow());
            for (key, values) in &params {
                let key_display = if args.decode {
                    decode_component(key)
                } else {
                    key.to_string()
                };
                
                for value in values {
                    let value_display = if args.decode {
                        decode_component(value)
                    } else {
                        value.to_string()
                    };
                    println!("  {} = {}", key_display.bright_white(), value_display);
                }
            }
        }
    }
    
    if let Some(fragment) = parsed.fragment() {
        let fragment_display = if args.decode {
            decode_component(fragment)
        } else {
            fragment.to_string()
        };
        println!();
        println!("{:15} {}", "Fragment:".magenta(), fragment_display);
    }
    
    // Additional info
    println!();
    println!("{}", "Additional Info:".bold());
    println!("{:15} {}", "Has authority:".bright_black(), parsed.has_authority());
    println!("{:15} {}", "Cannot be base:".bright_black(), parsed.cannot_be_a_base());
    
    let username = parsed.username();
    if !username.is_empty() {
        println!("{:15} {}", "Username:".red(), username);
    }
    
    if let Some(pass) = parsed.password() {
        println!("{:15} {}", "Password:".red(), "*".repeat(pass.len()));
    }
}

mod urlencoding {
    pub fn decode(s: &str) -> Result<std::borrow::Cow<str>, std::string::FromUtf8Error> {
        let mut result = Vec::new();
        let mut chars = s.bytes();
        
        while let Some(byte) = chars.next() {
            if byte == b'%' {
                let high = chars.next().and_then(|b| char::from(b).to_digit(16));
                let low = chars.next().and_then(|b| char::from(b).to_digit(16));
                
                match (high, low) {
                    (Some(h), Some(l)) => result.push((h * 16 + l) as u8),
                    _ => {
                        result.push(b'%');
                    }
                }
            } else if byte == b'+' {
                result.push(b' ');
            } else {
                result.push(byte);
            }
        }
        
        String::from_utf8(result).map(std::borrow::Cow::Owned)
    }
}