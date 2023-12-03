use std::{fs, collections::{HashMap, HashSet}};

use crate::map::Map;

type Pos = (i32, i32);

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/day3.txt")?;

    let map = Map::from_str(&input);

    let mut part1 = 0;

    let mut gears: HashMap<Pos, Vec<u32>> = HashMap::new();

    for row in 0..map.height as i32 {
        let mut is_adjacent = false;
        let mut buf = String::new();
        let mut adj_gears: HashSet<Pos> = HashSet::new();

        for col in 0..map.width as i32 {
            if let Some(c) = map.get_xy(col, row) {
                if c.is_numeric() {
                    buf.push(c);

                    for y in row-1..=row+1 {
                        for x in col-1..=col+1 {
                            if let Some(adj) = map.get_xy(x, y) {
                                if !adj.is_numeric() && adj != '.' {
                                    is_adjacent = true;

                                    if adj == '*' {
                                        adj_gears.insert((x, y));
                                    }
                                }
                            }
                        }
                    }
                }
                
                // If we have a saved value and are either at a non-numeric char
                // or are at the end of the column.
                if buf.len() > 0 && (!c.is_numeric() || col == map.width as i32 - 1) {
                    let val: u32 = buf.parse().unwrap();

                    if is_adjacent {
                        part1 += val;
                    }

                    for pos in &adj_gears {
                        if !gears.contains_key(&pos) {
                            gears.insert(pos.clone(), Vec::new());
                        }

                        gears.get_mut(pos).unwrap().push(val);
                    }

                    buf = String::new();
                    is_adjacent = false;
                    adj_gears = HashSet::new();
                }
            }
        }
    }

    println!("Part 1 {}", part1);

    // Part 2
    let part2: u32 = gears.into_iter().map(|(_, v)| {
        if v.len() == 2 {
            v[0] * v[1]
        } else {
            0
        }
    }).sum();

    println!("Part 2 {}", part2);

    Ok(())
}
