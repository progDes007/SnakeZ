use crate::base::PlayerIndex;

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
    pub player_index: PlayerIndex,
    pub body_part: SnakeBodyPart,
}

/// Pizza rec structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PizzaRec
{
}

/// Cell enum represents the contents of a cell in the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCell {
    Empty,
    Snake(SnakeRec),
    Pizza(PizzaRec),
}

/// Grid type
pub type Grid = ndarray::Array2<GridCell>;