const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn scan(grid: Vec<Vec<char>>, smudges: u64) -> u64 {
    let cols = grid[0].len();
    let mut right_cursor = 1;
    while right_cursor < cols {
        let mut counter = 0;
        let left_cursor = right_cursor - 1;
        let length = usize::min(cols - right_cursor - 1, left_cursor);
        for line in grid.iter() {
            for (left, right) in std::iter::zip(
                &line[left_cursor - length..=left_cursor],
                (&line[right_cursor..=right_cursor + length])
                    .into_iter()
                    .rev(),
            ) {
                if *left != *right {
                    counter += 1;
                }
            }
        }

        if counter == smudges {
            return right_cursor as u64;
        }

        right_cursor += 1;
    }

    return 0;
}

fn part_1(input: &str) -> u64 {
    return input
        .split("\n\n")
        .map(|block| {
            let block: Vec<_> = block
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect();
            return scan(
                (0..block[0].len())
                    .map(|j| (0..block.len()).map(|i| block[i][j]).collect::<Vec<_>>())
                    .collect(),
                0,
            ) * 100
                + scan(block, 0);
        })
        .sum::<u64>();
}

fn part_2(input: &str) -> u64 {
    return input
        .split("\n\n")
        .map(|block| {
            let block: Vec<_> = block
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect();
            return scan(
                (0..block[0].len())
                    .map(|j| (0..block.len()).map(|i| block[i][j]).collect::<Vec<_>>())
                    .collect(),
                1,
            ) * 100
                + scan(block, 1);
        })
        .sum::<u64>();
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
        assert_eq!(super::part_1(INPUT), 405);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 400);
    }
}
