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
        std::process::exit(1);
    }
}