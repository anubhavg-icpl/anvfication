use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use std::sync::Arc;
use tokio::task::JoinSet;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Fast TCP port scanner", long_about = None)]
struct Args {
    #[arg(help = "Target host to scan")]
    host: String,
    
    #[arg(short, long, default_value = "1", help = "Start port")]
    start: u16,
    
    #[arg(short, long, default_value = "1000", help = "End port")]
    end: u16,
    
    #[arg(short, long, default_value = "100", help = "Concurrent connections")]
    threads: usize,
}

async fn scan_port(host: String, port: u16) -> Option<u16> {
    let addr = format!("{}:{}", host, port);
    let timeout = Duration::from_millis(500);
    
    match tokio::time::timeout(
        timeout,
        tokio::net::TcpStream::connect(&addr)
    ).await {
        Ok(Ok(_)) => Some(port),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Scanning {} ports {}..{}", args.host, args.start, args.end);
    println!("Using {} concurrent connections", args.threads);
    
    let mut tasks = JoinSet::new();
    let mut open_ports = Vec::new();
    
    for port in args.start..=args.end {
        let host = args.host.clone();
        
        while tasks.len() >= args.threads {
            if let Some(result) = tasks.join_next().await {
                if let Ok(Some(open_port)) = result {
                    open_ports.push(open_port);
                    println!("Port {} is open", open_port);
                }
            }
        }
        
        tasks.spawn(scan_port(host, port));
    }
    
    while let Some(result) = tasks.join_next().await {
        if let Ok(Some(open_port)) = result {
            open_ports.push(open_port);
            println!("Port {} is open", open_port);
        }
    }
    
    println!("\nScan complete. Open ports: {:?}", open_ports);
}