type NodeIndex = usize;
type Adj = Vec<Vec<NodeIndex>>;
type Seen = Vec<bool>;

fn solve(input: &str) -> i64 {
    let mut adj: Adj = Vec::new();
    let mut seen: Seen = Vec::new();

    for line in input.trim().lines() {
        let mut tokens = line.trim().split(" <-> ");

        tokens.next();
        
        let children = tokens.next().unwrap()
                .split(", ")
                .map(|t| t.parse().unwrap())
                .collect();

        adj.push(children);
        seen.push(false);
    }

    count(0, &adj, &mut seen)
}

fn count(i: NodeIndex, adj: &Adj, seen: &mut Seen) -> i64 {
    seen[i] = true;

    let mut sum = 1;

    for &j in &adj[i] {
        if !seen[j] {
            sum += count(j, adj, seen);
        }
    }

    sum
}

fn main() {
    println!("{}", solve(include_str!("../../data/12.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("
            0 <-> 2
            1 <-> 1
            2 <-> 0, 3, 4
            3 <-> 2, 4
            4 <-> 2, 3, 6
            5 <-> 6
            6 <-> 4, 5
        "), 6);
    }
}
 