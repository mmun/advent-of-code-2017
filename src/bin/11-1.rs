fn solve(dirs: &str) -> i64 {
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut z = 0_i64;

    for dir in dirs.trim().split(',') {
        match dir {
            "ne" => { x += 1; z -= 1; },
            "sw" => { x -= 1; z += 1; },
            "se" => { x += 1; y -= 1; },
            "nw" => { x -= 1; y += 1; },
            "n" => { y += 1; z -= 1; },
            "s" => { y -= 1; z += 1; },
            _ => unreachable!()
        }
    }

    (x.abs() + y.abs() + z.abs()) / 2
}

fn main() {
    println!("{}", solve(include_str!("../../data/11.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("ne,ne,ne"), 3);
        assert_eq!(solve("ne,ne,sw,sw"), 0);
        assert_eq!(solve("ne,ne,s,s"), 2);
        assert_eq!(solve("se,sw,se,sw,sw"), 3);
    }
}
 