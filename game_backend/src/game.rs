use crate::snake::Snake;
use crate::base::{Vector2i, PlayerIndex, Direction};
use crate::grid::{Grid, GridCell, PizzaRec, SnakeRec, SnakeBodyPart};
//use std::boxed::Box;
use std::sync::mpsc;
use std::time;
use rand;

const INITIAL_LENGTH : u32 = 2;
const UPDATE_INTERVAL : time::Duration = time::Duration::from_millis(500);


pub type UserControlRx = mpsc::Receiver<Direction>;

/// The object that stores data associated with single player in the game
struct Player
{
    /// There is no snake if player is dead
    snake : Option<Snake>,
    score : u32,
    control : Option<UserControlRx>,
}
/// Enum that describes one of the things that may happen with a snake during update step
#[derive (Debug, Clone, Copy, PartialEq, Eq)]
enum ActionStep
{
    /// Snake can't move because other snake competes for the same positition
    Hold,
    /// Snake moves in the direction it's looking at
    Move,
    /// Snake dies because it collides with other snake or with the wall
    Die,
}

/// Game object. Create and configure it to start a game.
pub struct Game {
    players : Vec<Player>,
    field_size : Vector2i,
    pizzas : Vec<Vector2i>,
    grid : Grid,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl Player{
    pub fn new() -> Player {
        Player {
            snake : Some(Snake::new(Vector2i::new(0, 0), 
                Direction::PlusX, INITIAL_LENGTH)),
            score : 0,
            control : None,
        }
    }

    // Read inputs for players
    fn read_inputs(&mut self) {
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
}

impl Game {
    /// Creates new unitialized game object
    pub fn new(field_size : Vector2i) -> Game {
        Game {
            players : Vec::new(),
            field_size,
            pizzas : Vec::new(),
            grid : Grid::from_elem((0,0), GridCell::Empty)
        }
    }
    /// Adds new player. Returns new player index that can
    /// be used for referencing this player
    pub fn register_player(&mut self, control : Option<UserControlRx>) -> PlayerIndex {
        let new_player_index = self.players.len();
        // make spawn point
        let (spaw_pos, spawn_dir) = Game::calc_spawn_pos(new_player_index, INITIAL_LENGTH, self.field_size);
        let mut player = Player::new();
        player.control = control;
        player.snake = Some(Snake::new(spaw_pos, spawn_dir, INITIAL_LENGTH));
        self.players.push(player);
        new_player_index
    }

    /// Helper function that moves player snake
    /// #panics
    /// When player dead
    fn move_player(&mut self, player_index : PlayerIndex) {
        let player = &mut self.players[player_index];
        // Get snake. Snake is expected
        let snake = player.snake.as_mut().unwrap();
        // Move the snake
        snake.move_forward();
        // see if there is pizza
        let head_pos = snake.body()[0];
        if let Some(pizza_index) = self.pizzas.iter().position(|p| *p == head_pos) {
            // Eat pizza
            snake.eat(1);
            player.score += 1;
            // Remove pizza
            self.pizzas.remove(pizza_index);
        }
    }

    /// Execute single update step
    fn step(&mut self) {
        // Predict the step action for every player
        let mut actions = Vec::new();
        // Predict action for each snake. Dead snakes just hold
        for player_index in 0..self.players.len() {
            let action= self.predict_next_action(player_index);
            actions.push(action);
        }

        // Apply the actions
        for player_index in 0..self.players.len() {
            // Match the action
            match actions[player_index] {
                ActionStep::Hold => {
                    // Do nothing
                },
                ActionStep::Move => {
                    // Move the snake
                    self.move_player(player_index);
                },
                ActionStep::Die => {
                    // Kill the snake
                    self.players[player_index].kill();
                },
            }
        }


    }

    /// Starts the game loop. This function will return only when game is over.
    /// Or shutdown command was received.
    pub fn game_loop(&mut self, shutdown_rx : mpsc::Receiver<()>) {

        // Generate initial grid
        self.grid = self.generate_grid();

        // Start the timer
        let mut timer = time::Instant::now();

        // Start actual loop
        loop {
            // The game is over when all players are dead
            let game_over = !(self.players.iter().any(|p| p.alive()));
            if game_over
            {
                break;
            }
            //Check shutdown
            if let Ok(_) = shutdown_rx.try_recv() {
                break;
            }

            // Read all players inputs on every loop
            for player in &mut self.players {
                player.read_inputs();
            }

            // Measure time elapsed
            let elapsed = timer.elapsed();
            if elapsed > UPDATE_INTERVAL {
                // Substract updated interval from running timer. That way any leftover time
                // will be counted towards the next update interval.
                timer = timer.checked_sub(UPDATE_INTERVAL).unwrap_or(time::Instant::now());

                // Do update step
                self.step();

                // Update grid
                self.grid = self.generate_grid();
            }
        }
    }

    /// REturns number of empty cells in the field.
    fn num_empty_cells(&self) -> i32 {
        let mut num = (self.field_size.x * self.field_size.y) as i32;
        // Substract pizas
        num -= self.pizzas.len() as i32;
        // Substract length of every snake that is alive
        for player in &self.players {
            if player.alive() {
                num -= player.snake.as_ref().unwrap().body().len() as i32;
            }
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
    fn calc_spawn_pos(index : PlayerIndex, length : u32, field_size : Vector2i) -> (Vector2i, Direction)
    {
        assert!(index < 4);
        let center = Vector2i::new(field_size.x / 2, field_size.y / 2);
        let mut pos = center;
        let dir: Direction;
        match index {
            0 => {
                pos.x -= length as i32;
                dir = Direction::MinusX;
            },
            1 => {
                pos.y -= length as i32;
                dir = Direction::MinusY;
            },
            2 => {
                pos.x += length as i32;
                dir = Direction::PlusX;
            },
            3 => {
                pos.y += length as i32;
                dir = Direction::PlusY;
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
            // Skip dead
            if !player.alive() { continue; }
            // Get snake
            let snake = player.snake.as_ref().unwrap();
            let player_i = player_i as PlayerIndex;
            let snake_len = snake.body().len();
            for (part_i, body_part) in snake.body().iter().enumerate() {

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

    /// Predicts the next action that particular player snake will do in next step.
    /// #panics
    /// If player is dead
    fn predict_next_action( &self, player_index : PlayerIndex) -> ActionStep {
        // Dead players alwats hold
        if !self.players[player_index as usize].alive() {
            return ActionStep::Hold;
        }
        // First estimate the coordinate of potential new head
        let player = &self.players[player_index as usize];
        let player_snake = player.snake.as_ref().unwrap();
        let mut new_head = player_snake.body()[0];
        new_head += Vector2i::from_direction(player_snake.look_direction());
        // Check if the new head is inside the field
        if new_head.x < 0 || new_head.x >= self.field_size.x ||
           new_head.y < 0 || new_head.y >= self.field_size.y {
            return ActionStep::Die;
        }

        // See if new head position is occupied by body OR head of any snake
        for player in &self.players {
            if !player.alive() { continue; }
            // Get the snake ref
            let any_snake = player.snake.as_ref().unwrap();
            // Check all body parts except last (tail)
            for body_part in &any_snake.body()[..any_snake.body().len() - 1] {
                if *body_part == new_head {
                    return ActionStep::Die;
                }
            }
        }

        // If any other snake compete to the same head position, then hold
        // Loop snake with index. Skip current.
        for (other_player_index, other_player) in self.players.iter().enumerate() {
            if other_player_index == player_index as usize || !other_player.alive() {
                continue;
            }
            // Get other snake
            let other_snake = other_player.snake.as_ref().unwrap();
            // Estimate this snake expected head position
            let mut other_new_head = other_snake.body()[0];
            other_new_head += Vector2i::from_direction(other_snake.look_direction());
            // If this position is the same - hold
            if other_new_head == new_head {
                return ActionStep::Hold;
            }
        }
        // In all other cases snake can move
        ActionStep::Move
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test each new player gets new index
    #[test]
    fn test_register_player() {
        let mut game = Game::new( Vector2i::new(10, 10));
        let player1 = game.register_player(None);
        let player2 = game.register_player(None);
        let player3 = game.register_player(None);
        assert_eq!(player1, 0);
        assert_eq!(player2, 1);
        assert_eq!(player3, 2);
        // Test that snake is created
        assert!(game.players[player1].snake.is_some());
        assert!(game.players[player2].snake.is_some());
        assert!(game.players[player3].snake.is_some());
    }

    // Test calc_spawn_pos
    #[test]
    fn test_calc_spawn_pos() {
        let field_size = Vector2i::new(10, 10);
        let (pos, dir) = Game::calc_spawn_pos(0, 3, field_size);
        assert_eq!(pos, Vector2i::new(2, 5));
        assert_eq!(dir, Direction::MinusX);

        let (pos, dir) = Game::calc_spawn_pos(1, 3, field_size);
        assert_eq!(pos, Vector2i::new(5, 2));
        assert_eq!(dir, Direction::MinusY);

        let (pos, dir) = Game::calc_spawn_pos(2, 3, field_size);
        assert_eq!(pos, Vector2i::new(8, 5));
        assert_eq!(dir, Direction::PlusX);

        let (pos, dir) = Game::calc_spawn_pos(3, 3, field_size);
        assert_eq!(pos, Vector2i::new(5, 8));
        assert_eq!(dir, Direction::PlusY);
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
        // Register dead player
        game.register_player(None);
        game.players[2].snake = None;
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
        game.players[player1].snake.as_mut().unwrap().set_body(vec![
            Vector2i::new(0, 0),
            Vector2i::new(0, 1),
            Vector2i::new(0, 2),
        ]);
        // Create another dead player, which should have 0 effect on grid
        {
            let player2 = game.register_player(None);
            game.players[player2].snake = None;
        }

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

    // Test predict_next_action
    #[test]
    fn test_predict_next_action() {
        // Create small 4x4 game
        let mut game = Game::new( Vector2i::new(4, 4));
        let player_index0 = game.register_player(None);
        
        // Single snake going out of bounds dies
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(0, 3),
                Vector2i::new(0, 2),
            ]);
            assert!( snake0.try_set_look_direction( Direction::PlusY ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Die);
        }
        // Single snake going to current tail pos: moves. This is because during the move
        // this cell will be freed
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(1, 2),
                Vector2i::new(2, 2),
                Vector2i::new(2, 1), 
                Vector2i::new(1, 1),   
            ]);     
            assert!( snake0.try_set_look_direction( Direction::MinusY ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Move);
        }
        // The snake that attempts to move to it's own body pos - dies
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(1, 2),
                Vector2i::new(2, 2),
                Vector2i::new(2, 1), 
                Vector2i::new(1, 1),   
                Vector2i::new(0, 1), 
            ]);     
            assert!( snake0.try_set_look_direction( Direction::MinusY ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Die);
        }
        // Add one more small snake for further tests
        let player_index1 = game.register_player(None);
        {
            let snake1 = game.players[player_index1].snake.as_mut().unwrap();
                snake1.set_body(vec![
                    Vector2i::new(3, 2),
                    Vector2i::new(3, 1),
                    Vector2i::new(3, 0),
                ]);
            assert!( snake1.try_set_look_direction( Direction::PlusY ));
        }
        // Add dead player. Mainly to make sure it doesn't crash. It should cause no real affects.
        {
            let player_index2 = game.register_player(None);
            game.players[player_index2].snake = None;
            // Dead player always hold
            assert_eq!(game.predict_next_action(player_index2), ActionStep::Hold);
        }

        // When snake tries to move to the head position of other snake - it dies
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(2, 2),
                Vector2i::new(1, 2),
            ]);     
            assert!( snake0.try_set_look_direction( Direction::PlusX ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Die);
        }
        // When snake tries to move to the body position of other snake - it dies
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(2, 3),
                Vector2i::new(1, 3),
            ]);     
            assert!( snake0.try_set_look_direction( Direction::PlusX ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Hold);
        }
        // When snake tries to move ot the tail position of other snake - it moves. This is because during the move
        // this position will be freed
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(2, 0),
                Vector2i::new(1, 0),
            ]);     
            assert!( snake0.try_set_look_direction( Direction::PlusX ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Move);
        }
        // When snake competes with other snake for same position - it holds
        {
            let snake0 = game.players[player_index0].snake.as_mut().unwrap();
            snake0.set_body(vec![
                Vector2i::new(2, 0),
                Vector2i::new(1, 0),
            ]);     
            assert!( snake0.try_set_look_direction( Direction::PlusX ));
            assert_eq!(game.predict_next_action(player_index0), ActionStep::Move);
        }

    }

    // Test move_player
    #[test]
    fn test_move_player() {
        // Create small 4x4 game
        let mut game = Game::new( Vector2i::new(4, 4));
        let player_index0 = game.register_player(None);
        {
            let snake = game.players[player_index0].snake.as_mut().unwrap();
            // Setup snake such that it can move forward 2 times
            snake.set_body(vec![
                Vector2i::new(0, 1), 
                Vector2i::new(0, 0),   
            ]);
            assert!( snake.try_set_look_direction( Direction::PlusY ));
        }
 
        // Also add one pizza
        game.pizzas.push(Vector2i::new(0, 3));
        // First move
        game.move_player(player_index0);
        // Doesn't eat pizza. Doesn't increase score
        assert_eq!(game.pizzas.len(), 1);
        assert_eq!(game.players[player_index0].score, 0);
        // Second move
        // Eats pizza and increase score
        game.move_player(player_index0);
        assert_eq!(game.pizzas.len(), 0);
        assert_eq!(game.players[player_index0].score, 1);
        // Also check final snake position
        assert_eq!(*game.players[player_index0].snake.as_ref().unwrap().body(), vec![
            Vector2i::new(0, 3), 
            Vector2i::new(0, 2)
        ]);

    }
}