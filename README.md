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

### 7. port-scanner
Fast TCP port scanner with concurrent connections.
```bash
cargo run --bin port-scanner -- example.com -s 1 -e 1000
```

### 8. file-hasher
Calculate file hashes with multiple algorithms (MD5, SHA256, SHA512).
```bash
cargo run --bin file-hasher -- file.txt --algorithm all
```

### 9. json-formatter
Format, validate, and colorize JSON data.
```bash
cat data.json | cargo run --bin json-formatter -- - --color
```

### 10. base64-tool
Encode and decode base64 data.
```bash
echo "Hello" | cargo run --bin base64-tool -- -
cargo run --bin base64-tool -- file.txt --decode
```

### 11. word-counter
Advanced word counter with frequency analysis.
```bash
cargo run --bin word-counter -- file.txt --frequency
```

### 12. timestamp
Convert between Unix timestamps and human-readable dates.
```bash
cargo run --bin timestamp -- now
cargo run --bin timestamp -- 1234567890
```

### 13. passgen
Generate secure passwords with strength checking.
```bash
cargo run --bin passgen -- --length 20 --check-strength
cargo run --bin passgen -- --memorable --length 30
```

### 14. url-parser
Parse and analyze URLs with query parameter extraction.
```bash
cargo run --bin url-parser -- "https://example.com/path?key=value" --decode
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