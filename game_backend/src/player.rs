use crate::base::{Direction, Vector2i};
use crate::snake::Snake;
use crate::events;
use std::sync::mpsc;


pub type UserControlRx = mpsc::Receiver<Direction>;
pub type UserControlTx = mpsc::Sender<Direction>;

/// The object that stores data associated with single player in the game
pub(crate) struct Player
{
    /// There is no snake if player is dead
    pub snake : Option<Snake>,
    pub score : u32,
    pub control : Option<UserControlRx>,
}


impl Player{
    pub fn new() -> Player {
        Player {
            snake : Some(Snake::new(Vector2i::new(0, 0), 
                Direction::PlusX, 2)),
            score : 0,
            control : None,
        }
    }

    // Read inputs for players
    pub fn read_inputs(&mut self) {
        if let Some(control) = &self.control {
            // Read all inputs.
            while let Ok(input) = control.try_recv() {
                if self.alive() {
                    self.snake.as_mut().unwrap().try_set_look_direction(input);
                }
            }
        }
    }

    /// Returns if player is alive
    pub fn alive(&self) -> bool {
        return self.snake.is_some();
    }

    /// Kills the player
    pub fn kill(&mut self) {
        self.snake = None;
    }

    /// Generates event summary
    pub fn summary(&self) -> events::PlayerSummary {
        events::PlayerSummary {
            score : self.score,
            alive : self.alive(),
        }
    }
}
