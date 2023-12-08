const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn parse(input: &str) -> (String, std::collections::HashMap<String, (String, String)>) {
    use regex::Regex;
    use std::collections::HashMap;
    let r = Regex::new("[1-9a-zA-Z]{3}").unwrap();
    let (steps, elements) = input.split_once("\n\n").unwrap();
    let elements = elements
        .lines()
        .map(|line| r.find_iter(line).map(|m| m.as_str()).collect::<Vec<_>>())
        .map(|a| (a[0].to_owned(), (a[1].to_owned(), a[2].to_owned())))
        .collect::<HashMap<_, _>>();
    return (String::from(steps), elements);
}

fn part_1(input: &str) -> u64 {
    let (steps, elements) = parse(input);
    let mut element = elements.iter().find(|k| k.0 == "AAA").unwrap().0;
    let mut index = 0;
    while !element.ends_with('Z') {
        let curr = &elements.get(element).unwrap();
        let instruction = steps.chars().nth(index % steps.len()).unwrap();
        element = if instruction == 'L' { &curr.0 } else { &curr.1 };
        index += 1;
    }
    index as u64
}

fn part_2(input: &str) -> u64 {
    let (steps, elements) = parse(input);
    let element = elements
        .iter()
        .filter(|k| k.0.ends_with('A'))
        .collect::<Vec<_>>();
    element
        .iter()
        .map(|start| {
            let mut index = 0;
            let mut element = start.0;
            while !element.ends_with('Z') {
                let curr = &elements.get(element).unwrap();
                let instruction = steps.chars().nth(index % steps.len()).unwrap();
                element = if instruction == 'L' { &curr.0 } else { &curr.1 };
                index += 1;
            }
            index
        })
        .fold(1, num::integer::lcm) as u64
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(
            super::part_1(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/data/p1.txt"
            ))),
            6
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            super::part_2(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/data/p2.txt"
            ))),
            6
        );
    }
}
