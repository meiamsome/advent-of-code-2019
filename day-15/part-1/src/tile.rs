#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Oxygen,
    Unexplored,
}

impl From<i64> for Tile {
    fn from(other: i64) -> Self {
        match other {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            _ => panic!(),
        }
    }
}

impl Into<&str> for &Tile {
    fn into(self) -> &'static str {
        match self {
            Tile::Empty => "·",
            Tile::Wall => "#",
            Tile::Oxygen => "O",
            Tile::Unexplored => " ",
        }
    }
}
