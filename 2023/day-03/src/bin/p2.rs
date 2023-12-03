fn find_num(input: &[&str], row: usize, col: usize) -> Result<u32, std::num::ParseIntError> {
    let mut buf = String::from(input[row].chars().nth(col).unwrap());

    let mut left: usize = col - 1;
    let mut right: usize = col + 1;

    while let Some(c) = input[row].chars().nth(left) {
        if c.is_digit(10) {
            buf.insert(0, c);
            if left == 0 {
                break;
            } else {
                left -= 1;
            }
        } else {
            break;
        }
    }

    while let Some(c) = input[row].chars().nth(right) {
        if c.is_digit(10) {
            buf.push(c);
            right += 1;
        } else {
            break;
        }
    }

    return buf.parse::<u32>();
}

fn solution(input: &[&str]) -> u32 {
    let mut sum = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let ch = input[row].chars().nth(col).unwrap();
            if ch == '*' {
                let mut vec: Vec<u32> = Vec::new();

                if col > 0 {
                    if let Ok(n) = find_num(input, row, col - 1) {
                        vec.push(n);
                    }
                }

                if col + 1 < input[row].len() {
                    if let Ok(n) = find_num(input, row, col + 1) {
                        vec.push(n);
                    }
                }

                if row + 1 < input.len() {
                    if let Ok(n) = find_num(input, row + 1, col) {
                        vec.push(n);
                    } else {
                        if col > 0 {
                            if let Ok(n) = find_num(input, row + 1, col - 1) {
                                vec.push(n);
                            }
                        }

                        if col + 1 < input[row + 1].len() {
                            if let Ok(n) = find_num(input, row + 1, col + 1) {
                                vec.push(n);
                            }
                        }
                    }
                }

                if row > 0 {
                    if let Ok(n) = find_num(input, row - 1, col) {
                        vec.push(n);
                    } else {
                        if col > 0 {
                            if let Ok(n) = find_num(input, row - 1, col - 1) {
                                vec.push(n);
                            }
                        }

                        if col + 1 < input[row - 1].len() {
                            if let Ok(n) = find_num(input, row - 1, col + 1) {
                                vec.push(n);
                            }
                        }
                    }
                }

                if vec.len() == 2 {
                    sum += vec[0] * vec[1];
                }
            }
        }
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
                "467..114..",
                "...*......",
                "..35..633.",
                "......#...",
                "617*......",
                ".....+.58.",
                "..592.....",
                "......755.",
                "...$.*....",
                ".664.598..",
            ]),
            467835,
        );
    }
}
