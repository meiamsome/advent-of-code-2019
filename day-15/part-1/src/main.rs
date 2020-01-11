use std::io::{stdin, stdout, Write};
use termion::screen::AlternateScreen;

mod astar;
mod dir;
mod explorer;
mod maze;
mod tile;

use self::astar::AStar;
use self::explorer::explore;
use self::maze::Maze;

type Position = (i64, i64);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = AlternateScreen::from(stdout());
    let maze: Maze = explore()?;
    println!("{}", maze);
    let mut astar: AStar = maze.into();
    while let Some(()) = astar.next() {
        write!(
            screen,
            "{}{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            astar,
            termion::cursor::Hide
        )?;
    }
    println!(
        "Path len: {}",
        astar.get_path_to(astar.target).unwrap().len()
    );
    println!("Path: {:?}", astar.get_path_to(astar.target).unwrap());
    screen.flush()?;
    stdin().read_line(&mut String::new())?;
    Ok(())
}
