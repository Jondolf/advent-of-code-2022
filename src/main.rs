mod days;
mod utils;

use days::*;
use std::{fs, time::Instant};

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<u8>,
}

type PuzzleSolver = fn(&str) -> Option<u32>;

fn get_day_solver(day: u8) -> Option<(PuzzleSolver, PuzzleSolver)> {
    match day {
        1 => Some((day01::part_one, day01::part_two)),
        2 => Some((day02::part_one, day02::part_two)),
        3 => Some((day03::part_one, day03::part_two)),
        _ => None,
    }
}

fn run_day(day: u8) {
    if let Some((part_one, part_two)) = get_day_solver(day) {
        println!("{}", format!("Day {day}").bold().bright_blue());

        if let Ok(input) = fs::read_to_string(format!("src/days/day{:02}/input.txt", day)) {
            let time_1 = Instant::now();
            let part_1_solution = part_one(&input);
            let part_1_elapsed_ms = time_1.elapsed().as_nanos() as f64 / 1_000_000.0;

            let time_2 = Instant::now();
            let part_2_solution = part_two(&input);
            let part_2_elapsed_ms = time_2.elapsed().as_nanos() as f64 / 1_000_000.0;

            let formatted_1_solution =
                part_1_solution.map_or("-".yellow(), |s| s.to_string().yellow());
            let formatted_2_solution =
                part_2_solution.map_or("-".yellow(), |s| s.to_string().yellow());

            let formatted_1_time = format!("({part_1_elapsed_ms} ms)").dimmed();
            let formatted_2_time = format!("({part_2_elapsed_ms} ms)").dimmed();

            println!("Part 1: {formatted_1_solution} {formatted_1_time}");
            println!("Part 2: {formatted_2_solution} {formatted_2_time}\n");
        } else {
            println!("Couldn't find input for day {day}.\n");
        }
    }
}

fn main() {
    println!("\n{}\n", "✨ Advent of Code 2022 ✨".bold().yellow());
    if let Some(day) = Args::parse().day {
        run_day(day);
    } else {
        println!("Running all solved puzzles.\n");
        for day in 0..=25 {
            run_day(day);
        }
    }
}