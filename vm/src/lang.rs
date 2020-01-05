use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::convert::TryFrom;

use super::{IntcodeVM, IntcodeVMIO, IntcodeVMMemory, OpCode};

fn get_parameter_addresses_with_modes(
    memory: &mut IntcodeVMMemory<i64>,
    count: usize,
) -> Vec<usize> {
    let mut op_code = memory.get(memory.instruction_pointer, 0) / 100;
    (0..count)
        .map(|offset| {
            let mode = op_code % 10;
            op_code /= 10;
            match mode {
                0 => {
                    usize::try_from(memory.get(memory.instruction_pointer + offset + 1, 0)).unwrap()
                }
                1 => memory.instruction_pointer + offset + 1,
                2 => usize::try_from(
                    *memory.metadata.get(0).unwrap_or(&0)
                        + memory.get(memory.instruction_pointer + offset + 1, 0),
                )
                .unwrap(),
                _ => panic!("Unknown mode {}", mode),
            }
        })
        .collect()
}

struct Halt;
impl OpCode<i64> for Halt {
    fn execute(
        &self,
        _memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        None
    }
}

struct Add;
impl OpCode<i64> for Add {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory.get(addresses[0], 0);
        let b = memory.get(addresses[1], 0);
        memory.set(addresses[2], a + b, 0);
        Some((memory.instruction_pointer + 4, None))
    }
}

struct Mul;
impl OpCode<i64> for Mul {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory.get(addresses[0], 0);
        let b = memory.get(addresses[1], 0);
        memory.set(addresses[2], a * b, 0);
        Some((memory.instruction_pointer + 4, None))
    }
}

struct Input;
impl OpCode<i64> for Input {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 1);
        memory.set(
            addresses[0],
            io.input
                .as_mut()
                .map(|input| input.next().unwrap())
                .expect("No IO input specified."),
            0,
        );
        Some((memory.instruction_pointer + 2, None))
    }
}

struct Output;
impl OpCode<i64> for Output {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 2);
        io.output.as_mut().map(|x| x(memory.get(addresses[0], 0)));
        Some((
            memory.instruction_pointer + 2,
            Some(memory.get(addresses[0], 0)),
        ))
    }
}

struct JumpIfTrue;
impl OpCode<i64> for JumpIfTrue {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 2);
        if memory.get(addresses[0], 0) != 0 {
            Some((usize::try_from(memory.get(addresses[1], 0)).unwrap(), None))
        } else {
            Some((memory.instruction_pointer + 3, None))
        }
    }
}

struct JumpIfFalse;
impl OpCode<i64> for JumpIfFalse {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 2);
        if memory.get(addresses[0], 0) == 0 {
            Some((usize::try_from(memory.get(addresses[1], 0)).unwrap(), None))
        } else {
            Some((memory.instruction_pointer + 3, None))
        }
    }
}

struct LessThan;
impl OpCode<i64> for LessThan {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory.get(addresses[0], 0);
        let b = memory.get(addresses[1], 0);
        memory.set(addresses[2], if a < b { 1 } else { 0 }, 0);
        Some((memory.instruction_pointer + 4, None))
    }
}

struct Equals;
impl OpCode<i64> for Equals {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 3);
        let a = memory.get(addresses[0], 0);
        let b = memory.get(addresses[1], 0);
        memory.set(addresses[2], if a == b { 1 } else { 0 }, 0);
        Some((memory.instruction_pointer + 4, None))
    }
}

struct RelativeBaseOffset;
impl OpCode<i64> for RelativeBaseOffset {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<i64>,
        _io: &mut IntcodeVMIO<i64>,
    ) -> Option<(usize, Option<i64>)> {
        let addresses = get_parameter_addresses_with_modes(memory, 1);
        let a = memory.get(addresses[0], 0);
        if memory.metadata.len() < 1 {
            memory.metadata.resize(1, 0)
        }
        memory.metadata[0] += a;
        Some((memory.instruction_pointer + 2, None))
    }
}

pub fn get_ops() -> HashMap<i64, Box<dyn OpCode<i64>>> {
    let mut ops: HashMap<i64, Box<dyn OpCode<i64>>> = HashMap::new();
    ops.insert(1, Box::new(Add));
    ops.insert(2, Box::new(Mul));
    ops.insert(3, Box::new(Input));
    ops.insert(4, Box::new(Output));
    ops.insert(5, Box::new(JumpIfTrue));
    ops.insert(6, Box::new(JumpIfFalse));
    ops.insert(7, Box::new(LessThan));
    ops.insert(8, Box::new(Equals));
    ops.insert(9, Box::new(RelativeBaseOffset));
    ops.insert(99, Box::new(Halt));
    return ops;
}

