use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::io::{self, Write};
use std::time::Instant;

const GRID_WIDTH: usize = 12;
const GRID_HEIGHT: usize = 10;
const NUM_MINES: usize = 15;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy)]
struct Cell {
    is_mine: bool,
    state: CellState,
    adjacent_mines: u8,
}

struct Game {
    grid: [[Cell; GRID_WIDTH]; GRID_HEIGHT],
    cursor_x: usize,
    cursor_y: usize,
    game_over: bool,
    won: bool,
    mines_left: i32,
    cells_revealed: usize,
    start_time: Option<Instant>,
    elapsed_time: u64,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            grid: [[Cell {
                is_mine: false,
                state: CellState::Hidden,
                adjacent_mines: 0,
            }; GRID_WIDTH]; GRID_HEIGHT],
            cursor_x: GRID_WIDTH / 2,
            cursor_y: GRID_HEIGHT / 2,
            game_over: false,
            won: false,
            mines_left: NUM_MINES as i32,
            cells_revealed: 0,
            start_time: None,
            elapsed_time: 0,
        };
        
        game.place_mines();
        game.calculate_adjacent_mines();
        game
    }
    
    fn place_mines(&mut self) {
        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;
        
        while mines_placed < NUM_MINES {
            let x = rng.gen_range(0..GRID_WIDTH);
            let y = rng.gen_range(0..GRID_HEIGHT);
            
            if !self.grid[y][x].is_mine {
                self.grid[y][x].is_mine = true;
                mines_placed += 1;
            }
        }
    }
    
    fn calculate_adjacent_mines(&mut self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if !self.grid[y][x].is_mine {
                    let mut count = 0;
                    
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            
                            if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                                if self.grid[ny as usize][nx as usize].is_mine {
                                    count += 1;
                                }
                            }
                        }
                    }
                    
                    self.grid[y][x].adjacent_mines = count;
                }
            }
        }
    }
    
    fn reveal(&mut self, x: usize, y: usize) {
        if self.game_over || self.won {
            return;
        }
        
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        
        if self.grid[y][x].state != CellState::Hidden {
            return;
        }
        
        self.grid[y][x].state = CellState::Revealed;
        self.cells_revealed += 1;
        
        if self.grid[y][x].is_mine {
            self.game_over = true;
            self.reveal_all_mines();
            return;
        }
        
        // Auto-reveal adjacent cells if this cell has no adjacent mines
        if self.grid[y][x].adjacent_mines == 0 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    
                    if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                        self.reveal(nx as usize, ny as usize);
                    }
                }
            }
        }
        
        // Check win condition
        if self.cells_revealed == GRID_WIDTH * GRID_HEIGHT - NUM_MINES {
            self.won = true;
        }
    }
    
    fn reveal_all_mines(&mut self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.grid[y][x].is_mine {
                    self.grid[y][x].state = CellState::Revealed;
                }
            }
        }
    }
    
    fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.game_over || self.won {
            return;
        }
        
        match self.grid[y][x].state {
            CellState::Hidden => {
                self.grid[y][x].state = CellState::Flagged;
                self.mines_left -= 1;
            }
            CellState::Flagged => {
                self.grid[y][x].state = CellState::Hidden;
                self.mines_left += 1;
            }
            _ => {}
        }
    }
    
    fn draw(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.queue(Clear(ClearType::All))?;
        
        // Update elapsed time
        if let Some(start) = self.start_time {
            if !self.game_over && !self.won {
                self.elapsed_time = start.elapsed().as_secs();
            }
        }
        
        // Title
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.queue(SetForegroundColor(Color::Yellow))?;
        stdout.queue(Print("MINESWEEPER - Rust Edition"))?;
        stdout.queue(ResetColor)?;
        
        // Stats
        stdout.queue(cursor::MoveTo(0, 1))?;
        stdout.queue(Print(format!("Mines: {} | Time: {}s", self.mines_left, self.elapsed_time)))?;
        
        // Grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                stdout.queue(cursor::MoveTo((x * 2) as u16, (y + 3) as u16))?;
                
                let is_cursor = x == self.cursor_x && y == self.cursor_y;
                
                if is_cursor {
                    stdout.queue(SetBackgroundColor(Color::DarkGrey))?;
                }
                
                match self.grid[y][x].state {
                    CellState::Hidden => {
                        stdout.queue(SetForegroundColor(Color::Grey))?;
                        stdout.queue(Print("■ "))?;
                    }
                    CellState::Flagged => {
                        stdout.queue(SetForegroundColor(Color::Red))?;
                        stdout.queue(Print("🚩"))?;
                    }
                    CellState::Revealed => {
                        if self.grid[y][x].is_mine {
                            stdout.queue(SetBackgroundColor(Color::Red))?;
                            stdout.queue(SetForegroundColor(Color::White))?;
                            stdout.queue(Print("💣"))?;
                        } else {
                            let count = self.grid[y][x].adjacent_mines;
                            if count == 0 {
                                stdout.queue(Print("  "))?;
                            } else {
                                stdout.queue(SetForegroundColor(match count {
                                    1 => Color::Blue,
                                    2 => Color::Green,
                                    3 => Color::Red,
                                    4 => Color::DarkBlue,
                                    5 => Color::DarkRed,
                                    6 => Color::Cyan,
                                    7 => Color::Black,
                                    8 => Color::Grey,
                                    _ => Color::White,
                                }))?;
                                stdout.queue(Print(format!("{} ", count)))?;
                            }
                        }
                    }
                }
                
                stdout.queue(ResetColor)?;
            }
        }
        
        // Instructions
        let inst_y = (GRID_HEIGHT + 4) as u16;
        stdout.queue(cursor::MoveTo(0, inst_y))?;
        stdout.queue(Print("Arrow keys: Move | Space: Reveal | F: Flag | R: New game | Q: Quit"))?;
        
        // Game state messages
        if self.won {
            stdout.queue(cursor::MoveTo(0, inst_y + 2))?;
            stdout.queue(SetForegroundColor(Color::Green))?;
            stdout.queue(Print(format!("🎉 You Win! Time: {}s", self.elapsed_time)))?;
        } else if self.game_over {
            stdout.queue(cursor::MoveTo(0, inst_y + 2))?;
            stdout.queue(SetForegroundColor(Color::Red))?;
            stdout.queue(Print("💥 Game Over! You hit a mine!"))?;
        }
        
        stdout.queue(ResetColor)?;
        stdout.flush()?;
        Ok(())
    }
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
                    KeyCode::Up => {
                        if game.cursor_y > 0 {
                            game.cursor_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if game.cursor_y < GRID_HEIGHT - 1 {
                            game.cursor_y += 1;
                        }
                    }
                    KeyCode::Left => {
                        if game.cursor_x > 0 {
                            game.cursor_x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if game.cursor_x < GRID_WIDTH - 1 {
                            game.cursor_x += 1;
                        }
                    }
                    KeyCode::Char(' ') => {
                        game.reveal(game.cursor_x, game.cursor_y);
                    }
                    KeyCode::Char('f') => {
                        game.toggle_flag(game.cursor_x, game.cursor_y);
                    }
                    KeyCode::Char('r') => {
                        game = Game::new();
                    }
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
    println!("Thanks for playing Minesweeper!");
    
    Ok(())
}