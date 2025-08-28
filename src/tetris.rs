use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::io::{self, Write};
use std::time::{Duration, Instant};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const PREVIEW_SIZE: usize = 4;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Filled(Color),
}

#[derive(Clone, Copy, Debug)]
enum TetrominoType {
    I, O, T, S, Z, J, L,
}

impl TetrominoType {
    fn color(&self) -> Color {
        match self {
            TetrominoType::I => Color::Cyan,
            TetrominoType::O => Color::Yellow,
            TetrominoType::T => Color::Magenta,
            TetrominoType::S => Color::Green,
            TetrominoType::Z => Color::Red,
            TetrominoType::J => Color::Blue,
            TetrominoType::L => Color::Rgb { r: 255, g: 165, b: 0 }, // Orange
        }
    }
    
    fn shapes(&self) -> Vec<Vec<Vec<bool>>> {
        match self {
            TetrominoType::I => vec![
                vec![vec![true, true, true, true]],
                vec![vec![true], vec![true], vec![true], vec![true]],
            ],
            TetrominoType::O => vec![
                vec![vec![true, true], vec![true, true]],
            ],
            TetrominoType::T => vec![
                vec![vec![false, true, false], vec![true, true, true]],
                vec![vec![true, false], vec![true, true], vec![true, false]],
                vec![vec![true, true, true], vec![false, true, false]],
                vec![vec![false, true], vec![true, true], vec![false, true]],
            ],
            TetrominoType::S => vec![
                vec![vec![false, true, true], vec![true, true, false]],
                vec![vec![true, false], vec![true, true], vec![false, true]],
            ],
            TetrominoType::Z => vec![
                vec![vec![true, true, false], vec![false, true, true]],
                vec![vec![false, true], vec![true, true], vec![true, false]],
            ],
            TetrominoType::J => vec![
                vec![vec![true, false, false], vec![true, true, true]],
                vec![vec![true, true], vec![true, false], vec![true, false]],
                vec![vec![true, true, true], vec![false, false, true]],
                vec![vec![false, true], vec![false, true], vec![true, true]],
            ],
            TetrominoType::L => vec![
                vec![vec![false, false, true], vec![true, true, true]],
                vec![vec![true, false], vec![true, false], vec![true, true]],
                vec![vec![true, true, true], vec![true, false, false]],
                vec![vec![true, true], vec![false, true], vec![false, true]],
            ],
        }
    }
}

struct Tetromino {
    tetromino_type: TetrominoType,
    x: i32,
    y: i32,
    rotation: usize,
}

impl Tetromino {
    fn new(tetromino_type: TetrominoType) -> Self {
        Self {
            tetromino_type,
            x: (BOARD_WIDTH as i32 / 2) - 1,
            y: 0,
            rotation: 0,
        }
    }
    
    fn shape(&self) -> &Vec<Vec<bool>> {
        let shapes = self.tetromino_type.shapes();
        &shapes[self.rotation % shapes.len()]
    }
    
    fn rotate(&mut self) {
        let shapes = self.tetromino_type.shapes();
        self.rotation = (self.rotation + 1) % shapes.len();
    }
}

struct Game {
    board: Vec<Vec<Cell>>,
    current_piece: Tetromino,
    next_piece: TetrominoType,
    score: u32,
    lines: u32,
    level: u32,
    game_over: bool,
    last_fall: Instant,
    fall_speed: Duration,
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let types = [
            TetrominoType::I, TetrominoType::O, TetrominoType::T,
            TetrominoType::S, TetrominoType::Z, TetrominoType::J, TetrominoType::L,
        ];
        
