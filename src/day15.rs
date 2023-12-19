use std::fs;

fn hash(str: &str) -> usize {
    let mut val: usize = 0;
    for c in str.chars() {
        let code = c as u8;
        val += code as usize;
        val = val * 17;
        val = val % 256;
    }

    val
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d15.txt")?;

    let part1 = input.split(",").map(hash).sum::<usize>();
    println!("Part one {}", part1);

    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![(String::new(), 0); 0]; 256];
    for ins in input.split(",") {
        if let Some(idx) = ins.find('=') {
            let key = ins[0..idx].to_owned();
            let hash = hash(&key);
            let val = ins[idx + 1..].parse::<u32>().unwrap();

            if let Some(idx) = boxes[hash].iter().position(|ins| ins.0.eq(&key)) {
                boxes[hash][idx] = (key, val);
            } else {
                boxes[hash].push((key, val));
            }
        }

        if let Some(idx) = ins.find('-') {
            let key = ins[0..idx].to_owned();
            let hash = hash(&key);

            if let Some(idx) = boxes[hash].iter().position(|ins| ins.0.eq(&key)) {
                boxes[hash].remove(idx);
            }
        }
    }

    let mut part2 = 0;
    for (box_idx, v) in boxes.iter().enumerate() {
        for (ins_idx, (_, val)) in v.iter().enumerate() {
            part2 += (box_idx + 1) * (ins_idx + 1) * (*val as usize);
        }
    }
    println!("Part two {}", part2);

    Ok(())
}
