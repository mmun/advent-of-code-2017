fn solve(input: &str) -> i64 {
    let input = input.trim();

    let mut sum = 0;

    for line in input.lines() {
        let mut nums = line.split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect::<Vec<i64>>();

        nums.sort();

        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                if nums[j] % nums[i] == 0 {
                    sum += nums[j] / nums[i];
                }
            }
        }
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
        assert_eq!(solve("5 9 2 8\n9 4 7 3\n3 8 6 5\n"), 9);
    }
}
