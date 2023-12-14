use std::{collections::HashMap, fs};

use crate::Args;

#[derive(Debug)]
struct Row {
    springs: Vec<char>,
    groups: Vec<u32>,
}

impl Row {
    fn iterations(&self) -> u64 {
        let mut cache = HashMap::new();
        iter_step(&self.springs, None, &self.groups, 0, &mut cache)
    }
}

fn iter_step(
    spr: &[char],
    prev: Option<char>,
    grp: &[u32],
    grp_amt: u32,
    cache: &mut HashMap<(usize, Option<char>, usize, u32), u64>,
) -> u64 {
    if spr.len() == 0 {
        // We are at the end. Validate our final group count.
        if grp.len() > 0 {
            if grp_amt == grp[0] && grp.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        } else {
            return 1;
        }
    }

    let key = (spr.len(), prev.clone(), grp.len(), grp_amt);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let next = spr[0];
    let mut result = 0;
    if next == '.' || next == '?' {
        if let Some('#') = prev {
            // We just ended a group, so check our amount and reset.
            if grp.len() > 0 && grp_amt == grp[0] {
                result += iter_step(&spr[1..], Some('.'), &grp[1..], 0, cache);
            }
        } else {
            result += iter_step(&spr[1..], Some('.'), grp, grp_amt, cache);
        }
    }
    if next == '#' || next == '?' {
        if grp.len() > 0 && grp_amt + 1 <= grp[0] {
            result += iter_step(&spr[1..], Some('#'), grp, grp_amt + 1, cache);
        }
    }

    cache.insert(key, result);

    result
}

pub fn solve(args: &Args) -> anyhow::Result<()> {
    let src = if args.example {
        "input/d12ex.txt"
    } else {
        "input/d12.txt"
    };
    let input = fs::read_to_string(src)?;

    let rows = input
        .lines()
        .into_iter()
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            let springs = parts[0].chars().to_owned().collect();
            let groups = parts[1]
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            Row { springs, groups }
        })
        .collect::<Vec<Row>>();

    let part1 = rows.iter().map(|row| row.iterations()).sum::<u64>();

    println!("Part one {}", part1);

    let rows = input
        .lines()
        .into_iter()
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            let spr: Vec<char> = parts[0].chars().to_owned().collect();
            let grps: Vec<u32> = parts[1]
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            let mut springs = Vec::new();
            springs.append(&mut spr.clone());
            for _ in 0..4 {
                springs.push('?');
                springs.append(&mut spr.clone());
            }

            let mut groups = Vec::new();
            for _ in 0..5 {
                groups.append(&mut grps.clone());
            }

            Row { springs, groups }
        })
        .collect::<Vec<Row>>();

    let part2 = rows
        .iter()
        .map(|row| {
            let iters = row.iterations();
            // println!("Done!");
            iters
        })
        .sum::<u64>();

    println!("Part two {}", part2);

    Ok(())
}
