use std::fmt::Display;

use crate::snake::{Direction, Position, Snake};

pub struct Board {
    width: usize,
    height: usize,
    fruits: Vec<Position>,
    snake: Snake,
}

impl Board {
    pub fn new_with_size(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            fruits: Vec::new(),
            snake: Snake::new(w, h),
        }
    }
    /// Advances in the direction of the head. Returns whether the snake hit a wall or itself while advancing.
    pub fn advance_snake(&mut self) -> bool {
        let Some(head) = self.snake.pieces.front().cloned() else { return true };

        self.snake.pieces.pop_back();

        let new_position = match self.snake.facing {
            Direction::Up => {
                if head.row > 0 {
                    Some(Position::new(head.row - 1, head.col))
                } else {
                    None
                }
            }
            Direction::Down => {
                if head.row + 1 <= self.height {
                    Some(Position::new(head.row + 1, head.col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if head.col > 0 {
                    Some(Position::new(head.row, head.col - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if head.col + 1 <= self.width {
                    Some(Position::new(head.row, head.col + 1))
                } else {
                    None
                }
            }
        };

        // if the new position is not out of bounds assin it to new_position, else return false
        let Some(new_position) = new_position else { return false };

        // return true if the snake hit itself
        if self.snake.pieces.contains(&new_position) {
            return true;
        }

        self.snake.pieces.push_front(new_position);
        
        false
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = vec!["_"; (self.width + 1) * self.height];
        for row in 0..self.height {
            repr[row * (self.width + 1)] = "\n";
        }

        for fruit in self.fruits.iter() {
            repr[fruit.row * (self.width + 1) + fruit.col] = "x";
        }

        for (idx, part) in self.snake.pieces.iter().enumerate() {
            let cell_repr = if idx == 0 {
                match self.snake.facing {
                    Direction::Up => "^",
                    Direction::Down => "V",
                    Direction::Left => "<",
                    Direction::Right => ">",
                }
            } else {
                "#"
            };

            repr[part.row * (self.width + 1) + part.col + 1] = cell_repr;
        }

        write!(f, "{}", repr.join(""))
    }
}
