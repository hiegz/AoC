use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn count(input: &Vec<Vec<char>>, position: (i64, i64, i64, i64)) -> usize {
    type Position = (i64, i64, i64, i64);
    let mut seen: Vec<Position> = Vec::new();
    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back(position);
    while !queue.is_empty() {
        let (row, col, rdelta, cdelta) = queue.pop_back().unwrap();
        let (row, col) = (row + rdelta, col + cdelta);
        if !seen.contains(&(row, col, rdelta, cdelta))
            && (row >= 0 && row < input.len() as i64)
            && (col >= 0 && col < input[0].len() as i64)
        {
            seen.push((row, col, rdelta, cdelta));
            let char = input[row as usize][col as usize];
            if char == '.' || (char == '-' && cdelta != 0) || (char == '|' && rdelta != 0) {
                queue.push_back((row, col, rdelta, cdelta));
            } else if char == '/' {
                queue.push_back((row, col, -cdelta, -rdelta));
            } else if char == '\\' {
                queue.push_back((row, col, cdelta, rdelta));
            } else {
                for dir in if char == '-' {
                    [(0, -1), (0, 1)]
                } else {
                    [(-1, 0), (1, 0)]
                } {
                    queue.push_back((row, col, dir.0, dir.1));
                }
            }
        }
    }
    return seen
        .iter()
        .map(|(row, col, _, _)| (row, col))
        .collect::<HashSet<_>>()
        .len();
}

fn part_1(input: &str) -> u64 {
    return count(
        &input
            .lines()
            .map(|line| line.chars().map(|c| c).collect())
            .collect(),
        (0, -1, 0, 1),
    ) as u64;
}

fn part_2(input: &str) -> u64 {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<_>>())
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let max = (0..rows).fold(0, |acc, r| {
        usize::max(
            usize::max(acc, count(&grid, (r as i64, -1, 0, 1))),
            count(&grid, (r as i64, rows as i64, 0, -1)),
        )
    });
    return (0..cols).fold(max, |acc, c| {
        usize::max(
            usize::max(acc, count(&grid, (-1, c as i64, 1, 0))),
            count(&grid, (cols as i64, c as i64, -1, 0)),
        )
    }) as u64;
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
        assert_eq!(super::part_1(INPUT), 46);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 51);
    }
}
