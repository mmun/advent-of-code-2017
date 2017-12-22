use std::collections::HashMap;

fn solve(input: &str, bursts: usize) -> u64 {
    let mut state: HashMap<(i64, i64), char> = HashMap::new();
    let mut size = 0;

    for (i, line) in input.lines().enumerate() {
        size += 1;

        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                state.insert((i as i64, j as i64), c);
            }
        }
    }

    let mut i: i64 = size/2;
    let mut j: i64 = size/2;
    let mut di: i64 = -1;
    let mut dj: i64 = 0;

    let mut count = 0;

    for _ in 0..bursts {
        match state.get(&(i, j)).cloned() {
            Some('F') => {
                state.remove(&(i, j));
                di = -di;
                dj = -dj;
            },
            Some('#') => {
                state.insert((i, j), 'F');
                let t = di;
                di = dj;
                dj = -t;                
            },
            Some('W') => {
                state.insert((i, j), '#');
                count += 1;
            },
            _ => {
                state.insert((i, j), 'W');
                let t = di;
                di = -dj;
                dj = t;
            }
        }

        i += di;
        j += dj;
    }

    count
}

fn main() {
    let input = include_str!("../../data/22.in");
    println!("{}", solve(input, 10000000));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        let input = include_str!("../../data/22.example.in");
        assert_eq!(solve(input, 100), 26);
        assert_eq!(solve(input, 10000000), 2511944);
    }
}
