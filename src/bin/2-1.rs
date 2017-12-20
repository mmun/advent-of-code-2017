fn solve(input: &str) -> i64 {
    let input = input.trim();

    let mut sum = 0;

    for line in input.lines() {
        let nums = line.split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect::<Vec<i64>>();

        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();

        sum += max - min;
    }

    sum
}

fn main() {
    println!("{}", solve(include_str!("../../data/2.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("5 1 9 5\n7 5 3\n2 4 6 8\n"), 18);
    }
}
