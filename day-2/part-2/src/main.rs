use core::ops::Add;
use core::ops::Mul;
use std::fs::File;
use std::io::prelude::*;

struct IntcodeVM<'a, T>
where T: Add<T> + Mul<T>
{
    instruction_pointer: usize,
    memory: &'a mut [T]
}

impl Iterator for IntcodeVM<'_, usize>
// where T: Add<T> + Mul<T>
{
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let ret_val = match self.memory[self.instruction_pointer] {
            1 => {
                let a = self.memory[self.memory[self.instruction_pointer + 1]];
                let b = self.memory[self.memory[self.instruction_pointer + 2]];
                self.memory[self.memory[self.instruction_pointer + 3]] = a + b;
                Some(())
            }
            2 => {
                let a = self.memory[self.memory[self.instruction_pointer + 1]];
                let b = self.memory[self.memory[self.instruction_pointer + 2]];
                self.memory[self.memory[self.instruction_pointer + 3]] = a * b;
                Some(())
            },
            99 => None,
            x => panic!("Invalid opcode {}", x)
        };
        self.instruction_pointer += 4;
        ret_val
    }
}


fn string_to_usize_list(data: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    data
        .split(',')
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let starting_memory = string_to_usize_list(contents.trim())?;
    for x in 0..=99 {
        for y in 0..=99 {
            let mut memory = starting_memory.clone();
            memory[1] = x;
            memory[2] = y;
            let vm = IntcodeVM {
                instruction_pointer: 0,
                memory: &mut memory,
            };
            vm.last();
            if memory[0] == 19690720 {
                println!("{:?}", memory);
                println!("{}", 100 * x + y);
                return Ok(())
            }
        }
    }
    // println!("No solution found!");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::string_to_usize_list;

    #[test]
    fn string_to_usize_list_testcase() {
        assert_eq!(string_to_usize_list("12,14").unwrap(), vec!(12, 14))
    }
}
