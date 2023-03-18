use game_backend::game::GlobalUpdateRx;
use game_backend::player::UserControlTx;
use game_backend::base::Direction;
use game_backend::events;
use game_backend::grid;
use std::option::Option;
use console_engine::*;

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
}

// Impl for Front
impl Front {
    /// Creates a new Front object.
    pub fn new( user_control_tx: UserControlTx,
                global_update_rx: GlobalUpdateRx)
                -> Front {

        let engine = ConsoleEngine::init(100, 30, 10).unwrap();

        Front { user_control_tx,
                global_update_rx,
                engine,
                last_grid: None,
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
                       /* let x1 = x as i32 * ASPECT_RATIO + offset_x;
                        let y1 = y as i32 + offset_y;
                        let x2 = x1 + ASPECT_RATIO - 1;
                        let y2 = y1;
                        self.engine.rect(x1, y1, x2, y2, pixel::pxl_bg(' ', Color::Yellow));*/
                    }
                }
            }
        }
    }


    /// Runs frontend until game is over
    pub fn run(&mut self) 
    {
        loop {
            self.engine.wait_frame();
            self.engine.clear_screen();

            // Read direction input
            if let Some(direction) = self.read_keyboard_input() {
                // Send to user, ignore errors
                let _ = self.user_control_tx.send(direction);
            }
            // If user pressed q, exit
            if self.engine.is_key_pressed(console_engine::KeyCode::Esc) {
                break;
            }
            // Read the global update channel
            if let Ok(global_update) = self.global_update_rx.try_recv() {
                // Match the global update message type
                match global_update {
                    // If it's a game over message, exit
                    events::GlobalEvent::GameOver(_game_over) => {
                        break;
                    }
                    // If it's a game update message, read it
                    events::GlobalEvent::Update(update) => {
                        // Remember grid
                        self.last_grid = Some(update.grid);
                    }
                }
            }

            // Draw the grid
            self.draw_grid();
            self.engine.draw();
        }
    }

    /// Reads the keyboard input and returns the direction (if any arrow key is pressed)
    fn read_keyboard_input(&self) -> Option<Direction> {
        if self.engine.is_key_pressed(console_engine::KeyCode::Left) {
            return Some(Direction::MinusX);
        }
        else if self.engine.is_key_pressed(console_engine::KeyCode::Right) {
            return Some(Direction::PlusX);
        }
        else if self.engine.is_key_pressed(console_engine::KeyCode::Up) {
            return Some(Direction::MinusY);
        }
        else if self.engine.is_key_pressed(console_engine::KeyCode::Down) {
            return Some(Direction::PlusY);
        }

        None
        
    }
}
