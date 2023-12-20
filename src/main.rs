use clap::Parser;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod map;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, action)]
    all: bool,

    #[arg(short, long, action)]
    example: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    println!("Starting...");

    let suffix = if args.example { "ex.txt" } else { ".txt" };

    if args.all {
        day1::solve()?;
        day2::solve()?;
        day3::solve()?;
        day4::solve()?;
        day5::solve()?;
        day6::solve()?;
        day7::solve()?;
        day8::solve()?;
        day9::solve()?;
        day10::solve()?;
        day11::solve()?;
        day12::solve(&args)?;
        day13::solve()?;
        day14::solve()?;
        day15::solve()?;
        day16::solve()?;
        day17::solve(&suffix)?;

        day19::solve()?;
        day20::solve()?;
        day21::solve()?;
        day22::solve()?;
        day23::solve()?;
        day24::solve()?;
        day25::solve()?;
    }

    day18::solve(&suffix)?;

    println!("Finished!");

    Ok(())
}
