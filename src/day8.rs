use std::{collections::HashMap, fs};

fn gcd(n1: u64, n2: u64) -> u64 {
    let mut max = if n1 > n2 { n1 } else { n2 };
    let mut min = if n1 > n2 { n2 } else { n1 };

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() < 2 {
        return 0;
    }

    let mut iter = nums.iter();
    let mut first = *iter.next().unwrap();

    while let Some(second) = iter.next() {
        let second = *second;

        first = first * second / gcd(first, second);
    }

    first
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("d8.txt")?;

    let mut lines = input.lines();
    let commands = lines.next().unwrap();

    // Skip empty line.
    lines.next().unwrap();

    let mut curs: Vec<String> = Vec::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let fmt = line
            .replace(" = (", ",")
            .replace(", ", ",")
            .replace(")", "");

        let parts = fmt.split(",").map(String::from).collect::<Vec<String>>();

        if parts[0].ends_with("A") {
            curs.push(parts[0].clone());
        }

        nodes.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
    }

    let lengths = curs
        .iter()
        .map(|start| {
            let mut cur = start.clone();
            let mut steps = 0;

            loop {
                for c in commands.chars() {
                    let node = nodes.get(&cur).unwrap();
                    steps = steps + 1;

                    cur = match c {
                        'R' => node.1.clone(),
                        'L' => node.0.clone(),
                        _ => panic!("Unknown char"),
                    };

                    if cur.ends_with("Z") {
                        break;
                    }
                }

                if cur.ends_with("Z") {
                    break;
                }
            }

            steps
        })
        .collect::<Vec<u64>>();

    let ans = lcm(&lengths);
    println!("Part two {}", ans);

    // let mut cur = "AAA".to_string();
    // let mut steps = 0;

    // loop {
    //     for c in commands.chars() {
    //         let node = nodes.get(&cur).unwrap();
    //         steps = steps + 1;

    //         cur = match c {
    //             'R' => node.1.clone(),
    //             'L' => node.0.clone(),
    //             _ => panic!("Unknown char"),
    //         };

    //         // println!("Step {} => {}", c, cur);

    //         if &cur == "ZZZ" {
    //             break;
    //         }
    //     }

    //     if &cur == "ZZZ" {
    //         break;
    //     }
    // }

    // println!("Part one {}", steps);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(18, 27), 9);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[150, 210]), 1050);
    }
}