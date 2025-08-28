use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

const COURT_WIDTH: f32 = 60.0;
const COURT_HEIGHT: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 4.0;
const PADDLE_SPEED: f32 = 15.0;
const BALL_SPEED: f32 = 20.0;

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    fn new() -> Self {
        Self {
            x: COURT_WIDTH / 2.0,
            y: COURT_HEIGHT / 2.0,
            vx: BALL_SPEED,
            vy: BALL_SPEED / 2.0,
        }
    }
    
    fn reset(&mut self, towards_player: bool) {
        self.x = COURT_WIDTH / 2.0;
        self.y = COURT_HEIGHT / 2.0;
        self.vx = if towards_player { -BALL_SPEED } else { BALL_SPEED };
        self.vy = BALL_SPEED / 2.0;
    }
    
    fn update(&mut self, dt: f32, player_y: f32, ai_y: f32) -> Option<bool> {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        
        // Top and bottom collision
        if self.y <= 0.0 || self.y >= COURT_HEIGHT - 1.0 {
            self.vy = -self.vy;
        }
        
        // Player paddle collision
        if self.x <= 2.0 && self.x >= 1.0 {
            if self.y >= player_y && self.y <= player_y + PADDLE_HEIGHT {
                self.vx = BALL_SPEED;
                let paddle_center = player_y + PADDLE_HEIGHT / 2.0;
                let hit_pos = (self.y - paddle_center) / (PADDLE_HEIGHT / 2.0);
                self.vy = hit_pos * BALL_SPEED;
            }
        }
        
        // AI paddle collision
        if self.x >= COURT_WIDTH - 3.0 && self.x <= COURT_WIDTH - 2.0 {
            if self.y >= ai_y && self.y <= ai_y + PADDLE_HEIGHT {
                self.vx = -BALL_SPEED;
                let paddle_center = ai_y + PADDLE_HEIGHT / 2.0;
                let hit_pos = (self.y - paddle_center) / (PADDLE_HEIGHT / 2.0);
                self.vy = hit_pos * BALL_SPEED;
            }
        }
        
        // Score detection
        if self.x < 0.0 {
            return Some(false); // AI scores
        }
        if self.x > COURT_WIDTH {
            return Some(true); // Player scores
        }
        
        None
    }
}

struct Game {
    player_y: f32,
    ai_y: f32,
    ball: Ball,
    player_score: u32,
    ai_score: u32,
    game_over: bool,
    last_update: Instant,
}

impl Game {
    fn new() -> Self {
        Self {
            player_y: COURT_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            ai_y: COURT_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            ball: Ball::new(),
            player_score: 0,
            ai_score: 0,
            game_over: false,
            last_update: Instant::now(),
        }
    }
    
    fn update(&mut self) {
        let dt = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();
        
        // AI movement
        let target = self.ball.y - PADDLE_HEIGHT / 2.0;
        let diff = target - self.ai_y;
        let ai_speed = PADDLE_SPEED * 0.8; // Make AI slightly slower for fairness
        
        if diff.abs() > 1.0 {
            let move_amount = ai_speed * dt * diff.signum();
            self.ai_y += move_amount;
        }
        
        // Constrain AI paddle
        self.ai_y = self.ai_y.max(0.0).min(COURT_HEIGHT - PADDLE_HEIGHT);
        
        // Update ball
        if let Some(player_scored) = self.ball.update(dt, self.player_y, self.ai_y) {
            if player_scored {
                self.player_score += 1;
            } else {
                self.ai_score += 1;
            }
            
            if self.player_score >= 10 || self.ai_score >= 10 {
                self.game_over = true;
            } else {
                self.ball.reset(!player_scored);
            }
        }
    }
    
    fn move_player(&mut self, up: bool, dt: f32) {
        if up {
            self.player_y -= PADDLE_SPEED * dt;
        } else {
            self.player_y += PADDLE_SPEED * dt;
        }
        
        self.player_y = self.player_y.max(0.0).min(COURT_HEIGHT - PADDLE_HEIGHT);
    }
    
    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.queue(Clear(ClearType::All))?;
        
