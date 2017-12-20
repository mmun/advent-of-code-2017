fn solve(mut a: u64, mut b: u64, rounds: u64) -> u64 {
    let mut count = 0;

    for _ in 0..rounds {
        loop {
            a = (a * 16807) % 2147483647;
            if a % 4 == 0 { break }
        }

        loop {
            b = (b * 48271) % 2147483647;
            if b % 8 == 0 { break }
        }

        if a & 65535 == b & 65535 {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("{}", solve(634, 301, 5000000));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(65, 8921, 1056), 1);
        assert_eq!(solve(65, 8921, 5000000), 309);
    }
}
 