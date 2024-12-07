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
        // Add more match arms for other days and parts
        _ => eprintln!("Invalid day or part"),
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}