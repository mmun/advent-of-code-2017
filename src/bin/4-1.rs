fn check(passphrase: &str) -> bool {
    let mut words = passphrase
        .split_whitespace()
        .collect::<Vec<_>>();

    let word_count = words.len();

    words.sort();
    words.dedup();

    words.len() == word_count
}

fn main() {
    let input = include_str!("4.in").trim();
    let passphrases = input.lines();

    println!("{}", passphrases.filter(|p| check(p)).count());
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn examples() {
        assert_eq!(check("aa bb cc dd ee"), true);
        assert_eq!(check("aa bb cc dd aa"), false);
        assert_eq!(check("aa bb cc dd aaa"), true);
    }
}
