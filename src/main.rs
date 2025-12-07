#![allow(dead_code)]

use std::{
    collections::VecDeque,
    env,
    fs::{create_dir, read_dir, read_to_string},
    io::{Write, stdin, stdout},
    process::exit,
    time::Duration,
};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const ITALIC: &str = "\x1b[3m";
const BOLD_ITALIC: &str = "\x1b[1;3m";
const RED: &str = "\x1b[31m";
const RED_BOLD: &str = "\x1b[1;31m";
const RED_ITALIC: &str = "\x1b[3;31m";
const RED_BOLD_ITALIC: &str = "\x1b[1;3;31m";
const GREEN: &str = "\x1b[32m";
const GREEN_BOLD: &str = "\x1b[1;32m";
const GREEN_ITALIC: &str = "\x1b[3;32m";
const GREEN_BOLD_ITALIC: &str = "\x1b[1;3;32m";
const YELLOW: &str = "\x1b[33m";
const YELLOW_BOLD: &str = "\x1b[1;33m";
const YELLOW_ITALIC: &str = "\x1b[3;33m";
const YELLOW_BOLD_ITALIC: &str = "\x1b[1;3;33m";
const BLUE: &str = "\x1b[34m";
const BLUE_BOLD: &str = "\x1b[1;34m";
const BLUE_ITALIC: &str = "\x1b[3;34m";
const BLUE_BOLD_ITALIC: &str = "\x1b[1;3;34m";
const MAGENTA: &str = "\x1b[35m";
const MAGENTA_BOLD: &str = "\x1b[1;35m";
const MAGENTA_ITALIC: &str = "\x1b[3;35m";
const MAGENTA_BOLD_ITALIC: &str = "\x1b[1;3;35m";
const CYAN: &str = "\x1b[36m";
const CYAN_BOLD: &str = "\x1b[1;36m";
const CYAN_ITALIC: &str = "\x1b[3;36m";
const CYAN_BOLD_ITALIC: &str = "\x1b[1;3;36m";
const WHITE: &str = "\x1b[37m";
const WHITE_BOLD: &str = "\x1b[1;37m";
const WHITE_ITALIC: &str = "\x1b[3;37m";
const WHITE_BOLD_ITALIC: &str = "\x1b[1;3;37m";
const MIN_DAY: usize = 1;
const MAX_DAY: usize = 7;

enum Part {
    One,
    Two,
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

trait Solve<T: ToString> {
    fn solve(&self, part: Part) -> (String, f64) {
        let timer = std::time::Instant::now();
        (
            match part {
                Part::One => self.solve1().to_string(),
                Part::Two => self.solve2().to_string(),
            },
            timer.elapsed().as_secs_f64(),
        )
    }

