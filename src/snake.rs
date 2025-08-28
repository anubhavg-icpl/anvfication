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

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Game {
            game_over: false,
            dir: Direction::Stop,
            x: WIDTH / 2,
            y: HEIGHT / 2,
            fruit_x: rng.gen_range(0..WIDTH),
            fruit_y: rng.gen_range(0..HEIGHT),
            score: 0,
            tail_x: Vec::new(),
            tail_y: Vec::new(),
        }
    }
    
    fn draw(&self) {
        print!("\x1b[2J\x1b[1;1H"); // Clear screen
        
        // Top border
        for _ in 0..WIDTH + 2 {
            print!("#");
        }
        println!();
        
        // Game field
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if j == 0 {
                    print!("#");
                }
                
                if i == self.y && j == self.x {
                    print!("C");
                } else if i == self.fruit_y && j == self.fruit_x {
                    print!("o");
                } else {
                    let mut printed = false;
                    for k in 0..self.tail_x.len() {
                        if self.tail_x[k] == j && self.tail_y[k] == i {
                            print!("c");
                            printed = true;
                            break;
                        }
                    }
                    if !printed {
                        print!(" ");
                    }
                }
                
                if j == WIDTH - 1 {
                    print!("#");
                }
            }
            println!();
        }
        
        // Bottom border
        for _ in 0..WIDTH + 2 {
            print!("#");
        }
        println!();
        println!("Score: {}", self.score);
    }
}

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.draw();
    Ok(())
}