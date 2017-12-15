type Row = [u8; 16];
type Grid = [Row; 128];

fn knot_hash(input: &[u8]) -> Row {
    let mut numbers = [0; 256];

    for i in 0..256 {
        numbers[i] = i as u8;
    }

    let mut index = 0;
    let mut skip_count = 0;

    for _ in 0..64 {
        for &byte in input.iter().chain(&[17, 31, 73, 47, 23])  {
            let length = byte as usize;

            for i in 0..length/2 {
                numbers.swap((index + i) % 256, (index + length - 1 - i) % 256);
            }

            index = (index + length + skip_count) % 256;
            skip_count += 1;
        }
    }

    let mut output = [0; 16];

    for (byte, block) in output.iter_mut().zip(numbers.chunks(16)) {
        *byte = block.iter().fold(0, |acc, &num| acc ^ num);
    }

    output
}

fn solve(input: &str) -> u64 {
    let mut used = [[0; 16]; 128];

    for i in 0..128 {
        let hash_input = format!("{}-{}", input, i);
        used[i] = knot_hash(hash_input.as_bytes());
    }

    let mut seen = [[0; 16]; 128];
    let mut region_count = 0;

    for i in 0..128 {
        for j in 0..128 {
            if is_unseen(&used, &seen, i, j) {
                region_count += 1;
                visit(&used, &mut seen, i, j);
            }
        }
    }

    region_count
}

fn visit(used: &Grid, seen: &mut Grid, i: usize, j: usize) {
    see(seen, i, j);

    if i > 0 && is_unseen(used, seen, i-1, j) { visit(used, seen, i-1, j) }
    if j > 0 && is_unseen(used, seen, i, j-1) { visit(used, seen, i, j-1) }
    if i < 127 && is_unseen(used, seen, i+1, j) { visit(used, seen, i+1, j) }
    if j < 127 && is_unseen(used, seen, i, j+1) { visit(used, seen, i, j+1) }
}

fn is_unseen(used: &Grid, seen: &Grid, i: usize, j: usize) -> bool {
    let k = j/8;
    let m = 1 << (7 - j%8);

    (used[i][k] & m != 0) && (seen[i][k] & m == 0)
}

fn see(seen: &mut Grid, i: usize, j: usize) {
    seen[i][j/8] |= 1 << (7-j%8);
}

fn main() {
    println!("{}", solve("hxtvlmkl"));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("flqrgnkx"), 1242);
    }
}
 