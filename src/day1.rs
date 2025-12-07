use crate::Solve;

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Clone, Debug)]
struct Step {
    direction: Direction,
    amount: isize
}

pub struct Puzzle {
    steps: Vec<Step>
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        Puzzle {
            steps: self.lines().map(|step| {
                let direction = match step.chars().nth(0) {
                    Some('L') => Direction::Left,
                    Some('R') => Direction::Right,
                    _ => {panic!()},
                };
                let amount: isize = step[1..step.len()].parse().unwrap();
                
                Step {
                    direction,
                    amount,
                }
            }).collect()
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        let mut code = 0;
        let mut dial = 50;
        for step in &self.steps {
            let amount = step.amount % 100;

            dial = match step.direction {
                Direction::Left => dial + (100 - amount),
                Direction::Right => dial + amount,
            } % 100;
            
            if dial == 0 {
                code += 1;
            }
        }
        code
    }

    fn solve2(&self) -> isize {
        let mut code = 0;
        let mut dial = 50;
        for step in &self.steps {
            // println!("{step:?}");
            let amount = step.amount % 100;
            dial = match step.direction {
                Direction::Left => {
                    let add = if dial == 0 {
                        dial - step.amount
                    } else {
                        dial - (step.amount + 100)
                    } / -100;
                    
                    code += add;
                    dial + (100 - amount)
                },
                Direction::Right => {
                    let add = (dial + step.amount) / 100;
                    
                    code += add;
                    dial + amount
                },
            } % 100;
        }
        code
    }
}