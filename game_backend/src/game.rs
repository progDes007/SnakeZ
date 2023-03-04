use crate::snake::Snake;
use crate::base::{Vector2i, PlayerIndex, Direction};
use crate::grid::{Grid, GridCell, PizzaRec, SnakeRec, SnakeBodyPart};
//use std::boxed::Box;
use std::sync::mpsc;
use rand;

const INITIAL_LENGTH : u32 = 2;

type UserControlRx = mpsc::Receiver<Direction>;

/// The object that stores data associated with single player in the game
struct Player
{
    snake : Snake,
    score : u32,
    control : Option<UserControlRx>,
}

/// Game object. Create and configure it to start a game.
pub struct Game {
    players : Vec<Player>,
    field_size : Vector2i,
    pizzas : Vec<Vector2i>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl Player{
    pub fn new() -> Player {
        Player {
            snake : Snake::new(Vector2i::new(0, 0), 
             Direction::PlusX, INITIAL_LENGTH),
            score : 0,
            control : None,
        }
    }
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
    /// Adds new player. Returns new player index that can
    /// be used for referencing this player
    pub fn register_player(&mut self, control : Option<UserControlRx>) -> PlayerIndex {
        let new_player_index = self.players.len();
        // make spawn point
        let spawn_pos = Game::calc_spawn_pos(new_player_index, INITIAL_LENGTH, self.field_size);
        let mut player = Player::new();
        player.control = control;
        self.players.push(player);
        new_player_index
    }

    /// Starts the game loop. This function will return only when game is over.
    /// Or shutdown command was received.
    pub fn game_loop(&mut self, shutdown_rx : mpsc::Receiver<()>) {
        
        loop {
            //Check shutdown
            if let Ok(_) = shutdown_rx.try_recv() {
                break;
            }
        }
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

    /// Generate the grid that represents the current state of the game
    pub fn generate_grid(&self) -> Grid {
        let mut grid = 
            Grid::from_elem(
                (self.field_size.x as usize, self.field_size.y as usize),
                 GridCell::Empty);
        // Add pizzas
        for pizza in &self.pizzas {
            grid[[pizza.x as usize, pizza.y as usize]] = GridCell::Pizza(PizzaRec{});
        }

        // Add snakes
        for (player_i, player) in self.players.iter().enumerate()
        {
            let player_i = player_i as PlayerIndex;
            let snake_len = player.snake.body().len();
            for (part_i, body_part) in player.snake.body().iter().enumerate() {

                let cell = match part_i {
                    0 => 
                        GridCell::Snake(SnakeRec{body_part : SnakeBodyPart::Head, player_index : player_i}),
                    _ if part_i == snake_len - 1 => 
                        GridCell::Snake(SnakeRec{body_part : SnakeBodyPart::Tail, player_index : player_i}),
                    _ => 
                        GridCell::Snake(SnakeRec{body_part : SnakeBodyPart::Body, player_index : player_i}),
                };
                grid[[body_part.x as usize, body_part.y as usize]] = cell;
            }
        }

        // Return
        grid
    }

    /// Calculate spawn position for the pizza
    fn calc_spaw_pos_for_pizza(grid : &Grid, estimated_free_cells : usize) -> Vector2i {
        // Randomly generate the free cell index
        let mut free_cell_counter = rand::random::<usize>() % estimated_free_cells;   
        // Loop the grid and find empty cell with the given index
        for ((x, y), cell) in grid.indexed_iter() {
            if *cell == GridCell::Empty {
                if free_cell_counter == 0 {
                    return Vector2i::new(x as i32, y as i32);
                }
                else {
                     free_cell_counter -= 1; 
                };
            }
        }
        // Should never happen
        panic!("Could not find free cell");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test each new player gets new index
    #[test]
    fn test_register_controlled_player() {
        let mut game = Game::new( Vector2i::new(10, 10));
        let player1 = game.register_player(None);
        let player2 = game.register_player(None);
        let player3 = game.register_player(None);
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
        game.register_player(None);
        assert_eq!(game.num_empty_cells(), 100 - INITIAL_LENGTH as i32);
        game.register_player(None);
        assert_eq!(game.num_empty_cells(), 100 - 2 * INITIAL_LENGTH as i32);
        
        // Add some food
        game.pizzas.push(Vector2i::new(0, 0));
        game.pizzas.push(Vector2i::new(0, 1));
        
        assert_eq!(game.num_empty_cells(), 100 - 2 * INITIAL_LENGTH as i32 - 2);
    }

    // Test generate gird
    #[test]
    fn test_generate_grid() {
        let mut game = Game::new( Vector2i::new(3, 3));
        let player1 = game.register_player(None);
        // Manually set the snake points to make it easier to test
        game.players[player1].snake.set_body(vec![
            Vector2i::new(0, 0),
            Vector2i::new(0, 1),
            Vector2i::new(0, 2),
        ]);
        // Add one pizza
        game.pizzas.push(Vector2i::new(2, 2));
        // Generate grid
        let grid = game.generate_grid();
        // Check grid
        assert_eq!(grid[[0, 0]], GridCell::Snake(SnakeRec{body_part : SnakeBodyPart::Head, player_index : player1}));
        assert_eq!(grid[[0, 1]], GridCell::Snake(SnakeRec{body_part : SnakeBodyPart::Body, player_index : player1}));
        assert_eq!(grid[[0, 2]], GridCell::Snake(SnakeRec{body_part : SnakeBodyPart::Tail, player_index : player1}));
        assert_eq!(grid[[1, 0]], GridCell::Empty);
        assert_eq!(grid[[1, 1]], GridCell::Empty);
        assert_eq!(grid[[1, 2]], GridCell::Empty);
        assert_eq!(grid[[2, 0]], GridCell::Empty);
        assert_eq!(grid[[2, 1]], GridCell::Empty);
        assert_eq!(grid[[2, 2]], GridCell::Pizza(PizzaRec{}));
    }

    // Test game_loop shutdown
    #[test]
    fn test_game_loop_shutdown() {
        // Create mpsc channel
        let (tx, rx) = mpsc::channel();

        let mut game = Box::new(Game::new( Vector2i::new(10, 10)));

        let handle = std::thread::spawn(move || {
            // Start game loop
            game.game_loop(rx);
        });
        // Send shutdown immediately;
        tx.send(()).unwrap();

        // Wait 20 times for 50 ms. Every time checking that thread finished
        for _ in 0..20 {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if handle.is_finished() {
                break;
            }
        }

        // Check the thread is done
        assert!(handle.is_finished());

        // Wait for thread to finish
        handle.join().unwrap();
    }

}