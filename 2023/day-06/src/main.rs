const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn parse_1(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

fn part_1(input: &str) -> u64 {
    let mut input = input.lines();
    let time = parse_1(input.next().unwrap());
    let duration = parse_1(input.next().unwrap());
    let mut counter: u64 = 1;
    for i in 0..time.len() {
        counter *= (0..time[i])
            .filter(|x| x * (time[i] - x) > duration[i])
            .count() as u64;
    }
    return counter;
}

fn parse_2(line: &str) -> u64 {
    line.chars()
        .filter(|c| (*c).is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let mut input = input.lines();
    let time = parse_2(input.next().unwrap());
    let duration = parse_2(input.next().unwrap());
    return (0..time).filter(|x| x * (time - x) > duration).count() as u64;
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
        assert_eq!(super::part_1(INPUT), 288);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 71503);
    }
}
