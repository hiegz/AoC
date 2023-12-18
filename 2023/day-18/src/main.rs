const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn part_1(input: &str) -> usize {
    let dirs: std::collections::HashMap<char, (usize, usize)> = std::collections::HashMap::from([
        ('U', (usize::MAX, 0)),
        ('R', (0, 1)),
        ('D', (1, 0)),
        ('L', (0, usize::MAX)),
    ]);

    let mut boundaries = 0;
    let mut points: Vec<(usize, usize)> = vec![(0, 0)];
    for line in input.lines() {
        let (lr, lc) = points.last().unwrap();
        let words: Vec<_> = line.split(' ').collect();
        let (dir, n) = (
            dirs[&words[0].chars().nth(0).unwrap()],
            words[1].parse::<usize>().unwrap(),
        );
        boundaries += n;
        points.push((0..n).fold((*lr, *lc), |acc, _| {
            (acc.0.wrapping_add(dir.0), acc.1.wrapping_add(dir.1))
        }));
    }

    return i64::abs(
        (0..points.len())
            .map(|i| {
                return points[i].0 as i64
                    * (points[(i + points.len() - 1) % points.len()].1 as i64
                        - points[(i + 1) % points.len()].1 as i64);
            })
            .sum::<i64>()
            / 2,
    ) as usize
        - boundaries / 2
        + 1
        + boundaries;
}

fn part_2(input: &str) -> usize {
    let dirs: std::collections::HashMap<char, (usize, usize)> = std::collections::HashMap::from([
        ('U', (usize::MAX, 0)),
        ('R', (0, 1)),
        ('D', (1, 0)),
        ('L', (0, usize::MAX)),
    ]);

    let mut boundaries = 0;
    let mut points: Vec<(usize, usize)> = vec![(0, 0)];
    for line in input.lines() {
        let (lr, lc) = points.last().unwrap();
        let words: Vec<_> = line.split(' ').collect();
        let (dir, n) = (
            dirs[&['R', 'D', 'L', 'U'][words[2]
                .chars()
                .nth(words[2].len() - 2)
                .unwrap()
                .to_digit(10)
                .unwrap() as usize]],
            usize::from_str_radix(&words[2][2..words[2].len() - 2], 16).unwrap(),
        );
        boundaries += n;
        points.push((0..n).fold((*lr, *lc), |acc, _| {
            (acc.0.wrapping_add(dir.0), acc.1.wrapping_add(dir.1))
        }));
    }

    return i64::abs(
        (0..points.len())
            .map(|i| {
                return points[i].0 as i64
                    * (points[(i + points.len() - 1) % points.len()].1 as i64
                        - points[(i + 1) % points.len()].1 as i64);
            })
            .sum::<i64>()
            / 2,
    ) as usize
        - boundaries / 2
        + 1
        + boundaries;
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
        assert_eq!(super::part_1(INPUT), 62);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 952408144115);
    }
}
