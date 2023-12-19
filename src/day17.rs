use std::{fs, collections::{VecDeque, HashMap}};

use crate::map::Map;

pub fn solve(suffix: &str) -> anyhow::Result<()> {
    let input = fs::read_to_string(format!("input/d17{}", suffix))?;

    let map: Map<usize> = Map::from_str_map(&input, |c| c.to_string().parse::<usize>().unwrap());

    let mut lowest = vec![vec![usize::MAX; map.width]; map.height];
    let mut cache = HashMap::new();
    let mut stack = VecDeque::new();
    stack.push_back((1, 0, 1, 0, 0, 1));
    stack.push_back((0, 1, 0, 1, 0, 1));

    while stack.len() > 0 {
        let (x, y, vx, vy, cur, straight) = stack.pop_front().unwrap();

        if let Some(heat) = map.get_xy(x, y) {
            let new_heat = cur + heat;

            if new_heat < lowest[y as usize][x as usize] {
                lowest[y as usize][x as usize] = new_heat;
            }

            let key = (x, y, vx, vy, straight);
            let cached = cache.get(&key).unwrap_or(&usize::MAX);
            if *cached > new_heat {
                if straight < 3 {
                    stack.push_back((x + vx, y + vy, vx, vy, new_heat, straight + 1));
                }

                stack.push_back((x + vy, y + vx, vy, vx, new_heat, 1));
                stack.push_back((x - vy, y - vx, -vy, -vx, new_heat, 1));

                cache.insert(key, new_heat);
            }
        }
    }

    let part1 = lowest[map.height - 1 as usize][map.width - 1 as usize];
    println!("Part one {}", part1);

    let mut lowest = vec![vec![usize::MAX; map.width]; map.height];
    let mut cache = HashMap::new();
    let mut stack = VecDeque::new();
    stack.push_back((1, 0, 1, 0, 0, 1));
    stack.push_back((0, 1, 0, 1, 0, 1));

    while stack.len() > 0 {
        let (x, y, vx, vy, cur, straight) = stack.pop_front().unwrap();

        // Special case for stopping.
        if x == map.width as i32 - 1 && y == map.height as i32 - 1 && straight < 4 {
            continue;
        }

        if let Some(heat) = map.get_xy(x, y) {
            let new_heat = cur + heat;

            if new_heat < lowest[y as usize][x as usize] {
                lowest[y as usize][x as usize] = new_heat;
            }

            let key = (x, y, vx, vy, straight);
            let cached = cache.get(&key).unwrap_or(&usize::MAX);
            if *cached > new_heat {
                if straight < 10 {
                    stack.push_back((x + vx, y + vy, vx, vy, new_heat, straight + 1));
                }

                if straight >= 4 {
                    stack.push_back((x + vy, y + vx, vy, vx, new_heat, 1));
                    stack.push_back((x - vy, y - vx, -vy, -vx, new_heat, 1));
                }

                cache.insert(key, new_heat);
            }
        }
    }

    let part2 = lowest[map.height - 1 as usize][map.width - 1 as usize];
    println!("Part two {}", part2);

    Ok(())
}
