use std::collections::HashMap;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut vm = load_from_file("./input.txt")?;
    while let Some(x) = vm.next() {
        if let Some(y) = vm.next() {
            if let Some(tile_id) = vm.next() {
                tiles.insert((x, y), tile_id.into());
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
    println!("Painted tiles: {}", tiles.len());
    let min_x = *tiles.keys().map(|(x, _)| x).min().unwrap();
    let min_y = *tiles.keys().map(|(_, y)| y).min().unwrap();
    let max_x = *tiles.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *tiles.keys().map(|(_, y)| y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                match *tiles.get(&(x, y)).unwrap_or(&Tile::Empty) {
                    Tile::Empty => " ",
                    Tile::Wall => "█",
                    Tile::Block => "░",
                    Tile::Paddle => "▔",
                    Tile::Ball => "•",
                }
            );
        }
        println!();
    }
    println!(
        "Blocks: {}",
        tiles.values().filter(|&&x| x == Tile::Block).count()
    );
    Ok(())
}
