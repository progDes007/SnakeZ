use game_backend::Snake;
use game_backend::Vector2i;

fn main() {
    let snake = Snake::new(Vector2i{x : 0, y: 0}, Vector2i{x : 1, y: 0}, 3);
    println!("snake.direction: {:?}", snake.direction());
    println!("snake.body: {:?}", snake.body());
}
