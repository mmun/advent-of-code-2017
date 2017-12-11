fn solve(input: &str) -> String {
    let mut numbers = (0..256).collect::<Vec<_>>();
    let mut index = 0;
    let mut skip_count = 0;
    
    let mut lengths = input.as_bytes().iter().map(|&b| b as usize).collect::<Vec<_>>();
    lengths.extend(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for length in &lengths  {
            for i in 0..length/2 {
                numbers.swap((index + i) % 256, (index + length - 1 - i) % 256);
            }

            index = (index + length + skip_count) % 256;
            skip_count += 1;
        }
    }

    let mut output = String::new();

    for block in numbers.chunks(16) {
        output += &format!("{:02x}", block.iter().fold(0, |acc, &num| acc ^ num));
    }

    output
}

fn main() {
    println!("{}", solve("183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88"));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(solve("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(solve("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(solve("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
