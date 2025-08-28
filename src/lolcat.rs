use std::io::{self, Read};
use std::f64::consts::PI;

fn color_screen(i: usize, freq: f64) -> (u8, u8, u8) {
    let freq = if freq == 0.0 { 0.1 } else { freq };
    let i = i as f64;
    
    let r = (freq * i + 0.0).sin() * 127.0 + 128.0;
    let g = (freq * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let b = (freq * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    
    (r as u8, g as u8, b as u8)
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    
    for (i, ch) in buffer.chars().enumerate() {
        let (r, g, b) = color_screen(i, 0.1);
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch);
    }
}