fn sort_str(word: &str) -> String {
    let mut chars = word.chars().collect::<Vec<_>>();
    chars.sort();
    chars.iter().collect()
}

fn check(passphrase: &str) -> bool {
    let mut words = passphrase
        .split_whitespace()
        .map(sort_str)
        .collect::<Vec<_>>();

    let word_count = words.len();

    words.sort();
    words.dedup();

    words.len() == word_count
}

fn main() {
    let input = include_str!("../../data/4.in").trim();
    let passphrases = input.lines();

    println!("{}", passphrases.filter(|p| check(p)).count());
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn examples() {
        assert_eq!(check("abcde fghij"), true);
        assert_eq!(check("abcde xyz ecdab"), false);
        assert_eq!(check("a ab abc abd abf abj"), true);
        assert_eq!(check("iiii oiii ooii oooi oooo"), true);
        assert_eq!(check("oiii ioii iioi iiio"), false);
    }
}
