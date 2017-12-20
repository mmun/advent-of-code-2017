fn solve(input: &str) -> i64 {
    let mut jumps = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut count = 0;
    let mut i = 0;

    loop {
        count += 1;

        let j = i + jumps[i as usize];

        if j >= 0 && j < jumps.len() as i64 {
            if jumps[i as usize] >= 3 {
                jumps[i as usize] -= 1;
            } else {
                jumps[i as usize] += 1;
            }

            i = j;
        } else {
            break;
        }

    }

    count
}

fn main() {
    println!("{}", solve(include_str!("../../data/5.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("0\n3\n0\n1\n-3\n"), 10);
    }
}
