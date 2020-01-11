use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Mutex;
use vm::lang::load_from_file;

use super::dir::Dir;
use super::maze::Maze;
use super::tile::Tile;
use super::Position;

#[derive(Clone)]
pub struct InstructionIterator {
    pub map: Rc<Mutex<HashMap<Position, Tile>>>,
    pub history_stack: Rc<Mutex<Vec<Position>>>,
}

impl InstructionIterator {
    fn current_position(&self) -> Option<Position> {
        let location_stack = self.history_stack.lock().unwrap();
        location_stack
            .get(location_stack.len().saturating_sub(1))
            .cloned()
    }

    fn handle_output(&mut self, value: Tile) {
        if let Some(position) = self.current_position() {
            self.map.lock().unwrap().insert(position, value);
            if value == Tile::Wall {
                self.history_stack.lock().unwrap().pop();
            }
        }
    }

    fn should_continue(&self) -> bool {
        self.current_position().is_some()
    }
}

impl Iterator for InstructionIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_position = self.current_position().unwrap();
        let seen_locations = self.map.lock().unwrap();
        let mut location_stack = self.history_stack.lock().unwrap();
        for dir in Dir::all() {
            let location = dir.move_in_dir(current_position);
            if !seen_locations.contains_key(&location) {
                location_stack.push(location);
                return Some(dir.into());
            }
        }
        location_stack.pop().unwrap();
        drop(location_stack);
        if let Some(next_location) = self.current_position() {
            Some(
                Dir::between(current_position, next_location)
                    .expect("Invalid from/to Dir::between")
                    .into(),
            )
        } else {
            Some(Dir::Up.into())
        }
    }
}

impl Display for InstructionIterator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let current_position = self.current_position().unwrap();
        let location_stack = self.history_stack.lock().unwrap();
        let seen_locations = self.map.lock().unwrap();

        let min_x = *seen_locations.keys().map(|(x, _)| x).min().unwrap();
        let min_y = *seen_locations.keys().map(|(_, y)| y).min().unwrap();
        let max_x = *seen_locations.keys().map(|(x, _)| x).max().unwrap();
        let max_y = *seen_locations.keys().map(|(_, y)| y).max().unwrap();
        write!(
            f,
            "{}",
            (min_y..=max_y)
                .map(|y| {
                    (min_x..=max_x)
                        .map(|x| {
                            if current_position == (x, y) {
                                return "D";
                            }
                            if location_stack.contains(&(x, y)) {
                                return "P";
                            }
                            seen_locations
                                .get(&(x, y))
                                .unwrap_or(&Tile::Unexplored)
                                .into()
                        })
                        .collect::<String>()
                        + "\n"
                })
                .collect::<String>()
        )
    }
}

pub fn explore() -> Result<Maze, Box<dyn std::error::Error>> {
    let mut iterator = InstructionIterator {
        map: Rc::new(Mutex::new(HashMap::new())),
        history_stack: Rc::new(Mutex::new(vec![(0, 0)])),
    };
    let mut vm = load_from_file("./input.txt")?;
    vm.io.input = Some(Box::new(iterator.clone()));
    while iterator.should_continue() {
        iterator.handle_output(vm.next().unwrap().into());
    }
    Ok(iterator.into())
}
