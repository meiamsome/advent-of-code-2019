use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

type Chemical = String;
#[derive(Debug, PartialEq)]
struct ChemicalQuantity(Chemical, i64);

#[derive(Debug, PartialEq)]
enum ChemicalQuantityParseError {
    IncorrectParts(usize),
    InvalidQuantity(ParseIntError),
}

impl Display for ChemicalQuantityParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ChemicalQuantityParseError::IncorrectParts(quantity) => write!(
                f,
                "Invalid Quantity of parts to the ChemicalQuantity ({})",
                quantity
            ),
            ChemicalQuantityParseError::InvalidQuantity(err) => err.fmt(f),
        }
    }
}

impl Error for ChemicalQuantityParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl From<ParseIntError> for ChemicalQuantityParseError {
    fn from(error: ParseIntError) -> ChemicalQuantityParseError {
        ChemicalQuantityParseError::InvalidQuantity(error)
    }
}

impl FromStr for ChemicalQuantity {
    type Err = ChemicalQuantityParseError;

    fn from_str(s: &str) -> Result<ChemicalQuantity, Self::Err> {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        if parts.len() != 2 {
            return Err(ChemicalQuantityParseError::IncorrectParts(parts.len()));
        }
        Ok(ChemicalQuantity(parts[1].to_string(), parts[0].parse()?))
    }
}

#[derive(Debug, PartialEq)]
struct Equation {
    inputs: Vec<ChemicalQuantity>,
    output: ChemicalQuantity,
}

#[derive(Debug, PartialEq)]
enum EquationParseError {
    IncorrectParts(usize),
    ChemicalQuantityParseError(ChemicalQuantityParseError),
}

impl From<ChemicalQuantityParseError> for EquationParseError {
    fn from(error: ChemicalQuantityParseError) -> EquationParseError {
        EquationParseError::ChemicalQuantityParseError(error)
    }
}

impl Display for EquationParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            EquationParseError::IncorrectParts(quantity) => write!(
                f,
                "Invalid Quantity of parts to the equation ({})",
                quantity
            ),
            EquationParseError::ChemicalQuantityParseError(err) => err.fmt(f),
        }
    }
}

impl Error for EquationParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl FromStr for Equation {
    type Err = EquationParseError;

    fn from_str(s: &str) -> Result<Equation, Self::Err> {
        let parts: Vec<&str> = s.trim().split("=>").collect();
        if parts.len() != 2 {
            return Err(EquationParseError::IncorrectParts(parts.len()));
        }
        let lhs = parts[0];
        let rhs = parts[1];
        let output = rhs.trim().parse()?;
        let inputs = lhs
            .split(',')
            .map(|chem| chem.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Equation { inputs, output })
    }
}

#[derive(Debug, PartialEq)]
enum ChemicalCreationError {
    InsufficientChemical(ChemicalQuantity),
}

impl Display for ChemicalCreationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ChemicalCreationError::InsufficientChemical(quantity) => write!(
                f,
                "Insufficient quantity of {} (missing at least {})",
                quantity.0, quantity.1
            ),
        }
    }
}

impl Error for ChemicalCreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug, PartialEq)]
struct EquationSet {
    equations: HashMap<Chemical, Equation>,
}

impl FromStr for EquationSet {
    type Err = EquationParseError;

    fn from_str(s: &str) -> Result<EquationSet, Self::Err> {
        let equations = s
            .trim()
            .split('\n')
            .map(|line| line.parse())
            .collect::<Result<Vec<Equation>, _>>()?
            .into_iter()
            .map(|equation| (equation.output.0.clone(), equation))
            .collect();
        Ok(EquationSet { equations })
    }
}

fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

impl EquationSet {
    fn create_chemicals(
        &self,
        mut inventory: HashMap<Chemical, (i64, i64)>,
    ) -> Result<HashMap<Chemical, (i64, i64)>, ChemicalCreationError> {
        while let Some((chemical, (required, available))) = inventory
            .iter_mut()
            .find(|(_, (required, available))| required > available)
        {
            let delta = *required - *available;
            let equation = self.equations.get(chemical).ok_or_else(|| {
                ChemicalCreationError::InsufficientChemical(ChemicalQuantity(
                    chemical.clone(),
                    delta,
                ))
            })?;
            let times = ceil_div(delta, equation.output.1);

            *available += times * equation.output.1;

            for chemical_quantity in equation.inputs.iter() {
                let (mut input_required, input_available) = inventory
                    .get(&chemical_quantity.0)
                    .copied()
                    .unwrap_or((0, 0));
                input_required += chemical_quantity.1 * times;
                inventory.insert(
                    chemical_quantity.0.clone(),
                    (input_required, input_available),
                );
            }
        }
        Ok(inventory)
    }

