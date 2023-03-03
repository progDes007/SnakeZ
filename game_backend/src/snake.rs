use crate::base::Vector2i;

/// Snake struct.
/// direction: The direction the snake is moving.
/// body: The body of the snake. First element represents head.
/// grow_counter: The number of steps the snake can make with growth.
/// When snake does a "grow" step - the head moves, but tail doesn't. 
pub struct Snake {
    direction: Vector2i,
    body: Vec<Vector2i>,
    grow_counter : i32,
}

impl Snake
{
    /// Getter for direction
    pub fn direction(&self) -> Vector2i {
        self.direction
    }
    /// Getter for body.
    pub fn body(&self) -> &Vec<Vector2i> {
        &self.body
    }

    /// Create a new snake.
    /// position: The position of the head of the snake.
    /// direction: The direction the snake is moving.
    /// length: The length of the snake.
    /// 
    /// #panics
    /// Panics if the length is 0.
    pub fn new(position: Vector2i, direction: Vector2i, length: u32) -> Snake {
        assert!(length > 0);
        let mut body = Vec::new();
        for i in 0..length {
            body.push(position - direction * (i as i32));
        }
        Snake {
            direction,
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
        let new_head = self.body[0] + self.direction;
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
            Vector2i::unit_x(), 3);
        assert_eq!(snake.direction, Vector2i::unit_x());
        assert_eq!(snake.body, vec![Vector2i::new(0,0), Vector2i::new(-1,0), Vector2i::new(-2,0)]);
    }

    #[test]
    fn test_snake_move_forward() {
        let mut snake = Snake::new(
            Vector2i::new(0,0), 
            Vector2i::unit_x(), 3);
        snake.move_forward();
        assert_eq!(snake.direction, Vector2i::unit_x());
        assert_eq!(snake.body, vec![Vector2i::new(1,0), Vector2i::new(0,0), Vector2i::new(-1,0)]);
    }

    // Test move forward after eat
    #[test]
    fn test_snake_move_forward_after_eat() {
        let mut snake = Snake::new(
            Vector2i::new(0,0), 
            Vector2i::unit_x(), 3);
        snake.eat(2);
        snake.move_forward();
        assert_eq!(snake.direction, Vector2i::unit_x());
        assert_eq!(snake.body, vec![Vector2i::new(1,0), Vector2i::new(0,0), Vector2i::new(-1,0), Vector2i::new(-2,0)]);
        snake.move_forward();
        assert_eq!(snake.direction, Vector2i::unit_x());
        assert_eq!(snake.body, vec![Vector2i::new(2,0), Vector2i::new(1,0), 
            Vector2i::new(0,0), Vector2i::new(-1,0), Vector2i::new(-2,0)]);

    }
}