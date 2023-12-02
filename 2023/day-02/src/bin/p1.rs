pub fn solution(input: &[&str]) -> u32 {
    let mut sum = 0;
    'root: for instr in input {
        let mut id: u32 = 0;
        let mut u32_buf: u32 = 0;
        let mut str_buf: String = String::new();

        let mut iterator = instr.chars();
        while let Some(c) = iterator.next() {
            if c == ' ' {
                str_buf.clear();
            } else if c == ':' {
                id = str_buf.parse::<u32>().unwrap();
                str_buf.clear();
                break;
            } else {
                str_buf.push(c);
            }
        }

        iterator.next();

        while let Some(c) = iterator.next() {
            if c == ' ' && str_buf.len() > 0 {
                u32_buf = str_buf.parse::<u32>().unwrap();
                str_buf.clear();
            } else if c == ',' || c == ';' {
                if (str_buf == "red" && u32_buf > 12)
                    || (str_buf == "green" && u32_buf > 13)
                    || (str_buf == "blue" && u32_buf > 14)
                {
                    continue 'root;
                } else {
                    str_buf.clear();
                }
            } else if c != ' ' {
                str_buf.push(c);
            }
        }

        if (str_buf == "red" && u32_buf > 12)
            || (str_buf == "green" && u32_buf > 13)
            || (str_buf == "blue" && u32_buf > 14)
        {
            continue 'root;
        }

        sum += id;
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
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ]),
            8,
        );
    }
}
