use std::collections::HashMap;

fn helper<T>(mut it: T) -> Option<u32>
where
    T: Iterator<Item = char>,
{
    let map: std::collections::HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut buf: String = String::new();
    while let Some(x) = it.next() {
        buf.push(x);
        if map.contains_key(buf.as_str()) {
            return Some(map.get(buf.as_str()).unwrap().clone());
        }
    }

    return None;
}

pub fn solution(input: &[&str]) -> u32 {
    let mut sum = 0;

    for instr in input {
        let mut first: u32 = 0;

        let mut it = instr.chars();
        loop {
            let iterator = it.clone();
            if let None = it.next() {
                break;
            }

            if let Some(num) = helper(iterator.clone()) {
                first = num;
                break;
            } else if let Some(x) = iterator.clone().next() {
                if x.is_digit(10) {
                    first = x.to_digit(10).unwrap();
                    break;
                }
            }
        }

        let mut last: u32 = first;
        loop {
            let iterator = it.clone();
            if let None = it.next() {
                break;
            }

            if let Some(num) = helper(iterator.clone()) {
                last = num;
            } else if let Some(x) = iterator.clone().next() {
                if x.is_digit(10) {
                    last = x.to_digit(10).unwrap();
                }
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
            solution(&[
                "two1nine",
                "eightwothree",
                "abcone2threexyz",
                "xtwone3four",
                "4nineeightseven2",
                "zoneight234",
                "7pqrstsixteen"
            ]),
            281,
        );
    }
}
