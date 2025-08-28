use notify_rust::Notification;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

const BATTERY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const LOW_THRESHOLD: u32 = 20;

fn main() {
    println!("Battery monitor starting...");
}