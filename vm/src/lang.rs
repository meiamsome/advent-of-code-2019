use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::convert::TryFrom;

use super::{IntcodeVM, OpCode};

struct Halt;
impl OpCode<u32> for Halt {
    fn execute(&self, _instruction_pointer: usize, _memory: &mut Vec<u32>) -> Option<usize> {
        None
    }
}

struct Add;
impl OpCode<u32> for Add {
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<u32>) -> Option<usize> {
        let ptr = instruction_pointer;
        let a = memory[usize::try_from(memory[ptr + 1]).unwrap()];
        let b = memory[usize::try_from(memory[ptr + 2]).unwrap()];
        let cptr = usize::try_from(memory[ptr + 3]).unwrap();
        memory[cptr] = a + b;
        Some(4)
    }
}

struct Mul;
impl OpCode<u32> for Mul {
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<u32>) -> Option<usize> {
        let ptr = instruction_pointer;
        let a = memory[usize::try_from(memory[ptr + 1]).unwrap()];
        let b = memory[usize::try_from(memory[ptr + 2]).unwrap()];
        let cptr = usize::try_from(memory[ptr + 3]).unwrap();
        memory[cptr] = a * b;
        Some(4)
    }
}

pub fn get_ops() -> HashMap<u32, Box<dyn OpCode<u32>>> {
  let mut ops: HashMap<u32, Box<dyn OpCode<u32>>> = HashMap::new();
  ops.insert(1, Box::new(Add));
  ops.insert(2, Box::new(Mul));
  ops.insert(99, Box::new(Halt));
  return ops
}

fn string_to_u32_list(data: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
  data
      .split(',')
      .map(|x| x.parse::<u32>())
      .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
}

pub fn load_memory_from_file(filename: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
  let mut contents = String::new();
  {
      let mut file = File::open(filename)?;
      file.read_to_string(&mut contents)?;
  }
  Ok(string_to_u32_list(contents.trim())?)
}

pub fn load_from_file(filename: &str) -> Result<IntcodeVM<u32>, Box<dyn std::error::Error>> {
  Ok(IntcodeVM::create(
      load_memory_from_file(filename)?,
      get_ops()
  ))
}

#[cfg(test)]
mod test {
    use super::string_to_u32_list;
    use super::load_from_file;

    #[test]
    fn string_to_u32_list_testcase() {
        assert_eq!(string_to_u32_list("12,14").unwrap(), vec!(12, 14))
    }

    // Regression tests
    #[test]
    fn test_regression_day1_part1() {
      let vm = load_from_file("../day-2/part-1/input.txt").unwrap();
      let last_memory = vm.last().unwrap();
      assert_eq!(last_memory[0], 9581917);
    }

    #[test]
    fn test_regression_day1_part2() {
      let mut vm = load_from_file("../day-2/part-2/input.txt").unwrap();
      vm.memory[1] = 25;
      vm.memory[2] = 5;
      let last_memory = vm.last().unwrap();
      assert_eq!(last_memory[0], 19690720);
    }
}
