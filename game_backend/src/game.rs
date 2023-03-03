use crate::snake::Snake;
use crate::base::Vector2i;
use std::boxed::Box;

const INITIAL_LENGTH : u32 = 2;
type PlayerIndex = i32;

/// The object that communicates snake control input
pub struct SnakeControl
{

}

/// The object that stores data associated with single player in the game
struct Player
{
    snake : Snake,
    score : u32,
    control : Option<Box<SnakeControl>>,
}

impl Player{
    pub fn new() -> Player {
        Player {
            snake : Snake::new(Vector2i::new(0, 0), 
             Vector2i::new(1, 0), INITIAL_LENGTH),
            score : 0,
            control : None,
        }
    }
}

/// Game object. Create and configure it to start a game.
pub struct Game {
    players : Vec<Player>,
    field_size : Vector2i,
    pizzas : Vec<Vector2i>,
}

impl Game {
    /// Creates new unitialized game object
    pub fn new(field_size : Vector2i) -> Game {
        Game {
            players : Vec::new(),
            field_size,
            pizzas : Vec::new(),
        }
    }
    /// Adds new player that has control. Returns new player index that can
    /// be used for referencing this player
    pub fn register_controlled_player(&mut self, control : Box<SnakeControl>) -> PlayerIndex {
        let new_player_index = self.players.len() as PlayerIndex;
        // make spawn point
        let spawn_pos = Game::calc_spawn_pos(new_player_index, INITIAL_LENGTH, self.field_size);
        let mut player = Player::new();
        player.control = Some(control);
        self.players.push(player);
        new_player_index
    }
    /// REturns number of empty cells in the field.
    fn num_empty_cells(&self) -> i32 {
        let mut num = (self.field_size.x * self.field_size.y) as i32;
        // Substract pizas
        num -= self.pizzas.len() as i32;
        // Substract length of every snake
        for player in &self.players {
            num -= player.snake.body().len() as i32;
        }
        num
    }

    /// Calculate the spaw position for the snake with given index.
    /// All snakes start from the center outwards.
    /// Maximum of 4 snakes can be spawned.
    /// Returns position and direction
    /// 
    /// #panics
    /// Panics if the index > 4
    fn calc_spawn_pos(index : PlayerIndex, length : u32, field_size : Vector2i) -> (Vector2i, Vector2i)
    {
        assert!(index < 4);
        let center = Vector2i::new(field_size.x / 2, field_size.y / 2);
        let mut pos = center;
        let dir: Vector2i;
        match index {
            0 => {
                pos.x -= length as i32;
                dir = Vector2i::new(-1, 0);
            },
            1 => {
                pos.y -= length as i32;
                dir = Vector2i::new(0, -1);
            },
            2 => {
                pos.x += length as i32;
                dir = Vector2i::new(1, 0);
            },
            3 => {
                pos.y += length as i32;
                dir = Vector2i::new(0, 1);
            },
            _ => panic!("Too many snakes"),
        }
        (pos, dir)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test each new player gets new index
    #[test]
    fn test_register_controlled_player() {
        let mut game = Game::new( Vector2i::new(10, 10));
        let player1 = game.register_controlled_player(Box::new(SnakeControl{}));
        let player2 = game.register_controlled_player(Box::new(SnakeControl{}));
        let player3 = game.register_controlled_player(Box::new(SnakeControl{}));
        assert_eq!(player1, 0);
        assert_eq!(player2, 1);
        assert_eq!(player3, 2);
    }

    // Test calc_spawn_pos
    #[test]
    fn test_calc_spawn_pos() {
        let field_size = Vector2i::new(10, 10);
        let (pos, dir) = Game::calc_spawn_pos(0, 3, field_size);
        assert_eq!(pos, Vector2i::new(2, 5));
        assert_eq!(dir, Vector2i::new(-1, 0));

        let (pos, dir) = Game::calc_spawn_pos(1, 3, field_size);
        assert_eq!(pos, Vector2i::new(5, 2));
        assert_eq!(dir, Vector2i::new(0, -1));

        let (pos, dir) = Game::calc_spawn_pos(2, 3, field_size);
        assert_eq!(pos, Vector2i::new(8, 5));
        assert_eq!(dir, Vector2i::new(1, 0));

        let (pos, dir) = Game::calc_spawn_pos(3, 3, field_size);
        assert_eq!(pos, Vector2i::new(5, 8));
        assert_eq!(dir, Vector2i::new(0, 1));
    }
    // Calling test_cal_spawn_pos with index > 4 should panic
    #[test]
    #[should_panic]
    fn test_calc_spawn_pos_panic() {
        Game::calc_spawn_pos(4, 3, Vector2i::new(10, 10));
    }

    // Test num_empty_cells
    #[test]
    fn test_num_empty_cells() {
        let mut game = Game::new( Vector2i::new(10, 10));
        assert_eq!(game.num_empty_cells(), 100);
        game.register_controlled_player(Box::new(SnakeControl{}));
        assert_eq!(game.num_empty_cells(), 100 - INITIAL_LENGTH as i32);
        game.register_controlled_player(Box::new(SnakeControl{}));
        assert_eq!(game.num_empty_cells(), 100 - 2 * INITIAL_LENGTH as i32);
        
        // Add some food
        game.pizzas.push(Vector2i::new(0, 0));
        game.pizzas.push(Vector2i::new(0, 1));
        
        assert_eq!(game.num_empty_cells(), 100 - 2 * INITIAL_LENGTH as i32 - 2);
    }

}