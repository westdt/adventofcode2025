use crate::Solve;

pub struct Puzzle {
    fresh_ingredients: Vec<(isize, isize)>,
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

                        Some((lhs, rhs))
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
                if (range.0..=range.1).contains(&ingredient) {
                    sum += 1;
                    break;
                }
            }
        }
        sum
    }

    fn solve2(&self) -> isize {
        let mut ingredients = self.fresh_ingredients.clone();
        ingredients.sort();

        // Had to compare my own code to myleshyson's code to get this working correct
        // as I was struggling with how to merge the ranges. Turns out I was overthinking it
        //
        // https://github.com/myleshyson/advent-of-code/blob/main/2025/src/bin/day5.rs
        ingredients
            .iter()
            .fold(Vec::new(), | mut acc: Vec<(isize, isize)>, &range| {
                match acc.last_mut() {
                    Some(last) => {
                        if range.0 <= last.1 {
                            last.1 = last.1.max(range.1)
                        } else {
                            acc.push(range);
                        }
                        acc
                    }
                    _ => {
                        acc.push(range);
                        acc
                    }
                }
            })
            .iter()
            .map(|(min, max)| (max - min) + 1)
            .sum()
    }
}
