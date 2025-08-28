use clap::Parser;
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate secure passwords", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "16", help = "Password length")]
    length: usize,
    
    #[arg(short = 'n', long, default_value = "1", help = "Number of passwords")]
    count: usize,
    
    #[arg(short = 'u', long, help = "Include uppercase letters")]
    upper: bool,
    
    #[arg(short = 'l', long, help = "Include lowercase letters")]
    lower: bool,
    
    #[arg(short = 'd', long, help = "Include digits")]
    digits: bool,
    
    #[arg(short = 's', long, help = "Include special characters")]
    special: bool,
    
    #[arg(long, help = "Custom character set")]
    custom: Option<String>,
    
    #[arg(long, help = "Exclude similar characters (0O, 1lI)")]
    no_similar: bool,
    
    #[arg(long, help = "Generate memorable password (word-based)")]
    memorable: bool,
    
    #[arg(long, help = "Check password strength")]
    check_strength: bool,
}

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SPECIAL: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
const SIMILAR: &str = "0O1lI";

const WORDS: &[&str] = &[
    "correct", "horse", "battery", "staple", "dragon", "phoenix", "thunder", "lightning",
    "crystal", "shadow", "mystic", "cosmic", "stellar", "nebula", "quantum", "cipher",
    "matrix", "vector", "delta", "omega", "alpha", "beta", "gamma", "nova",
];

fn calculate_entropy(charset_size: usize, length: usize) -> f64 {
    let combinations = (charset_size as f64).powi(length as i32);
    combinations.log2()
}

fn check_password_strength(password: &str) -> (String, String) {
    let mut charset_size = 0;
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_special = false;
    
    for ch in password.chars() {
        if ch.is_lowercase() && !has_lower {
            charset_size += 26;
            has_lower = true;
        }
        if ch.is_uppercase() && !has_upper {
            charset_size += 26;
            has_upper = true;
        }
        if ch.is_ascii_digit() && !has_digit {
            charset_size += 10;
            has_digit = true;
        }
        if !ch.is_alphanumeric() && !has_special {
            charset_size += 32;
            has_special = true;
        }
    }
    
    let entropy = calculate_entropy(charset_size, password.len());
    
    let strength = if entropy < 30.0 {
        "Very Weak".red()
    } else if entropy < 50.0 {
        "Weak".yellow()
    } else if entropy < 70.0 {
        "Fair".yellow()
    } else if entropy < 90.0 {
        "Strong".green()
    } else {
        "Very Strong".bright_green()
    };
    
    (strength.to_string(), format!("{:.1} bits", entropy))
}

fn generate_password(charset: &str, length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();
    
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

fn generate_memorable_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    
    while password.len() < length {
        let word = WORDS.choose(&mut rng).unwrap();
        if password.is_empty() {
            password.push_str(word);
        } else {
            password.push('-');
            password.push_str(word);
        }
        
        // Add random number
        if rng.gen_bool(0.5) {
            password.push_str(&rng.gen_range(0..100).to_string());
        }
    }
    
    // Capitalize random word
    let words: Vec<&str> = password.split('-').collect();
    if !words.is_empty() {
        let idx = rng.gen_range(0..words.len());
        let mut result = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if i == idx {
                result.push(word.chars()
                    .enumerate()
                    .map(|(i, c)| if i == 0 { c.to_uppercase().to_string() } else { c.to_string() })
                    .collect::<String>());
            } else {
                result.push(word.to_string());
            }
        }
        password = result.join("-");
    }
    
    if password.len() > length {
        password.truncate(length);
    }
    
    password
}

fn main() {
    let args = Args::parse();
    
    if args.memorable {
        println!("{}", "Memorable Passwords:".bold());
        println!("─────────────────────");
        
        for _ in 0..args.count {
            let password = generate_memorable_password(args.length);
            
            if args.check_strength {
                let (strength, entropy) = check_password_strength(&password);
                println!("{} [{}] ({})", password.bright_white(), strength, entropy);
            } else {
                println!("{}", password.bright_white());
            }
        }
        return;
    }
    
    // Build character set
    let mut charset = String::new();
    
    if let Some(custom) = args.custom {
        charset = custom;
    } else {
        // Default: all character types if none specified
        let use_all = !args.upper && !args.lower && !args.digits && !args.special;
        
        if args.lower || use_all {
            charset.push_str(LOWERCASE);
        }
        if args.upper || use_all {
            charset.push_str(UPPERCASE);
        }
        if args.digits || use_all {
            charset.push_str(DIGITS);
        }
        if args.special || use_all {
            charset.push_str(SPECIAL);
        }
    }
    
    // Remove similar characters if requested
    if args.no_similar {
        charset = charset.chars()
            .filter(|c| !SIMILAR.contains(*c))
            .collect();
    }
    
    if charset.is_empty() {
        eprintln!("{} No character set specified!", "Error:".red());
        std::process::exit(1);
    }
    
    // Show charset info
    println!("{} {} characters", "Character set:".cyan(), charset.len());
    println!("{} {:.1} bits per password\n", "Entropy:".cyan(), 
             calculate_entropy(charset.len(), args.length));
    
    println!("{}", "Generated Passwords:".bold());
    println!("─────────────────────");
    
    for _ in 0..args.count {
        let password = generate_password(&charset, args.length);
        
        if args.check_strength {
            let (strength, entropy) = check_password_strength(&password);
            println!("{} [{}] ({})", password.bright_white(), strength, entropy);
        } else {
            println!("{}", password.bright_white());
        }
    }
}