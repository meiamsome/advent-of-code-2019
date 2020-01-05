#[macro_use]
extern crate log;

use core::ops::{Index, IndexMut};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub mod io;
pub mod lang;

pub struct IntcodeVMMemory<T> {
    pub instruction_pointer: usize,
    pub memory: Vec<T>,
    pub metadata: Vec<T>,
}

impl<T> IntcodeVMMemory<T>
where
    T: Copy,
{
    fn get(&self, position: usize, default: T) -> T {
        *self.memory.get(position).unwrap_or(&default)
    }

    fn set(&mut self, position: usize, value: T, default_memory_value: T) {
        if position >= self.memory.len() {
            self.memory.resize(position + 1, default_memory_value)
        }
        self.memory[position] = value;
    }
}

impl<T> Index<usize> for IntcodeVMMemory<T> {
    type Output = T;

    fn index(&self, location: usize) -> &Self::Output {
        &self.memory[location]
    }
}

impl<T> IndexMut<usize> for IntcodeVMMemory<T> {
    fn index_mut(&mut self, location: usize) -> &mut Self::Output {
        &mut self.memory[location]
    }
}

pub struct IntcodeVMIO<'a, T> {
    pub input: Option<Box<dyn Iterator<Item = T> + 'a>>,
    pub output: Option<Box<dyn FnMut(T) -> () + 'a>>,
}

pub trait OpCode<T> {
    fn execute(
        &self,
        memory: &mut IntcodeVMMemory<T>,
        io: &mut IntcodeVMIO<T>,
    ) -> Option<(usize, Option<T>)>;
}

pub struct IntcodeVM<'a, T> {
    pub memory: IntcodeVMMemory<T>,
    pub op_codes: HashMap<T, Box<dyn OpCode<T>>>,
    pub op_code_map: &'a dyn Fn(T) -> T,
    pub io: IntcodeVMIO<'a, T>,
}

impl<'a, T> IntcodeVM<'a, T>
where
    T: Hash + Eq + PartialEq,
{
    pub fn create(
        memory: Vec<T>,
        op_codes: HashMap<T, Box<dyn OpCode<T>>>,
        op_code_map: &'a dyn Fn(T) -> T,
        input: Option<Box<dyn Iterator<Item = T>>>,
        output: Option<Box<dyn FnMut(T) -> ()>>,
    ) -> IntcodeVM<'a, T> {
        IntcodeVM {
            memory: IntcodeVMMemory {
                instruction_pointer: 0,
                memory: memory,
                metadata: Vec::new(),
            },
            op_codes: op_codes,
            op_code_map: op_code_map,
            io: IntcodeVMIO {
                input: input,
                output: output,
            },
        }
    }
}

impl<T> Iterator for IntcodeVM<'_, T>
where
    T: Copy + Hash + Eq + PartialEq + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let op_code = (self.op_code_map)(self.memory.memory[self.memory.instruction_pointer]);
            let op = self.op_codes.get(&op_code);
            if op.is_none() {
                panic!("Unknown OpCode! {:?}", op_code);
            }

            match op.unwrap().execute(&mut self.memory, &mut self.io) {
                Some((new_instruction_pointer, ret_val_option)) => {
                    debug!(
                        "Processed op code: {:?}",
                        self.memory.memory[self.memory.instruction_pointer]
                    );
                    self.memory.instruction_pointer = new_instruction_pointer;
                    if let Some(ret_val) = ret_val_option {
                        return Some(ret_val);
                    }
                }
                None => return None,
            }
        }
    }
}
