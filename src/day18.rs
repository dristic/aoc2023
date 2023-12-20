use std::fs;

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

fn shoelace(instructions: &Vec<(Dir, i64)>) -> i64 {
    let mut v1 = (0, 0);
    let mut sum = 0;
    let mut border = 0;

    for (dir, val) in instructions {
        let v2 = match dir {
            Dir::Right => (v1.0 + val, v1.1),
            Dir::Left => (v1.0 - val, v1.1),
            Dir::Up => (v1.0, v1.1 - val),
            Dir::Down => (v1.0, v1.1 + val),
        };

        sum += v1.0 * v2.1 - v2.0 * v1.1;
        v1 = v2;

        border += val;
    }

    (sum.abs() / 2) + (border / 2) + 1
}

pub fn solve(suffix: &str) -> anyhow::Result<()> {
    let input = fs::read_to_string(format!("input/d18{}", suffix))?;

    let instructions = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let dir = match parts[0] {
                "R" => Dir::Right,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "U" => Dir::Up,
                _ => panic!("Unknown dir."),
            };
            let val = parts[1].parse::<i64>().unwrap();

            (dir, val)
        })
        .collect::<Vec<_>>();

    println!("Part one {}", shoelace(&instructions));

    let instructions = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let hex = parts[2];
            let val = i64::from_str_radix(&hex[2..7], 16).unwrap();
            let hex_dir = i64::from_str_radix(&hex[7..8], 16).unwrap();
            let dir = match hex_dir {
                0 => Dir::Right,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Up,
                _ => panic!("Unknown dir."),
            };

            (dir, val)
        })
        .collect::<Vec<_>>();

    println!("Part two {}", shoelace(&instructions));

    Ok(())
}
