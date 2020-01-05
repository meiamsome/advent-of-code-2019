use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use vm::lang::load_from_str;

#[derive(Clone)]
struct LoopBackIterator<'a> {
    data: Rc<RwLock<Vec<i64>>>,
    iter: Rc<Mutex<Option<Box<dyn Iterator<Item = i64> + 'a>>>>,
    position: usize,
}
impl<'a> LoopBackIterator<'a> {
    fn get_at(&mut self, position: usize) -> Option<i64> {
        while position >= (*self.data.read().unwrap()).len() {
            let mut iter = self.iter.lock().unwrap();
            if let Some(ref mut iterator) = *iter {
                if let Some(next) = iterator.next() {
                    (*self.data.write().unwrap()).push(next);
                } else {
                    return None;
                }
            } else {
                panic!("No iterator");
            }
        }
        Some((*self.data.read().unwrap())[position])
    }

    fn set_iter(&mut self, iter: Box<dyn Iterator<Item = i64> + 'a>) {
        *self.iter.lock().unwrap() = Some(iter);
    }
}

impl Iterator for LoopBackIterator<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let result = self.get_at(self.position);
        self.position += 1;
        result
    }
}

fn run_with_phases(program: &str, phases: Vec<i64>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut loop_back = LoopBackIterator {
        data: Rc::new(RwLock::new(vec![0])),
        iter: Rc::new(Mutex::new(None)),
        position: 0,
    };
    loop_back.set_iter(phases.into_iter().fold::<Result<
        Box<dyn Iterator<Item = i64>>,
        Box<dyn std::error::Error>,
    >, _>(
        Ok(Box::new(loop_back.clone())),
        |iterator, phase| {
            iterator.and_then::<Box<dyn Iterator<Item = i64>>, _>(|iter| {
                let mut vm = load_from_str(program)?;
                vm.io.input = Some(Box::new(vec![phase].into_iter().chain(iter)));
                Ok(Box::new(vm))
            })
        },
    )?);
    let result = loop_back.clone().last().unwrap();
    Ok(result)
}

fn get_optimal_phase(
    program: &str,
) -> Result<(i64, (i64, i64, i64, i64, i64)), Box<dyn std::error::Error>> {
    let mut max = 0;
    let mut arg_max = (0, 0, 0, 0, 0);
    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    for e in 5..=9 {
                        if vec![a, b, c, d, e]
                            .into_iter()
                            .collect::<HashSet<_>>()
                            .len()
                            != 5
                        {
                            continue;
                        }
                        let score = run_with_phases(program, vec![a, b, c, d, e])?;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
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
            get_optimal_phase("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5").unwrap(),
            (139629729, (9, 8, 7, 6, 5)),
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            get_optimal_phase("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10").unwrap(),
            (18216, (9, 7, 8, 5, 6)),
        )
    }
}
