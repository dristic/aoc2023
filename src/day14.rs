use std::fs;

use enum_iterator::Sequence;

use crate::map::Map;

#[derive(Clone, Debug, Sequence, Eq, PartialEq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn tilt(map: &mut Map<char>, direction: Direction) {
    match direction {
        Direction::North => {
            for x in 0..map.width as i32 {
                let mut next = 0;
                for y in 0..map.height as i32 {
                    match map.get_xy(x, y).unwrap() {
                        'O' => {
                            map.swap(x, y, x, next);
                            next += 1;
                        }
                        '#' => next = y + 1,
                        '.' => (),
                        _ => panic!("Unknown char"),
                    }
                }
            }
        }
        Direction::West => {
            for y in 0..map.height as i32 {
                let mut next = 0;
                for x in 0..map.width as i32 {
                    match map.get_xy(x, y).unwrap() {
                        'O' => {
                            map.swap(x, y, next, y);
                            next += 1;
                        }
                        '#' => next = x + 1,
                        '.' => (),
                        _ => panic!("Unknown char"),
                    }
                }
            }
        }
        Direction::South => {
            for x in 0..map.width as i32 {
                let mut next = map.height as i32 - 1;
                for y in (0..map.height as i32).rev() {
                    match map.get_xy(x, y).unwrap() {
                        'O' => {
                            map.swap(x, y, x, next);
                            next -= 1;
                        }
                        '#' => next = y - 1,
                        '.' => (),
                        _ => panic!("Unknown char"),
                    }
                }
            }
        }
        Direction::East => {
            for y in 0..map.height as i32 {
                let mut next = map.width as i32 - 1;
                for x in (0..map.width as i32).rev() {
                    match map.get_xy(x, y).unwrap() {
                        'O' => {
                            map.swap(x, y, next, y);
                            next -= 1;
                        }
                        '#' => next = x - 1,
                        '.' => (),
                        _ => panic!("Unknown char"),
                    }
                }
            }
        }
    }
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d14.txt")?;

    let mut map: Map<char> = Map::from_str(&input);
    tilt(&mut map, Direction::North);

    let part1 = map
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| {
            if *ch == 'O' {
                let (_, y) = map.get_loc(idx);
                Some(map.height as i32 - y)
            } else {
                None
            }
        })
        .sum::<i32>();
    println!("Part one {}", part1);

    let mut map: Map<char> = Map::from_str(&input);
    let mut seen = vec![map.as_key()];
    loop {
        tilt(&mut map, Direction::North);
        tilt(&mut map, Direction::West);
        tilt(&mut map, Direction::South);
        tilt(&mut map, Direction::East);

        let key = map.as_key();
        if let Some(idx) = seen.iter().position(|other| other == &key) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            map.replace(&seen[final_idx]);
            break;
        }
        seen.push(key);
    }

    let part2 = map
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| {
            if *ch == 'O' {
                let (_, y) = map.get_loc(idx);
                Some(map.height as i32 - y)
            } else {
                None
            }
        })
        .sum::<i32>();
    println!("Part two {}", part2);

    Ok(())
}
