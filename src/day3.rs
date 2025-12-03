use crate::Solve;

type Bank = Vec<isize>;

pub struct Puzzle {
    banks: Vec<Bank>,
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        Puzzle {
            banks: self
                .lines()
                .map(|bank| {
                    bank.chars()
                        .map(|joltage| joltage.to_digit(10).unwrap() as isize)
                        .collect()
                })
                .collect(),
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        let mut sum = 0;
        for bank in &self.banks {
            let len = bank.len();
            let (lhs, lhs_idx) = bank.iter().enumerate().fold(
                (0, 0),
                |(max_joltage, joltage_idx), (idx, joltage)| {
                    if *joltage > max_joltage && idx < len - 1 {
                        (*joltage, idx)
                    } else {
                        (max_joltage, joltage_idx)
                    }
                },
            );

            let (rhs, _) = bank.iter().enumerate().fold(
                (0, 0),
                |(max_joltage, joltage_idx), (idx, joltage)| {
                    if *joltage > max_joltage && idx > lhs_idx {
                        (*joltage, idx)
                    } else {
                        (max_joltage, joltage_idx)
                    }
                },
            );

            sum += lhs * 10 + rhs;
        }
        sum
    }

    fn solve2(&self) -> isize {
        const COUNT: usize = 12;

        let mut sum = 0;
        for bank in &self.banks {
            let len = bank.len();

            let mut joltages = Vec::with_capacity(COUNT);
            let mut max_idx = None;
            for i in 0..COUNT {
                let (joltage, idx) = bank.iter().enumerate().fold(
                    (0, 0),
                    |(max_joltage, joltage_idx), (idx, joltage)| {
                        if *joltage > max_joltage && idx < len - (COUNT - (i + 1)) && max_idx.is_none_or(|max_idx| idx > max_idx) {
                            (*joltage, idx)
                        } else {
                            (max_joltage, joltage_idx)
                        }
                    },
                );

                max_idx = Some(idx);
                joltages.push(joltage);
            }

            for i in 0..COUNT {
                sum += joltages[i] * 10_isize.pow((COUNT - (i + 1)) as u32);
            }
        }
        sum
    }
}
