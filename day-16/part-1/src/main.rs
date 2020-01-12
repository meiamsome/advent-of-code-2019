use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

use ::day16part1::FFTPhases;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let mut fft: FFTPhases = (contents.as_str().trim(), 0).try_into().unwrap();
    println!(
        "{:?}",
        fft.nth(99).unwrap().into_iter().take(8).collect::<Vec<_>>()
    );
    Ok(())
}
