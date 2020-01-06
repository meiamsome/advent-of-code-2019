use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use gcd::Gcd;
use rayon::prelude::*;

#[derive(Debug, PartialEq)]
enum AsteroidFieldParseError {
    InvalidCharacter(char),
}
use AsteroidFieldParseError::*;

impl Display for AsteroidFieldParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            InvalidCharacter(chr) => write!(
                f,
                "Unparsable asteroid field: Invalid character '{}' in field",
                chr
            ),
        }
    }
}

impl Error for AsteroidFieldParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        Some(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct AsteroidField {
    width: i32,
    height: i32,
    data: HashSet<(i32, i32)>,
}

impl FromStr for AsteroidField {
    type Err = AsteroidFieldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.trim().split('\n').collect();
        let height = rows.len() as i32;
        let width = rows[0].len() as i32;
        let data = rows
            .iter()
            .enumerate()
            .flat_map(|(y, sub_str)| {
                sub_str
                    .chars()
                    .enumerate()
                    .filter_map(|(x, chr)| {
                        if chr == '#' {
                            Some(Ok((x as i32, y as i32)))
                        } else if chr == '.' {
                            None
                        } else {
                            Some(Err(InvalidCharacter(chr)))
                        }
                    })
                    .collect::<Vec<Result<_, _>>>()
            })
            .collect::<Result<_, _>>()?;
        Ok(AsteroidField {
            width,
            height,
            data,
        })
    }
}

impl AsteroidField {
    fn get_visible_from(&self, coords: (i32, i32)) -> Vec<(i32, i32)> {
        let (x, y) = coords;
        self.data
            .iter()
            .filter_map(|&other_coords| {
                if other_coords == coords {
                    return None;
                }

                let delta = (other_coords.0 - x, other_coords.1 - y);

                let gcd = (delta.0.abs() as u32).gcd(delta.1.abs() as u32) as i32;
                let delta_min = (delta.0 / gcd, delta.1 / gcd);
                let mut delta_current = delta_min;

                while delta_current != delta {
                    if self
                        .data
                        .contains(&(delta_current.0 + x, delta_current.1 + y))
                    {
                        return None;
                    }
                    delta_current = (delta_current.0 + delta_min.0, delta_current.1 + delta_min.1);
                }

                Some(other_coords)
            })
            .collect()
    }

    fn count_visible_from(&self, coords: (i32, i32)) -> usize {
        self.get_visible_from(coords).len()
    }

    fn find_best_spotter(&self) -> Option<(&(i32, i32), usize)> {
        self.data
            .par_iter()
            .map(|coords| (coords, self.count_visible_from(*coords)))
            .max_by_key(|(_, visible)| *visible)
    }

    fn vaporization_list(&self, coords: (i32, i32)) -> VaporizationList {
        VaporizationList {
            field: self.clone(),
            current_angle: 3f64 * std::f64::consts::FRAC_PI_2 - 0.0001f64,
            coords,
        }
    }
}

struct VaporizationList {
    field: AsteroidField,
    current_angle: f64,
    coords: (i32, i32),
}

const TWO_PI: f64 = std::f64::consts::PI * 2f64;