        // Title and score
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.queue(SetForegroundColor(Color::Cyan))?;
        stdout.queue(Print("PONG - Rust Edition"))?;
        stdout.queue(ResetColor)?;
        stdout.queue(cursor::MoveTo(25, 0))?;
        stdout.queue(Print(format!("Player: {} | AI: {}", self.player_score, self.ai_score)))?;
        
        // Draw court
        for y in 0..COURT_HEIGHT as u16 {
            // Net
            stdout.queue(cursor::MoveTo(COURT_WIDTH as u16 / 2, y + 2))?;
            if y % 2 == 0 {
                stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                stdout.queue(Print("│"))?;
            }
            
            // Player paddle
            if y >= self.player_y as u16 && y < (self.player_y + PADDLE_HEIGHT) as u16 {
                stdout.queue(cursor::MoveTo(1, y + 2))?;
                stdout.queue(SetForegroundColor(Color::Green))?;
                stdout.queue(Print("█"))?;
            }
            
            // AI paddle
            if y >= self.ai_y as u16 && y < (self.ai_y + PADDLE_HEIGHT) as u16 {
                stdout.queue(cursor::MoveTo(COURT_WIDTH as u16 - 2, y + 2))?;
                stdout.queue(SetForegroundColor(Color::Red))?;
                stdout.queue(Print("█"))?;
            }
        }
        
        // Draw ball
        stdout.queue(cursor::MoveTo(self.ball.x as u16, self.ball.y as u16 + 2))?;
        stdout.queue(SetForegroundColor(Color::Yellow))?;
        stdout.queue(Print("●"))?;
        
        // Draw borders
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        for x in 0..=COURT_WIDTH as u16 {
            stdout.queue(cursor::MoveTo(x, 1))?;
            stdout.queue(Print("─"))?;
            stdout.queue(cursor::MoveTo(x, COURT_HEIGHT as u16 + 2))?;
            stdout.queue(Print("─"))?;
        }
        
        // Instructions
        stdout.queue(ResetColor)?;
        stdout.queue(cursor::MoveTo(0, COURT_HEIGHT as u16 + 4))?;
        stdout.queue(Print("W/S or ↑/↓: Move paddle | R: Restart | Q: Quit"))?;
        
        if self.game_over {
            stdout.queue(cursor::MoveTo(20, COURT_HEIGHT as u16 / 2 + 2))?;
            if self.player_score > self.ai_score {
                stdout.queue(SetForegroundColor(Color::Green))?;
                stdout.queue(Print("YOU WIN!"))?;
            } else {
                stdout.queue(SetForegroundColor(Color::Red))?;
                stdout.queue(Print("AI WINS!"))?;
            }
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
    let mut up_pressed = false;
    let mut down_pressed = false;
    
    loop {
        // Handle input
        while event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up | KeyCode::Char('w') => up_pressed = true,
                    KeyCode::Down | KeyCode::Char('s') => down_pressed = true,
                    KeyCode::Char('r') => game = Game::new(),
                    KeyCode::Char('q') | KeyCode::Esc => {
                        stdout.execute(cursor::Show)?;
                        stdout.execute(ResetColor)?;
                        terminal::disable_raw_mode()?;
                        stdout.execute(Clear(ClearType::All))?;
                        println!("Thanks for playing Pong!");
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        
        // Update game
        if !game.game_over {
            let dt = 0.016; // ~60 FPS
            
            if up_pressed {
                game.move_player(true, dt);
                up_pressed = false;
            }
            if down_pressed {
                game.move_player(false, dt);
                down_pressed = false;
            }
            
            game.update();
        }
        
        game.draw(&mut stdout)?;
        std::thread::sleep(Duration::from_millis(16));
    }
}