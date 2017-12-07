use std::collections::HashSet;

fn solve(input: &str) -> i64 {
    let mut mem = input
        .trim()
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i64>>();

    let mem_len = mem.len();

    let mut seen = HashSet::new();
    let mut count = 0_i64;

    while !seen.contains(&mem) {
        seen.insert(mem.clone());
        count += 1;

        let mut max_i = 0;
        let mut max_blocks = 0;

        for i in 0..mem_len {
            if mem[i] > max_blocks {
                max_i = i;
                max_blocks = mem[i];
            }
        }

        mem[max_i] = 0;

        for i in 0..max_blocks {
            mem[(max_i + 1 + i as usize) % mem_len] += 1;
        }
    }

    count
}

fn main() {
    println!("{}", solve(include_str!("6.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("0 2 7 0"), 5);
    }
}
