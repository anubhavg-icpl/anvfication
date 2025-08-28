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
    
    fn input(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('a') | KeyCode::Left => self.dir = Direction::Left,
                    KeyCode::Char('d') | KeyCode::Right => self.dir = Direction::Right,
                    KeyCode::Char('w') | KeyCode::Up => self.dir = Direction::Up,
                    KeyCode::Char('s') | KeyCode::Down => self.dir = Direction::Down,
                    KeyCode::Char('x') | KeyCode::Esc => self.game_over = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
    
    fn logic(&mut self) {
        // Move tail
        let mut prev_x = if !self.tail_x.is_empty() { self.tail_x[0] } else { self.x };
        let mut prev_y = if !self.tail_y.is_empty() { self.tail_y[0] } else { self.y };
        
        if !self.tail_x.is_empty() {
            self.tail_x[0] = self.x;
            self.tail_y[0] = self.y;
        }
        
        for i in 1..self.tail_x.len() {
            let prev2_x = self.tail_x[i];
            let prev2_y = self.tail_y[i];
            self.tail_x[i] = prev_x;
            self.tail_y[i] = prev_y;
            prev_x = prev2_x;
            prev_y = prev2_y;
        }
        
        // Move head
        match self.dir {
            Direction::Left => {
                if self.x > 0 { self.x -= 1; } else { self.game_over = true; }
            }
            Direction::Right => {
                if self.x < WIDTH - 1 { self.x += 1; } else { self.game_over = true; }
            }
            Direction::Up => {
                if self.y > 0 { self.y -= 1; } else { self.game_over = true; }
            }
            Direction::Down => {
                if self.y < HEIGHT - 1 { self.y += 1; } else { self.game_over = true; }
            }
            Direction::Stop => {}
        }
        
        // Check collision with tail
        for i in 0..self.tail_x.len() {
            if self.tail_x[i] == self.x && self.tail_y[i] == self.y {
                self.game_over = true;
            }
        }
        
        // Check fruit collection
        if self.x == self.fruit_x && self.y == self.fruit_y {
            self.score += 10;
            let mut rng = rand::thread_rng();
            self.fruit_x = rng.gen_range(0..WIDTH);
            self.fruit_y = rng.gen_range(0..HEIGHT);
            self.tail_x.push(0);
            self.tail_y.push(0);
        }
    }
}

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.draw();
    Ok(())
}