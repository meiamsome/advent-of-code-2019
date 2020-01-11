use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use super::dir::Dir;
use super::maze::Maze;
use super::tile::Tile;
use super::Position;

fn dist(a: Position, b: Position) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

pub struct AStar {
    pub dist_fn: &'static dyn (Fn(Position, Position) -> i64),
    pub open_set: HashMap<Position, (i64, Option<Position>)>,
    pub closed_set: HashMap<Position, (i64, Option<Position>)>,
    pub maze: Maze,
    pub target: Position,
}

impl AStar {
    pub fn get_path_to(&self, position: Position) -> Option<Vec<Position>> {
        self.closed_set
            .get(&position)
            .and_then(|(_, previous)| {
                previous
                    .and_then(|previous| self.get_path_to(previous))
                    .or_else(|| Some(vec![]))
            })
            .map(|mut positions| {
                positions.push(position);
                positions
            })
    }
}

impl Iterator for AStar {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((&pos, _)) = self
            .open_set
            .iter()
            .map(|(pos, (path_len, _))| (pos, path_len + (self.dist_fn)(*pos, self.target)))
            .min_by_key(|(_, score)| *score)
        {
            let (path_len, last) = self.open_set.remove(&pos).unwrap();
            self.closed_set.insert(pos, (path_len, last));

            if pos == self.target {
                return None;
            }

            for dir in Dir::all() {
                let new_pos = dir.move_in_dir(pos);
                if self.maze.map[pos.1 as usize][pos.0 as usize] == Tile::Wall {
                    continue;
                }
                if self.closed_set.contains_key(&new_pos) {
                    continue;
                }
                if let Some((dist, _)) = self.open_set.get(&new_pos) {
                    if *dist < path_len + 1 {
                        continue;
                    }
                }
                self.open_set.insert(new_pos, (path_len + 1, Some(pos)));
            }

            return Some(());
        }
        None
    }
}

impl From<Maze> for AStar {
    fn from(other: Maze) -> AStar {
        let target = other
            .map
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| ((x as i64, y as i64), *tile))
                    .find(|(_, tile)| *tile == Tile::Oxygen)
            })
            .map(|(pos, _)| pos)
            .next()
            .unwrap();
        AStar {
            dist_fn: &dist,
            open_set: vec![(other.start, (0, None))].into_iter().collect(),
            closed_set: HashMap::new(),
            maze: other,
            target,
        }
    }
}

impl Display for AStar {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.maze
                .map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, tile)| {
                            if (x as i64, y as i64) == self.maze.start {
                                return "S";
                            }
                            if self.open_set.contains_key(&(x as i64, y as i64)) {
                                return "X";
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
