use std::fs;

use regex::{Match, Regex};

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d6.txt")?;

    // Input parsing.
    let mut lines = input.lines();
    let reg = Regex::new(r"\s+([0-9]+)")?;
    let match_to_u32 = |v: Match| v.as_str().trim().parse::<u32>().unwrap();
    let times: Vec<u32> = reg
        .find_iter(&lines.next().unwrap())
        .map(match_to_u32)
        .collect::<Vec<u32>>();
    let distance: Vec<u32> = reg
        .find_iter(&lines.next().unwrap())
        .map(match_to_u32)
        .collect::<Vec<u32>>();

    // Part one.
    let mut num_wins = vec![0; times.len()];
    for (i, time) in times.iter().enumerate() {
        //println!("Race {}", i+1);
        let dist = distance[i];

        for speed in 0..=*time {
            let time_left = time - speed;
            let total_dist = time_left * speed;

            if total_dist > dist {
                num_wins[i] += 1;
            }

            // println!("Speed {} dist {} wins {}", speed, total_dist, num_wins[i]);
        }
    }

    let answer: u32 = num_wins.iter().product();
    println!("Part one {}", answer);

    // Part two.
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();

    let mut num_wins = 0;
    for speed in 0..=time {
        let time_left = time - speed;
        let total_dist = time_left * speed;

        if total_dist > dist {
            num_wins += 1;
        }

        // println!("Speed {} dist {} wins {}", speed, total_dist, num_wins[i]);
    }
    println!("Part two {}", num_wins);

    Ok(())
}
