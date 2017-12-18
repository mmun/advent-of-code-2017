fn solve(steps: usize) -> u64 {
    let mut cursor = 0;
    let mut buffer = vec![0];

    for i in 1.. {
        if i == 2017 {
            return buffer[(cursor + steps + 1) % buffer.len()];
        } else {
            cursor = (cursor + steps) % buffer.len();
            buffer.insert(cursor + 1, i);
            cursor = (cursor + 1) % buffer.len();
        }
    }

    unreachable!();
}

fn main() {
    println!("{}", solve(394));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(3), 638);
    }
}
 