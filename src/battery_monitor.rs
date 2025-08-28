use notify_rust::Notification;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

const BATTERY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const LOW_THRESHOLD: u32 = 20;

fn check_battery() -> Result<u32, std::io::Error> {
    let content = fs::read_to_string(BATTERY_PATH)?;
    content.trim().parse::<u32>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn push_notification(level: u32) {
    let message = format!("Your system battery is running low at {}%", level);
    
    if let Err(e) = Notification::new()
        .summary("Low Battery")
        .body(&message)
        .icon("battery-low")
        .timeout(5000)
        .show() {
        eprintln!("Failed to show notification: {}", e);
    }
}

fn main() {
    println!("Battery monitor starting...");
    
    if !Path::new(BATTERY_PATH).exists() {
        eprintln!("Battery capacity file not found at {}", BATTERY_PATH);
        eprintln!("This might be a different system or no battery present");
        eprintln!("Note: On macOS, use 'pmset -g batt' instead");
        std::process::exit(1);
    }
    
    println!("Monitoring battery level...");
    println!("Press Ctrl+C to stop");
    
    loop {
        match check_battery() {
            Ok(level) => {
                println!("Battery level: {}%", level);
                
                if level <= LOW_THRESHOLD {
                    push_notification(level);
                }
                
                // Sleep duration based on battery level
                let sleep_seconds = if level <= LOW_THRESHOLD {
                    60 // Check every minute when low
                } else {
                    level as u64 * 10 // Check less frequently when higher
                };
                
                thread::sleep(Duration::from_secs(sleep_seconds));
            }
            Err(e) => {
                eprintln!("Error reading battery: {}", e);
                thread::sleep(Duration::from_secs(60));
            }
        }
    }
}