use vm::lang::load_from_file;

use day15part1::dir::Dir;
use day15part1::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Scaffolding,
    BotUp,
}

impl From<char> for Tile {
    fn from(other: char) -> Self {
        match other {
            '.' => Tile::Empty,
            '#' => Tile::Scaffolding,
            '^' => Tile::BotUp,
            _ => panic!(),
        }
    }
}

impl Into<&str> for &Tile {
    fn into(self) -> &'static str {
        match self {
            Tile::Empty => ".",
            Tile::Scaffolding => "#",
            Tile::BotUp => "^",
        }
    }
}

struct Map {
    grid: Vec<Vec<Tile>>
}

impl Map {
    fn alignment_parameters(&self) -> Vec<i64> {
        (1..(self.grid.len() as i64 - 1))
            .flat_map(|y| {
                (1..(self.grid[0].len() as i64 - 1))
                    .filter_map(|x| {
                        let pos = (x, y);
                        if self.tile_at(&pos) != Tile::Scaffolding {
                            return None;
                        }
                        for dir in Dir::all() {
                            if self.tile_at(&dir.move_in_dir(pos)) != Tile::Scaffolding {
                                return None;
                            }
                        }
                        Some(x * y)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn tile_at(&self, pos: &Position) -> Tile {
        self.grid[pos.1 as usize][pos.0 as usize]
    }
}

impl From<&str> for Map {
    fn from(other: &str) -> Map {
        Map {
            grid: other.trim()
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|x| x.into())
                        .collect()
                })
                .collect()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm = load_from_file("./input.txt")?;
    let map: Map = vm.map(|x| (x as u8) as char).collect::<String>().as_str().into();
    println!("{:?}", map.alignment_parameters());
    println!(
        "{:?}",
        map.alignment_parameters()
            .iter()
            .sum::<i64>()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_alignment_parameters() {
        assert_eq!(
            Map::from("
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
            ").alignment_parameters(),
            vec![4, 8, 24, 40]
        )
    }
}
