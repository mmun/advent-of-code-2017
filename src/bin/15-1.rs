fn solve(mut a: u64, mut b: u64, rounds: u64) -> u64 {
    let mut count = 0;

    for _ in 0..rounds {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        if a & 65535 == b & 65535 {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("{}", solve(634, 301, 40000000));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(65, 8921, 5), 1);
        assert_eq!(solve(65, 8921, 40000000), 588);
    }
}
 