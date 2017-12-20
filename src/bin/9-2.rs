fn score(input: &str) -> i64 {
    let mut chars = input.chars();

    let mut in_garbage = false;
    let mut garbage_char_count = 0;

    while let Some(chr) = chars.next() {
        if in_garbage {
            match chr {
                '>' => { in_garbage = false; },
                '!' => { chars.next(); },
                _ => { garbage_char_count += 1; }
            }
        } else {
            match chr {
                '<' => { in_garbage = true; },
                _ => {}
            }            
        }
    }

    garbage_char_count
}

fn main() {
    println!("{}", score(include_str!("../../data/9.in")));
}

#[cfg(test)]
mod tests {
    use super::score;

    #[test]
    fn examples() {
        assert_eq!(score("<>"), 0);
        assert_eq!(score("<random characters>"), 17);
        assert_eq!(score("<<<<>"), 3);
        assert_eq!(score("<{!>}>"), 2);
        assert_eq!(score("<!!>"), 0);
        assert_eq!(score("<!!!>>"), 0);
        assert_eq!(score("<{o\"i!a,<{i<a>"), 10);
    }
}
 