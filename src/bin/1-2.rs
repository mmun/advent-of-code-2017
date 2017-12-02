fn solve(input: &str) -> i64 {
    let input = input.trim().as_bytes();
    let len = input.len();

    let mut sum = 0;

    for i in 0..len {
        let j = (i + len/2) % len;

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
        assert_eq!(solve("1212"), 6);
        assert_eq!(solve("1221"), 0);
        assert_eq!(solve("123425"), 4);
        assert_eq!(solve("123123"), 12);
        assert_eq!(solve("12131415"), 4);
    }
}
