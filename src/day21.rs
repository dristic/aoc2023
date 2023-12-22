use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use crate::map::Map;

fn find_plots(input: &str, max_steps: u64) -> u64 {
    let map: Map<char> = Map::from_str(&input);
    let mut visit = HashSet::new();

    let (sx, sy) = map.get_loc(map.iter().position(|c| c == &'S').unwrap());

    let mut steps = VecDeque::new();
    steps.push_back((sx, sy, 0));
    let mut ans = 0;
    while steps.len() > 0 {
        let (x, y, num) = steps.pop_front().unwrap();

        if visit.contains(&(x, y)) {
            continue;
        }

        visit.insert((x, y));

        if num % 2 == max_steps % 2 {
            ans += 1;
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            let mut shift_x = nx % map.width as i32;
            let mut shift_y = ny % map.height as i32;

            if shift_x < 0 {
                shift_x += map.width as i32
            }
            if shift_y < 0 {
                shift_y += map.height as i32
            }

            if let Some(&ch) = map.get_xy(shift_x, shift_y) {
                if ch != '#' && num < max_steps {
                    steps.push_back((nx, ny, num + 1));
                }
            }
        }
    }

    ans
}

pub fn solve(suffix: &str) -> anyhow::Result<()> {
    let input = fs::read_to_string(format!("input/d21{}", suffix))?;

    println!("Part one {}", find_plots(&input, 64));

    let max_width = 26501365 / 131;
    let x1 = find_plots(&input, 65);
    let x2 = find_plots(&input, 65 + 131);
    let x3 = find_plots(&input, 65 + 131 + 131);
    let part2 = x1 + x2 * max_width + (max_width * (max_width - 1) / 2) * ((x3 - x2) - (x2 - x1));
    println!("Part two {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = std::include_str!("../input/d21ex.txt");

    #[test]
    fn test_6() {
        assert_eq!(find_plots(&TEST_INPUT, 6), 16);
    }

    #[test]
    fn test_10() {
        assert_eq!(find_plots(&TEST_INPUT, 10), 50);
    }

    #[test]
    fn test_50() {
        assert_eq!(find_plots(&TEST_INPUT, 50), 1594);
    }

    #[test]
    fn test_100() {
        assert_eq!(find_plots(&TEST_INPUT, 100), 6536);
    }

    #[test]
    fn test_500() {
        assert_eq!(find_plots(&TEST_INPUT, 500), 167004);
    }
}
