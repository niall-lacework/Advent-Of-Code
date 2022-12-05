use clap::{Parser, Subcommand};

use solution::Answer;
mod solution;
mod solutions;

#[derive(Parser)]
#[command(name = "advent_of_code")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run all the solutions to all the problems")]
    All { year: Option<u32> },
    #[command(about = "Run a solution to a problem")]
    Run {
        day: u32,
        part: char,
        year: Option<u32>,
    },
    #[command(about = "List all solutions for a given year")]
    List { year: Option<u32> },
}

const DEFAULT_YEAR: u32 = 2022;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::All {year} => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = solutions::get_year(year);

            for (i, e) in solutions.iter().enumerate() {
                let day = (i + 1).try_into().unwrap();
                let input = solution::load(year, day, "input.txt");
                
                println!("Running: {}. {}", day, e.name());
                
                let result_a = e.part_a(&input);
                println!("[+] RESULT A: {result_a}");
                
                let result_b = e.part_b(&input);
                println!("[+] RESULT B: {result_b}");
                println!()
            }
        }
        Commands::Run { year, day, part } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let day_index = day.saturating_sub(1);

            let solutions = solutions::get_year(year);
            let solution = match solutions.get(day_index as usize) {
                Some(s) => s,
                None => {
                    println!("No solution for day {} in year {}", day, year);
                    return;
                }
            };

            println!("[*] Running: {} ({})", solution.name(), part.to_uppercase());

            let input = solution::load(year, day, "input.txt");
            let result = match part.to_lowercase().to_string().as_str() {
                "a" => solution.part_a(&input),
                "b" => solution.part_b(&input),
                _ => return println!("[-] Invalid Part {}", part),
            };

            println!("[+] RESULT: {result}");
        }
        Commands::List { year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = solutions::get_year(year);
            println!("[*] Solutions for {year}:");

            for (i, e) in solutions.iter().enumerate() {
                println!(
                    " {} Day {}: {}",
                    if i + 1 == solutions.len() {
                        "└"
                    } else {
                        "├"
                    },
                    i + 1,
                    e.name()
                );
            }
        }
    }
}
