use std::fs;

use crate::map::Map;

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("d11.txt")?;

    let map: Map<char> = Map::from_str(&input);

    let galaxies_orig = map
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == '#' {
                Some(map.get_loc(i))
            } else {
                None
            }
        })
        .collect::<Vec<(i32, i32)>>();

    let mut x_empty = Vec::new();
    for x in 0..map.width {
        let mut empty = true;
        for y in 0..map.height {
            if *map.get_xy(x as i32, y as i32).unwrap() == '#' {
                empty = false;
            }
        }

        if empty {
            x_empty.push(x.to_owned());
        }
    }

    let mut y_empty = Vec::new();
    for y in 0..map.height {
        let mut empty = true;
        for x in 0..map.width {
            if *map.get_xy(x as i32, y as i32).unwrap() == '#' {
                empty = false;
            }
        }

        if empty {
            y_empty.push(y.to_owned());
        }
    }

    let mut galaxies = Vec::new();
    for (x, y) in galaxies_orig {
        let nx = x as i128;
        let ny = y as i128;

        let dx = x_empty
            .iter()
            .map(|x| if nx > *x as i128 { 999999 } else { 0 })
            .sum::<i128>();
        let dy = y_empty
            .iter()
            .map(|y| if ny > *y as i128 { 999999 } else { 0 })
            .sum::<i128>();

        galaxies.push((nx + dx, ny + dy));
    }

    let mut part1 = 0;
    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            let (x2, y2) = galaxies[j];

            let dist = (x2 - x1).abs() + (y2 - y1).abs();
            //println!("Galaxy {} to {} dist {}", i+1, j+1, dist);
            part1 += dist;
        }
    }
    println!("Part one {}", part1);

    Ok(())
}
