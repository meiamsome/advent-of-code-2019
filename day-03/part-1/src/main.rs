use std::fs::File;
use std::io::prelude::*;

use std::collections::HashSet;

fn trace_step(from: (i32, i32), instruction: &str) -> ((i32, i32), HashSet<(i32, i32)>) {
    let (direction, len_str) = instruction.split_at(1);
    let len = len_str.parse::<i32>().unwrap();
    let component = match direction {
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        _ => panic!("Unknown direction {}", direction),
    };
    let mut pos = from;
    let mut set = HashSet::new();
    for _ in 0..len {
        pos.0 += component.0;
        pos.1 += component.1;
        set.insert(pos.clone());
    }
    (pos, set)
}

fn trace_wire(instructions: &str) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();
    let mut pos = (0, 0);
    for instruction in instructions.split(',') {
        let (new_pos, new_set) = trace_step(pos, instruction);
        pos = new_pos;
        set.extend(new_set.iter());
    }
    set
}

fn find_intersections(instruction_sets: &str) -> HashSet<(i32, i32)> {
    let instructions: Vec<&str> = instruction_sets.split_whitespace().collect();
    if instructions.len() != 2 {
        panic!(
            "Only two instruction sets allowed, found {}",
            instructions.len()
        );
    }
    let set_a = trace_wire(instructions[0]);
    let set_b = trace_wire(instructions[1]);
    set_a.intersection(&set_b).copied().collect()
}

fn smallest_intersection(instruction_sets: &str) -> Option<((i32, i32), i32)> {
    find_intersections(instruction_sets)
        .into_iter()
        .fold(None, |best, (pos_x, pos_y)| {
            let len = pos_x.abs() + pos_y.abs();
            if let Some((_, best_len)) = best {
                if best_len < len {
                    return best;
                }
            }
            Some(((pos_x, pos_y), len))
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
    use super::find_intersections;
    use super::smallest_intersection;
    use super::trace_step;
    use super::trace_wire;

    #[test]
    fn trace_step_1() {
        let result_set = vec![
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ]
        .into_iter()
        .collect();
        assert_eq!(trace_step((0, 0), "R8"), ((8, 0), result_set));
    }

    #[test]
    fn trace_step_2() {
        let result_set = vec![(8, 1), (8, 2), (8, 3), (8, 4), (8, 5)]
            .into_iter()
            .collect();
        assert_eq!(trace_step((8, 0), "U5"), ((8, 5), result_set));
    }

    #[test]
    fn trace_step_3() {
        let result_set = vec![(7, 5), (6, 5), (5, 5), (4, 5), (3, 5)]
            .into_iter()
            .collect();
        assert_eq!(trace_step((8, 5), "L5"), ((3, 5), result_set));
    }

    #[test]
    fn trace_step_4() {
        let result_set = vec![(3, 4), (3, 3), (3, 2)].into_iter().collect();
        assert_eq!(trace_step((3, 5), "D3"), ((3, 2), result_set));
    }

    #[test]
    fn trace_wire_1() {
        assert_eq!(
            trace_wire("R8,U5,L5,D3"),
            vec!(
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (7, 5),
                (6, 5),
                (5, 5),
                (4, 5),
                (3, 5),
                (3, 4),
                (3, 3),
                (3, 2),
            )
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn find_intersections_1() {
        assert_eq!(
            find_intersections("R8,U5,L5,D3\nU7,R6,D4,L4"),
            vec!((3, 3), (6, 5),).into_iter().collect()
        )
    }

    #[test]
    fn find_intersections_2() {
        assert_eq!(
            find_intersections(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ),
            vec!((146, 46), (155, 11), (158, -12), (155, 4),)
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn find_intersections_3() {
        assert_eq!(
            find_intersections(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            vec!((107, 51), (107, 47), (107, 71), (124, 11), (157, 18),)
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn smallest_intersection_1() {
        assert_eq!(
            smallest_intersection("R8,U5,L5,D3\nU7,R6,D4,L4"),
            Some(((3, 3), 6))
        )
    }

    #[test]
    fn smallest_intersection_2() {
        assert_eq!(
            smallest_intersection(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Some(((155, 4), 159))
        )
    }

    #[test]
    fn smallest_intersection_3() {
        assert_eq!(
            smallest_intersection(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Some(((124, 11), 135))
        )
    }
}
