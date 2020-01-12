use std::fs::File;
use std::io::Read;
use std::iter;

use ::day16part1::FFTPhases;

fn proper_fft(input: &str) -> FFTPhases {
    let data: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let long_data: Vec<u32> = iter::repeat_with(|| data.iter().cloned())
        .take(10000)
        .flatten()
        .collect();
    let offset = long_data.iter().take(7).fold(0, |acc, &v| 10 * acc + v) as usize;
    println!("{}, {}", long_data.len(), offset);
    (
        long_data.into_iter().skip(offset).collect::<Vec<_>>(),
        offset,
    )
        .into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = String::new();
    {
        let mut file = File::open("./input.txt")?;
        file.read_to_string(&mut contents)?;
    }
    let mut result = proper_fft(&contents);
    println!(
        "{:?}",
        result
            .nth(99)
            .unwrap()
            .into_iter()
            .take(8)
            .collect::<Vec<_>>()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_proper_fft_03036732577212944063491565474664() {
        let example = "03036732577212944063491565474664";
        let mut fft = proper_fft(example);

        assert_eq!(fft.current.len(), (example.len() * 10000) - 303_673);

        assert_eq!(fft.offset, 303_673);

        assert_eq!(
            fft.nth(99).unwrap().into_iter().take(8).collect::<Vec<_>>(),
            vec![8, 4, 4, 6, 2, 0, 2, 6]
        );
    }

    #[test]
    fn test_proper_fft_02935109699940807407585447034323() {
        let example = "02935109699940807407585447034323";
        let mut fft = proper_fft(example);

        assert_eq!(fft.current.len(), (example.len() * 10000) - 293_510);

        assert_eq!(fft.offset, 293_510);

        assert_eq!(
            fft.nth(99).unwrap().into_iter().take(8).collect::<Vec<_>>(),
            vec![7, 8, 7, 2, 5, 2, 7, 0]
        );
    }

    #[test]
    fn test_proper_fft_03081770884921959731165446850517() {
        let example = "03081770884921959731165446850517";
        let mut fft = proper_fft(example);

        assert_eq!(fft.current.len(), (example.len() * 10000) - 308_177);

        assert_eq!(fft.offset, 308_177);

        assert_eq!(
            fft.nth(99).unwrap().into_iter().take(8).collect::<Vec<_>>(),
            vec![5, 3, 5, 5, 3, 7, 3, 1]
        );
    }
}
