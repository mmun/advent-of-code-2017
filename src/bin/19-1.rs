fn solve(input: &str) -> String {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line.as_bytes().to_vec());
    }

    let get = |i, j| {
        grid[i as usize][j as usize].into()
    };

    let mut i: i32 = 0;
    let mut j: i32 = grid[0].iter().position(|&b| b == b'|').unwrap() as i32;
    let mut di: i32 = 1;
    let mut dj: i32 = 0;

    let mut letters = String::new();

    loop {
        i += di;
        j += dj;

        match get(i, j) {
            ' ' => return letters,

            letter @ 'A' ... 'Z' => letters.push(letter),

            '|' | '-' => {},

            '+' => {
                let t = dj;
                dj = di;
                di = t;

                if get(i + di, j + dj) == ' ' {
                    di *= -1;
                    dj *= -1;
                }
            },

            _ => unreachable!()
        }
    }
}

fn main() {
    println!("{}", solve(include_str!("../../data/19.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(include_str!("../../data/19.example.in")), "ABCDEF");
    }
}
