use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

use vm::lang::load_from_str;

fn run_with_phases(program: &str, phases: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>>  {
    Ok(
        phases.into_iter()
            .fold::<Result<Box<dyn Iterator<Item=i32>>, Box<dyn std::error::Error>>, _>(
                Ok(Box::new(vec!(0).into_iter())),
                |iterator, phase| {
                    iterator.and_then::<Box<dyn Iterator<Item=i32>>, _>(|iter| {
                        let mut vm = load_from_str(program)?;
                        vm.io.input = Some(Box::new(vec!(phase).into_iter().chain(iter)));
                        Ok(Box::new(vm))
                    })
                }
            )?
            .last()
            .unwrap()
    )
}

fn get_optimal_phase(program: &str) -> Result<(i32, (i32, i32, i32, i32, i32)), Box<dyn std::error::Error>> {
    let mut max = 0;
    let mut arg_max = (0, 0, 0, 0, 0);
    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    for e in 0..=4 {
                        if vec!(a, b, c, d, e).into_iter().collect::<HashSet<_>>().len() != 5 {
                            continue;
                        }
                        let score = run_with_phases(program, vec!(a, b, c, d, e))?;
                        if score > max {
                            max = score;
                            arg_max = (a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }
    Ok((max, arg_max))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let (max, arg_max) = get_optimal_phase(&contents.trim())?;
    println!("{:?}: {}", arg_max, max);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            get_optimal_phase("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap(),
            (43210, (4, 3, 2, 1, 0)),
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            get_optimal_phase("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0").unwrap(),
            (54321, (0, 1, 2, 3, 4)),
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            get_optimal_phase("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0").unwrap(),
            (65210, (1, 0, 4, 3, 2)),
        )
    }
}
