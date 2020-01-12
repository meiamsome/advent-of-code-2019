use std::convert::TryFrom;

const BASE_PATTERN: &[i64] = &[0, 1, 0, -1];

fn fft_step(arr: &[i64]) -> Vec<i64> {
    arr.iter()
        .enumerate()
        .map(|(y, _)| {
            arr.iter()
                .enumerate()
                .map(|(x, value)| value * BASE_PATTERN[((x + 1) / (y + 1)) % BASE_PATTERN.len()])
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect()
}

pub struct FFTPhases {
    current: Vec<i64>,
}

impl<T> From<Vec<T>> for FFTPhases
where
    i64: From<T>,
{
    fn from(current: Vec<T>) -> FFTPhases {
        FFTPhases {
            current: current.into_iter().map(|e| e.into()).collect(),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidDigit(char),
}

impl TryFrom<&str> for FFTPhases {
    type Error = ParseError;

    fn try_from(string: &str) -> Result<FFTPhases, ParseError> {
        Ok(string
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| ParseError::InvalidDigit(c)))
            .collect::<Result<Vec<u32>, _>>()?
            .into())
    }
}

impl Iterator for FFTPhases {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = fft_step(&self.current);
        Some(self.current.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn fft_step_12345678() {
        assert_eq!(
            fft_step(&[1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        )
    }

    #[test]
    fn fft_step_48226158() {
        assert_eq!(
            fft_step(&[4, 8, 2, 2, 6, 1, 5, 8]),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        )
    }

    #[test]
    fn fft_step_34040438() {
        assert_eq!(
            fft_step(&[3, 4, 0, 4, 0, 4, 3, 8]),
            vec![0, 3, 4, 1, 5, 5, 1, 8]
        )
    }

    #[test]
    fn fft_step_03415518() {
        assert_eq!(
            fft_step(&[0, 3, 4, 1, 5, 5, 1, 8]),
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        )
    }

    #[test]
    fn fft_phases_12345678() {
        let mut phases: FFTPhases = vec![1, 2, 3, 4, 5, 6, 7, 8].into();

        assert_eq!(phases.next(), Some(vec![4, 8, 2, 2, 6, 1, 5, 8]));

        assert_eq!(phases.next(), Some(vec![3, 4, 0, 4, 0, 4, 3, 8]));

        assert_eq!(phases.next(), Some(vec![0, 3, 4, 1, 5, 5, 1, 8]));

        assert_eq!(phases.next(), Some(vec![0, 1, 0, 2, 9, 4, 9, 8]));
    }

    #[test]
    fn fft_phases_80871224585914546619083218645595() {
        let mut phases: FFTPhases = "80871224585914546619083218645595".try_into().unwrap();

        assert_eq!(
            phases
                .nth(99)
                .unwrap()
                .into_iter()
                .take(8)
                .collect::<Vec<_>>(),
            vec![2, 4, 1, 7, 6, 1, 7, 6]
        );
    }

    #[test]
    fn fft_phases_19617804207202209144916044189917() {
        let mut phases: FFTPhases = "19617804207202209144916044189917".try_into().unwrap();

        assert_eq!(
            phases
                .nth(99)
                .unwrap()
                .into_iter()
                .take(8)
                .collect::<Vec<_>>(),
            vec![7, 3, 7, 4, 5, 4, 1, 8]
        );
    }

    #[test]
    fn fft_phases_69317163492948606335995924319873() {
        let mut phases: FFTPhases = "69317163492948606335995924319873".try_into().unwrap();

        assert_eq!(
            phases
                .nth(99)
                .unwrap()
                .into_iter()
                .take(8)
                .collect::<Vec<_>>(),
            vec![5, 2, 4, 3, 2, 1, 3, 3]
        );
    }
}
