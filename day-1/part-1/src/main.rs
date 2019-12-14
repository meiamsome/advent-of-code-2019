use std::fs::File;
use std::io::prelude::*;
use std::iter::Sum;

fn module_mass_to_fuel(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn module_list_to_fuel(masses: &[u32]) -> u32 {
    u32::sum(
        masses
            .iter()
            .map(|&x| module_mass_to_fuel(x))
    )
}

fn string_to_u32_list(data: String) -> Result<Vec<u32>, std::num::ParseIntError> {
    data
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    println!("{}", module_list_to_fuel(&string_to_u32_list(contents)?));
    Ok(())
}


#[cfg(test)]
mod test {
    use super::module_mass_to_fuel;
    use super::module_list_to_fuel;
    use super::string_to_u32_list;

    #[test]
    fn module_mass_to_fuel_mass_12() {
        assert_eq!(module_mass_to_fuel(12), 2)
    }

    #[test]
    fn module_mass_to_fuel_mass_14() {
        assert_eq!(module_mass_to_fuel(14), 2)
    }

    #[test]
    fn module_mass_to_fuel_mass_1969() {
        assert_eq!(module_mass_to_fuel(1969), 654)
    }

    #[test]
    fn module_mass_to_fuel_mass_100756() {
        assert_eq!(module_mass_to_fuel(100756), 33583)
    }

    #[test]
    fn module_list_to_fuel_testcase() {
        assert_eq!(module_list_to_fuel(&[
            12,
            14
        ]), 4)
    }

    #[test]
    fn string_to_u32_list_testcase() {
        assert_eq!(string_to_u32_list("12\n14\n".to_string()).unwrap(), vec!(12, 14))
    }
}
