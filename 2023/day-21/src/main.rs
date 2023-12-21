const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

use std::collections::HashMap;
use std::collections::VecDeque;

fn distances(root: (usize, usize), grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), usize> {
    let mut result: HashMap<(usize, usize), usize> = HashMap::from([((root.0, root.1), 0)]);
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::from([(root.0, root.1, 0)]);

    while let Some((row, col, depth)) = queue.pop_front() {
        for (dr, dc) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)] {
            let (nr, nc, nd) = (row.wrapping_add(dr), col.wrapping_add(dc), depth + 1);
            if (0..grid.len()).contains(&nr)
                && (0..grid[0].len()).contains(&nc)
                && !result.contains_key(&(nr, nc))
                && grid[nr][nc] != '#'
            {
                result.insert((nr, nc), nd);
                queue.push_back((nr, nc, nd));
            }
        }
    }

    return result;
}

fn part_1(input: &str, depth: usize) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let root = (0..grid.len())
        .find_map(|r| {
            (0..grid[0].len()).find_map(|c| {
                if grid[r][c] == 'S' {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
        .unwrap();
    return distances(root, &grid)
        .values()
        .filter(|v| **v <= depth && **v % 2 == 0)
        .count();
}

fn part_2(input: &str, depth: usize) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let root = (0..grid.len())
        .find_map(|r| {
            (0..grid[0].len()).find_map(|c| {
                if grid[r][c] == 'S' {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let n = (depth - (grid.len() / 2)) / grid.len();

    let distances = distances(root, &grid);

    let even_corners = distances
        .values()
        .filter(|v| **v > 65 && **v % 2 == 0)
        .count();
    let even = n * n;

    let odd_corners = distances
        .values()
        .filter(|v| **v > 65 && **v % 2 == 1)
        .count();
    let odd = (n + 1) * (n + 1);

    return odd * distances.values().filter(|v| **v % 2 == 1).count()
        + even * distances.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT, 64));
    println!("Part 2: {:?}", part_2(INPUT, 26501365));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/example.txt"));

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT, 6), 16);
    }
}
