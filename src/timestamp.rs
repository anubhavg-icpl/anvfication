use chrono::{DateTime, Local, TimeZone, Utc, NaiveDateTime};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about = "Convert between timestamps and dates", long_about = None)]
struct Args {
    #[arg(help = "Input value (timestamp or date string, 'now' for current time)")]
    input: Option<String>,
    
    #[arg(short, long, help = "Output format (default: multiple formats)")]
    format: Option<String>,
    
    #[arg(short, long, help = "Input timezone (default: local)")]
    timezone: Option<String>,
    
    #[arg(short, long, help = "Convert to Unix timestamp")]
    unix: bool,
    
    #[arg(short, long, help = "Show in milliseconds")]
    millis: bool,
}

fn parse_timestamp(input: &str) -> Option<DateTime<Utc>> {
    // Try parsing as Unix timestamp (seconds)
    if let Ok(ts) = input.parse::<i64>() {
        if ts > 1_000_000_000_000 {
            // Likely milliseconds
            return Utc.timestamp_opt(ts / 1000, ((ts % 1000) * 1_000_000) as u32).single();
        } else {
            return Utc.timestamp_opt(ts, 0).single();
        }
    }
    
    // Try parsing as ISO 8601
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Some(dt.with_timezone(&Utc));
    }
    
    // Try parsing as RFC 2822
    if let Ok(dt) = DateTime::parse_from_rfc2822(input) {
        return Some(dt.with_timezone(&Utc));
    }
    
    // Try common date formats
    let formats = vec![
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%d-%m-%Y %H:%M:%S",
        "%Y-%m-%d",
        "%Y/%m/%d",
        "%d/%m/%Y",
    ];
    
    for fmt in formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(input, fmt) {
            return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
        }
        // Try without time
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(input, fmt) {
            let naive_dt = naive_date.and_hms_opt(0, 0, 0)?;
            return Some(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
        }
    }
    
    None
}

fn main() {
    let args = Args::parse();
    
    let dt = match args.input.as_deref() {
        Some("now") | None => Utc::now(),
        Some(input) => {
            match parse_timestamp(input) {
                Some(dt) => dt,
                None => {
                    eprintln!("{} Could not parse: {}", "Error:".red(), input);
                    eprintln!("Try Unix timestamp, ISO 8601, or YYYY-MM-DD HH:MM:SS format");
                    std::process::exit(1);
                }
            }
        }
    };
    
    if args.unix {
        if args.millis {
            println!("{}", dt.timestamp_millis());
        } else {
            println!("{}", dt.timestamp());
        }
        return;
    }
    
    // Custom format
    if let Some(fmt) = args.format {
        let local = dt.with_timezone(&Local);
        println!("{}", local.format(&fmt));
        return;
    }
    
    // Default: show multiple formats
    let local = dt.with_timezone(&Local);
    
    println!("{}", "Timestamp Conversions:".bold());
    println!("─────────────────────");
    
    println!("{:20} {}", "Unix (seconds):".cyan(), dt.timestamp());
    println!("{:20} {}", "Unix (millis):".cyan(), dt.timestamp_millis());
    println!("{:20} {}", "Unix (micros):".cyan(), dt.timestamp_micros());
    
    println!();
    println!("{:20} {}", "UTC:".green(), dt.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("{:20} {}", "Local:".green(), local.format("%Y-%m-%d %H:%M:%S %Z"));
    println!("{:20} {}", "ISO 8601:".green(), dt.to_rfc3339());
    println!("{:20} {}", "RFC 2822:".green(), dt.to_rfc2822());
    
    println!();
    println!("{:20} {}", "Date:".yellow(), local.format("%A, %B %d, %Y"));
    println!("{:20} {}", "Time:".yellow(), local.format("%I:%M:%S %p"));
    println!("{:20} {}", "Week:".yellow(), local.format("Week %U of %Y"));
    println!("{:20} {}", "Day of Year:".yellow(), local.format("Day %j"));
}