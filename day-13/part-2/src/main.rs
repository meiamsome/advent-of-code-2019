use std::io::{stdin, stdout, Write};
use std::rc::Rc;
use std::sync::Mutex;
use std::cmp::Ordering;
use termion::screen::AlternateScreen;
use vm::lang::load_from_file;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(other: i64) -> Self {
        match other {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Input {
    positions: Rc<Mutex<(i64, i64)>>,
}

impl Iterator for Input {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let lock = self.positions.lock().unwrap();
        Some(match (*lock).0.cmp(&(*lock).1) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        })
    }
}

impl Input {
    fn set_ball_pos(&mut self, ball: i64) {
        let mut lock = self.positions.lock().unwrap();
        *lock = (ball, (*lock).1);
    }

    fn set_paddle_pos(&mut self, paddle: i64) {
        let mut lock = self.positions.lock().unwrap();
        *lock = ((*lock).0, paddle);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = AlternateScreen::from(stdout());
    let mut vm = load_from_file("./input.txt")?;
    let mut input = Input {
        positions: Rc::new(Mutex::new((0, 0))),
    };
    vm.io.input = Some(Box::new(input.clone()));

    write!(
        screen,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine,
        termion::cursor::Hide
    )?;
    while let Some(x) = vm.next() {
        if let Some(y) = vm.next() {
            if let Some(tile_id_or_score) = vm.next() {
                if x == -1 {
                    write!(
                        screen,
                        "{}Score: {}{}",
                        termion::cursor::Goto(1, 1),
                        tile_id_or_score,
                        termion::cursor::Hide
                    )?;
                } else {
                    let tile = tile_id_or_score.into();
                    write!(
                        screen,
                        "{}{}{}",
                        termion::cursor::Goto((x + 1) as u16, (y + 2) as u16),
                        match tile {
                            Tile::Empty => " ",
                            Tile::Wall => "█",
                            Tile::Block => "░",
                            Tile::Paddle => "▔",
                            Tile::Ball => "•",
                        },
                        termion::cursor::Hide
                    )?;
                    if tile == Tile::Paddle {
                        input.set_paddle_pos(x)
                    }
                    if tile == Tile::Ball {
                        input.set_ball_pos(x)
                    }
                }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
        screen.flush()?;
    }
    screen.flush()?;
    stdin().read_line(&mut String::new())?;
    Ok(())
}
