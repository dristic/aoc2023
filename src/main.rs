use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod map;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Starting...");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--all" {
        day1::solve()?;
        day2::solve()?;
        day3::solve()?;
        day4::solve()?;
    }

    day5::solve()?;
    println!("Finished!");

    Ok(())
}
