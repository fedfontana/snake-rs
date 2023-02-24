use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    /// The direction faced by the head of the snake
    pub facing: Direction,
    /// The positions of the segments of the snake. Segments are ordered, with the head in position 0.
    pub pieces: VecDeque<Position>,
}

impl Snake {
    pub fn new(canvas_w: usize, canvas_h: usize) -> Self {
        let mut pieces = VecDeque::new();
        pieces.push_back(Position::new(canvas_h / 2, canvas_w / 2));
        Self {
            facing: Direction::Right,
            pieces,
        }
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.facing = dir;
    }

    pub fn grow(&mut self, new_segment_pos: Position) {
        self.pieces.push_back(new_segment_pos);
    }
}
