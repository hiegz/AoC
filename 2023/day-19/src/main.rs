const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn part_1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: std::collections::HashMap<&str, Vec<&str>> = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            return (name, rest[..rest.len() - 1].split(',').collect::<Vec<_>>());
        })
        .collect();

    let parts: Vec<Vec<u64>> = parts
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(',')
                .map(|category| category[2..].parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut accepted: usize = 0;
    for part in parts.iter() {
        let mut key = "in";

        while key != "A" && key != "R" {
            for condition in workflows[key].iter().copied() {
                if let Some((condition, next)) = condition.split_once(':') {
                    let category = condition.chars().nth(0).unwrap();
                    let index = "xmas".chars().position(|c| c == category).unwrap();
                    let value = condition[2..].parse::<u64>().unwrap();
                    if (condition.chars().nth(1).unwrap() == '>' && part[index] > value)
                        || (condition.chars().nth(1).unwrap() == '<' && part[index] < value)
                    {
                        key = next;
                        break;
                    }
                } else {
                    key = condition;
                }
            }
        }

        if key == "A" {
            accepted += (&part).iter().sum::<u64>() as usize;
        }
    }

    return accepted;
}

fn part_2(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let mut accepted: usize = 0;
    let workflows: std::collections::HashMap<&str, Vec<&str>> = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            return (name, rest[..rest.len() - 1].split(',').collect::<Vec<_>>());
        })
        .collect();

    let mut queue: std::collections::VecDeque<([std::ops::RangeInclusive<u64>; 4], &str)> =
        std::collections::VecDeque::new();
    queue.push_back(([(1..=4000), (1..=4000), (1..=4000), (1..=4000)], "in"));

    while let Some((mut ranges, key)) = queue.pop_back() {
        if key == "R" {
            continue;
        } else if key == "A" {
            accepted += ranges[0].clone().count()
                * ranges[1].clone().count()
                * ranges[2].clone().count()
                * ranges[3].clone().count();
            continue;
        }
        for condition in workflows[key].iter().copied() {
            if let Some((condition, next)) = condition.split_once(':') {
                let category = condition.chars().nth(0).unwrap();
                let index = "xmas".chars().position(|c| c == category).unwrap();
                let value = condition[2..].parse::<u64>().unwrap();
                let mut nranges = ranges.clone();
                if condition.chars().nth(1).unwrap() == '<' {
                    nranges[index] = *nranges[index].start()..=value - 1;
                    ranges[index] = value..=*ranges[index].end();
                } else if condition.chars().nth(1).unwrap() == '>' {
                    nranges[index] = value + 1..=*nranges[index].end();
                    ranges[index] = *ranges[index].start()..=value;
                }
                queue.push_back((nranges, next));
            } else {
                queue.push_back((ranges.clone(), condition));
            }
        }
    }

    return accepted;
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
        assert_eq!(super::part_1(INPUT), 19114);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 167409079868000);
    }
}
