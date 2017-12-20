use std::collections::HashMap;

fn solve(input: &str) -> i64 {
    let mut registers = HashMap::new();
    let mut max_value = 0;

    for line in input.trim().lines() {
        let mut tokens = line.split_whitespace();
        
        let target_register_name = tokens.next().unwrap();
        let target_operator = tokens.next().unwrap();
        let target_argument = tokens.next().unwrap().parse::<i64>().unwrap();
        tokens.next().unwrap();
        let test_register_name = tokens.next().unwrap();
        let test_operator = tokens.next().unwrap();
        let test_argument = tokens.next().unwrap().parse::<i64>().unwrap();

        let test = {
                let test_register = registers.entry(test_register_name).or_insert(0);

                match test_operator {
                    ">" => *test_register > test_argument,
                    "<" => *test_register < test_argument,
                    ">=" => *test_register >= test_argument,
                    "<=" => *test_register <= test_argument,
                    "==" => *test_register == test_argument,
                    "!=" => *test_register != test_argument,
                    _ => unreachable!()
                }
        };

        if test {
            let target_register = registers.entry(target_register_name).or_insert(0);

            match target_operator {
                "inc" => *target_register += target_argument,
                "dec" => *target_register -= target_argument,
                _ => unreachable!()
            }

            if max_value < *target_register {
                max_value = *target_register;
            }
        }
    }

    max_value
}

fn main() {
    println!("{}", solve(include_str!("../../data/8.in")));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn examples() {
        assert_eq!(solve("
            b inc 5 if a > 1
            a inc 1 if b < 5
            c dec -10 if a >= 1
            c inc -20 if c == 10
        "), 1);
    }
}
