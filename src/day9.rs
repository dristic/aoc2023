use std::fs;

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d9.txt")?;

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let nums = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut cur = nums.clone();
        let mut next = Vec::new();
        let mut delta_forward = 0;
        let mut back = Vec::new();
        back.push(nums.first().unwrap().to_owned());
        loop {
            for (i, n) in cur.iter().enumerate() {
                if i < cur.len() - 1 {
                    let n2 = cur[i + 1];
                    next.push(n2 - n);
                }
            }

            if next.iter().all(|n| *n == 0) {
                break;
            } else {
                delta_forward += next.last().unwrap();
                back.push(next.first().unwrap().to_owned());
                cur = next.clone();
                next = Vec::new();
            }
        }

        let forward = nums.last().unwrap() + delta_forward;

        back.reverse();
        let mut backward = 0;
        for (_, n) in back.iter().enumerate() {
            backward = n - backward;
        }

        part1 += forward;
        part2 += backward;
    }

    println!("Part one {}", part1);
    println!("Part two {}", part2);

    Ok(())
}
