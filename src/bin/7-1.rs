use std::collections::HashSet;

fn solve(input: &str) -> &str {
    let mut parents = HashSet::new();
    let mut children = HashSet::new();

    for line in input.trim().lines() {
        let mut line_parts = line.split(" -> ");

        let before_arrow = line_parts.next();
        let after_arrow = line_parts.next();

        let mut parent_tokens = before_arrow.unwrap().split_whitespace();

        parents.insert(parent_tokens.next().unwrap());

        if let Some(after_arrow) = after_arrow {
            for child_name in after_arrow.split(", ") {
                children.insert(child_name);
            }
        }
    }

    parents.difference(&children).next().unwrap()
}

fn main() {
    println!("{}", solve(include_str!("../../data/7.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("
            pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)
        "), "tknk");
    }
}
