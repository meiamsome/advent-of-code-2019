use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Layer {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Layer {
    fn from_vec(input: Vec<u32>, width: usize, height: usize) -> Layer {
        if input.len() != width * height {
            panic!(
                "Invalid layer format: {} != {}",
                input.len(),
                width * height
            );
        }
        Layer {
            width,
            height,
            data: input,
        }
    }
}

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn from_vec(input: Vec<u32>, width: usize, height: usize) -> Image {
        if input.len() % (width * height) != 0 {
            panic!(
                "Invalid file format: {} extra bytes",
                input.len() % (width * height)
            );
        }
        Image {
            width,
            height,
            layers: input
                .chunks(width * height)
                .map(|vals| Layer::from_vec(vals.to_vec(), width, height))
                .collect(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let numbers = contents
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let image = Image::from_vec(numbers, 25, 6);
    let (ones, twos) = image
        .layers
        .into_iter()
        .min_by_key(|layer| {
            layer
                .data
                .iter()
                .fold(0, |acc, &value| if value == 0 { acc + 1 } else { acc })
        })
        .unwrap()
        .data
        .into_iter()
        .fold((0, 0), |(ones, twos), value| {
            if value == 1 {
                (ones + 1, twos)
            } else if value == 2 {
                (ones, twos + 1)
            } else {
                (ones, twos)
            }
        });
    println!("{}", ones * twos);
    Ok(())
}