    fn create_chemical(
        &self,
        required: ChemicalQuantity,
    ) -> Result<HashMap<Chemical, (i64, i64)>, ChemicalCreationError> {
        self.create_chemicals(
            vec![
                ("ORE".to_string(), (0, 1_000_000_000_000)),
                (required.0, (required.1, 0)),
            ]
            .into_iter()
            .collect(),
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let equation_set: EquationSet = contents.parse()?;
    let mut min = 0;
    let mut max = 1_000_000_000_000;
    while min != max {
        let amount = (max + min) / 2;
        let result = equation_set.create_chemical(ChemicalQuantity("FUEL".to_string(), amount));
        if result.is_ok() {
            min = amount;
        } else {
            max = amount - 1;
        }
    }
    println!(
        "Max fuel: {}, Ore: {}",
        min,
        equation_set
            .create_chemical(ChemicalQuantity("FUEL".to_string(), min))
            .unwrap()
            .get(&"ORE".to_string())
            .unwrap()
            .0,
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_EQUATIONS: &str = "\
                                     9 ORE => 2 A\n\
                                     8 ORE => 3 B\n\
                                     7 ORE => 5 C\n\
                                     3 A, 4 B => 1 AB\n\
                                     5 B, 7 C => 1 BC\n\
                                     4 C, 1 A => 1 CA\n\
                                     2 AB, 3 BC, 4 CA => 1 FUEL\n\
                                     ";

    #[test]
    fn parse_chemical_quantity() {
        assert_eq!("10 A".parse(), Ok(ChemicalQuantity("A".to_string(), 10)))
    }

    #[test]
    fn parse_equation() {
        assert_eq!(
            "2 B, 10 A => 4 C".parse(),
            Ok(Equation {
                inputs: vec![
                    ChemicalQuantity("B".to_string(), 2),
                    ChemicalQuantity("A".to_string(), 10),
                ],
                output: ChemicalQuantity("C".to_string(), 4)
            })
        )
    }

    #[test]
    fn parse_example_equations() {
        assert_eq!(
            EXAMPLE_EQUATIONS.parse(),
            Ok(EquationSet {
                equations: vec![
                    (
                        "A".to_string(),
                        Equation {
                            inputs: vec![ChemicalQuantity("ORE".to_string(), 9)],
                            output: ChemicalQuantity("A".to_string(), 2),
                        }
                    ),
                    (
                        "B".to_string(),
                        Equation {
                            inputs: vec![ChemicalQuantity("ORE".to_string(), 8)],
                            output: ChemicalQuantity("B".to_string(), 3),
                        }
                    ),
                    (
                        "C".to_string(),
                        Equation {
                            inputs: vec![ChemicalQuantity("ORE".to_string(), 7)],
                            output: ChemicalQuantity("C".to_string(), 5),
                        }
                    ),
                    (
                        "AB".to_string(),
                        Equation {
                            inputs: vec![
                                ChemicalQuantity("A".to_string(), 3),
                                ChemicalQuantity("B".to_string(), 4),
                            ],
                            output: ChemicalQuantity("AB".to_string(), 1),
                        }
                    ),
                    (
                        "BC".to_string(),
                        Equation {
                            inputs: vec![
                                ChemicalQuantity("B".to_string(), 5),
                                ChemicalQuantity("C".to_string(), 7),
                            ],
                            output: ChemicalQuantity("BC".to_string(), 1),
                        }
                    ),
                    (
                        "CA".to_string(),
                        Equation {
                            inputs: vec![
                                ChemicalQuantity("C".to_string(), 4),
                                ChemicalQuantity("A".to_string(), 1),
                            ],
                            output: ChemicalQuantity("CA".to_string(), 1),
                        }
                    ),
                    (
                        "FUEL".to_string(),
                        Equation {
                            inputs: vec![
                                ChemicalQuantity("AB".to_string(), 2),
                                ChemicalQuantity("BC".to_string(), 3),
                                ChemicalQuantity("CA".to_string(), 4),
                            ],
                            output: ChemicalQuantity("FUEL".to_string(), 1),
                        }
                    ),
                ]
                .into_iter()
                .collect()
            })
        )
    }

    #[test]
    fn run_example_equations() {
        let equation_set: EquationSet = EXAMPLE_EQUATIONS.parse().unwrap();

        let example_1 = equation_set.create_chemical(ChemicalQuantity("A".to_string(), 10));
        assert_eq!(example_1.get(&"ORE".to_string()).unwrap().0, 45);

        let example_2 = equation_set.create_chemical(ChemicalQuantity("B".to_string(), 24));
        assert_eq!(example_2.get(&"ORE".to_string()).unwrap().0, 64);

        let example_3 = equation_set.create_chemical(ChemicalQuantity("C".to_string(), 40));
        assert_eq!(example_3.get(&"ORE".to_string()).unwrap().0, 56);

        let example_4 = equation_set.create_chemical(ChemicalQuantity("AB".to_string(), 2));
        assert_eq!(example_4.get(&"A".to_string()).unwrap().0, 6);
        assert_eq!(example_4.get(&"B".to_string()).unwrap().0, 8);

        let example_5 = equation_set.create_chemical(ChemicalQuantity("BC".to_string(), 3));
        assert_eq!(example_5.get(&"B".to_string()).unwrap().0, 15);
        assert_eq!(example_5.get(&"C".to_string()).unwrap().0, 21);

        let example_6 = equation_set.create_chemical(ChemicalQuantity("CA".to_string(), 4));
        assert_eq!(example_6.get(&"C".to_string()).unwrap().0, 16);
        assert_eq!(example_6.get(&"A".to_string()).unwrap().0, 4);

        let example_7 = equation_set.create_chemical(ChemicalQuantity("FUEL".to_string(), 1));
        assert_eq!(example_7.get(&"AB".to_string()).unwrap().0, 2);
        assert_eq!(example_7.get(&"BC".to_string()).unwrap().0, 3);
        assert_eq!(example_7.get(&"CA".to_string()).unwrap().0, 4);
        assert_eq!(example_7.get(&"FUEL".to_string()).unwrap().0, 1);
    }
}
