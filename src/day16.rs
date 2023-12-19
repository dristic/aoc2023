use std::{collections::HashSet, fs};

use crate::map::Map;

struct Beam {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    active: bool,
}

fn find_energized(map: &Map<char>, x: i32, y: i32, vx: i32, vy: i32) -> u32 {
    let mut energized = vec![vec![0; map.width]; map.height];

    let mut beams = Vec::new();
    beams.push(Beam {
        x,
        y,
        vx,
        vy,
        active: true,
    });

    let mut cache: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    while beams.iter().any(|b| b.active) {
        let mut new_beams = Vec::new();

        for beam in beams.iter_mut() {
            let x = beam.x + beam.vx;
            let y = beam.y + beam.vy;

            if let Some(ch) = map.get_xy(x, y) {
                energized[y as usize][x as usize] = 1;

                match ch {
                    '.' => (),
                    '|' => {
                        if beam.vx != 0 {
                            // The beam is moving horizontally
                            // Stop it and create two new vertical beams
                            beam.active = false;

                            new_beams.push(Beam {
                                x,
                                y,
                                vx: 0,
                                vy: 1,
                                active: true,
                            });
                            new_beams.push(Beam {
                                x,
                                y,
                                vx: 0,
                                vy: -1,
                                active: true,
                            });
                        }
                    }
                    '-' => {
                        if beam.vy != 0 {
                            // The beam is moving vertically
                            // Stop it and create two new horizontal beams
                            beam.active = false;

                            new_beams.push(Beam {
                                x,
                                y,
                                vx: 1,
                                vy: 0,
                                active: true,
                            });
                            new_beams.push(Beam {
                                x,
                                y,
                                vx: -1,
                                vy: 0,
                                active: true,
                            });
                        }
                    }
                    '\\' => {
                        if beam.vx != 0 {
                            beam.vy = beam.vx;
                            beam.vx = 0;
                        } else {
                            beam.vx = beam.vy;
                            beam.vy = 0;
                        }
                    }
                    '/' => {
                        if beam.vx != 0 {
                            beam.vy = -beam.vx;
                            beam.vx = 0;
                        } else {
                            beam.vx = -beam.vy;
                            beam.vy = 0;
                        }
                    }
                    _ => (),
                }
            } else {
                // This went off the map and is no longer active.
                beam.active = false;
            }

            beam.x = x;
            beam.y = y;
        }

        for beam in new_beams {
            let key = (beam.x, beam.y, beam.vx, beam.vy);
            if !cache.contains(&key) {
                cache.insert(key);
                beams.push(beam);
            }
        }

        beams.retain(|b| b.active);
    }

    energized.iter().flatten().sum()
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d16.txt")?;

    let map: Map<char> = Map::from_str(&input);

    let part1 = find_energized(&map, -1, 0, 1, 0);
    println!("Part one {}", part1);

    let mut part2 = 0;
    for x in 0..map.width as i32 {
        let en_max = find_energized(&map, x, -1, 0, 1).max(find_energized(
            &map,
            x,
            map.height as i32,
            0,
            -1,
        ));
        if en_max > part2 {
            part2 = en_max;
        }
    }
    for y in 0..map.height as i32 {
        let en_max =
            find_energized(&map, -1, y, 1, 0).max(find_energized(&map, map.width as i32, y, -1, 0));
        if en_max > part2 {
            part2 = en_max;
        }
    }
    println!("Part two {}", part2);

    Ok(())
}
