use crate::base::Vector2i;
use crate::base::Direction;

/// Snake struct.
/// look_direction: The direction the snake is looking to move.
/// body: The body of the snake. First element represents head.
/// grow_counter: The number of steps the snake can make with growth.
/// When snake does a "grow" step - the head moves, but tail doesn't. 
#[derive(Debug, Clone)]
pub struct Snake {
    look_direction: Direction,
    body: Vec<Vector2i>,
    grow_counter : i32,
}

impl Snake
{
    /// Getter for direction
    pub fn look_direction(&self) -> Direction {
        self.look_direction
    }
    /// Returns backward direction. This is the direction snake came from.
    /// Basically it's a difference between head and second element of the body.
    pub fn backward_direction(&self) -> Vector2i {
        let res = self.body[1] - self.body[0];
        // Make sure the length is 1
        assert!(res.x.abs() + res.y.abs() == 1, "Add support for gaps between snake body parts");
        
        res
    }

    /// Getter for body.
    pub fn body(&self) -> &Vec<Vector2i> {
        &self.body
    }
    /// Setter for body
    pub fn set_body(&mut self, body: Vec<Vector2i>) {
        self.body = body;
    }

    /// Tries to set new look direction if possible.
    /// It is not possible to set look direction that is the same
    /// as backward direction.
    /// Returns true if resulting direction is same as specified
    pub fn try_set_look_direction(&mut self, direction: Direction) -> bool {
        let backward_dir = self.backward_direction();
        let new_dir = Vector2i::from_direction(direction);
        if backward_dir != new_dir {
            self.look_direction = direction;
        }
        
        direction == self.look_direction
    }

    /// Create a new snake.
    /// position: The position of the head of the snake.
    /// direction: The direction the snake is moving.
    /// length: The length of the snake.
    /// 
    /// #panics
    /// Panics if the length is < 2.
    pub fn new(position: Vector2i, direction: Direction, length: u32) -> Snake {
        // There is a logic in system that relies on head and tail to be different cells
        assert!(length >= 2, "Snake length must be >= 2");
        let dir_vec = Vector2i::from_direction(direction);
        let mut body = Vec::new();
        for i in 0..length {
            body.push(position - dir_vec * (i as i32));
        }
        Snake {
            look_direction : direction,
            body,
            grow_counter : 0,
        }
    }

    /// Eat specified amount of food
    pub fn eat(&mut self, food: i32) {
        self.grow_counter += food;
    }

    /// Move the snake 1 step in current direction
    pub fn move_forward(&mut self) {
        let move_dir = Vector2i::from_direction(self.look_direction);
        let new_head = self.body[0] + move_dir;
        self.body.insert(0, new_head);

        // Snake grows if grow_counter > 0
        if self.grow_counter > 0 {
            self.grow_counter -= 1;
        } 
        else {
            self.body.pop();
        }
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snake_new() {

        let snake = Snake::new(
            Vector2i::new(0,0), 
            Direction::PlusX, 3);
        assert_eq!(snake.look_direction, Direction::PlusX);
        assert_eq!(snake.body, vec![Vector2i::new(0,0), Vector2i::new(-1,0), Vector2i::new(-2,0)]);
    }

    #[test]
    fn test_snake_move_forward() {
        let mut snake = Snake::new(
            Vector2i::new(0,0), 
            Direction::PlusX, 3);
        snake.move_forward();
        assert_eq!(snake.look_direction, Direction::PlusX);
        assert_eq!(snake.body, vec![Vector2i::new(1,0), Vector2i::new(0,0), Vector2i::new(-1,0)]);
    }

    // Test move forward after eat
    #[test]
    fn test_snake_move_forward_after_eat() {
        let mut snake = Snake::new(
            Vector2i::new(0,0), 
            Direction::PlusX, 3);
        snake.eat(2);
        snake.move_forward();
        assert_eq!(snake.look_direction, Direction::PlusX);
        assert_eq!(snake.body, vec![Vector2i::new(1,0), Vector2i::new(0,0), Vector2i::new(-1,0), Vector2i::new(-2,0)]);
        snake.move_forward();
        assert_eq!(snake.look_direction, Direction::PlusX);
        assert_eq!(snake.body, vec![Vector2i::new(2,0), Vector2i::new(1,0), 
            Vector2i::new(0,0), Vector2i::new(-1,0), Vector2i::new(-2,0)]);

    }

    // test try_set_look_direction
    #[test]
    fn test_snake_try_set_look_direction() {
        let mut snake = Snake::new(
            Vector2i::new(0,0), 
            Direction::PlusX, 3);
        assert_eq!(snake.try_set_look_direction(Direction::PlusY), true);
        assert_eq!(snake.look_direction, Direction::PlusY);
        assert_eq!(snake.try_set_look_direction(Direction::MinusX), false);
        assert_eq!(snake.look_direction, Direction::PlusY);
        assert_eq!(snake.try_set_look_direction(Direction::MinusY), true);
        assert_eq!(snake.look_direction, Direction::MinusY);
        assert_eq!(snake.try_set_look_direction(Direction::PlusX), true);
        assert_eq!(snake.look_direction, Direction::PlusX);
        
    }
}