fn solve(steps: usize) -> u64 {
    let mut cursor = 0;
    let mut after = 0;

    for i in 1..50_000_001 {
        cursor = (cursor + steps) % i;
        if cursor == 0 {
            after = i as u64;
        }
        cursor = (cursor + 1) % (i+1);
    }

    after
}

fn main() {
    println!("{}", solve(394));
}
