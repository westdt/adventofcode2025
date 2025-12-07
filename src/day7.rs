use std::collections::HashMap;

use crate::Solve;

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Empty,
    Start,
    Pipe,
    Splitter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle {
    manifold: Vec<Vec<State>>,
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        Puzzle {
            manifold: self
                .lines()
                .map(|row| {
                    row.chars()
                        .map(|element| match element {
                            'S' => State::Start,
                            '^' => State::Splitter,
                            _ => State::Empty,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        let mut sum = 0;
        let mut beams = Vec::new();
        for row in self.manifold.iter() {
            for (i, element) in row.iter().enumerate() {
                beams.sort();
                beams.dedup();

                if element == &State::Start {
                    beams.push(i);
                } else if element == &State::Splitter && beams.contains(&i) {
                    let index = beams
                        .iter()
                        .enumerate()
                        .find(|(_, e)| **e == i)
                        .unwrap()
                        .0
                        .clone();
                    beams.remove(index);

                    if i > 1 {
                        let lhs = i - 1;
                        beams.push(lhs);
                    }

                    let rhs = i + 1;
                    beams.push(rhs);

                    sum += 1;
                }
            }
        }

        sum
    }

    fn solve2(&self) -> isize {
        let mut beams: HashMap<usize, isize> = HashMap::new();
        for (_, row) in self.manifold.iter().enumerate() {
            for (i, element) in row.iter().enumerate() {
                if element == &State::Start {
                    beams.insert(i, 1);
                } else if element == &State::Splitter {
                    if let Some(beam) = beams.get(&i) {
                        let count = beam.clone();
                        beams.remove(&i);

                        let lhs = i - 1;
                        let rhs = i + 1;
                        
                        if let Some(beam) = beams.get_mut(&lhs) {
                            *beam += count;
                        } else {
                            beams.insert(lhs, count);
                        }
                        
                        if let Some(beam) = beams.get_mut(&rhs) {
                            *beam += count;
                        } else {
                            beams.insert(rhs, count);
                        }
                    }
                }
            }
        }

        beams.iter().fold(0, |acc, (_, count)| acc + count) as isize
    }
}
