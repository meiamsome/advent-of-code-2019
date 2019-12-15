use std::fs::File;
use std::io::prelude::*;

use std::collections::{HashSet, HashMap};

fn trace_step(step: i32, from: (i32, i32), instruction: &str) -> (i32, (i32, i32), HashMap<(i32, i32), i32>) {
    let (direction, len_str) = instruction.split_at(1);
    let len = len_str.parse::<i32>().unwrap();
    let component = match direction {
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        _ => panic!("Unknown direction {}", direction),
    };
    let mut current_step = step;
    let mut pos = from;
    let mut set = HashMap::new();
    for _ in 0..len {
        pos.0 += component.0;
        pos.1 += component.1;
        current_step += 1;
        set.insert(pos.clone(), current_step);
    }
    (current_step, pos, set)
}

fn trace_wire(instructions: &str) -> HashMap<(i32, i32), i32> {
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut distance = 0;
    for instruction in instructions.split(',') {
        let (new_distance, new_pos, new_map) = trace_step(distance, pos, instruction);
        pos = new_pos;
        distance = new_distance;
        // TODO
        map.extend(
            new_map
                .into_iter()
                .filter(|(key, _)| !map.contains_key(key))
                .collect::<Vec<((i32, i32), i32)>>()
        );
    }
    map
}

fn find_intersections(instruction_sets: &str) -> HashMap<(i32, i32), i32> {
    let instructions: Vec<&str> = instruction_sets.split_whitespace().collect();
    if instructions.len() != 2 {
        panic!("Only two instruction sets allowed, found {}", instructions.len());
    }
    let map_a = trace_wire(instructions[0]);
    let set_a: HashSet<&(i32, i32)> = map_a.keys().collect();
    let map_b = trace_wire(instructions[1]);
    let set_b: HashSet<&(i32, i32)> = map_b.keys().collect();
    set_a
        .intersection(&set_b)
        .map(|&x| (*x, map_a.get(x).unwrap() + map_b.get(x).unwrap()))
        .collect()
}

fn smallest_intersection(instruction_sets: &str) -> Option<((i32, i32), i32)> {
    find_intersections(instruction_sets)
        .into_iter()
        .fold(None, |best, ((pos_x, pos_y), len)| {
            if let Some((_, best_len)) = best {
                if best_len < len {
                    return best
                }
            }
            return Some(((pos_x, pos_y), len))
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let ((pos_x, pos_y), len) = smallest_intersection(&contents).unwrap();
    println!("Shortest crossing: ({}, {}), {}", pos_x, pos_y, len);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::trace_step;
    use super::trace_wire;
    use super::find_intersections;
    use super::smallest_intersection;

    #[test]
    fn trace_step_1() {
        let result_set = vec!(
            ((1, 0), 1),
            ((2, 0), 2),
            ((3, 0), 3),
            ((4, 0), 4),
            ((5, 0), 5),
            ((6, 0), 6),
            ((7, 0), 7),
            ((8, 0), 8),
        ).into_iter().collect();
        assert_eq!(
            trace_step(0, (0, 0), "R8"),
            (8, (8, 0), result_set)
        );
    }

    #[test]
    fn trace_step_2() {
        let result_set = vec!(
            ((8, 1), 9),
            ((8, 2), 10),
            ((8, 3), 11),
            ((8, 4), 12),
            ((8, 5), 13),
        ).into_iter().collect();
        assert_eq!(
            trace_step(8, (8, 0), "U5"),
            (13, (8, 5), result_set)
        );
    }

    #[test]
    fn trace_step_3() {
        let result_set = vec!(
            ((7, 5), 14),
            ((6, 5), 15),
            ((5, 5), 16),
            ((4, 5), 17),
            ((3, 5), 18),
        ).into_iter().collect();
        assert_eq!(
            trace_step(13, (8, 5), "L5"),
            (18, (3, 5), result_set)
        );
    }

    #[test]
    fn trace_step_4() {
        let result_set = vec!(
            ((3, 4), 19),
            ((3, 3), 20),
            ((3, 2), 21),
        ).into_iter().collect();
        assert_eq!(
            trace_step(18, (3, 5), "D3"),
            (21, (3, 2), result_set)
        );
    }

    #[test]
    fn trace_wire_1() {
        assert_eq!(
            trace_wire("R8,U5,L5,D3"),
            vec!(
                ((1, 0), 1),
                ((2, 0), 2),
                ((3, 0), 3),
                ((4, 0), 4),
                ((5, 0), 5),
                ((6, 0), 6),
                ((7, 0), 7),
                ((8, 0), 8),
                ((8, 1), 9),
                ((8, 2), 10),
                ((8, 3), 11),
                ((8, 4), 12),
                ((8, 5), 13),
                ((7, 5), 14),
                ((6, 5), 15),
                ((5, 5), 16),
                ((4, 5), 17),
                ((3, 5), 18),
                ((3, 4), 19),
                ((3, 3), 20),
                ((3, 2), 21),
            ).into_iter().collect()
        )
    }

    #[test]
    fn trace_wire_self_cross() {
        assert_eq!(
            trace_wire("R8,U1,L5,D3"),
            vec!(
                ((1, 0), 1),
                ((2, 0), 2),
                ((3, 0), 3),
                ((4, 0), 4),
                ((5, 0), 5),
                ((6, 0), 6),
                ((7, 0), 7),
                ((8, 0), 8),
                ((8, 1), 9),
                ((7, 1), 10),
                ((6, 1), 11),
                ((5, 1), 12),
                ((4, 1), 13),
                ((3, 1), 14),
                // Not counted because of alternate lower value
                // ((3, 0), 15),
                ((3, -1), 16),
                ((3, -2), 17),
            ).into_iter().collect()
        )
    }

    #[test]
    fn find_intersections_1() {
        assert_eq!(
            find_intersections(
                "R8,U5,L5,D3\nU7,R6,D4,L4"
            ),
            vec!(
                ((3, 3), 40),
                ((6, 5), 30),
            ).into_iter().collect()
        )
    }

    #[test]
    fn find_intersections_2() {
        assert_eq!(
            find_intersections(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ),
            vec!(
                ((146, 46), 624),
                ((155, 11), 850),
                ((158, -12), 610),
                ((155, 4), 726),
            ).into_iter().collect()
        )
    }

    #[test]
    fn find_intersections_3() {
        assert_eq!(
            find_intersections(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            vec!(
                ((107, 51), 700),
                ((107, 47), 410),
                ((107, 71), 636),
                ((124, 11), 516),
                ((157, 18), 650),
            ).into_iter().collect()
        )
    }

    #[test]
    fn smallest_intersection_1() {
        assert_eq!(
            smallest_intersection(
                "R8,U5,L5,D3\nU7,R6,D4,L4"
            ),
            Some(((6, 5), 30))
        )
    }

    #[test]
    fn smallest_intersection_2() {
        assert_eq!(
            smallest_intersection(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Some(((158, -12), 610))
        )
    }

    #[test]
    fn smallest_intersection_3() {
        assert_eq!(
            smallest_intersection(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Some(((107, 47), 410))
        )
    }
}
