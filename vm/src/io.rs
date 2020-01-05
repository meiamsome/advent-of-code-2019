use std::fmt::{Debug, Display};
use std::io::stdin;
use std::marker::PhantomData;
use std::str::FromStr;

use super::IntcodeVMIO;

struct IntcodeVMInput<T> {
    phantom: PhantomData<T>,
}
impl<T> Iterator for IntcodeVMInput<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut input_text = String::new();
        stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        Some(trimmed.parse::<Self::Item>().unwrap())
    }
}

pub fn create_stdio_vmio<'a, T: 'a>() -> IntcodeVMIO<'a, T>
where
    T: Display + FromStr,
    <T as FromStr>::Err: Debug,
{
    IntcodeVMIO {
        input: Some(Box::new(IntcodeVMInput::<T> {
            phantom: PhantomData,
        })),
        output: Some(Box::new(|x| println!("Output: {}", x))),
    }
}
