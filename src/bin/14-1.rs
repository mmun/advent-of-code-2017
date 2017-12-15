fn knot_hash(input: &[u8]) -> [u8; 16] {
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
    let mut square_count = 0;

    for i in 0..128 {
        let row_input = format!("{}-{}", input, i);
        let row_hash = knot_hash(row_input.as_bytes());

        square_count += row_hash.iter().map(|b| b.count_ones() as u64).sum::<u64>();
    }

    square_count
}

fn main() {
    println!("{}", solve("hxtvlmkl"));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("flqrgnkx"), 8108);
    }
}
 