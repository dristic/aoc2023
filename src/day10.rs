use std::{collections::VecDeque, fs};

use crate::map::Map;

#[derive(Clone, Debug)]
enum Dir {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    None,
    Start,
    Unknown,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '|' => Dir::NS,
            '-' => Dir::EW,
            'L' => Dir::NE,
            'J' => Dir::NW,
            '7' => Dir::SW,
            'F' => Dir::SE,
            '.' => Dir::None,
            'S' => Dir::Start,
            _ => Dir::Unknown,
        }
    }
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d10.txt")?;

    let map: Map<Dir> = Map::from_str(&input);

    let start = map
        .iter()
        .enumerate()
        .find_map(|(idx, dir)| {
            if matches!(dir, Dir::Start) {
                Some(map.get_loc(idx))
            } else {
                None
            }
        })
        .unwrap();

    let mut dist = vec![vec![0; map.width]; map.height];
    let mut stack = VecDeque::new();
    stack.push_back((start.0, start.1, 0, start.0, start.1));

    while stack.len() > 0 {
        let (x, y, cur_dist, prevx, prevy) = stack.pop_front().unwrap();

        // Make sure this is a valid location.
        if let Some(dir) = map.get_xy(x, y) {
            // Update shortest distance.
            let ix = x as usize;
            let iy = y as usize;
            if cur_dist < dist[iy][ix] || dist[iy][ix] == 0 {
                dist[iy][ix] = cur_dist;
            } else {
                continue;
            }

            let next_dist = cur_dist + 1;
            let dx = x - prevx;
            let dy = y - prevy;

            // Add next location to our stack.
            match dir {
                Dir::NS => stack.push_back((x + dx, y + dy, next_dist, x, y)),
                Dir::EW => stack.push_back((x + dx, y + dy, next_dist, x, y)),
                Dir::NE => stack.push_back((x + dy, y + dx, next_dist, x, y)),
                Dir::NW => stack.push_back((x - dy, y - dx, next_dist, x, y)),
                Dir::SW => stack.push_back((x + dy, y + dx, next_dist, x, y)),
                Dir::SE => stack.push_back((x - dy, y - dx, next_dist, x, y)),
                Dir::None => (),
                Dir::Start => {
                    // Check four locations around the start and make sure they
                    // are connected to this location.
                    if let Some(dir) = map.get_xy(x + 1, y) {
                        if matches!(dir, Dir::EW | Dir::NW | Dir::SW) {
                            stack.push_back((x + 1, y, next_dist, x, y));
                        }
                    }

                    if let Some(dir) = map.get_xy(x - 1, y) {
                        if matches!(dir, Dir::EW | Dir::NE | Dir::SE) {
                            stack.push_back((x - 1, y, next_dist, x, y));
                        }
                    }

                    if let Some(dir) = map.get_xy(x, y + 1) {
                        if matches!(dir, Dir::NS | Dir::NW | Dir::NE) {
                            stack.push_back((x, y + 1, next_dist, x, y));
                        }
                    }

                    if let Some(dir) = map.get_xy(x, y - 1) {
                        if matches!(dir, Dir::NS | Dir::SE | Dir::SW) {
                            stack.push_back((x, y - 1, next_dist, x, y));
                        }
                    }

                    // Hack for part 2
                    dist[iy][ix] = 1;
                }
                Dir::Unknown => println!("Unknown {} {}", x, y),
            }
        }
    }

    let part1 = dist
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();

    println!("Part one {}", part1);

    // Use even-odd to find inside cells.
    let mut part2 = 0;
    for y in 0..map.height {
        let mut inside = false;
        for x in 0..map.width {
            if dist[y][x] > 0 {
                if let Some(dir) = map.get_xy(x as i32, y as i32) {
                    if matches!(dir, Dir::NS | Dir::NE | Dir::NW | Dir::Start) {
                        inside = !inside;
                    }
                }
            } else if inside {
                part2 += 1;
            }
        }
    }
    println!("Part two {}", part2);

    Ok(())
}
