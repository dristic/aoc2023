use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    rank_mem: Option<u32>,
}

impl Hand {
    fn new(cards: Vec<u32>, bid: u32) -> Hand {
        // Get count of our cards.
        let mut nums = [0; 15];
        for n in &cards {
            nums[*n as usize] += 1;
        }

        // Calc the type by counting the matching pairs, ranking more matching
        // cards higher.
        let hand_type = nums
            .iter()
            .map(|n| match n {
                0 => 0,
                1 => 0,
                2 => 1,
                3 => 3,
                4 => 8,
                5 => 10,
                _ => 0,
            })
            .sum::<u32>();

        // For debugging.
        let _rank_str = format!(
            "{} {} {} {} {} {}",
            hand_type, cards[0], cards[1], cards[2], cards[3], cards[4]
        );

        // Use 6 bytes to store our info:
        // 0000 0000 TYPE CARD1 CARD2 CARD3 CARD4 CARD5
        let rank_val = hand_type << 20
            | cards[0] << 16
            | cards[1] << 12
            | cards[2] << 8
            | cards[3] << 4
            | cards[4];

        Hand {
            cards,
            bid,
            rank_mem: Some(rank_val),
        }
    }

    fn new_alt_rules(cards: Vec<u32>, bid: u32) -> Hand {
        // Jokers are now 1 point
        let cards = cards
            .into_iter()
            .map(|c| if c == 11 { 1 } else { c })
            .collect::<Vec<u32>>();

        // Get count of our cards.
        let mut highest = (0, 0);
        let mut nums = [0; 15];
        for n in &cards {
            let idx = *n as usize;
            nums[idx] += 1;

            // If this is not a joker, record the new highest
            if nums[idx] > highest.0 && idx != 1 {
                highest = (nums[idx], *n);
            }
        }

        // Apply the jokers to the highest matches, if the hand is not all jokers.
        if nums[1] != 5 {
            nums[highest.1 as usize] += nums[1];
            nums[1] = 0;
        }
        //println!("{:?} | {:?} | {:?}", cards, highest, nums);

        // Calc the type by counting the matching pairs, ranking more matching
        // cards higher.
        let hand_type = nums
            .iter()
            .map(|n| match n {
                0 => 0,
                1 => 0,
                2 => 1,
                3 => 3,
                4 => 8,
                5 => 10,
                _ => 0,
            })
            .sum::<u32>();

        // For debugging.
        let _rank_str = format!(
            "{} {} {} {} {} {}",
            hand_type, cards[0], cards[1], cards[2], cards[3], cards[4]
        );

        // Use 6 bytes to store our info:
        // 0000 0000 TYPE CARD1 CARD2 CARD3 CARD4 CARD5
        let rank_val = hand_type << 20
            | cards[0] << 16
            | cards[1] << 12
            | cards[2] << 8
            | cards[3] << 4
            | cards[4];

        Hand {
            cards,
            bid,
            rank_mem: Some(rank_val),
        }
    }

    fn rank(&self) -> u32 {
        // memoize our rank for sorting.
        self.rank_mem.unwrap()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() -> anyhow::Result<()> {
    let input = fs::read_to_string("input/d7.txt")?;

    let mut hands = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();

            let cards = parts[0]
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
                .collect::<Vec<u32>>();

            Hand::new(cards, parts[1].parse::<u32>().unwrap())
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    let answer: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum();
    println!("Part one {}", answer);

    let mut hands_alt = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();

            let cards = parts[0]
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
                .collect::<Vec<u32>>();

            Hand::new_alt_rules(cards, parts[1].parse::<u32>().unwrap())
        })
        .collect::<Vec<Hand>>();

    hands_alt.sort();

    let answer: u32 = hands_alt
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum();
    println!("Part two {}", answer);

    Ok(())
}
