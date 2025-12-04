use std::ops::RangeInclusive;

use crate::Solve;

pub struct Puzzle {
    ranges: Vec<RangeInclusive<isize>>,
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        Puzzle {
            ranges: self
                .split(",")
                .map(|range| {
                    let (lhs, rhs) = range.split_once("-").unwrap();

                    let lhs: isize = lhs.trim().parse().unwrap();
                    let rhs: isize = rhs.trim().parse().unwrap();

                    lhs..=rhs
                })
                .collect(),
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        let mut sum = 0;
        for range in &self.ranges {
            for n in range.clone() {
                let str = n.to_string();
                let len = str.len();
                if len % 2 == 1 {
                    continue;
                }
                let mid = (len + 1) / 2;
                let lhs = &str[..mid];
                let rhs = &str[mid..];

                if lhs == rhs {
                    sum += n
                }
            }
        }
        sum
    }

    fn solve2(&self) -> isize {
        let mut sum = 0;
        for range in &self.ranges {
            for n in range.clone() {
                let str = n.to_string();
                let len = str.len();
                let mid = (len + 1) / 2;
                for l in 1..=mid {
                    if len % l == 0 {
                        let mut vec = Vec::with_capacity(len / l);

                        for i in (0..len).step_by(l) {
                            vec.push(&str[i..i + l]);
                        }

                        if vec.len() >= 2 && vec.windows(2).all(|window| window[0] == window[1]) {
                            sum += n;
                            break;
                        }
                    }
                }
            }
        }
        sum
    }
}
