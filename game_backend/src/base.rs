use std::ops::Add;

pub type PlayerIndex = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

/// Enum describing direction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    PlusX,
    MinusX,
    PlusY,
    MinusY,
}

impl Vector2i
{
    /// Make unit X vector
    pub fn unit_x() -> Vector2i {
        Vector2i { x: 1, y: 0 }
    }
    /// Make unit Y vector
    pub fn unit_y() -> Vector2i {
        Vector2i { x: 0, y: 1 }
    }
    /// Make zero vector
    pub fn zero() -> Vector2i {
        Vector2i { x: 0, y: 0 }
    }
    /// Make vector from 2 values
    pub fn new(x: i32, y: i32) -> Vector2i {
        Vector2i { x, y }
    }
    /// Make vector from array of 2
    pub fn from_array(array: [i32; 2]) -> Vector2i {
        Vector2i { x: array[0], y: array[1] }
    }
    /// Make vector from direction
    pub fn from_direction(direction: Direction) -> Vector2i {
        match direction {
            Direction::PlusX => Vector2i::unit_x(),
            Direction::MinusX => -Vector2i::unit_x(),
            Direction::PlusY => Vector2i::unit_y(),
            Direction::MinusY => -Vector2i::unit_y(),
        }
    }
}

impl Add for Vector2i {
    type Output = Vector2i;

    fn add(self, other: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2i {
    type Output = Vector2i;

    fn sub(self, other: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Neg for Vector2i {
    type Output = Vector2i;

    fn neg(self) -> Vector2i {
        Vector2i {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, other: i32) -> Vector2i {
        Vector2i {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    // Test vector addition
    #[test]
    fn test_vector_add() {
        let a = Vector2i { x: 1, y: 2 };
        let b = Vector2i { x: 3, y: 4 };
        let c = a + b;
        assert_eq!(c, Vector2i { x: 4, y: 6 });
    }

    // Test vector subtraction
    #[test]
    fn test_vector_sub() {
        let a = Vector2i { x: 1, y: 2 };
        let b = Vector2i { x: 3, y: 4 };
        let c = a - b;
        assert_eq!(c, Vector2i { x: -2, y: -2 });
    }

    // Test vector negation
    #[test]
    fn test_vector_neg() {
        let a = Vector2i { x: 1, y: 2 };
        let b = -a;
        assert_eq!(b, Vector2i { x: -1, y: -2 });
    }
}