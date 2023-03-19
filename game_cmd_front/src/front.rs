use console_engine::crossterm::event::KeyEvent;
use game_backend::game::GlobalUpdateRx;
use game_backend::player::UserControlTx;
use game_backend::base::Direction;
use game_backend::grid;
use game_backend::events;
use std::option::Option;
use console_engine::*;
use console_engine::events::*;

const ASPECT_RATIO : i32 = 3;

/// Front represents a frontend object.
pub struct Front {
    // User control channel
    user_control_tx: UserControlTx,
    // Global update channel
    global_update_rx: GlobalUpdateRx,
    // Console engine for rendering
    engine: ConsoleEngine,

    // Last recieved grid. Optional
    last_grid: Option<grid::Grid>,
    // Vector of last player infos
    last_player_summary: Vec<events::PlayerSummary>,
}

// Impl for Front
impl Front {
    /// Creates a new Front object.
    pub fn new( user_control_tx: UserControlTx,
                global_update_rx: GlobalUpdateRx)
                -> Front {

        let engine = ConsoleEngine::init(120, 30, 10).unwrap();

        Front { user_control_tx,
                global_update_rx,
                engine,
                last_grid: None,
                last_player_summary: Vec::new(),
        }
    }

    /// Function that is drawing a boder for field. Accepts position and size
    fn draw_border(engine : &mut ConsoleEngine, x: i32, y: i32, width: i32, height: i32) {
        let x1 = x * ASPECT_RATIO - 1;
        let y1 = y- 1;
        let x2 = x1 + width * ASPECT_RATIO + 1 - 1;
        let y2 = y1 + height + 1 - 1;

        let border_style = console_engine::rect_style::BorderStyle::new_double();
        engine.rect_border(x1, y1, x2, y2, border_style);
    }

    /// Function that is drawing snake head
    fn draw_snake_head(engine : &mut ConsoleEngine, x: i32, y: i32) {
        let x1 = x * ASPECT_RATIO;
        let y1 = y;
        let x2 = x1 + ASPECT_RATIO - 1;
        let y2 = y1;
        engine.rect(x1, y1, x2, y2, pixel::pxl_bg(' ', Color::Red));
    }
    /// Function for drawing snake body
    fn draw_snake_body(engine : &mut ConsoleEngine, x: i32, y: i32) {
        let x1 = x * ASPECT_RATIO;
        let y1 = y;
        let x2 = x1 + ASPECT_RATIO - 1;
        let y2 = y1;
        engine.rect(x1, y1, x2, y2, pixel::pxl_bg(' ', Color::White));
    }
    /// Function for drawing pizza
    fn draw_pizza(engine : &mut ConsoleEngine, x: i32, y: i32) {
        let x1 = x * ASPECT_RATIO;
        let y1 = y;
        let x2 = x1 + ASPECT_RATIO - 1;
        let y2 = y1;
        engine.rect(x1, y1, x2, y2, pixel::pxl_bg(' ', Color::Yellow));
    }

    /// Draw a summary for specified player. Accepts summary object and position
    fn draw_player_summary(engine : &mut ConsoleEngine, summary: &events::PlayerSummary, player_index: i32, x: i32, y: i32) {
        let text =format!("Player {}: {}", player_index, summary.score);
        engine.print(x, y, &text);
    }
   
    // Function that is drawing the entire grid
    fn draw_grid(&mut self) {
        // If there is no grid, return
        if self.last_grid.is_none() {
            return;
        }
        // Get the grid
        let grid = self.last_grid.as_ref().unwrap();
        let offset_x = 1;
        let offset_y = 1;
        // First draw the border based on the grid size
        Self::draw_border(&mut self.engine, offset_x, offset_y, grid.dim().0 as i32, grid.dim().1 as i32);

        // Draw grid cells
        for y in 0..grid.dim().1 {
            for x in 0..grid.dim().0 {
                // Get the cell
                let cell = grid[[x, y]];
                // Match the cell type
                match cell {
                    // If it's empty, do nothing
                    grid::GridCell::Empty => {}
                    // If it's a snake, draw it
                    grid::GridCell::Snake(snake_rec) => {
                        // Match the snake body part
                        match snake_rec.body_part {
                            // If it's a head, draw it
                            grid::SnakeBodyPart::Head => {
                                Self::draw_snake_head(&mut self.engine, x as i32 + offset_x, y as i32 + offset_y);
                            }
                            // Body or tail
                            _ => {
                                Self::draw_snake_body(&mut self.engine, x as i32 + offset_x, y as i32 + offset_y);
                            }
                        }
                    }
                    // If it's a pizza, draw it
                    grid::GridCell::Pizza(_pizza_rec) => {
                        Self::draw_pizza(&mut self.engine, x as i32 + offset_x, y as i32 + offset_y);
                    }
                }
            }
        }
    }

    /// Function to handle frame update
    fn handle_frame(&mut self) {
        // Clear the screen
        self.engine.clear_screen();
        // Read the global update channel
        if let Ok(global_update) = self.global_update_rx.try_recv() {
            // Match the global update message type
            match global_update {
                // If it's a game over message, exit
                events::GlobalEvent::GameOver(_game_over) => {
                    return;
                }
                // If it's a game update message, read it
                events::GlobalEvent::Update(update) => {
                    // Remember grid
                    self.last_grid = Some(update.grid);
                    // Remember player infos
                    self.last_player_summary = update.players_summary;
                }
            }
        }
        // Render
        self.draw_grid();
        // Draw player summary
        for (i, summary) in self.last_player_summary.iter().enumerate() {
            Self::draw_player_summary(&mut self.engine, summary, i as i32 + 1, 80, 2 + i as i32);
        }

        self.engine.draw();
    }


    /// Runs frontend until game is over
    pub fn run(&mut self) 
    {
        loop {
            // Poll next event
            match self.engine.poll() {
                // A frame has passed
                Event::Frame => {
                    // Handle frame
                    self.handle_frame();
                }
        
                // A Key has been pressed
                Event::Key(keyevent) => {
                    // Exit if ESC
                    if keyevent.code == console_engine::KeyCode::Esc {
                        break;
                    }    
                    // Read direction input
                    if let Some(direction) = Self::key_to_direction(keyevent.code) {
                        // Send to user, ignore errors
                        let _ = self.user_control_tx.send(direction);
                    }
                }
        
                // Mouse has been moved or clicked
                _ => {}
            }
        }
        
    }

    /// Function that converts key code into direction
    /// Returns None if no direction is pressed
    fn key_to_direction(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Left => Some(Direction::MinusX),
            KeyCode::Right => Some(Direction::PlusX),
            KeyCode::Up => Some(Direction::MinusY),
            KeyCode::Down => Some(Direction::PlusY),
            _ => None,
        }
    }
}
