fn get3(input: &[&str], row: usize, col: usize) -> (char, char, char) {
    (
        if row > 0 && row < input.len() && col < input[row].len() {
            input[row - 1].chars().nth(col).unwrap()
        } else {
            '.'
        },
        if row < input.len() && col < input[row].len() {
            input[row].chars().nth(col).unwrap()
        } else {
            '.'
        },
        if row + 1 < input.len() && row + 1 > 0 && col < input[row].len() {
            input[row + 1].chars().nth(col).unwrap()
        } else {
            '.'
        },
    )
}

fn solution(input: &[&str]) -> u32 {
    let mut sum = 0;
    for row in 0..input.len() {
        let mut is_adjacent: bool = false;
        let mut buf: String = String::new();
        let mut col: usize = 0;
        while col <= input[row].len() {
            let (top, cen, bot) = get3(input, row, col);

            is_adjacent = (is_adjacent && buf.len() > 0)
                || (is_adjacent && cen.is_digit(10))
                || (top.is_ascii_punctuation() && top != '.')
                || (cen.is_ascii_punctuation() && cen != '.')
                || (bot.is_ascii_punctuation() && bot != '.');

            // First: Write to temporary buffer or flush it
            if cen.is_digit(10) {
                buf.push(cen);
            } else {
                if is_adjacent && buf.len() > 0 {
                    sum += buf.parse::<u32>().unwrap();
                    buf.clear();
                    continue;
                }

                buf.clear();
            }

            col += 1;
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
            4361,
        );
    }
}
