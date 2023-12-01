use std::{fs, path::Path};

fn get_digit(s: &str) -> Option<u32> {
    const PREFIX: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for pair in PREFIX {
        if s.ends_with(pair.0) {
            return Some(pair.1);
        }
    }

    None
}

pub fn solve() -> Result<(), std::io::Error> {
    const RADIX: u32 = 10;
    let input_path = Path::new("input/day1.txt");
    let input = fs::read_to_string(&input_path)?;

    // Part one.
    let mut answer = 0;
    for line in input.lines() {
        let num_str = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<char>>();
        let first_last = format!("{}{}", num_str.first().unwrap(), num_str.last().unwrap());
        answer += u32::from_str_radix(&first_last, RADIX).unwrap();
    }

    println!("Part One Answer: {}", answer);

    // Part two.
    let input_path = Path::new("input/day1.txt");
    let input = fs::read_to_string(&input_path)?;

    let mut answer = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        let mut buf = String::new();

        for ch in line.chars() {
            if ch.is_numeric() {
                if first.is_none() {
                    first = Some(ch.to_digit(RADIX).unwrap());
                }
                last = Some(ch.to_digit(RADIX).unwrap());
                buf = String::new();
            } else {
                buf.insert(buf.len(), ch);

                if let Some(num) = get_digit(&buf) {
                    if first.is_none() {
                        first = Some(num);
                    }
                    last = Some(num);
                }
            }
        }

        let total = (first.unwrap() * 10) + last.unwrap();

        answer += total;
    }

    println!("Part Two Answer: {}", answer);

    Ok(())
}