        Self {
            board: vec![vec![Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_piece: Tetromino::new(types[rng.gen_range(0..types.len())]),
            next_piece: types[rng.gen_range(0..types.len())],
            score: 0,
            lines: 0,
            level: 1,
            game_over: false,
            last_fall: Instant::now(),
            fall_speed: Duration::from_millis(1000),
        }
    }
    
    fn is_valid_position(&self, piece: &Tetromino) -> bool {
        let shape = piece.shape();
        
        for (dy, row) in shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell {
                    let x = piece.x + dx as i32;
                    let y = piece.y + dy as i32;
                    
                    if x < 0 || x >= BOARD_WIDTH as i32 || y >= BOARD_HEIGHT as i32 {
                        return false;
                    }
                    
                    if y >= 0 && self.board[y as usize][x as usize] != Cell::Empty {
                        return false;
                    }
                }
            }
        }
        true
    }
    
    fn lock_piece(&mut self) {
        let shape = self.current_piece.shape();
        let color = self.current_piece.tetromino_type.color();
        
        for (dy, row) in shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell {
                    let x = self.current_piece.x + dx as i32;
                    let y = self.current_piece.y + dy as i32;
                    
                    if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                        self.board[y as usize][x as usize] = Cell::Filled(color);
                    }
                }
            }
        }
        
        self.clear_lines();
        self.spawn_new_piece();
    }
    
    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        let mut y = BOARD_HEIGHT - 1;
        
        while y > 0 {
            if self.board[y].iter().all(|&cell| cell != Cell::Empty) {
                self.board.remove(y);
                self.board.insert(0, vec![Cell::Empty; BOARD_WIDTH]);
                lines_cleared += 1;
            } else {
                if y > 0 {
                    y -= 1;
                }
            }
        }
        
        if lines_cleared > 0 {
            self.lines += lines_cleared as u32;
            self.score += match lines_cleared {
                1 => 100 * self.level,
                2 => 300 * self.level,
                3 => 500 * self.level,
                4 => 800 * self.level,
                _ => 0,
            };
            
            // Level up every 10 lines
            self.level = (self.lines / 10) + 1;
            self.fall_speed = Duration::from_millis((1000.0 / (self.level as f64).sqrt()) as u64);
        }
    }
    
    fn spawn_new_piece(&mut self) {
        let mut rng = rand::thread_rng();
        let types = [
            TetrominoType::I, TetrominoType::O, TetrominoType::T,
            TetrominoType::S, TetrominoType::Z, TetrominoType::J, TetrominoType::L,
        ];
        
        self.current_piece = Tetromino::new(self.next_piece);
        self.next_piece = types[rng.gen_range(0..types.len())];
        
        if !self.is_valid_position(&self.current_piece) {
            self.game_over = true;
        }
    }
    
    fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        self.current_piece.x += dx;
        self.current_piece.y += dy;
        
        if self.is_valid_position(&self.current_piece) {
            true
        } else {
            self.current_piece.x -= dx;
            self.current_piece.y -= dy;
            false
        }
    }
    
    fn rotate_piece(&mut self) {
        self.current_piece.rotate();
        if !self.is_valid_position(&self.current_piece) {
            // Try wall kicks
            for offset in &[(1, 0), (-1, 0), (0, -1)] {
                self.current_piece.x += offset.0;
                self.current_piece.y += offset.1;
                if self.is_valid_position(&self.current_piece) {
                    return;
                }
                self.current_piece.x -= offset.0;
                self.current_piece.y -= offset.1;
            }
            // Rotation failed, revert
            let shapes = self.current_piece.tetromino_type.shapes();
            self.current_piece.rotation = (self.current_piece.rotation + shapes.len() - 1) % shapes.len();
        }
    }
    
    fn hard_drop(&mut self) {
        while self.move_piece(0, 1) {
            self.score += 2;
        }
        self.lock_piece();
    }
    
    fn update(&mut self) {
        if self.game_over {
            return;
        }
        
        if self.last_fall.elapsed() >= self.fall_speed {
            if !self.move_piece(0, 1) {
                self.lock_piece();
            }
            self.last_fall = Instant::now();
        }
    }
    
    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        
        // Draw title
        stdout.queue(SetForegroundColor(Color::Cyan))?;
        stdout.queue(Print("TETRIS - Rust Edition"))?;
        stdout.queue(ResetColor)?;
        
        // Draw board
        for y in 0..BOARD_HEIGHT {
            stdout.queue(cursor::MoveTo(0, y as u16 + 2))?;
            stdout.queue(Print("│"))?;
            
            for x in 0..BOARD_WIDTH {
                let mut cell = self.board[y][x];
                
                // Check if current piece is here
                let shape = self.current_piece.shape();
                for (dy, row) in shape.iter().enumerate() {
                    for (dx, &is_filled) in row.iter().enumerate() {
                        if is_filled {
                            let px = self.current_piece.x + dx as i32;
                            let py = self.current_piece.y + dy as i32;
                            if px == x as i32 && py == y as i32 {
                                cell = Cell::Filled(self.current_piece.tetromino_type.color());
                            }
                        }
                    }
                }
                
                match cell {
                    Cell::Empty => stdout.queue(Print(" "))?,
                    Cell::Filled(color) => {
                        stdout.queue(SetBackgroundColor(color))?;
                        stdout.queue(Print(" "))?;
                        stdout.queue(ResetColor)?;
                    }
                }
            }
            stdout.queue(Print("│"))?;
        }
        
        // Draw bottom border
        stdout.queue(cursor::MoveTo(0, BOARD_HEIGHT as u16 + 2))?;
        stdout.queue(Print("└"))?;
        for _ in 0..BOARD_WIDTH {
            stdout.queue(Print("─"))?;
        }
        stdout.queue(Print("┘"))?;
        
        // Draw stats
        let stats_x = (BOARD_WIDTH + 3) as u16;
        stdout.queue(cursor::MoveTo(stats_x, 2))?;
        stdout.queue(Print(format!("Score: {}", self.score)))?;
        
        stdout.queue(cursor::MoveTo(stats_x, 3))?;
        stdout.queue(Print(format!("Lines: {}", self.lines)))?;
        
        stdout.queue(cursor::MoveTo(stats_x, 4))?;
        stdout.queue(Print(format!("Level: {}", self.level)))?;
        
        // Draw next piece
        stdout.queue(cursor::MoveTo(stats_x, 6))?;
        stdout.queue(Print("Next:"))?;
        
        let next_shape = &self.next_piece.shapes()[0];
        for (y, row) in next_shape.iter().enumerate() {
            stdout.queue(cursor::MoveTo(stats_x, 7 + y as u16))?;
            for &cell in row {
                if cell {
                    stdout.queue(SetBackgroundColor(self.next_piece.color()))?;
                    stdout.queue(Print("  "))?;
                    stdout.queue(ResetColor)?;
                } else {
                    stdout.queue(Print("  "))?;
                }
            }
        }
        
        // Controls
        stdout.queue(cursor::MoveTo(stats_x, 12))?;
        stdout.queue(Print("Controls:"))?;
        stdout.queue(cursor::MoveTo(stats_x, 13))?;
        stdout.queue(Print("← → Move"))?;
        stdout.queue(cursor::MoveTo(stats_x, 14))?;
        stdout.queue(Print("↓   Soft drop"))?;
        stdout.queue(cursor::MoveTo(stats_x, 15))?;
        stdout.queue(Print("↑   Rotate"))?;
        stdout.queue(cursor::MoveTo(stats_x, 16))?;
        stdout.queue(Print("SPC Hard drop"))?;
        stdout.queue(cursor::MoveTo(stats_x, 17))?;
        stdout.queue(Print("Q   Quit"))?;
        
        if self.game_over {
            stdout.queue(cursor::MoveTo(1, BOARD_HEIGHT as u16 / 2))?;
            stdout.queue(SetBackgroundColor(Color::Red))?;
            stdout.queue(SetForegroundColor(Color::White))?;
            stdout.queue(Print(" GAME OVER "))?;
            stdout.queue(ResetColor)?;
        }
        
        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide)?;
    
    let mut game = Game::new();
    let mut last_input = Instant::now();
    
    loop {
        game.draw(&mut stdout)?;
        
        if game.game_over {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }
        } else {
            // Handle input
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Left => { game.move_piece(-1, 0); }
                        KeyCode::Right => { game.move_piece(1, 0); }
                        KeyCode::Down => { 
                            if game.move_piece(0, 1) {
                                game.score += 1;
                            }
                        }
                        KeyCode::Up => { game.rotate_piece(); }
                        KeyCode::Char(' ') => { game.hard_drop(); }
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                    last_input = Instant::now();
                }
            }
            
            game.update();
        }
        
        std::thread::sleep(Duration::from_millis(50));
    }
    
    stdout.execute(cursor::Show)?;
    stdout.execute(ResetColor)?;
    terminal::disable_raw_mode()?;
    stdout.execute(Clear(ClearType::All))?;
    println!("Thanks for playing! Final Score: {}", game.score);
    
    Ok(())
}