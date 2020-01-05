use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::fmt;

#[derive(Debug)]
struct Layer {
    width: usize,
    height: usize,
    data: Vec<u32>
}

impl Layer {
    fn from_vec(input: Vec<u32>, width: usize, height: usize) -> Layer {
        if input.len() != width * height {
            panic!("Invalid layer format: {} != {}", input.len(), width * height);
        }
        Layer {
            width,
            height,
            data: input
        }
    }
}

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>
}

impl Image {
    fn from_vec(input: Vec<u32>, width: usize, height: usize) -> Image {
        if input.len() % (width * height) != 0 {
            panic!("Invalid file format: {} extra bytes", input.len() % (width * height));
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

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.layers[0].data.iter()
            .enumerate()
            .map(|(i, _)| self.layers.iter().fold(2, |current, ref layer| {
                if current == 2 {
                    layer.data[i]
                } else {
                    current
                }
            }))
            .map(|x| std::char::from_digit(x, 10).unwrap())
            .collect::<Vec<char>>()
            .chunks(self.width)
            .map(|x| {
                let mut vec = x.to_vec();
                vec.push('\n');
                String::from_iter(vec)
            })
            .collect::<Vec<String>>()
            .into_iter()
            .collect::<String>()
        )
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
    println!("{}", image);
    Ok(())
}
