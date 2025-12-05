use std::ops::RangeInclusive;

use crate::Solve;

pub struct Puzzle {
    fresh_ingredients: Vec<RangeInclusive<isize>>,
    available_ingredients: Vec<isize>,
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        Puzzle {
            fresh_ingredients: self
                .lines()
                .filter_map(|line| {
                    if let Some((lhs, rhs)) = line.split_once("-") {
                        let lhs: isize = lhs.trim().parse().unwrap();
                        let rhs: isize = rhs.trim().parse().unwrap();

                        Some(lhs..=rhs)
                    } else {
                        None
                    }
                })
                .collect(),
            available_ingredients: self
                .lines()
                .filter_map(|line| {
                    if let Ok(id) = line.trim().parse() {
                        Some(id)
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        let mut sum = 0;
        for ingredient in &self.available_ingredients {
            for range in &self.fresh_ingredients {
                if range.contains(&ingredient) {
                    sum += 1;
                    break;
                }
            }
        }
        sum
    }

    fn solve2(&self) -> isize {
        let sum = self
            .fresh_ingredients
            .iter()
            .fold(0, |sum, n| sum + n.clone().count() as isize);
        let mut sub = 0;
        let len = self.fresh_ingredients.len();

        for i in 0..len {
            for j in (i + 1)..len {
                let a = &self.fresh_ingredients[i];
                let b = &self.fresh_ingredients[j];

                let min_a = a.clone().min().unwrap();
                let max_a = a.clone().max().unwrap();
                let min_b = b.clone().min().unwrap();
                let max_b = b.clone().max().unwrap();

                if max_a < min_b || min_a > max_b {
                    // ranges have no overlap
                } else if max_a >= min_b && max_a <= max_b {
                    if min_a >= min_b {
                        // range a is within range b
                        sub += a.clone().count() as isize;
                        println!("range a {a:?} within range b {b:?}");
                    } else {
                        // range a is lower than range b and they overlap
                        sub += (max_a - min_b) + 1;
                        println!("range a {a:?} lower than range b {b:?}");
                    }
                } else if max_b >= min_a && max_b <= max_a {
                    if min_b >= min_a {
                        // range b is within range a
                        sub += b.clone().count() as isize;
                        println!("range b {b:?} within range a {a:?}");
                    } else {
                        // range b is lower than range a and they overlap
                        sub += (max_b - min_a) + 1;
                        println!("range b {b:?} lower than range a {a:?}");
                    }
                } else {
                    // ranges have no overlap
                }
            }
        }

        println!("sum {sum}, sub {sub}");

        sum - sub
    }
}
