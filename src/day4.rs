use std::fs;

use log::debug;
use regex::Regex;

#[derive(Debug)]
struct Card {
    num: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    fn calc_points(&self) -> u32 {
        let matches = self.matches();

        if matches == 0 {
            matches
        } else {
            1 << (matches - 1)
        }
    }

    fn matches(&self) -> u32 {
        let mut matches = 0;

        for n in &self.have {
            if self.winning.contains(&n) {
                matches += 1;
            }
        }

        matches
    }
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d4.txt")?;

    // Input parsing.
    let mut cards = Vec::new();
    for line in input.lines() {
        let reg = Regex::new(r"Card\s+(?<game>[0-9]+): (?<winning>[0-9\s]+) \| (?<have>[0-9\s]+)")?;
        let parts = reg.captures(line).unwrap();
        debug!("Parts {:?}", parts);

        let num = parts["game"].parse::<u32>().unwrap();
        let winning = parts["winning"]
            .trim()
            .split(" ")
            .filter_map(|n| n.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let have = parts["have"]
            .trim()
            .split(" ")
            .filter_map(|n| n.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>();

        cards.push(Card { num, winning, have });
    }

    // Part one.
    let part1 = cards.iter().map(|c| c.calc_points()).sum::<u32>();
    println!("Part one {}", part1);

    // Part two.
    let mut total_cards = 0;
    let mut copies = vec![1; 300];

    for card in &cards {
        let wins = card.matches();

        let copy = copies[card.num as usize];
        for n in 1..=wins {
            let idx = card.num as usize + n as usize;
            copies[idx] = copies[idx] + (1 * copy);
        }

        total_cards += copy;
    }

    println!("Part two {}", total_cards);

    Ok(())
}
