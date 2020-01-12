use std::convert::TryFrom;
use std::ops::Range;

use rayon::prelude::*;

struct FFTStepPart<'a> {
    slice: &'a [i64],
    index: usize,
    multiple: usize,
    offset: usize,
    accumulator: i64,
}

fn get_range(index: usize, offset: usize, multiple: usize) -> Range<usize> {
    let lower = (index + offset) * multiple;
    let upper = lower + index + offset;
    lower.saturating_sub(offset + 1)..upper.saturating_sub(offset + 1)
}

impl Iterator for FFTStepPart<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            return None;
        }
        let previous_range = get_range(self.index, self.offset, self.multiple);
        self.index += 1;
        let range = get_range(self.index, self.offset, self.multiple);
        if self.index != 1 {
            for x in previous_range.start..usize::min(range.start, previous_range.end) {
                if x < self.slice.len() {
                    self.accumulator -= self.slice[x]
                }
            }
            for x in usize::max(previous_range.end, range.start)..range.end {
                if x < self.slice.len() {
                    self.accumulator += self.slice[x]
                }
            }
        } else {
            for x in range.start..range.end {
                if x < self.slice.len() {
                    self.accumulator += self.slice[x]
                }
            }
        }
        Some(self.accumulator)
    }
}

fn fft_step(arr: &[i64], offset: usize) -> Vec<i64> {
    let size = arr.len() + offset;
    let mut steppers: Vec<FFTStepPart> = (1..size)
        .step_by(2)
        .map(|i| FFTStepPart {
            slice: arr,
            accumulator: 0,
            index: 0,
            offset,
            multiple: i,
        })
        .collect();
    (0..arr.len())
        .map(|_| {
            while !steppers.is_empty() {
                let last_stepper = &steppers[steppers.len() - 1];
                if get_range(
                    last_stepper.index,
                    last_stepper.offset,
                    last_stepper.multiple,
                )
                .start
                    < arr.len()
                {
                    break;
                }
                steppers.pop().unwrap();
            }
            steppers
                .par_iter_mut()
                .enumerate()
                .map(|(i, stepper)| {
                    if i % 2 == 0 {
                        stepper.next().unwrap()
                    } else {
                        -stepper.next().unwrap()
                    }
                })
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect()
}

pub struct FFTPhases {
    pub current: Vec<i64>,
    pub offset: usize,
}

impl<T> From<(Vec<T>, usize)> for FFTPhases
where
    i64: From<T>,
{
    fn from((current, offset): (Vec<T>, usize)) -> FFTPhases {
        FFTPhases {
            current: current.into_iter().map(|e| e.into()).collect(),
            offset,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidDigit(char),
}

impl TryFrom<(&str, usize)> for FFTPhases {
    type Error = ParseError;

    fn try_from((string, offset): (&str, usize)) -> Result<FFTPhases, ParseError> {
        Ok((
            string
                .chars()
                .map(|c| c.to_digit(10).ok_or_else(|| ParseError::InvalidDigit(c)))
                .collect::<Result<Vec<u32>, _>>()?,
            offset,
        )
            .into())
    }
}

impl Iterator for FFTPhases {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = fft_step(&self.current, self.offset);
        Some(self.current.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_get_range() {
        assert_eq!(get_range(0, 0, 1), 0..0);
        assert_eq!(get_range(1, 0, 1), 0..1);
        assert_eq!(get_range(2, 0, 1), 1..3);
        assert_eq!(get_range(3, 0, 1), 2..5);
        assert_eq!(get_range(4, 0, 1), 3..7);
        assert_eq!(get_range(5, 0, 1), 4..9);
        assert_eq!(get_range(6, 0, 1), 5..11);
        assert_eq!(get_range(7, 0, 1), 6..13);
        assert_eq!(get_range(8, 0, 1), 7..15);

        assert_eq!(get_range(0, 1, 1), 0..0);
        assert_eq!(get_range(1, 1, 1), 0..2);
        assert_eq!(get_range(2, 1, 1), 1..4);
        assert_eq!(get_range(3, 1, 1), 2..6);
        assert_eq!(get_range(4, 1, 1), 3..8);
        assert_eq!(get_range(5, 1, 1), 4..10);
        assert_eq!(get_range(6, 1, 1), 5..12);
        assert_eq!(get_range(7, 1, 1), 6..14);
        assert_eq!(get_range(8, 1, 1), 7..16);

        assert_eq!(get_range(0, 0, 3), 0..0);
        assert_eq!(get_range(1, 0, 3), 2..3);
        assert_eq!(get_range(2, 0, 3), 5..7);
        assert_eq!(get_range(3, 0, 3), 8..11);
        assert_eq!(get_range(4, 0, 3), 11..15);
        assert_eq!(get_range(5, 0, 3), 14..19);
        assert_eq!(get_range(6, 0, 3), 17..23);
        assert_eq!(get_range(7, 0, 3), 20..27);
        assert_eq!(get_range(8, 0, 3), 23..31);
        assert_eq!(get_range(0, 1, 3), 1..2);
        assert_eq!(get_range(1, 1, 3), 4..6);
        assert_eq!(get_range(2, 1, 3), 7..10);
        assert_eq!(get_range(3, 1, 3), 10..14);
        assert_eq!(get_range(4, 1, 3), 13..18);
        assert_eq!(get_range(5, 1, 3), 16..22);
        assert_eq!(get_range(6, 1, 3), 19..26);
        assert_eq!(get_range(7, 1, 3), 22..30);
    }
    #[test]
    fn fft_step_12345678() {
        assert_eq!(
            fft_step(&[1, 2, 3, 4, 5, 6, 7, 8], 0),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        )
    }

    #[test]
    fn fft_step_12345678_offsets() {
        let input: [i64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let output: [i64; 8] = [4, 8, 2, 2, 6, 1, 5, 8];

        for skip in 1..input.len() {
            assert_eq!(
                fft_step(&input.iter().cloned().skip(skip).collect::<Vec<_>>(), skip),
                output.iter().cloned().skip(skip).collect::<Vec<_>>()
            )
        }
    }

    #[test]
    fn fft_step_48226158() {
        assert_eq!(
            fft_step(&[4, 8, 2, 2, 6, 1, 5, 8], 0),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        )
    }

    #[test]
    fn fft_step_34040438() {
        assert_eq!(
            fft_step(&[3, 4, 0, 4, 0, 4, 3, 8], 0),
            vec![0, 3, 4, 1, 5, 5, 1, 8]
        )
    }

    #[test]
    fn fft_step_03415518() {
        assert_eq!(
            fft_step(&[0, 3, 4, 1, 5, 5, 1, 8], 0),
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        )
    }

    #[test]
    fn fft_phases_12345678() {
        let mut phases: FFTPhases = (vec![1, 2, 3, 4, 5, 6, 7, 8], 0).into();

        assert_eq!(phases.next(), Some(vec![4, 8, 2, 2, 6, 1, 5, 8]));

        assert_eq!(phases.next(), Some(vec![3, 4, 0, 4, 0, 4, 3, 8]));

        assert_eq!(phases.next(), Some(vec![0, 3, 4, 1, 5, 5, 1, 8]));

        assert_eq!(phases.next(), Some(vec![0, 1, 0, 2, 9, 4, 9, 8]));
    }

    #[test]
    fn fft_phases_80871224585914546619083218645595() {
        let mut phases: FFTPhases = ("80871224585914546619083218645595", 0).try_into().unwrap();

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
        let mut phases: FFTPhases = ("19617804207202209144916044189917", 0).try_into().unwrap();

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
        let mut phases: FFTPhases = ("69317163492948606335995924319873", 0).try_into().unwrap();

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
