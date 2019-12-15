use std::fs::File;
use std::io::stdin;
use std::io::prelude::*;

use std::collections::HashMap;
use std::convert::TryFrom;

use super::{IntcodeVM, OpCode};

fn get_parameter_addresses_with_modes(instruction_pointer: usize, memory: &mut Vec<i32>, count: usize) -> Vec<usize> {
  let mut op_code = memory[instruction_pointer] / 100;
  (0..count)
    .map(|offset| {
      let mode = op_code % 10;
      op_code /= 10;
      match mode {
        0 => usize::try_from(memory[instruction_pointer + offset + 1]).unwrap(),
        1 => instruction_pointer + offset + 1,
        _ => panic!("Unknown mode {}", mode),
      }
    })
    .collect()
}

struct Halt;
impl OpCode<i32> for Halt {
    fn execute(&self, _instruction_pointer: usize, _memory: &mut Vec<i32>) -> Option<usize> {
        None
    }
}

struct Add;
impl OpCode<i32> for Add {
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(instruction_pointer, memory, 3);
        let a = memory[addresses[0]];
        let b = memory[addresses[1]];
        memory[addresses[2]] = a + b;
        Some(4)
    }
}

struct Mul;
impl OpCode<i32> for Mul {
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(instruction_pointer, memory, 3);
        let a = memory[addresses[0]];
        let b = memory[addresses[1]];
        memory[addresses[2]] = a * b;
        Some(4)
    }
}

struct Input;
impl OpCode<i32> for Input {
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(instruction_pointer, memory, 1);
        let mut input_text = String::new();
        stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        memory[addresses[0]] = trimmed.parse::<i32>().unwrap();
        Some(2)
    }
}

struct Output;
impl OpCode<i32> for Output {
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(instruction_pointer, memory, 1);
        println!("Output: {}", memory[addresses[0]]);
        Some(2)
    }
}

pub fn get_ops() -> HashMap<i32, Box<dyn OpCode<i32>>> {
  let mut ops: HashMap<i32, Box<dyn OpCode<i32>>> = HashMap::new();
  ops.insert(1, Box::new(Add));
  ops.insert(2, Box::new(Mul));
  ops.insert(3, Box::new(Input));
  ops.insert(4, Box::new(Output));
  ops.insert(99, Box::new(Halt));
  return ops
}

fn string_to_i32_list(data: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
  data
      .split(',')
      .map(|x| x.parse::<i32>())
      .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
}

pub fn load_memory_from_file(filename: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
  let mut contents = String::new();
  {
      let mut file = File::open(filename)?;
      file.read_to_string(&mut contents)?;
  }
  Ok(string_to_i32_list(contents.trim())?)
}

fn op_code_lookup(input: i32) -> i32 {
  input % 100
}

pub fn load_from_str(program: &str) -> Result<IntcodeVM<i32>, Box<dyn std::error::Error>> {
  Ok(IntcodeVM::create(
      string_to_i32_list(program.trim())?,
      get_ops(),
      &op_code_lookup
  ))
}

pub fn load_from_file(filename: &str) -> Result<IntcodeVM<i32>, Box<dyn std::error::Error>> {
  Ok(IntcodeVM::create(
      load_memory_from_file(filename)?,
      get_ops(),
      &op_code_lookup
  ))
}

#[cfg(test)]
mod test {
    use super::string_to_i32_list;
    use super::load_from_str;
    use super::load_from_file;

    #[test]
    fn string_to_i32_list_testcase() {
        assert_eq!(string_to_i32_list("12,14").unwrap(), vec!(12, 14))
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

    #[test]
    fn test_parameter_mode_1() {
      let vm = load_from_str("1002,4,3,4,33").unwrap();
      let last_memory = vm.last().unwrap();
      assert_eq!(last_memory, vec!(1002, 4, 3, 4, 99));
    }
}
