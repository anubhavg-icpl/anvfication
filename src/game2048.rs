use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::io::{self, Write};

const GRID_SIZE: usize = 4;

#[derive(Clone, Copy, PartialEq)]
struct Tile {
    value: u32,
}

impl Tile {
    fn new(value: u32) -> Self {
        Self { value }
    }
    
    fn color(&self) -> Color {
        match self.value {
            2 => Color::Rgb { r: 238, g: 228, b: 218 },
            4 => Color::Rgb { r: 237, g: 224, b: 200 },
            8 => Color::Rgb { r: 242, g: 177, b: 121 },
            16 => Color::Rgb { r: 245, g: 149, b: 99 },
            32 => Color::Rgb { r: 246, g: 124, b: 95 },
            64 => Color::Rgb { r: 246, g: 94, b: 59 },
            128 => Color::Rgb { r: 237, g: 207, b: 114 },
            256 => Color::Rgb { r: 237, g: 204, b: 97 },
            512 => Color::Rgb { r: 237, g: 200, b: 80 },
            1024 => Color::Rgb { r: 237, g: 197, b: 63 },
            2048 => Color::Rgb { r: 237, g: 194, b: 46 },
            _ => Color::Rgb { r: 60, g: 58, b: 50 },
        }
    }
    
    fn text_color(&self) -> Color {
        if self.value <= 4 {
            Color::Rgb { r: 119, g: 110, b: 101 }
        } else {
            Color::Rgb { r: 249, g: 246, b: 242 }
        }
    }
}

