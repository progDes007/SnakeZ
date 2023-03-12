use game_backend::game::GlobalUpdateRx;
use game_backend::player::UserControlTx;
use game_backend::base::Direction;
use game_backend::events;
use std::option::Option;
use console_engine;

/// Front represents a frontend object.
pub struct Front {
    // User control channel
    user_control_tx: UserControlTx,
    // Global update channel
    global_update_rx: GlobalUpdateRx,
    // Console engine for rendering
    engine: console_engine::ConsoleEngine,
}

// Impl for Front
impl Front {
    /// Creates a new Front object.
    pub fn new( user_control_tx: UserControlTx,
                global_update_rx: GlobalUpdateRx)
                -> Front {

        let engine = console_engine::ConsoleEngine::init(50, 30, 10).unwrap();

        Front { user_control_tx,
                global_update_rx,
                engine
        }
    }

    /// Runs frontend until game is over
    pub fn run(&mut self) 
    {
        loop {
            self.engine.wait_frame();

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
                    events::GlobalEvent::Update(_update) => {
                        
                    }
                }
            }
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
