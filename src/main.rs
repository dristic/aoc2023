mod day1;
mod day2;
mod day3;

mod map;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Starting...");

    // day1::solve()?;
    // day2::solve()?;
    day3::solve()?;

    println!("Finished!");

    Ok(())
}
