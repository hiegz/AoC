const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    return input
        .lines()
        .map(|line| {
            let (config, groups) = line.split_once(' ').unwrap();
            let config = config.chars().collect();
            let groups = groups.split(',').map(|w| w.parse::<_>().unwrap()).collect();
            return (config, groups);
        })
        .collect();
}

fn count(
    config: Vec<char>,
    groups: Vec<usize>,
    memo: &mut std::collections::HashMap<(Vec<char>, Vec<usize>), u64>,
) -> u64 {
    if config.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    } else if groups.is_empty() {
        return if config.contains(&'#') { 0 } else { 1 };
    }

    if memo.contains_key(&(config.clone(), groups.clone())) {
        return memo[&(config, groups)];
    }

    let mut sum = 0;

    if config[0] == '.' || config[0] == '?' {
        sum += count(config[1..].to_vec(), groups[..].to_vec(), memo);
    }

    if config[0] == '#' || config[0] == '?' {
        if groups[0] <= config.len()
            && !config[..groups[0]].contains(&'.')
            && (groups[0] == config.len() || config[groups[0]] != '#')
        {
            sum += count(
                config[groups[0] + if groups[0] == config.len() { 0 } else { 1 }..].to_vec(),
                groups[1..].to_vec(),
                memo,
            );
        }
    }

    memo.insert((config, groups), sum);
    return sum;
}

fn part_1(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|s| {
            count(
                s.0.clone(),
                s.1.clone(),
                &mut std::collections::HashMap::new(),
            )
        })
        .sum::<u64>()
}

fn part_2(input: &str) -> u64 {
    let mut vec = parse(input);
    for (config, groups) in vec.iter_mut() {
        let (cfg, gr) = (config.clone(), groups.clone());
        for _ in 0..4 {
            config.push('?');
            config.extend(cfg.iter());
            groups.extend(gr.iter());
        }
    }
    return vec
        .iter()
        .map(|s| {
            count(
                s.0.clone(),
                s.1.clone(),
                &mut std::collections::HashMap::new(),
            )
        })
        .sum::<u64>() as u64;
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
        assert_eq!(super::part_1(INPUT), 21);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 525152);
    }
}