impl Iterator for VaporizationList {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.field
            .get_visible_from(self.coords)
            .iter()
            .map(|asteroid_coords| {
                let delta_x = asteroid_coords.0 - self.coords.0;
                let delta_y = asteroid_coords.1 - self.coords.1;
                let angle = (delta_y as f64).atan2(delta_x as f64);
                let delta_angle = (angle - self.current_angle + TWO_PI + TWO_PI) % TWO_PI;
                (
                    asteroid_coords,
                    angle,
                    if delta_angle == 0f64 {
                        TWO_PI
                    } else {
                        delta_angle
                    },
                )
            })
            .min_by(|(_, _, d_angle_1), (_, _, d_angle_2)| {
                d_angle_1.partial_cmp(d_angle_2).unwrap()
            })
            .map(|(coords, angle, _)| {
                self.field.data.remove(&coords);
                self.current_angle = angle;
                *coords
            })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let field: AsteroidField = contents.parse()?;
    if let Some((&coords, _)) = field.find_best_spotter() {
        let mut list = field.vaporization_list(coords);
        println!("{:?}", list.nth(199));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::AsteroidField;
    use super::AsteroidFieldParseError::*;

    const SMALL_ASTEROID_FIELD: &str = "\
                                        .#..#\n\
                                        .....\n\
                                        #####\n\
                                        ....#\n\
                                        ...##\n\
                                        ";

    const MEDIUM_ASTEROID_1: &str = "\
                                     ......#.#.\n\
                                     #..#.#....\n\
                                     ..#######.\n\
                                     .#.#.###..\n\
                                     .#..#.....\n\
                                     ..#....#.#\n\
                                     #..#....#.\n\
                                     .##.#..###\n\
                                     ##...#..#.\n\
                                     .#....####\n\
                                     ";

    const MEDIUM_ASTEROID_2: &str = "\
                                     #.#...#.#.\n\
                                     .###....#.\n\
                                     .#....#...\n\
                                     ##.#.#.#.#\n\
                                     ....#.#.#.\n\
                                     .##..###.#\n\
                                     ..#...##..\n\
                                     ..##....##\n\
                                     ......#...\n\
                                     .####.###.\n\
                                     ";

    const MEDIUM_ASTEROID_3: &str = "\
                                     .#..#..###\n\
                                     ####.###.#\n\
                                     ....###.#.\n\
                                     ..###.##.#\n\
                                     ##.##.#.#.\n\
                                     ....###..#\n\
                                     ..#.#..#.#\n\
                                     #..#.#.###\n\
                                     .##...##.#\n\
                                     .....#.#..\n\
                                     ";

    const BIG_ASTEROID_FIELD: &str = "\
                                      .#..##.###...#######\n\
                                      ##.############..##.\n\
                                      .#.######.########.#\n\
                                      .###.#######.####.#.\n\
                                      #####.##.#.##.###.##\n\
                                      ..#####..#.#########\n\
                                      ####################\n\
                                      #.####....###.#.#.##\n\
                                      ##.#################\n\
                                      #####.##.###..####..\n\
                                      ..######..##.#######\n\
                                      ####.##.####...##..#\n\
                                      .#####..#.######.###\n\
                                      ##...#.##########...\n\
                                      #.##########.#######\n\
                                      .####.#.###.###.#.##\n\
                                      ....##.##.###..#####\n\
                                      .#.#.###########.###\n\
                                      #.#.#.#####.####.###\n\
                                      ###.##.####.##.#..##\n\
                                      ";

    const VAPORIZATION_EXAMPLE: &str = "\
                                        .#....#####...#..\n\
                                        ##...##.#####..##\n\
                                        ##...#...#.#####.\n\
                                        ..#.....#...###..\n\
                                        ..#.#.....#....##\n\
                                        ";

    // AsteroidField FromStr
    #[test]
    fn test_small_asteroid_field_parse() {
        assert_eq!(
            SMALL_ASTEROID_FIELD.parse(),
            Ok(AsteroidField {
                width: 5,
                height: 5,
                data: vec!(
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4),
                )
                .into_iter()
                .collect()
            }),
        )
    }

    #[test]
    fn test_parse_error_invalid_character() {
        assert_eq!(
            "asldjka".parse::<AsteroidField>(),
            Err(InvalidCharacter('a'))
        )
    }

    // Count Visible From
    #[test]
    fn test_visible_from() {
        let tests = vec![
            ((1, 0), 7),
            ((4, 0), 7),
            ((0, 2), 6),
            ((1, 2), 7),
            ((2, 2), 7),
            ((3, 2), 7),
            ((4, 2), 5),
            ((4, 3), 7),
            ((3, 4), 8),
            ((4, 4), 7),
        ];
        for (coord, count) in tests {
            assert_eq!(
                SMALL_ASTEROID_FIELD
                    .parse::<AsteroidField>()
                    .unwrap()
                    .count_visible_from(coord),
                count
            );
        }
    }

