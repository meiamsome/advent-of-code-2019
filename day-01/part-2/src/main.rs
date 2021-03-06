use std::fs::File;
use std::io::prelude::*;
use std::iter::Sum;

fn string_to_u32_list(data: String) -> Result<Vec<u32>, std::num::ParseIntError> {
    data.split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
}

struct FuelCalculation {
    current: u32,
}

impl FuelCalculation {
    fn for_mass(mass: u32) -> FuelCalculation {
        FuelCalculation { current: mass }
    }
}

impl Iterator for FuelCalculation {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.current = (self.current / 3).saturating_sub(2);
        if self.current != 0 {
            Some(self.current)
        } else {
            None
        }
    }
}

fn full_fuel_amount_for_payload(payload_mass: u32) -> u32 {
    u32::sum(FuelCalculation::for_mass(payload_mass))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let summed_per_module_fuels = u32::sum(
        string_to_u32_list(contents)?
            .into_iter()
            .map(full_fuel_amount_for_payload),
    );
    println!("{}", summed_per_module_fuels);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::full_fuel_amount_for_payload;
    use super::string_to_u32_list;

    #[test]
    fn string_to_u32_list_testcase() {
        assert_eq!(
            string_to_u32_list("12\n14\n".to_string()).unwrap(),
            vec!(12, 14)
        )
    }

    #[test]
    fn full_fuel_amount_for_payload_14() {
        assert_eq!(full_fuel_amount_for_payload(14), 2)
    }

    #[test]
    fn full_fuel_amount_for_payload_1969() {
        assert_eq!(full_fuel_amount_for_payload(1969), 966)
    }

    #[test]
    fn full_fuel_amount_for_payload_100756() {
        assert_eq!(full_fuel_amount_for_payload(100_756), 50346)
    }
}
