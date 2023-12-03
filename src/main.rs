mod day1;
mod day2;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Starting...");

    // day1::solve()?;
    day2::solve()?;

    println!("Finished!");

    Ok(())
}
