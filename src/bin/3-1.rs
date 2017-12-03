fn isqrt(n: i64) -> i64 {
    (n as f64).sqrt() as i64
}

fn solve(n: i64) -> i64 {
    let level = (isqrt(n-1) + 1) / 2;
    let prev_square = (2*level-1).pow(2);    
    let offset = ((n - prev_square) % (2 * level) - level).abs();

    level + offset
}

fn main() {
    println!("{}", solve(361527));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(2), 1);
        assert_eq!(solve(3), 2);
        assert_eq!(solve(8), 1);
        assert_eq!(solve(9), 2);
        assert_eq!(solve(10), 3);
        assert_eq!(solve(12), 3);
        assert_eq!(solve(23), 2);
        assert_eq!(solve(1024), 31);
    }
}
