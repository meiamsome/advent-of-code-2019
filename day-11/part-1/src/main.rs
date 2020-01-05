use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use vm::lang::load_from_file;

#[derive(Clone)]
struct LoopBackIterator {
    data: Rc<RwLock<Vec<i64>>>,
    position: usize,
}
impl LoopBackIterator {
    fn get_at(&mut self, position: usize) -> Option<i64> {
        if position >= (*self.data.read().unwrap()).len() {
            panic!("Invalid read");
        }
        Some((*self.data.read().unwrap())[position])
    }

    fn push(&mut self, next: i64) {
        (*self.data.write().unwrap()).push(next);
    }
}

impl Iterator for LoopBackIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let result = self.get_at(self.position);
        self.position += 1;
        result
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self, turn: i64) -> Dir {
        match turn {
            0 => match self {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
            },
            1 => match self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            },
            _ => panic!(),
        }
    }

    fn move_in_dir(&self, coords: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::Up => (coords.0, coords.1 - 1),
            Dir::Left => (coords.0 - 1, coords.1),
            Dir::Down => (coords.0, coords.1 + 1),
            Dir::Right => (coords.0 + 1, coords.1),
        }
    }
}

#[derive(Clone, Copy)]
enum Colour {
    Black = 0,
    White = 1,
}

impl From<i64> for Colour {
    fn from(other: i64) -> Self {
        match other {
            0 => Colour::Black,
            1 => Colour::White,
            _ => panic!(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut loop_back = LoopBackIterator {
        data: Rc::new(RwLock::new(vec![0])),
        position: 0,
    };
    let mut colours: HashMap<(i64, i64), Colour> = HashMap::new();
    let mut dir = Dir::Up;
    let mut pos = (0, 0);
    let mut vm = load_from_file("./input.txt")?;
    vm.io.input = Some(Box::new(loop_back.clone()));
    loop {
        if let Some(paint) = vm.next() {
            colours.insert(pos, paint.into());
            if let Some(turn) = vm.next() {
                dir = dir.turn(turn);
                pos = dir.move_in_dir(pos);
                loop_back.push(*colours.get(&pos).unwrap_or(&Colour::Black) as i64);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("Painted tiles: {}", colours.len());
    Ok(())
}
