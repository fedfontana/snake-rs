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
    pub facing: Direction,
    pub pieces: Vec<Position>,
}

impl Snake {
    pub fn new(canvas_w: usize, canvas_h: usize) -> Self {
        Self {
            facing: Direction::Right,
            pieces: vec![Position {
                col: canvas_w / 2,
                row: canvas_h / 2,
            }],
        }
    }
}
