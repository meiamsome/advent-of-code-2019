use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::fmt::Debug;

pub mod lang;

pub trait OpCode<T>
{
    fn execute(&self, instruction_pointer: usize, memory: &mut Vec<T>) -> Option<usize>;
}

pub struct IntcodeVM<T>
{
    pub instruction_pointer: usize,
    pub memory: Vec<T>,
    pub op_codes: HashMap<T, Box<dyn OpCode<T>>>,
}


impl<T> IntcodeVM<T>
where
    T: Hash + Eq + PartialEq
{
    pub fn create(memory: Vec<T>, op_codes: HashMap<T, Box<dyn OpCode<T>>>) -> IntcodeVM<T> {
        IntcodeVM {
            instruction_pointer: 0,
            memory: memory,
            op_codes: op_codes
        }
    }
}

impl<T> Iterator for IntcodeVM<T>
where
    T: Clone + Hash + Eq + PartialEq + Debug
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let op_code = &self.memory[self.instruction_pointer];
        let op = self.op_codes.get(op_code);
        if op.is_none() {
            panic!("Unknown OpCode! {:?}", op_code);
        }

        match op.unwrap().execute(self.instruction_pointer, &mut self.memory) {
            Some(x) => {
                self.instruction_pointer += x;
                Some(self.memory.clone())
            },
            None => None
        }
    }
}
