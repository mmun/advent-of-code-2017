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

    let mut count = 0;

    for i in 0..adj.len() {
        if !seen[i] {
            count += 1;
            visit(i, &adj, &mut seen);
        }
    }
    
    count
}

fn visit(i: NodeIndex, adj: &Adj, seen: &mut Seen) {
    seen[i] = true;

    for &j in &adj[i] {
        if !seen[j] {
            visit(j, adj, seen);
        }
    }
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
        "), 2);
    }
}
 