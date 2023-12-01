pub fn solution(input: &[&str]) -> u32 {
    let mut sum: u32 = 0;

    for instr in input {
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        for c in instr.chars() {
            if c.is_digit(10) {
                first = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in instr.chars().rev() {
            if c.is_digit(10) {
                last = c.to_digit(10).unwrap();
                break;
            }
        }

        sum += first * 10 + last;
    }

    return sum;
}

fn main() {
    let mut input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"))
        .split("\n")
        .collect::<Vec<&str>>();
    input.pop();
    println!("{:?}", solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            solution(&["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]),
            142,
        );
    }
}
