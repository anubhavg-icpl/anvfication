use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::io;
use std::time::Duration;

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Stop,
    Left,
    Right,
    Up,
    Down,
}

struct Game {
    game_over: bool,
    dir: Direction,
    x: usize,
    y: usize,
    fruit_x: usize,
    fruit_y: usize,
    score: usize,
    tail_x: Vec<usize>,
    tail_y: Vec<usize>,
}

fn main() -> io::Result<()> {
    println!("Snake game starting...");
    Ok(())
}