struct Game {
    grid: [[Option<Tile>; GRID_SIZE]; GRID_SIZE],
    score: u32,
    best_score: u32,
    game_over: bool,
    won: bool,
    moves: u32,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            grid: [[None; GRID_SIZE]; GRID_SIZE],
            score: 0,
            best_score: 0,
            game_over: false,
            won: false,
            moves: 0,
        };
        game.add_random_tile();
        game.add_random_tile();
        game
    }
    
    fn add_random_tile(&mut self) {
        let mut empty_cells = Vec::new();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if self.grid[y][x].is_none() {
                    empty_cells.push((x, y));
                }
            }
        }
        
        if !empty_cells.is_empty() {
            let mut rng = rand::thread_rng();
            let (x, y) = empty_cells[rng.gen_range(0..empty_cells.len())];
            let value = if rng.gen_range(0..10) < 9 { 2 } else { 4 };
            self.grid[y][x] = Some(Tile::new(value));
        }
    }
    
    fn move_tiles(&mut self, direction: Direction) -> bool {
        let old_grid = self.grid.clone();
        
        match direction {
            Direction::Left => {
                for y in 0..GRID_SIZE {
                    let mut row = Vec::new();
                    for x in 0..GRID_SIZE {
                        if let Some(tile) = self.grid[y][x] {
                            row.push(tile.value);
                        }
                    }
                    let new_row = self.merge_line(row);
                    for x in 0..GRID_SIZE {
                        self.grid[y][x] = if x < new_row.len() {
                            Some(Tile::new(new_row[x]))
                        } else {
                            None
                        };
                    }
                }
            }
            Direction::Right => {
                for y in 0..GRID_SIZE {
                    let mut row = Vec::new();
                    for x in (0..GRID_SIZE).rev() {
                        if let Some(tile) = self.grid[y][x] {
                            row.push(tile.value);
                        }
                    }
                    let new_row = self.merge_line(row);
                    for x in 0..GRID_SIZE {
                        let idx = GRID_SIZE - 1 - x;
                        self.grid[y][idx] = if x < new_row.len() {
                            Some(Tile::new(new_row[x]))
                        } else {
                            None
                        };
                    }
                }
            }
            Direction::Up => {
                for x in 0..GRID_SIZE {
                    let mut col = Vec::new();
                    for y in 0..GRID_SIZE {
                        if let Some(tile) = self.grid[y][x] {
                            col.push(tile.value);
                        }
                    }
                    let new_col = self.merge_line(col);
                    for y in 0..GRID_SIZE {
                        self.grid[y][x] = if y < new_col.len() {
                            Some(Tile::new(new_col[y]))
                        } else {
                            None
                        };
                    }
                }
            }
            Direction::Down => {
                for x in 0..GRID_SIZE {
                    let mut col = Vec::new();
                    for y in (0..GRID_SIZE).rev() {
                        if let Some(tile) = self.grid[y][x] {
                            col.push(tile.value);
                        }
                    }
                    let new_col = self.merge_line(col);
                    for y in 0..GRID_SIZE {
                        let idx = GRID_SIZE - 1 - y;
                        self.grid[idx][x] = if y < new_col.len() {
                            Some(Tile::new(new_col[y]))
                        } else {
                            None
                        };
                    }
                }
            }
        }
        
        let moved = self.grid != old_grid;
        
        if moved {
            self.moves += 1;
            self.add_random_tile();
            self.check_game_state();
        }
        
        moved
    }
    
    fn merge_line(&mut self, line: Vec<u32>) -> Vec<u32> {
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < line.len() {
            if i + 1 < line.len() && line[i] == line[i + 1] {
                let merged = line[i] * 2;
                result.push(merged);
                self.score += merged;
                if self.score > self.best_score {
                    self.best_score = self.score;
                }
                if merged == 2048 {
                    self.won = true;
                }
                i += 2;
            } else {
                result.push(line[i]);
                i += 1;
            }
        }
        
        result
    }
    
    fn check_game_state(&mut self) {
        // Check if any move is possible
        let mut has_empty = false;
        let mut has_merge = false;
        
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if self.grid[y][x].is_none() {
                    has_empty = true;
                }
                
                if let Some(tile) = self.grid[y][x] {
                    // Check horizontal merge
                    if x < GRID_SIZE - 1 {
                        if let Some(right) = self.grid[y][x + 1] {
                            if tile.value == right.value {
                                has_merge = true;
                            }
                        }
                    }
                    // Check vertical merge
                    if y < GRID_SIZE - 1 {
                        if let Some(down) = self.grid[y + 1][x] {
                            if tile.value == down.value {
                                has_merge = true;
                            }
                        }
                    }
                }
            }
        }
        
        self.game_over = !has_empty && !has_merge;
    }
    
    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        
        // Title
        stdout.queue(SetForegroundColor(Color::Yellow))?;
        stdout.queue(Print("2048 - Rust Edition"))?;
        stdout.queue(ResetColor)?;
        stdout.queue(Print(format!("  Score: {} | Best: {} | Moves: {}", self.score, self.best_score, self.moves)))?;
        
        // Grid
        for y in 0..GRID_SIZE {
            stdout.queue(cursor::MoveTo(0, (y * 3 + 2) as u16))?;
            stdout.queue(Print("┌─────┬─────┬─────┬─────┐"))?;
            
            stdout.queue(cursor::MoveTo(0, (y * 3 + 3) as u16))?;
            for x in 0..GRID_SIZE {
                stdout.queue(Print("│"))?;
                
                if let Some(tile) = self.grid[y][x] {
                    stdout.queue(SetBackgroundColor(tile.color()))?;
                    stdout.queue(SetForegroundColor(tile.text_color()))?;
                    let text = format!("{:^5}", tile.value);
                    stdout.queue(Print(text))?;
                    stdout.queue(ResetColor)?;
                } else {
                    stdout.queue(Print("     "))?;
                }
            }
            stdout.queue(Print("│"))?;
            
            if y < GRID_SIZE - 1 {
                stdout.queue(cursor::MoveTo(0, (y * 3 + 4) as u16))?;
                stdout.queue(Print("├─────┼─────┼─────┼─────┤"))?;
            }
        }
        
        stdout.queue(cursor::MoveTo(0, (GRID_SIZE * 3 + 1) as u16))?;
        stdout.queue(Print("└─────┴─────┴─────┴─────┘"))?;
        
        // Instructions
        let inst_y = (GRID_SIZE * 3 + 3) as u16;
        stdout.queue(cursor::MoveTo(0, inst_y))?;
        stdout.queue(Print("Use arrow keys or WASD to move tiles"))?;
        stdout.queue(cursor::MoveTo(0, inst_y + 1))?;
        stdout.queue(Print("Press R to restart, Q to quit"))?;
        
        if self.won && !self.game_over {
            stdout.queue(cursor::MoveTo(0, inst_y + 3))?;
            stdout.queue(SetForegroundColor(Color::Green))?;
            stdout.queue(Print("🎉 You reached 2048! Keep playing for a higher score!"))?;
            stdout.queue(ResetColor)?;
        }
        
        if self.game_over {
            stdout.queue(cursor::MoveTo(0, inst_y + 3))?;
            stdout.queue(SetForegroundColor(Color::Red))?;
            stdout.queue(Print(format!("Game Over! Final Score: {}", self.score)))?;
            stdout.queue(ResetColor)?;
        }
        
        stdout.flush()?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up, Down, Left, Right,
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide)?;
    
    let mut game = Game::new();
    
    loop {
        game.draw(&mut stdout)?;
        
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up | KeyCode::Char('w') => { game.move_tiles(Direction::Up); }
                    KeyCode::Down | KeyCode::Char('s') => { game.move_tiles(Direction::Down); }
                    KeyCode::Left | KeyCode::Char('a') => { game.move_tiles(Direction::Left); }
                    KeyCode::Right | KeyCode::Char('d') => { game.move_tiles(Direction::Right); }
                    KeyCode::Char('r') => { game = Game::new(); }
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }
    
    stdout.execute(cursor::Show)?;
    stdout.execute(ResetColor)?;
    terminal::disable_raw_mode()?;
    stdout.execute(Clear(ClearType::All))?;
    println!("Thanks for playing! Final Score: {}", game.score);
    
    Ok(())
}