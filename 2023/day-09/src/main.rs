const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn predict(numbers: Vec<i64>) -> i64 {
    if numbers.iter().any(|i| *i != 0) {
        return predict(
            numbers
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        ) + numbers.last().unwrap();
    }

    return 0;
}

fn part_1(input: &str) -> i64 {
    let parse = |line: &str| line.split(' ').map(|word| word.parse().unwrap()).collect();
    return input.lines().map(|i| predict(parse(i))).sum();
}

fn part_2(input: &str) -> i64 {
    let parse = |line: &str| {
        line.split(' ')
            .map(|word| word.parse().unwrap())
            .rev()
            .collect()
    };

    return input.lines().map(|line| predict(parse(line))).sum();
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/example.txt"));

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 114);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 2);
    }
}
