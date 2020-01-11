use std::fmt::{Display, Formatter};

use super::explorer::InstructionIterator;
use super::tile::Tile;
use super::Position;

pub struct Maze {
    pub map: Vec<Vec<Tile>>,
    pub start: Position,
}

impl From<InstructionIterator> for Maze {
    fn from(input: InstructionIterator) -> Maze {
        let seen_locations = input.map.lock().unwrap();

        let min_x = *seen_locations.keys().map(|(x, _)| x).min().unwrap();
        let min_y = *seen_locations.keys().map(|(_, y)| y).min().unwrap();
        let max_x = *seen_locations.keys().map(|(x, _)| x).max().unwrap();
        let max_y = *seen_locations.keys().map(|(_, y)| y).max().unwrap();

        Maze {
            map: (min_y..=max_y)
                .map(|y| {
                    (min_x..=max_x)
                        .map(|x| *seen_locations.get(&(x, y)).unwrap_or(&Tile::Unexplored))
                        .collect()
                })
                .collect(),
            start: (-min_x, -min_y),
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, tile)| {
                            if (x as i64, y as i64) == self.start {
                                return "S";
                            }
                            tile.into()
                        })
                        .collect::<String>()
                        + "\n"
                })
                .collect::<String>()
        )
    }
}
