use crate::Solve;

#[derive(Clone, Debug)]
pub enum Value {
    Number(isize),
    Blank,
    Add,
    Mul,
}

pub enum Operator {
    Add,
    Mul,
}

pub struct Problem {
    numbers: Vec<isize>,
    operator: Operator,
}

pub struct Puzzle {
    problems: Vec<Problem>,
    columns: Vec<Vec<Value>>,
}

impl Into<Puzzle> for String {
    fn into(self) -> Puzzle {
        let mut vec: Vec<Vec<String>> = Vec::new();
        for line in self.lines() {
            for (index, element) in line.split_whitespace().enumerate() {
                if let Some(vec) = vec.get_mut(index) {
                    vec.push(element.trim().to_string());
                } else {
                    vec.push(vec![element.trim().to_string()]);
                }
            }
        }

        let mut columns: Vec<Vec<Value>> = Vec::new();
        for (_, line) in self.lines().enumerate() {
            for (index, char) in line.chars().enumerate() {
                let value = if let Some(number) = char.to_digit(10) {
                    Value::Number(number as isize)
                } else if char == '*' {
                    Value::Mul
                } else if char == '+' {
                    Value::Add
                } else {
                    Value::Blank
                };

                if let Some(column) = columns.get_mut(index) {
                    column.push(value);
                } else {
                    columns.push(vec![value]);
                }
            }
        }

        Puzzle {
            problems: vec
                .into_iter()
                .map(|problem: Vec<String>| {
                    let mut operator = None;
                    let mut numbers = Vec::new();

                    for element in problem {
                        if let Ok(number) = element.parse() {
                            numbers.push(number);
                        } else if element == "*" {
                            operator = Some(Operator::Mul);
                        } else if element == "+" {
                            operator = Some(Operator::Add);
                        }
                    }

                    Problem {
                        numbers,
                        operator: operator.unwrap(),
                    }
                })
                .collect(),
            columns,
        }
    }
}

impl Solve<isize> for Puzzle {
    fn solve1(&self) -> isize {
        self.problems
            .iter()
            .map(|problem| {
                let mut numbers = problem.numbers.clone();
                let last = numbers.pop().unwrap();
                numbers
                    .iter()
                    .fold(last, |acc, element| match problem.operator {
                        Operator::Add => acc + element,
                        Operator::Mul => acc * element,
                    })
            })
            .sum()
    }

    fn solve2(&self) -> isize {
        // This solution is awful and I am not proud of it
        let mut sum = 0;
        let mut numbers: Vec<isize> = Vec::new();
        let mut current_column = 0;
        for column in self.columns.iter().rev() {
            for (_, value) in column.iter().enumerate() {
                match value {
                    Value::Number(num) => {
                        if let Some(number) = numbers.get_mut(current_column as usize) {
                            *number = (*number * 10) + num;
                        } else {
                            numbers.push(*num);
                        }
                    }
                    Value::Blank => {
                        if numbers.get(current_column as usize).is_none() {
                            numbers.push(0);
                        }
                    }
                    Value::Add => {sum += numbers.iter().fold(0, |acc, element| acc + element); numbers.clear(); current_column = 0},
                    Value::Mul => {sum += numbers.iter().fold(1, |acc, element| acc * element.max(&1)); numbers.clear(); current_column = 0},
                }
            }
            current_column += 1;
        }

        sum
    }
}
