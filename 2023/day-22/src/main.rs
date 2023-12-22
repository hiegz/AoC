use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn parse(input: &str) -> Vec<Vec<Vec<usize>>> {
    return input
        .lines()
        .map(|line| {
            line.split('~')
                .map(|w| w.split(',').map(|c| c.parse().unwrap()).collect())
                .collect()
        })
        .collect();
}

fn part_1(input: &str) -> usize {
    let mut bricks = parse(input);
    bricks.sort_by(|a, b| a[0][2].cmp(&b[0][2]));
    for upper in 0..bricks.len() {
        let mut zlevel = 1;
        for lower in 0..upper {
            if usize::max(bricks[upper][0][0], bricks[lower][0][0])
                <= usize::min(bricks[upper][1][0], bricks[lower][1][0])
                && usize::max(bricks[upper][0][1], bricks[lower][0][1])
                    <= usize::min(bricks[upper][1][1], bricks[lower][1][1])
            {
                zlevel = usize::max(zlevel, bricks[lower][1][2] + 1);
            }
        }
        bricks[upper][1][2] = (bricks[upper][1][2] - bricks[upper][0][2]) + zlevel;
        bricks[upper][0][2] = zlevel;
    }

    bricks.sort_by(|a, b| a[0][2].cmp(&b[0][2]));

    let mut supporters: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut supported_by: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();

    for upper in 0..bricks.len() {
        for lower in 0..upper {
            if usize::max(bricks[upper][0][0], bricks[lower][0][0])
                <= usize::min(bricks[upper][1][0], bricks[lower][1][0])
                && usize::max(bricks[upper][0][1], bricks[lower][0][1])
                    <= usize::min(bricks[upper][1][1], bricks[lower][1][1])
                && bricks[lower][1][2] + 1 == bricks[upper][0][2]
            {
                supporters.get_mut(&lower).unwrap().insert(upper);
                supported_by.get_mut(&upper).unwrap().insert(lower);
            }
        }
    }

    return (0..bricks.len())
        .filter(|index| {
            supporters[&index]
                .iter()
                .all(|p| supported_by[p].len() >= 2)
        })
        .count();
}

fn part_2(input: &str) -> usize {
    let mut bricks = parse(input);
    bricks.sort_by(|a, b| a[0][2].cmp(&b[0][2]));
    for upper in 0..bricks.len() {
        let mut zlevel = 1;
        for lower in 0..upper {
            if usize::max(bricks[upper][0][0], bricks[lower][0][0])
                <= usize::min(bricks[upper][1][0], bricks[lower][1][0])
                && usize::max(bricks[upper][0][1], bricks[lower][0][1])
                    <= usize::min(bricks[upper][1][1], bricks[lower][1][1])
            {
                zlevel = usize::max(zlevel, bricks[lower][1][2] + 1);
            }
        }
        bricks[upper][1][2] = (bricks[upper][1][2] - bricks[upper][0][2]) + zlevel;
        bricks[upper][0][2] = zlevel;
    }

    bricks.sort_by(|a, b| a[0][2].cmp(&b[0][2]));

    let mut supporters: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut supported_by: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();

    for upper in 0..bricks.len() {
        for lower in 0..upper {
            if usize::max(bricks[upper][0][0], bricks[lower][0][0])
                <= usize::min(bricks[upper][1][0], bricks[lower][1][0])
                && usize::max(bricks[upper][0][1], bricks[lower][0][1])
                    <= usize::min(bricks[upper][1][1], bricks[lower][1][1])
                && bricks[lower][1][2] + 1 == bricks[upper][0][2]
            {
                supporters.get_mut(&lower).unwrap().insert(upper);
                supported_by.get_mut(&upper).unwrap().insert(lower);
            }
        }
    }

    return (0..bricks.len())
        .map(|index| {
            let f = |j: &usize| supporters[&index].contains(&j) && supported_by[j].len() == 1;
            let mut queue: VecDeque<usize> = (index + 1..bricks.len()).filter(f).collect();
            let mut seen: HashSet<usize> = (index + 1..bricks.len()).filter(f).collect();
            while let Some(brick) = queue.pop_back() {
                for supported in supporters[&brick].iter() {
                    if !seen.contains(supported)
                        && supported_by[supported].iter().all(|f| seen.contains(&f))
                    {
                        seen.insert(*supported);
                        queue.push_back(*supported);
                    }
                }
            }
            return seen.len();
        })
        .sum::<usize>();
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
        assert_eq!(super::part_1(INPUT), 5);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 7);
    }
}
