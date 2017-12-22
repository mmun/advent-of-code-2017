use std::collections::HashSet;

fn solve(input: &str, bursts: usize) -> u64 {
    let mut infected = HashSet::new();
    let mut size = 0;

    for (i, line) in input.lines().enumerate() {
        size += 1;

        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                infected.insert((i as i64, j as i64));
            }
        }
    }

    let mut i: i64 = size/2;
    let mut j: i64 = size/2;
    let mut di: i64 = -1;
    let mut dj: i64 = 0;

    let mut count = 0;

    for _ in 0..bursts {
        if infected.contains(&(i, j)) {
            infected.remove(&(i, j));
            let t = di;
            di = dj;
            dj = -t;
        } else {
            infected.insert((i, j));
            let t = di;
            di = -dj;
            dj = t;
            count += 1;
        }

        i += di;
        j += dj;
    }

    count
}

fn main() {
    let input = include_str!("../../data/22.in");
    println!("{}", solve(input, 10000));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        let input = include_str!("../../data/22.example.in");
        assert_eq!(solve(input, 7), 5);
        assert_eq!(solve(input, 70), 41);
        assert_eq!(solve(input, 10000), 5587);
    }
}
