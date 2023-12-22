pub fn gcd(n1: u64, n2: u64) -> u64 {
    let mut max = if n1 > n2 { n1 } else { n2 };
    let mut min = if n1 > n2 { n2 } else { n1 };

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() < 2 {
        return 0;
    }

    let mut iter = nums.iter();
    let mut first = *iter.next().unwrap();

    while let Some(second) = iter.next() {
        let second = *second;

        first = first * second / gcd(first, second);
    }

    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(18, 27), 9);
    }

    #[test]
    fn test_gcd_2() {
        assert_eq!(gcd(6, 5), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[150, 210]), 1050);
    }
}
