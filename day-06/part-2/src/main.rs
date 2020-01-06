use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn parse_orbit(input: &str) -> (&str, &str) {
    let split: Vec<&str> = input.split(')').collect();
    if split.len() != 2 {
        panic!("Unexpected length of orbit line: {}", split.len());
    }
    (split[0], split[1])
}

fn get_planet_ancestors(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut planets: HashMap<&str, &str> = HashMap::new();
    for (parent, child) in input.split('\n').map(parse_orbit) {
        planets.insert(child, parent);
    }
    planets
        .keys()
        .map(|&planet| {
            let mut ancestors = vec![];
            let mut current = planet;
            while let Some(next) = planets.get(current) {
                current = next;
                ancestors.push(current);
            }
            (planet, ancestors)
        })
        .collect()
}

fn get_shortest_path<'a>(ancestors: HashMap<&'a str, Vec<&'a str>>, from: &str, to: &str) -> usize {
    let mut from_list: HashSet<&str> = HashSet::new();
    from_list.extend(ancestors.get(from).expect("From not in map").iter());
    let mut to_list: HashSet<&str> = HashSet::new();
    to_list.extend(ancestors.get(to).expect("To not in map").iter());
    from_list.symmetric_difference(&to_list).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    println!(
        "{:?}",
        get_shortest_path(get_planet_ancestors(&contents.trim()), "YOU", "SAN")
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_planet_ancestors() {
        assert_eq!(
            get_planet_ancestors("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
            vec!(
                ("B", vec!("COM")),
                ("C", vec!("B", "COM")),
                ("D", vec!("C", "B", "COM")),
                ("E", vec!("D", "C", "B", "COM")),
                ("F", vec!("E", "D", "C", "B", "COM")),
                ("G", vec!("B", "COM")),
                ("H", vec!("G", "B", "COM")),
                ("I", vec!("D", "C", "B", "COM")),
                ("J", vec!("E", "D", "C", "B", "COM")),
                ("K", vec!("J", "E", "D", "C", "B", "COM")),
                ("L", vec!("K", "J", "E", "D", "C", "B", "COM")),
            )
            .into_iter()
            .collect(),
        )
    }

    #[test]
    fn test_get_shortest_path() {
        let ancestors = get_planet_ancestors(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        assert_eq!(get_shortest_path(ancestors, "YOU", "SAN"), 4);
    }

    // TODO
    /*
    #[test]
    fn test_get_shortest_path2() {
        let ancestors = get_planet_ancestors("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");

        assert_eq!(
            get_shortest_path(ancestors, "L", "I"),
            4
        );
    }
    */
}
