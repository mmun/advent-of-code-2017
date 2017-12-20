fn solve(input: &str) -> u64 {
    let mut severity = 0;

    for line in input.trim().lines() {
        let mut tokens = line.trim().split(": ");

        let depth = tokens.next().unwrap().parse::<u64>().unwrap();
        let range = tokens.next().unwrap().parse::<u64>().unwrap();

        if depth % (2*range - 2) == 0 {
            severity += depth * range;
        }
    }

    severity
}

fn main() {
    println!("{}", solve(include_str!("../../data/13.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("
            0: 3
            1: 2
            4: 4
            6: 4
        "), 24);
    }
}
 