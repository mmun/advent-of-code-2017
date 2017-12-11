fn solve(size: usize, lengths: &str) -> usize {
    let mut numbers = (0..size).collect::<Vec<usize>>();
    let mut index = 0;
    let mut skip = 0;
    
    for length in lengths.split(',') {
        let length = length.parse::<usize>().unwrap();

        for i in 0..length/2 {
            numbers.swap((index + i) % size, (index + length - 1 - i) % size);
        }

        index = (index + length + skip) % size;
        skip += 1;
    }


    numbers[0] * numbers[1]
}

fn main() {
    println!("{}", solve(256, "183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88"));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve(5, "3,4,1,5"), 12);
    }
}
 