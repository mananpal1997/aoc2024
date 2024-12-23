use std::fs;
use std::time::Instant;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Manan", version = "1.0", about = "Advent of Code Solver", long_about = None)]
struct Cli {
    #[arg(long)]
    day: u32,

    #[arg(long)]
    part: u32,
}

fn main() {
    let args = Cli::parse();

    let input_file = format!("days/day{}/input/part{}.txt", args.day, args.part);

    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Could not read input file: {}", input_file));


    let now = Instant::now();
    match (args.day, args.part) {
        (1, 1) => println!("{}", day1::part1::solve(&input)),
        (1, 2) => println!("{}", day1::part2::solve(&input)),
        (2, 1) => println!("{}", day2::part1::solve(&input)),
        (2, 2) => println!("{}", day2::part2::solve(&input)),
        (3, 1) => println!("{}", day3::part1::solve(&input)),
        (3, 2) => println!("{}", day3::part2::solve(&input)),
        (4, 1) => println!("{}", day4::part1::solve(&input)),
        (4, 2) => println!("{}", day4::part2::solve(&input)),
        (5, 1) => println!("{}", day5::part1::solve(&input)),
        (5, 2) => println!("{}", day5::part2::solve(&input)),
        (6, 1) => println!("{}", day6::part1::solve(&input)),
        (6, 2) => println!("{}", day6::part2::solve(&input)),
        (7, 1) => println!("{}", day7::part1::solve(&input)),
        (7, 2) => println!("{}", day7::part2::solve(&input)),
        (8, 1) => println!("{}", day8::part1::solve(&input)),
        (8, 2) => println!("{}", day8::part2::solve(&input)),
        (9, 1) => println!("{}", day9::part1::solve(&input)),
        (9, 2) => println!("{}", day9::part2::solve(&input)),
        (10, 1) => println!("{}", day10::part1::solve(&input)),
        (10, 2) => println!("{}", day10::part2::solve(&input)),
        (11, 1) => println!("{}", day11::part1::solve(&input)),
        (11, 2) => println!("{}", day11::part2::solve(&input)),
        (12, 1) => println!("{}", day12::part1::solve(&input)),
        (12, 2) => println!("{}", day12::part2::solve(&input)),
        (13, 1) => println!("{}", day13::part1::solve(&input)),
        (13, 2) => println!("{}", day13::part2::solve(&input)),
        (14, 1) => println!("{}", day14::part1::solve(&input)),
        (14, 2) => println!("{}", day14::part2::solve(&input)),
        (15, 1) => println!("{}", day15::part1::solve(&input)),
        (15, 2) => println!("{}", day15::part2::solve(&input)),
        (16, 1) => println!("{}", day16::part1::solve(&input)),
        (16, 2) => println!("{}", day16::part2::solve(&input)),
        (17, 1) => println!("{}", day17::part1::solve(&input)),
        (17, 2) => println!("{}", day17::part2::solve(&input)),
        (18, 1) => println!("{}", day18::part1::solve(&input)),
        (18, 2) => println!("{}", day18::part2::solve(&input)),
        (19, 1) => println!("{}", day19::part1::solve(&input)),
        (19, 2) => println!("{}", day19::part2::solve(&input)),
        (20, 1) => println!("{}", day20::part1::solve(&input)),
        (20, 2) => println!("{}", day20::part2::solve(&input)),
        (21, 1) => println!("{}", day21::part1::solve(&input)),
        (21, 2) => println!("{}", day21::part2::solve(&input)),
        (22, 1) => println!("{}", day22::part1::solve(&input)),
        (22, 2) => println!("{}", day22::part2::solve(&input)),
        (23, 1) => println!("{}", day23::part1::solve(&input)),
        (23, 2) => println!("{}", day23::part2::solve(&input)),
        // Add more match arms for other days and parts
        _ => eprintln!("Invalid day or part"),
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}