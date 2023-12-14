use std::fs;

use crate::map::Map;

fn find_reflection(map: &Map<char>) -> usize {
    // Vertical.
    for line in 1..map.width {
        let mut misses = 0;

        // Test every character on the left side of the line.
        for x in 0..line {
            for y in 0..map.height {
                let orig = map.get_xy(x as i32, y as i32).unwrap();
                let dist = line - x;
                let opp_x = line + dist - 1;
                let opposite = map.get_xy(opp_x as i32, y as i32);

                if let Some(opp) = opposite {
                    if opp != orig {
                        misses += 1;
                        if misses > 1 {
                            break;
                        }
                    }
                }
            }

            if misses > 1 {
                break;
            }
        }

        if misses == 1 {
            //println!("{}reflect on vert {}", map, line);
            return line;
        }
    }

    // Horizontal.
    for line in 1..map.height {
        let mut misses = 0;

        // Test every character on the top side of the line.
        for x in 0..map.width {
            for y in 0..line {
                let orig = map.get_xy(x as i32, y as i32).unwrap();
                let dist = line - y;
                let opp_y = line + dist - 1;
                let opposite = map.get_xy(x as i32, opp_y as i32);

                if let Some(opp) = opposite {
                    if opp != orig {
                        misses += 1;
                        if misses > 1 {
                            break;
                        }
                    }
                }
            }

            if misses > 1 {
                break;
            }
        }

        if misses == 1 {
            //println!("{}reflect on horiz {}", map, line);
            return line * 100;
        }
    }

    0
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("d13.txt")?;

    let mut maps: Vec<Map<char>> = Vec::new();
    let mut current = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            maps.push(Map::from_str(&current.join("\n")));
            current.clear();
        } else {
            current.push(line);
        }
    }
    maps.push(Map::from_str(&current.join("\n")));

    let part2 = maps.iter().map(find_reflection).sum::<usize>();
    println!("Part two {}", part2);

    Ok(())
}
