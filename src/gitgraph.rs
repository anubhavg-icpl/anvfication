use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    add: Option<String>,
    
    #[arg(short, long, default_value = "your@email.com")]
    email: String,
}

fn get_path(dir: &str, file: &mut std::fs::File) -> std::io::Result<()> {
    let skip_dirs = vec!["node_modules", ".cargo", "pip", ".cache", ".config", ".local"];
    
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            !skip_dirs.contains(&e.file_name().to_str().unwrap_or(""))
        }) {
        let entry = entry?;
        let path = entry.path();
        
        if path.file_name() == Some(std::ffi::OsStr::new(".git")) {
            if path.is_dir() {
                let parent = path.parent().unwrap();
                writeln!(file, "{}", parent.display())?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    if let Some(dir) = args.add {
        let filename = format!("{}.gitgraph", args.email);
        
        let handle = tokio::spawn(async move {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&filename)
                .expect("Failed to open file");
                
            if let Err(e) = get_path(&dir, &mut file) {
                eprintln!("Error: {}", e);
            }
        });
        
        handle.await.unwrap();
    }
}