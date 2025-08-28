# Anvfication - Multi-Tool Rust Project

A collection of useful command-line tools written in Rust by anubhavg@infopercept.com

## Tools Included

### 1. gitgraph
Recursively finds all git repositories in a directory tree.
```bash
cargo run --bin gitgraph -- --add /path/to/search --email your@email.com
```

### 2. http-status
Concurrent HTTP status checker for multiple URLs.
```bash
cargo run --bin http-status -- https://example.com paths.txt
```

### 3. lolcat
Rainbow text colorizer for terminal output.
```bash
echo "Hello World" | cargo run --bin lolcat
```

### 4. snake
Classic snake game in the terminal.
```bash
cargo run --bin snake
```

### 5. dns-resolver
Simple DNS resolver for domain names.
```bash
cargo run --bin dns-resolver -- example.com
```

### 6. battery-monitor
Battery level monitor with low battery notifications.
```bash
cargo run --bin battery-monitor
```

## Building

Build all binaries:
```bash
cargo build --release
```

Build specific binary:
```bash
cargo build --release --bin lolcat
```

## Installation

Install all binaries to ~/.cargo/bin/:
```bash
cargo install --path .
```

## Requirements

- Rust 2021 edition or later
- Linux/macOS/Windows (some tools may be OS-specific)
- For battery-monitor: Linux with /sys/class/power_supply/

## Author

anubhavg@infopercept.com (anubhavg-icpl)