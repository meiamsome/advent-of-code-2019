use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn parse_orbit(input: &str) -> (&str, &str) {
    let split: Vec<&str> = input.split(')').collect();
    if split.len() != 2 {
        panic!("Unexpected length of orbit line: {}", split.len());
    }
    (split[0], split[1])
}

fn get_total_orbits(input: &str) -> u32 {
    let mut planets: HashMap<&str, &str> = HashMap::new();
    for (parent, child) in input.split('\n').map(parse_orbit) {
        planets.insert(child, parent);
    }
    planets
        .keys()
        .map(|planet| {
            let mut count = 0;
            let mut current = planet;
            while let Some(next) = planets.get(current) {
                current = next;
                count += 1;
            }
            count
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    println!("{}", get_total_orbits(&contents.trim()));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            42,
            get_total_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
        )
    }
}
