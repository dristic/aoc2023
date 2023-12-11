use std::{fs, collections::HashMap};

use enum_iterator::Sequence;

#[derive(Sequence, Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Types {
    None,
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humid,
}

struct Range {
    dest: u64,
    source: u64,
    dist: u64,
}

impl Range {
    fn convert(&self, src: u64) -> Option<u64> {
        if self.source <= src && self.source + self.dist >= src {
            let delta = src - self.source;
            Some(self.dest + delta)
        } else {
            None
        }
    }
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("d5.txt")?;

    let mut seeds = Vec::new();
    let mut maps = HashMap::new();
    maps.insert(Types::Seed, Vec::new());
    maps.insert(Types::Soil, Vec::new());
    maps.insert(Types::Fertilizer, Vec::new());
    maps.insert(Types::Water, Vec::new());
    maps.insert(Types::Light, Vec::new());
    maps.insert(Types::Temp, Vec::new());
    maps.insert(Types::Humid, Vec::new());

    let mut cur_type = Types::None;
    let mut collect = false;

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            seeds = line
                .replace("seeds: ", "")
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        } else {
            if !collect {
                match line {
                    "seed-to-soil map:" => cur_type = Types::Seed,
                    "soil-to-fertilizer map:" => cur_type = Types::Soil,
                    "fertilizer-to-water map:" => cur_type = Types::Fertilizer,
                    "water-to-light map:" => cur_type = Types::Water,
                    "light-to-temperature map:" => cur_type = Types::Light,
                    "temperature-to-humidity map:" => cur_type = Types::Temp,
                    "humidity-to-location map:" => cur_type = Types::Humid,
                    _ => println!("Skipping line {}", line),
                }

                if !matches!(cur_type, Types::None) {
                    collect = true;
                }
            } else {
                if line.len() == 0 {
                    collect = false;
                    cur_type = Types::None;
                } else {
                    let values = line
                        .split(" ")
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();

                    maps.get_mut(&cur_type).unwrap().push(Range {
                        dest: values[0],
                        source: values[1],
                        dist: values[2],
                    });
                }
            }
        }
    }

    let locations = seeds.iter().map(|seed| {
        let mut loc = *seed;

        for val in enum_iterator::all::<Types>().filter(|t| !matches!(t, Types::None)) {
            let new_val = maps
                .get(&val)
                .unwrap()
                .iter()
                .find_map(|r| r.convert(loc));

            match new_val {
                Some(v) => loc = v,
                None => {},
            }
        }

        loc
    })
    .collect::<Vec<u64>>();

    println!("{:?}", locations);

    let mut answer = u64::MAX;
    for loc in locations {
        if loc < answer {
            answer = loc;
        }
    }
    println!("Answer is {}", answer);

    // Part two
    let mut answer = u64::MAX;
    let mut it = seeds.iter();
    while let Some(start) = it.next() {
        let end = start + it.next().unwrap();

        for n in *start..end {
            let mut loc = n;

            for val in enum_iterator::all::<Types>().filter(|t| !matches!(t, Types::None)) {
                let new_val = maps
                    .get(&val)
                    .unwrap()
                    .iter()
                    .find_map(|r| r.convert(loc));

                match new_val {
                    Some(v) => loc = v,
                    None => {},
                }
            }

            if loc < answer {
                println!("New answer {}", loc);
                answer = loc;
            }
        }
    }

    println!("Part two {}", answer);

    Ok(())
}