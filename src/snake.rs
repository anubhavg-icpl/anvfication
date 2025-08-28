use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{Duration, Instant};

const WIDTH: u16 = 30;
const HEIGHT: u16 = 20;
const INITIAL_SPEED: u64 = 150; // milliseconds

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position {
    x: u16,
    y: u16,
}

struct Game {
    snake: VecDeque<Position>,
    direction: Direction,
    next_direction: Direction,
    food: Position,
    score: u32,
    game_over: bool,
    speed: u64,
    last_update: Instant,
}

impl Game {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position { x: WIDTH / 2, y: HEIGHT / 2 });
        snake.push_back(Position { x: WIDTH / 2 - 1, y: HEIGHT / 2 });
        snake.push_back(Position { x: WIDTH / 2 - 2, y: HEIGHT / 2 });
        
        let mut game = Game {
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position { x: 0, y: 0 },
            score: 0,
            game_over: false,
            speed: INITIAL_SPEED,
            last_update: Instant::now(),
        };
        
        game.spawn_food();
        game
    }
    
    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let pos = Position {
                x: rng.gen_range(1..WIDTH - 1),
                y: rng.gen_range(1..HEIGHT - 1),
            };
            
            if !self.snake.contains(&pos) {
                self.food = pos;
                break;
            }
        }
    }
    
    fn update(&mut self) {
        if self.game_over {
            return;
        }
        
        // Update direction
        self.direction = self.next_direction;
        
        // Calculate new head position
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => {
                if head.y == 1 {
                    self.game_over = true;
                    return;
                }
                Position { x: head.x, y: head.y - 1 }
            }
            Direction::Down => {
                if head.y >= HEIGHT - 2 {
                    self.game_over = true;
                    return;
                }
                Position { x: head.x, y: head.y + 1 }
            }
            Direction::Left => {
                if head.x == 1 {
                    self.game_over = true;
                    return;
                }
                Position { x: head.x - 1, y: head.y }
            }
            Direction::Right => {
                if head.x >= WIDTH - 2 {
                    self.game_over = true;
                    return;
                }
                Position { x: head.x + 1, y: head.y }
            }
        };
        
        // Check self collision
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }
        
        // Move snake
        self.snake.push_front(new_head);
        
        // Check if food is eaten
        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
            // Speed up slightly
            if self.speed > 50 {
                self.speed = (self.speed as f64 * 0.95) as u64;
            }
        } else {
            self.snake.pop_back();
        }
    }
    
    fn handle_input(&mut self, key: KeyCode) {
        let new_dir = match key {
            KeyCode::Up | KeyCode::Char('w') => Direction::Up,
            KeyCode::Down | KeyCode::Char('s') => Direction::Down,
            KeyCode::Left | KeyCode::Char('a') => Direction::Left,
            KeyCode::Right | KeyCode::Char('d') => Direction::Right,
            KeyCode::Esc | KeyCode::Char('q') => {
                self.game_over = true;
                return;
            }
            _ => return,
        };
        
        // Prevent going back into itself
        let valid = match (self.direction, new_dir) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => false,
            _ => true,
        };
        
        if valid {
            self.next_direction = new_dir;
        }
    }
    
    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        // Clear screen
        stdout.queue(Clear(ClearType::All))?;
        
        // Draw borders
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        
        // Top border
        for x in 0..WIDTH {
            stdout.queue(cursor::MoveTo(x, 0))?;
            stdout.queue(Print("█"))?;
        }
        
        // Side borders and game area
        for y in 1..HEIGHT - 1 {
            // Left border
            stdout.queue(cursor::MoveTo(0, y))?;
            stdout.queue(Print("█"))?;
            
            // Right border
            stdout.queue(cursor::MoveTo(WIDTH - 1, y))?;
            stdout.queue(Print("█"))?;
        }
        
        // Bottom border
        for x in 0..WIDTH {
            stdout.queue(cursor::MoveTo(x, HEIGHT - 1))?;
            stdout.queue(Print("█"))?;
        }
        
        // Draw snake
        for (i, segment) in self.snake.iter().enumerate() {
            stdout.queue(cursor::MoveTo(segment.x, segment.y))?;
            if i == 0 {
                // Head
                stdout.queue(SetForegroundColor(Color::Green))?;
                stdout.queue(Print("●"))?;
            } else {
                // Body
                stdout.queue(SetForegroundColor(Color::DarkGreen))?;
                stdout.queue(Print("○"))?;
            }
        }
        
        // Draw food
        stdout.queue(cursor::MoveTo(self.food.x, self.food.y))?;
        stdout.queue(SetForegroundColor(Color::Red))?;
        stdout.queue(Print("♦"))?;
        
        // Draw score and instructions
        stdout.queue(ResetColor)?;
        stdout.queue(cursor::MoveTo(0, HEIGHT))?;
        stdout.queue(Print(format!(
            "Score: {} | Length: {} | Speed: {}ms | Use WASD/Arrows to move, ESC/Q to quit",
            self.score, self.snake.len(), self.speed
        )))?;
        
        // Game over message
        if self.game_over {
            let msg = "GAME OVER! Press any key to exit...";
            let x = (WIDTH - msg.len() as u16) / 2;
            let y = HEIGHT / 2;
            
            stdout.queue(cursor::MoveTo(x - 1, y - 1))?;
            stdout.queue(SetBackgroundColor(Color::Red))?;
            stdout.queue(SetForegroundColor(Color::White))?;
            stdout.queue(Print(format!(" {} ", " ".repeat(msg.len()))))?;
            
            stdout.queue(cursor::MoveTo(x - 1, y))?;
            stdout.queue(Print(format!(" {} ", msg)))?;
            
            stdout.queue(cursor::MoveTo(x - 1, y + 1))?;
            stdout.queue(Print(format!(" {} ", " ".repeat(msg.len()))))?;
            
            stdout.queue(ResetColor)?;
        }
        
        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide)?;
    stdout.execute(Clear(ClearType::All))?;
    
    let mut game = Game::new();
    
    // Game loop
    loop {
        // Draw
        game.draw(&mut stdout)?;
        
        if game.game_over {
            // Wait for key press before exiting
            loop {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(_) = event::read()? {
                        break;
                    }
                }
            }
            break;
        }
        
        // Handle input
        while event::poll(Duration::from_millis(1))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                game.handle_input(code);
            }
        }
        
        // Update game state at the right speed
        if game.last_update.elapsed() >= Duration::from_millis(game.speed) {
            game.update();
            game.last_update = Instant::now();
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    
    // Cleanup
    stdout.execute(cursor::Show)?;
    stdout.execute(ResetColor)?;
    terminal::disable_raw_mode()?;
    stdout.execute(Clear(ClearType::All))?;
    println!("Thanks for playing! Final Score: {}", game.score);
    
    Ok(())
}