    // Find best spotter
    #[test]
    fn test_find_best_spotter_small() {
        assert_eq!(
            SMALL_ASTEROID_FIELD
                .parse::<AsteroidField>()
                .unwrap()
                .find_best_spotter(),
            Some((&(3, 4), 8))
        );
    }

    #[test]
    fn test_find_best_spotter_medium_1() {
        assert_eq!(
            MEDIUM_ASTEROID_1
                .parse::<AsteroidField>()
                .unwrap()
                .find_best_spotter(),
            Some((&(5, 8), 33))
        );
    }

    #[test]
    fn test_find_best_spotter_medium_2() {
        assert_eq!(
            MEDIUM_ASTEROID_2
                .parse::<AsteroidField>()
                .unwrap()
                .find_best_spotter(),
            Some((&(1, 2), 35))
        );
    }

    #[test]
    fn test_find_best_spotter_medium_3() {
        assert_eq!(
            MEDIUM_ASTEROID_3
                .parse::<AsteroidField>()
                .unwrap()
                .find_best_spotter(),
            Some((&(6, 3), 41))
        );
    }

    #[test]
    fn test_find_best_spotter_big() {
        assert_eq!(
            BIG_ASTEROID_FIELD
                .parse::<AsteroidField>()
                .unwrap()
                .find_best_spotter(),
            Some((&(11, 13), 210))
        );
    }

    #[test]
    fn test_vaporize_small_3_4() {
        assert_eq!(
            SMALL_ASTEROID_FIELD
                .parse::<AsteroidField>()
                .unwrap()
                .vaporization_list((3, 4))
                .collect::<Vec<_>>(),
            vec!(
                (3, 2),
                (4, 0),
                (4, 2),
                (4, 3),
                (4, 4),
                (0, 2),
                (1, 2),
                (2, 2),
                (1, 0),
            )
        )
    }

    #[test]
    fn test_vaporize_example() {
        assert_eq!(
            VAPORIZATION_EXAMPLE
                .parse::<AsteroidField>()
                .unwrap()
                .vaporization_list((8, 3))
                .collect::<Vec<_>>(),
            vec!(
                (8, 1),
                (9, 0),
                (9, 1),
                (10, 0),
                (9, 2),
                (11, 1),
                (12, 1),
                (11, 2),
                (15, 1),
                (12, 2),
                (13, 2),
                (14, 2),
                (15, 2),
                (12, 3),
                (16, 4),
                (15, 4),
                (10, 4),
                (4, 4),
                (2, 4),
                (2, 3),
                (0, 2),
                (1, 2),
                (0, 1),
                (1, 1),
                (5, 2),
                (1, 0),
                (5, 1),
                (6, 1),
                (6, 0),
                (7, 0),
                (8, 0),
                (10, 1),
                (14, 0),
                (16, 1),
                (13, 3),
                (14, 3)
            )
        )
    }

    #[test]
    fn test_vaporize_big_field() {
        let vaporization = BIG_ASTEROID_FIELD
            .parse::<AsteroidField>()
            .unwrap()
            .vaporization_list((11, 13))
            .collect::<Vec<_>>();
        assert_eq!(vaporization.len(), 299);
        for (iter, coord) in [
            (0, (11, 12)),
            (1, (12, 1)),
            (2, (12, 2)),
            (9, (12, 8)),
            (19, (16, 0)),
            (49, (16, 9)),
            (99, (10, 16)),
            (198, (9, 6)),
            (199, (8, 2)),
            (200, (10, 9)),
            (298, (11, 1)),
        ]
        .iter()
        .cloned()
        {
            assert_eq!(vaporization[iter], coord);
        }
    }
}
