use crate::grid::Grid;

/// The short summary information about player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerSummary
{
    pub score : u32,
    pub alive : bool,
}

/// The structure that represents an update event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update
{
    pub grid : Grid,
    pub players_summary : Vec<PlayerSummary>,
}

/// The structure that represents the game over event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOver
{
    pub players_summary : Vec<PlayerSummary>,
}

/// The enum that represents a global game event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalEvent
{
    Update(Update),
    GameOver(GameOver),
}