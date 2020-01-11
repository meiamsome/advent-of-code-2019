use std::io::{stdin, stdout, Write};
use termion::screen::AlternateScreen;

use day15part1::astar::AStar;
use day15part1::explorer::explore;
use day15part1::maze::Maze;

type Position = (i64, i64);

fn zero(_: Position, _: Position) -> i64 {
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = AlternateScreen::from(stdout());
    let maze: Maze = explore()?;
    println!("{}", maze);
    let mut astar: AStar = maze.into();
    astar.open_set.clear();
    astar.open_set.insert(astar.target, (0, None));
    astar.target = (500, 500);
    astar.dist_fn = &zero;
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
    let (pos, (len, _)) = astar
        .closed_set
        .iter()
        .max_by_key(|(_, (path_len, _))| *path_len)
        .unwrap();
    println!("Max len: {}", len);
    println!("Path: {:?}", astar.get_path_to(*pos).unwrap());
    screen.flush()?;
    stdin().read_line(&mut String::new())?;
    Ok(())
}
