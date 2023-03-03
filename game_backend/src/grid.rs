
/// Snake body part enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnakeBodyPart {
    /// Snake head
    Head,
    /// Snake body
    Body,
    /// Snake tail
    Tail,
}

/// Snake rec describes the cell that is occupied by a snake.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnakeRec
{
    pub player_index: i32,
    pub part: SnakeBodyPart,
}

/// Food rec structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoodRec
{
}

/// Cell enum represents the contents of a cell in the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Snake(SnakeRec),
    Food(FoodRec),
}

/// Grid type
type Grid = ndarray::Array2<Cell>;