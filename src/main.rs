mod days;
mod utils;

use days::*;
use std::{
    fmt::Display,
    fs,
    time::{Duration, Instant},
};

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<u8>,
}

struct DayResult {
    part_one_solution: String,
    part_two_solution: String,
    part_one_duration: Duration,
    part_two_duration: Duration,
}

type PuzzleSolver<T> = fn(&str) -> Option<T>;

impl DayResult {
    fn from_solvers<T: Display>(
        input: &str,
        part_one: PuzzleSolver<T>,
        part_two: PuzzleSolver<T>,
    ) -> Self {
        let time_one = Instant::now();
        let part_one_solution = part_one(input).map_or("-".to_string(), |val| val.to_string());
        let part_one_duration = time_one.elapsed();

        let time_two = Instant::now();
        let part_two_solution = part_two(input).map_or("-".to_string(), |val| val.to_string());
        let part_two_duration = time_two.elapsed();

        Self {
            part_one_solution,
            part_two_solution,
            part_one_duration,
            part_two_duration,
        }
    }
}

fn get_day_result(input: &str, day: u8) -> DayResult {
    match day {
        1 => DayResult::from_solvers(input, day01::part_one, day01::part_two),
        2 => DayResult::from_solvers(input, day02::part_one, day02::part_two),
        3 => DayResult::from_solvers(input, day03::part_one, day03::part_two),
        4 => DayResult::from_solvers(input, day04::part_one, day04::part_two),
        5 => DayResult::from_solvers(input, day05::part_one, day05::part_two),
        _ => panic!("Couldn't run day {day}. "),
    }
}

fn run_day(day: u8) -> std::io::Result<String> {
    let input = fs::read_to_string(format!("src/days/day{:02}/input.txt", day))?;
    let res = get_day_result(&input, day);

    println!("{}", format!("Day {day}").bold().bright_blue());

    let formatted_one_solution = res.part_one_solution.yellow();
    let formatted_two_solution = res.part_two_solution.yellow();

    let part_one_dur = res.part_one_duration.as_micros() as f32 / 1000.0;
    let part_two_dur = res.part_two_duration.as_micros() as f32 / 1000.0;

    let formatted_one_time = format!("({part_one_dur} ms)").dimmed();
    let formatted_two_time = format!("({part_two_dur} ms)").dimmed();

    println!("Part 1: {formatted_one_solution} {formatted_one_time}");
    println!("Part 2: {formatted_two_solution} {formatted_two_time}\n");

    Ok(format!("Ran day {day}"))
}

fn main() {
    println!("\n{}\n", "✨ Advent of Code 2022 ✨".bold().yellow());
    if let Some(day) = Args::parse().day {
        run_day(day).expect("Couldn't find input");
    } else {
        println!("Running all solved puzzles.\n");
        for day in 0..=25 {
            let _ = run_day(day);
        }
    }
}
