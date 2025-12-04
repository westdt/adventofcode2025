use std::collections::HashMap;

use crate::Solve;

pub struct Puzzle {
    grid: Vec<Vec<bool>>,
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        Puzzle {
            grid: self
                .lines()
                .map(|row| {
                    row.chars()
                        .map(|char| match char {
                            '@' => true,
                            _ => false,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Puzzle {
    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        match self.grid.get(y as usize) {
            Some(row) => match row.get(x as usize) {
                Some(value) => *value,
                None => false,
            },
            None => false,
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        let mut sum = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value {
                    let mut neighbors = 0;

                    'outer: for i in -1..=1 {
                        for j in -1..=1 {
                            if i == j && i == 0 {
                                continue;
                            }

                            if self.get(x as isize + i, y as isize + j) {
                                neighbors += 1;

                                if neighbors >= 4 {
                                    break 'outer;
                                }
                            }
                        }
                    }

                    if neighbors < 4 {
                        sum += 1;
                    }
                }
            }
        }
        sum
    }

    fn solve2(&self) -> isize {
        let mut full_sum = 0;
        let mut removed = HashMap::new();
        let mut removed_queue = HashMap::new();

        loop {
            let mut sum = 0;
            for (y, row) in self.grid.iter().enumerate() {
                for (x, value) in row.iter().enumerate() {
                    if *value && removed.get(&(x as isize, y as isize)).is_none() {
                        let mut neighbors = 0;

                        'outer: for i in -1..=1 {
                            for j in -1..=1 {
                                if i == j && i == 0 {
                                    continue;
                                }

                                if self.get(x as isize + i, y as isize + j) && removed.get(&(x as isize + i, y as isize + j)).is_none() {
                                    neighbors += 1;

                                    if neighbors >= 4 {
                                        break 'outer;
                                    }
                                }
                            }
                        }

                        if neighbors < 4 {
                            sum += 1;
                            removed_queue.insert((x as isize, y as isize), ());
                        }
                    }
                }
            }
            full_sum += sum;
            removed_queue.drain().for_each(|(k, v)| {removed.insert(k, v);});

            if sum == 0 {
                break;
            }
        }

        full_sum
    }
}
