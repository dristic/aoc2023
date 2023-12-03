use std::{collections::HashMap, fs, path::Path};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            _ => Color::Blue,
        }
    }
}

#[derive(Debug)]
struct Round {
    cubes: HashMap<Color, u32>,
}

impl Round {
    fn possible(&self, max: &Vec<(Color, u32)>) -> bool {
        for pair in max {
            if let Some(amt) = self.cubes.get(&pair.0) {
                if amt > &pair.1 {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Debug)]
struct Game {
    num: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn possible(&self, max: &Vec<(Color, u32)>) -> bool {
        for round in &self.rounds {
            if round.possible(max) == false {
                return false;
            }
        }

        true
    }

    fn power(&self) -> u32 {
        let mut lowest = HashMap::new();

        for round in &self.rounds {
            for pair in &round.cubes {
                if let Some(amt) = lowest.get_mut(pair.0) {
                    if *amt < *pair.1 {
                        *amt = *pair.1;
                    }
                } else {
                    lowest.insert(pair.0.clone(), pair.1.clone());
                }
            }
        }

        lowest.values().product()
    }
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string(Path::new("input/day2.txt"))?;

    let mut games = Vec::new();

    // Parse the input into our structures.
    for line in input.lines() {
        let reg = Regex::new(r"Game (?<game>.+): (?<rounds>.*)")?;
        let parts = reg.captures(line).unwrap();

        let num: u32 = parts["game"].parse().unwrap();
        let rounds = parts["rounds"]
            .split(";")
            .map(|t| {
                let t = t.trim();
                let parts = t.split(",").map(|t| t.trim()).collect::<Vec<&str>>();
                let mut cubes = HashMap::new();

                for part in parts {
                    let pair = part.split(" ").collect::<Vec<&str>>();
                    let num: u32 = pair.first().unwrap().parse().unwrap();
                    let color = Color::from(*pair.last().unwrap());

                    cubes.insert(color, num);
                }

                Round { cubes }
            })
            .collect::<Vec<Round>>();

        games.push(Game { num, rounds });
    }

    // Part one.
    let max = vec![(Color::Blue, 14), (Color::Red, 12), (Color::Green, 13)];

    let mut answer = 0;
    for game in &games {
        if game.possible(&max) {
            answer += game.num;
        }
    }

    println!("Part one: {}", answer);

    // Part two.
    let answer: u32 = games.into_iter().map(|g| g.power()).sum();

    println!("Part two: {}", answer);

    Ok(())
}
