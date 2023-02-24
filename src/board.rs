use std::fmt::Display;

use crate::snake::{Position, Snake};

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
            snake: Snake::new(w, h)
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = vec!["_"; (self.width+1) * self.height];
        for row in 0..self.height {
            repr[row * (self.width+1)] = "\n";
        }
        
        for fruit in self.fruits.iter() {
            repr[fruit.row * (self.width+1) + fruit.col] = "x";
        }

        for (idx, part) in self.snake.pieces.iter().enumerate() {
            let cell_repr = if idx == 0 {
                match self.snake.facing {
                    crate::snake::Direction::Up => "^",
                    crate::snake::Direction::Down => "V",
                    crate::snake::Direction::Left => "<",
                    crate::snake::Direction::Right => ">",
                }
            } else {
                "#"
            };

            repr[part.row * (self.width+1) + part.col + 1] = cell_repr;            
        }

        write!(f, "{}", repr.join(""))
    }
}