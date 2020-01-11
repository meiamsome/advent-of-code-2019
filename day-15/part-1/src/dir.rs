use super::Position;

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn all() -> Vec<Dir> {
        vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }

    pub fn between(start: Position, end: Position) -> Option<Dir> {
        match (end.0 - start.0, end.1 - start.1) {
            (1, 0) => Some(Dir::Right),
            (-1, 0) => Some(Dir::Left),
            (0, 1) => Some(Dir::Down),
            (0, -1) => Some(Dir::Up),
            _ => None,
        }
    }

    pub fn move_in_dir(&self, coords: Position) -> Position {
        match self {
            Dir::Up => (coords.0, coords.1 - 1),
            Dir::Left => (coords.0 - 1, coords.1),
            Dir::Down => (coords.0, coords.1 + 1),
            Dir::Right => (coords.0 + 1, coords.1),
        }
    }
}

impl Into<i64> for Dir {
    fn into(self) -> i64 {
        match self {
            Dir::Up => 1,
            Dir::Left => 3,
            Dir::Down => 2,
            Dir::Right => 4,
        }
    }
}
