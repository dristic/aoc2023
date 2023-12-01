mod day1;

fn main() -> std::io::Result<()> {
    println!("Starting...");

    day1::solve()?;

    println!("Finished!");

    Ok(())
}
