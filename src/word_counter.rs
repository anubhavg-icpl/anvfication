use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about = "Count words, lines, and characters", long_about = None)]
struct Args {
    #[arg(help = "Files to count (use '-' for stdin)")]
    files: Vec<String>,
    
    #[arg(short, long, help = "Count lines")]
    lines: bool,
    
    #[arg(short, long, help = "Count words")]
    words: bool,
    
    #[arg(short = 'c', long, help = "Count bytes")]
    bytes: bool,
    
    #[arg(short = 'm', long, help = "Count characters")]
    chars: bool,
    
    #[arg(short = 'L', long, help = "Show longest line length")]
    max_line_length: bool,
    
    #[arg(long, help = "Show word frequency")]
    frequency: bool,
}

#[derive(Default, Debug)]
struct FileStats {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    max_line_length: usize,
    word_freq: HashMap<String, usize>,
}

fn count_file(reader: Box<dyn BufRead>) -> io::Result<FileStats> {
    let mut stats = FileStats::default();
    
    for line in reader.lines() {
        let line = line?;
        stats.lines += 1;
        stats.chars += line.chars().count() + 1; // +1 for newline
        stats.bytes += line.len() + 1; // +1 for newline
        
        if line.len() > stats.max_line_length {
            stats.max_line_length = line.len();
        }
        
        for word in line.split_whitespace() {
            stats.words += 1;
            let word_lower = word.to_lowercase();
            *stats.word_freq.entry(word_lower).or_insert(0) += 1;
        }
    }
    
    Ok(stats)
}

fn print_stats(stats: &FileStats, filename: &str, args: &Args) {
    let mut output = Vec::new();
    
    if args.lines {
        output.push(format!("{:8}", stats.lines));
    }
    if args.words {
        output.push(format!("{:8}", stats.words));
    }
    if args.bytes {
        output.push(format!("{:8}", stats.bytes));
    }
    if args.chars {
        output.push(format!("{:8}", stats.chars));
    }
    if args.max_line_length {
        output.push(format!("{:8}", stats.max_line_length));
    }
    
    // Default: show lines, words, bytes
    if output.is_empty() {
        output.push(format!("{:8}", stats.lines));
        output.push(format!("{:8}", stats.words));
        output.push(format!("{:8}", stats.bytes));
    }
    
    println!("{} {}", output.join(" "), filename.cyan());
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let files = if args.files.is_empty() {
        vec!["-".to_string()]
    } else {
        args.files.clone()
    };
    
    let mut total_stats = FileStats::default();
    let mut file_count = 0;
    
    for filename in &files {
        let reader: Box<dyn BufRead> = if filename == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            if !Path::new(filename).exists() {
                eprintln!("{}: No such file", filename);
                continue;
            }
            Box::new(BufReader::new(File::open(filename)?))
        };
        
        match count_file(reader) {
            Ok(stats) => {
                print_stats(&stats, filename, &args);
                
                // Add to totals
                total_stats.lines += stats.lines;
                total_stats.words += stats.words;
                total_stats.chars += stats.chars;
                total_stats.bytes += stats.bytes;
                if stats.max_line_length > total_stats.max_line_length {
                    total_stats.max_line_length = stats.max_line_length;
                }
                for (word, count) in stats.word_freq {
                    *total_stats.word_freq.entry(word).or_insert(0) += count;
                }
                
                file_count += 1;
            }
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }
    
    // Print totals if multiple files
    if file_count > 1 {
        println!("---");
        print_stats(&total_stats, "total", &args);
    }
    
    // Print word frequency if requested
    if args.frequency && !total_stats.word_freq.is_empty() {
        println!("\n{}", "Top 10 Most Frequent Words:".bold());
        let mut freq_vec: Vec<_> = total_stats.word_freq.iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        for (i, (word, count)) in freq_vec.iter().take(10).enumerate() {
            println!("{:2}. {:20} {}", i + 1, word.green(), count);
        }
    }
    
    Ok(())
}