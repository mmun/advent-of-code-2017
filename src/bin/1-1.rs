fn solve(input: &str) -> i64 {
    let input = input.trim().as_bytes();
    let len = input.len();

    let mut sum = 0;

    for i in 0..len {
        let j = (i + 1) % len;

        if input[i] == input[j] {
            sum += (input[i] - b'0') as i64;
        }
    }

    sum
}

fn main() {
    println!("{}", solve(include_str!("1.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("1122"), 3);
        assert_eq!(solve("1111"), 4);
        assert_eq!(solve("1234"), 0);
        assert_eq!(solve("91212129"), 9);
    }
}
