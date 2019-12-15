use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::convert::TryFrom;

use super::{IntcodeVM, IntcodeVMMemory, IntcodeVMIO, OpCode};

fn get_parameter_addresses_with_modes(memory: &mut IntcodeVMMemory<i32>, count: usize) -> Vec<usize> {
  let mut op_code = memory[memory.instruction_pointer] / 100;
  (0..count)
    .map(|offset| {
      let mode = op_code % 10;
      op_code /= 10;
      match mode {
        0 => usize::try_from(memory[memory.instruction_pointer + offset + 1]).unwrap(),
        1 => memory.instruction_pointer + offset + 1,
        _ => panic!("Unknown mode {}", mode),
      }
    })
    .collect()
}

struct Halt;
impl OpCode<i32> for Halt {
    fn execute(&self, _memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        None
    }
}

struct Add;
impl OpCode<i32> for Add {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory[addresses[0]];
        let b = memory[addresses[1]];
        memory[addresses[2]] = a + b;
        Some(memory.instruction_pointer + 4)
    }
}

struct Mul;
impl OpCode<i32> for Mul {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory[addresses[0]];
        let b = memory[addresses[1]];
        memory[addresses[2]] = a * b;
        Some(memory.instruction_pointer + 4)
    }
}

struct Input;
impl OpCode<i32> for Input {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 1);
        memory[addresses[0]] =  io.input.as_mut().map(|input| input.next().unwrap()).expect("No IO input specified.");
        Some(memory.instruction_pointer + 2)
    }
}

struct Output;
impl OpCode<i32> for Output {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 2);
        io.output.as_mut().map(|x| x(memory[addresses[0]])).expect("No IO output specified");
        Some(memory.instruction_pointer + 2)
    }
}

struct JumpIfTrue;
impl OpCode<i32> for JumpIfTrue {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 2);
        if memory[addresses[0]] != 0 {
          Some(usize::try_from(memory[addresses[1]]).unwrap())
        } else {
          Some(memory.instruction_pointer + 3)
        }
    }
}

struct JumpIfFalse;
impl OpCode<i32> for JumpIfFalse {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 2);
        if memory[addresses[0]] == 0 {
          Some(usize::try_from(memory[addresses[1]]).unwrap())
        } else {
          Some(memory.instruction_pointer + 3)
        }
    }
}

struct LessThan;
impl OpCode<i32> for LessThan {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory[addresses[0]];
        let b = memory[addresses[1]];
        memory[addresses[2]] = if a < b { 1 } else { 0 };
        Some(memory.instruction_pointer + 4)
    }
}

struct Equals;
impl OpCode<i32> for Equals {
    fn execute(&self, memory: &mut IntcodeVMMemory<i32>, _io: &mut IntcodeVMIO<i32>) -> Option<usize> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory[addresses[0]];
        let b = memory[addresses[1]];
        memory[addresses[2]] = if a == b { 1 } else { 0 };
        Some(memory.instruction_pointer + 4)
    }
}

pub fn get_ops() -> HashMap<i32, Box<dyn OpCode<i32>>> {
  let mut ops: HashMap<i32, Box<dyn OpCode<i32>>> = HashMap::new();
  ops.insert(1, Box::new(Add));
  ops.insert(2, Box::new(Mul));
  ops.insert(3, Box::new(Input));
  ops.insert(4, Box::new(Output));
  ops.insert(5, Box::new(JumpIfTrue));
  ops.insert(6, Box::new(JumpIfFalse));
  ops.insert(7, Box::new(LessThan));
  ops.insert(8, Box::new(Equals));
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
      &op_code_lookup,
      None,
      None,
  ))
}

pub fn load_from_file(filename: &str) -> Result<IntcodeVM<i32>, Box<dyn std::error::Error>> {
  Ok(IntcodeVM::create(
      load_memory_from_file(filename)?,
      get_ops(),
      &op_code_lookup,
      None,
      None,
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

    // Day 5 part 1
    #[test]
    fn test_parameter_mode_1() {
      let vm = load_from_str("1002,4,3,4,33").unwrap();
      let last_memory = vm.last().unwrap();
      assert_eq!(last_memory, vec!(1002, 4, 3, 4, 99));
    }

    // Day 5 part 2
    #[test]
    fn test_position_mode_1_lessthan() {
      let mut vm = load_from_str("3,9,8,9,10,9,4,9,99,-1,8").unwrap();
      vm.io.input = Some(Box::new(vec!(1).into_iter()));
      let mut count = 0;
      let mut last_value = 0;
      vm.io.output = Some(Box::new(|value| {
        count += 1;
        last_value = value;
      }));
      vm.last().unwrap();
      assert_eq!(count, 1);
      assert_eq!(last_value, 0);
    }

    #[test]
    fn test_position_mode_1_equal() {
      let mut vm = load_from_str("3,9,8,9,10,9,4,9,99,-1,8").unwrap();
      vm.io.input = Some(Box::new(vec!(8).into_iter()));
      let mut count = 0;
      let mut last_value = 0;
      vm.io.output = Some(Box::new(|value| {
        count += 1;
        last_value = value;
      }));
      vm.last().unwrap();
      assert_eq!(count, 1);
      assert_eq!(last_value, 1);
    }

    #[test]
    fn test_position_mode_1_greater_than() {
      let mut vm = load_from_str("3,9,8,9,10,9,4,9,99,-1,8").unwrap();
      vm.io.input = Some(Box::new(vec!(10).into_iter()));
      let mut count = 0;
      let mut last_value = 0;
      vm.io.output = Some(Box::new(|value| {
        count += 1;
        last_value = value;
      }));
      vm.last().unwrap();
      assert_eq!(count, 1);
      assert_eq!(last_value, 0);
    }
}
