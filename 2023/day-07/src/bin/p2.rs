const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn parse(input: &str) -> Vec<(Vec<u8>, u64)> {
    use std::collections::HashMap;
    let mappings: HashMap<char, u8> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    return input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            return (
                left.chars()
                    .map(|c| *mappings.get(&c).unwrap())
                    .collect::<Vec<u8>>(),
                right.parse::<u64>().unwrap(),
            );
        })
        .collect::<Vec<_>>();
}

fn value(arr: &[u8]) -> u8 {
    use std::collections::HashMap;
    let mut occurances: HashMap<u8, u8> = HashMap::new();
    let mappings: HashMap<(usize, u8), u8> = HashMap::from([
        ((5, 1), 0),
        ((4, 2), 1),
        ((3, 2), 2),
        ((3, 3), 3),
        ((2, 3), 4),
        ((2, 4), 5),
        ((1, 5), 6),
    ]);

    arr.iter().for_each(|card| {
        if let Some(occurance) = occurances.get_mut(card) {
            *occurance += 1;
        } else {
            occurances.insert(*card, 1);
        }
    });

    let mut max = 0;
    let mut jockers = 0;
    let mut length = occurances.len();
    if length == 1 {
        max = occurances[&0];
    } else {
        length = 0;
        for (key, value) in occurances.iter() {
            if *key != 0 {
                max = u8::max(max, *value);
                length += 1;
            } else {
                jockers = *value;
            }
        }
    }

    return *mappings.get(&(length, max + jockers)).unwrap();
}

fn compare(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    let a_value = value(a);
    let b_value = value(b);

    if a_value == b_value {
        let mut a_it = a.iter();
        let mut b_it = b.iter();
        while let (Some(x), Some(y)) = (a_it.next(), b_it.next()) {
            if x > y {
                return std::cmp::Ordering::Greater;
            } else if x < y {
                return std::cmp::Ordering::Less;
            }
        }
    }

    if a_value > b_value {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    }
}

fn solve(input: &str) -> u64 {
    let hands = parse(input);
    let mut indices = hands
        .iter()
        .enumerate()
        .map(|i| i.0)
        .collect::<Vec<usize>>();
    indices.sort_by(|x, y| compare(&hands[*x].0, &hands[*y].0));
    return indices
        .iter()
        .enumerate()
        .map(|x| hands[*x.1].1 * (x.0 as u64 + 1))
        .sum::<u64>();
}

fn main() {
    println!("Solution: {:?}", solve(INPUT));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/example.txt"));

    #[test]
    fn example() {
        assert_eq!(super::solve(INPUT), 5905);
    }
}