fn string_to_i64_list(data: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    data.split(',')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<i64>, std::num::ParseIntError>>()
}

pub fn load_memory_from_file(filename: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open(filename)?;
        file.read_to_string(&mut contents)?;
    }
    Ok(string_to_i64_list(contents.trim())?)
}

pub fn op_code_lookup(input: i64) -> i64 {
    input % 100
}

pub fn load_from_str(program: &str) -> Result<IntcodeVM<i64>, Box<dyn std::error::Error>> {
    Ok(IntcodeVM::create(
        string_to_i64_list(program.trim())?,
        get_ops(),
        &op_code_lookup,
        None,
        None,
    ))
}

pub fn load_from_file(filename: &str) -> Result<IntcodeVM<i64>, Box<dyn std::error::Error>> {
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
    use super::load_from_file;
    use super::load_from_str;
    use super::string_to_i64_list;

    #[test]
    fn string_to_i64_list_testcase() {
        assert_eq!(string_to_i64_list("12,14").unwrap(), vec!(12, 14))
    }

    // Regression tests
    #[test]
    fn test_regression_day2_part1() {
        let mut vm = load_from_file("../day-02/part-1/input.txt").unwrap();
        while let Some(_) = vm.next() {}
        let last_memory = vm.memory.memory;
        assert_eq!(last_memory[0], 9581917);
    }

    #[test]
    fn test_regression_day2_part2() {
        let mut vm = load_from_file("../day-02/part-2/input.txt").unwrap();
        vm.memory[1] = 25;
        vm.memory[2] = 5;
        while let Some(_) = vm.next() {}
        let last_memory = vm.memory.memory;
        assert_eq!(last_memory[0], 19690720);
    }

    // Day 5 part 1
    #[test]
    fn test_parameter_mode_1() {
        let mut vm = load_from_str("1002,4,3,4,33").unwrap();
        while let Some(_) = vm.next() {}
        let last_memory = vm.memory.memory;
        assert_eq!(last_memory, vec!(1002, 4, 3, 4, 99));
    }

    // regression
    #[test]
    fn test_regression_day5_part1() {
        let mut vm = load_from_file("../day-05/part-1/input.txt").unwrap();
        vm.io.input = Some(Box::new(vec![1].into_iter()));
        let mut count = 0;
        let mut last_value = 0;
        vm.io.output = Some(Box::new(|value| {
            count += 1;
            if last_value != 0 {
                panic!("Unexpected non-zero op code test");
            }
            last_value = value;
        }));
        vm.last().unwrap();
        assert_eq!(count, 10);
        assert_eq!(last_value, 16434972);
    }

    // Day 5 part 2
    #[test]
    fn test_position_mode_1_lessthan() {
        let mut vm = load_from_str("3,9,8,9,10,9,4,9,99,-1,8").unwrap();
        vm.io.input = Some(Box::new(vec![1].into_iter()));
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
        vm.io.input = Some(Box::new(vec![8].into_iter()));
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
        vm.io.input = Some(Box::new(vec![10].into_iter()));
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
    fn test_position_mode_2_lessthan() {
        let mut vm = load_from_str("3,9,7,9,10,9,4,9,99,-1,8").unwrap();
        vm.io.input = Some(Box::new(vec![1].into_iter()));
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
    fn test_position_mode_2_equal() {
        let mut vm = load_from_str("3,9,7,9,10,9,4,9,99,-1,8").unwrap();
        vm.io.input = Some(Box::new(vec![8].into_iter()));
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
    fn test_position_mode_2_greater_than() {
        let mut vm = load_from_str("3,9,7,9,10,9,4,9,99,-1,8").unwrap();
        vm.io.input = Some(Box::new(vec![10].into_iter()));
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
    fn test_immediate_mode_1_lessthan() {
        let mut vm = load_from_str("3,3,1108,-1,8,3,4,3,99").unwrap();
        vm.io.input = Some(Box::new(vec![1].into_iter()));
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
    fn test_immediate_mode_1_equal() {
        let mut vm = load_from_str("3,3,1108,-1,8,3,4,3,99").unwrap();
        vm.io.input = Some(Box::new(vec![8].into_iter()));
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
    fn test_immediate_mode_1_greater_than() {
        let mut vm = load_from_str("3,3,1108,-1,8,3,4,3,99").unwrap();
        vm.io.input = Some(Box::new(vec![10].into_iter()));
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
    fn test_immediate_mode_2_lessthan() {
        let mut vm = load_from_str("3,3,1107,-1,8,3,4,3,99").unwrap();
        vm.io.input = Some(Box::new(vec![1].into_iter()));
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
    fn test_immediate_mode_2_equal() {
        let mut vm = load_from_str("3,3,1107,-1,8,3,4,3,99").unwrap();
        vm.io.input = Some(Box::new(vec![8].into_iter()));
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
    fn test_immediate_mode_2_greater_than() {
        let mut vm = load_from_str("3,3,1107,-1,8,3,4,3,99").unwrap();
        vm.io.input = Some(Box::new(vec![10].into_iter()));
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
    fn test_position_mode_jump_zero() {
        let mut vm = load_from_str("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").unwrap();
        vm.io.input = Some(Box::new(vec![0].into_iter()));
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
    fn test_position_mode_jump_non_zero() {
        let mut vm = load_from_str("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").unwrap();
        vm.io.input = Some(Box::new(vec![10].into_iter()));
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
    fn test_immediate_mode_jump_zero() {
        let mut vm = load_from_str("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").unwrap();
        vm.io.input = Some(Box::new(vec![0].into_iter()));
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
    fn test_immediate_mode_jump_non_zero() {
        let mut vm = load_from_str("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").unwrap();
        vm.io.input = Some(Box::new(vec![10].into_iter()));
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
    fn test_8_compare_less_than() {
        let mut vm = load_from_str("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99").unwrap();
        vm.io.input = Some(Box::new(vec![5].into_iter()));
        let mut count = 0;
        let mut last_value = 0;
        vm.io.output = Some(Box::new(|value| {
            count += 1;
            last_value = value;
        }));
        vm.last().unwrap();
        assert_eq!(count, 1);
        assert_eq!(last_value, 999);
    }

    #[test]
    fn test_8_compare_equal() {
        let mut vm = load_from_str("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99").unwrap();
        vm.io.input = Some(Box::new(vec![8].into_iter()));
        let mut count = 0;
        let mut last_value = 0;
        vm.io.output = Some(Box::new(|value| {
            count += 1;
            last_value = value;
        }));
        vm.last().unwrap();
        assert_eq!(count, 1);
        assert_eq!(last_value, 1000);
    }

    #[test]
    fn test_8_compare_greater_than() {
        let mut vm = load_from_str("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99").unwrap();
        vm.io.input = Some(Box::new(vec![10].into_iter()));
        let mut count = 0;
        let mut last_value = 0;
        vm.io.output = Some(Box::new(|value| {
            count += 1;
            last_value = value;
        }));
        vm.last().unwrap();
        assert_eq!(count, 1);
        assert_eq!(last_value, 1001);
    }

    // regression
    #[test]
    fn test_regression_day5_part2() {
        let mut vm = load_from_file("../day-05/part-1/input.txt").unwrap();
        vm.io.input = Some(Box::new(vec![5].into_iter()));
        let mut count = 0;
        let mut last_value = 0;
        vm.io.output = Some(Box::new(|value| {
            count += 1;
            last_value = value;
        }));
        vm.last().unwrap();
        assert_eq!(count, 1);
        assert_eq!(last_value, 16694270);
    }

    // Quine: takes no input and produces a copy of itself as output.
    #[test]
    fn test_day9_part1_quine() {
        let vm =
            load_from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99").unwrap();
        assert_eq!(
            vm.collect::<Vec<i64>>(),
            vec!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99)
        );
    }

    // should output a 16-digit number.
    #[test]
    fn test_day9_part1_16digit() {
        let vm =
            load_from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99").unwrap();
        assert_eq!(
            vm.collect::<Vec<i64>>(),
            vec!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99)
        );
    }

    // should output the large number in the middle.
    #[test]
    fn test_day9_part1_large() {
        let vm = load_from_str("104,1125899906842624,99").unwrap();
        assert_eq!(vm.collect::<Vec<i64>>(), vec!(1125899906842624));
    }
}
