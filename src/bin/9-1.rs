fn score(input: &str) -> i64 {
    let mut chars = input.chars();

    let mut depth = 0;
    let mut score = 0;
    let mut in_garbage = false;

    while let Some(chr) = chars.next() {
        if in_garbage {
            match chr {
                '>' => { in_garbage = false; },
                '!' => { chars.next(); },
                _ => {}
            }
        } else {
            match chr {
                '{' => { depth += 1; },
                '}' => { depth -= 1; score += depth + 1; },
                '<' => { in_garbage = true; },
                _ => {}
            }            
        }
    }

    score
}

fn main() {
    println!("{}", score(include_str!("../../data/9.in")));
}

#[cfg(test)]
mod tests {
    use super::score;

    #[test]
    fn examples() {
        assert_eq!(score("{}"), 1);
        assert_eq!(score("{{{}}}"), 6);
        assert_eq!(score("{{},{}}"), 5);
        assert_eq!(score("{{{},{},{{}}}}"), 16);
        assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
 