    fn solve1(&self) -> T;
    fn solve2(&self) -> T;
}

fn get_day_from_str(day: &str) -> usize {
    match day.parse() {
        Ok(day) => match day {
            MIN_DAY..=MAX_DAY => day,
            _ => {
                println!(
                    "{RED_BOLD}Oops!{RESET} You entered an invalid day. Day must be an integer between {GREEN_BOLD}{MIN_DAY}{RESET}-{GREEN_BOLD}{MAX_DAY}{RESET}."
                );
                exit(1)
            }
        },
        _ => {
            println!(
                "{RED_BOLD}Oops!{RESET} You entered an invalid day. Day must be an integer between {GREEN_BOLD}{MIN_DAY}{RESET}-{GREEN_BOLD}{MAX_DAY}{RESET}."
            );
            exit(1)
        }
    }
}

fn get_part_from_str(part: &str) -> Part {
    match part {
        "1" => Part::One,
        "2" => Part::Two,
        _ => {
            println!(
                "{RED_BOLD}Oops!{RESET} You entered an invalid part. Part must be either {MAGENTA_BOLD}1{RESET} or {MAGENTA_BOLD}2{RESET}."
            );
            exit(1)
        }
    }
}

fn main() {
    let mut args = env::args().collect::<VecDeque<_>>();
    args.pop_front();

    let mut day = None;
    if let Some(arg) = args.pop_front() {
        day = Some(get_day_from_str(&arg));
    };

    let mut part = None;
    if let Some(arg) = args.pop_front() {
        part = Some(get_part_from_str(&arg));
    };

    let mut puzzle = args.pop_front();

    if day.is_none() || part.is_none() || puzzle.is_none() {
        println!("{}", "\n".repeat(100));
        println!("Welcome to Wesley's {YELLOW_BOLD}Advent of Code 2025{RESET} project!");
    }

    if day.is_none() {
        println!("Please enter a {GREEN_BOLD}number{RESET} to select the day.");
        println!("\t{GREEN_BOLD}1{RESET}\t{BOLD}Day 1{RESET}: Secret Entrance");
        println!("\t{GREEN_BOLD}2{RESET}\t{BOLD}Day 2{RESET}: Gift Shop");
        println!("\t{GREEN_BOLD}3{RESET}\t{BOLD}Day 3{RESET}: Lobby");
        println!("\t{GREEN_BOLD}4{RESET}\t{BOLD}Day 4{RESET}: Printing Department");
        println!("\t{GREEN_BOLD}5{RESET}\t{BOLD}Day 5{RESET}: Cafeteria");
        println!("\t{GREEN_BOLD}6{RESET}\t{BOLD}Day 6{RESET}: Trash Compactor");
        println!("\t{GREEN_BOLD}7{RESET}\t{BOLD}Day 7{RESET}: Laboratories");

        print!("... ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        day = Some(get_day_from_str(input.trim()));
    }
    let day = day.unwrap();

    if part.is_none() {
        println!("Please enter a {MAGENTA_BOLD}number{RESET} to select the part.");
        println!("\t{MAGENTA_BOLD}1{RESET}\t{BOLD}Part 1{RESET}");
        println!("\t{MAGENTA_BOLD}2{RESET}\t{BOLD}Part 2{RESET}");

        print!("... ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        part = Some(get_part_from_str(input.trim()));
    }
    let part = part.unwrap();

    if puzzle.is_none() {
        println!(
            "Please enter the desired {CYAN_BOLD}puzzle input filepath{RESET} or a {CYAN_BOLD}number{RESET} to select from the list below."
        );

        let mut options: Vec<String> = Vec::new();
        if let Ok(puzzles) = read_dir("./puzzles") {
            let mut puzzles = puzzles.collect::<Vec<_>>();
            puzzles.sort_by(|a, b| {
                if a.is_err() && b.is_err() {
                    std::cmp::Ordering::Equal
                } else if b.is_err() {
                    std::cmp::Ordering::Greater
                } else if a.is_err() {
                    std::cmp::Ordering::Less
                } else {
                    if a.as_ref().unwrap().file_name() == b.as_ref().unwrap().file_name() {
                        std::cmp::Ordering::Equal
                    } else if a.as_ref().unwrap().file_name() > b.as_ref().unwrap().file_name() {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }
            });

            for (index, puzzle) in puzzles.iter().enumerate() {
                if let Ok(puzzle) = puzzle {
                    let name = puzzle.file_name();
                    let name = name.to_string_lossy().to_string();

                    let mut nums = Vec::new();
                    let mut num_str = String::new();
                    for char in name.chars() {
                        if char.is_numeric() {
                            num_str.push(char);
                        } else {
                            if let Ok(num) = num_str.parse() {
                                nums.push(num);
                                num_str = String::new();
                            }
                        }
                    }

                    if nums.contains(&day) {
                        println!(
                            "\t{CYAN_BOLD}{}{RESET}\tpuzzles/{CYAN_BOLD}{name}{RESET}",
                            index + 1
                        );
                    } else {
                        println!("\t{CYAN_BOLD}{}{RESET}\tpuzzles/{name}", index + 1);
                    }

                    options.push(name);
                }
            }
        } else {
            let _ = create_dir("./puzzles");
        }

        print!("... ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        puzzle = Some(input.trim().to_string());
        if let Ok(number) = input.trim().parse::<usize>() {
            if let Some(option) = options.get(number - 1) {
                puzzle = Some(format!("puzzles/{}", option));
            }
        }
    }
    print!("Thinking");
    let puzzle = puzzle.unwrap();
    let puzzle = match read_to_string(&puzzle) {
        Ok(puzzle) => puzzle,
        _ => {
            println!(
                "\r{RED_BOLD}Oops!{RESET} I wasn't able to read that file. Please check and make sure the filepath is correct!"
            );
            exit(1)
        }
    };

    std::thread::spawn(|| {
        const N: usize = 5;
        let mut n = 2;
        loop {
            print!("\rThinking{}{}", ".".repeat(n), " ".repeat((N - 1) - n));
            stdout().flush().unwrap();
            n = (n + 1) % N;
            std::thread::sleep(Duration::from_millis(700));
        }
    });

    let (result, time) = match day {
        1 => Into::<day1::Puzzle>::into(puzzle).solve(part),
        2 => Into::<day2::Puzzle>::into(puzzle).solve(part),
        3 => Into::<day3::Puzzle>::into(puzzle).solve(part),
        4 => Into::<day4::Puzzle>::into(puzzle).solve(part),
        5 => Into::<day5::Puzzle>::into(puzzle).solve(part),
        6 => Into::<day6::Puzzle>::into(puzzle).solve(part),
        7 => Into::<day7::Puzzle>::into(puzzle).solve(part),
        _ => {
            println!(
                "\r{RED_BOLD}Oops!{RESET} You entered an invalid day. Please select between {GREEN_BOLD}{MIN_DAY}{RESET}-{GREEN_BOLD}{MAX_DAY}{RESET}."
            );
            exit(1)
        }
    };
    println!(
        "\r{GREEN_BOLD_ITALIC}Eureka!{RESET} The answer is {YELLOW_BOLD}{result}{RESET}\nCompleted in {CYAN_ITALIC}{time}s{RESET}"
    );
}
