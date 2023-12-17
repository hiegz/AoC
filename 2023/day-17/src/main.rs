const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn part_1(input: &str) -> u64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    type Position = (u32, usize, usize, (usize, usize, usize, usize), usize);
    let mut seen: HashSet<(usize, usize, (usize, usize, usize, usize), usize)> = HashSet::new();
    let mut queue: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0, (0, 0, 0, 0), 0)));
    while !queue.is_empty() {
        let Reverse((heat, row, col, dir, n)) = queue.pop().unwrap();
        if !seen.contains(&(row, col, dir, n)) {
            seen.insert((row, col, dir, n));
        } else {
            continue;
        }

        if row == grid.len() - 1 && col == grid[0].len() - 1 {
            return heat as u64;
        }

        if n < 3 && dir != (0, 0, 0, 0) {
            let (up, right, down, left) = dir;
            if row.checked_sub(up).is_some()
                && down.checked_add(row - up).is_some()
                && (row - up + down) < grid.len()
                && col.checked_sub(left).is_some()
                && right.checked_add(col - left).is_some()
                && (col - left + right) < grid[0].len()
            {
                let nrow = row - up + down;
                let ncol = col - left + right;
                queue.push(Reverse((
                    heat + grid[nrow][ncol],
                    nrow,
                    ncol,
                    (up, right, down, left),
                    n + 1,
                )));
            }
        }

        for (up, right, down, left) in [(1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)] {
            if (up, right, down, left) != dir
                && (up, right, down, left) != (dir.2, dir.3, dir.0, dir.1)
            {
                if row.checked_sub(up).is_some()
                    && down.checked_add(row - up).is_some()
                    && (row - up + down) < grid.len()
                    && col.checked_sub(left).is_some()
                    && right.checked_add(col - left).is_some()
                    && (col - left + right) < grid[0].len()
                {
                    let nrow = row - up + down;
                    let ncol = col - left + right;
                    queue.push(Reverse((
                        heat + grid[nrow][ncol],
                        nrow,
                        ncol,
                        (up, right, down, left),
                        1,
                    )));
                }
            }
        }
    }

    return 0;
}

fn part_2(input: &str) -> u64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    type Position = (u32, usize, usize, (usize, usize, usize, usize), usize);
    let mut seen: HashSet<(usize, usize, (usize, usize, usize, usize), usize)> = HashSet::new();
    let mut queue: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0, (0, 0, 0, 0), 0)));
    while !queue.is_empty() {
        let Reverse((heat, row, col, dir, n)) = queue.pop().unwrap();
        if !seen.contains(&(row, col, dir, n)) {
            seen.insert((row, col, dir, n));
        } else {
            continue;
        }

        if row == grid.len() - 1 && col == grid[0].len() - 1 && n >= 4 {
            return heat as u64;
        }

        if n < 10 && dir != (0, 0, 0, 0) {
            let (up, right, down, left) = dir;
            if row.checked_sub(up).is_some()
                && down.checked_add(row - up).is_some()
                && (row - up + down) < grid.len()
                && col.checked_sub(left).is_some()
                && right.checked_add(col - left).is_some()
                && (col - left + right) < grid[0].len()
            {
                let nrow = row - up + down;
                let ncol = col - left + right;
                queue.push(Reverse((
                    heat + grid[nrow][ncol],
                    nrow,
                    ncol,
                    (up, right, down, left),
                    n + 1,
                )));
            }
        }

        for (up, right, down, left) in [(1, 0, 0, 0), (0, 1, 0, 0), (0, 0, 1, 0), (0, 0, 0, 1)] {
            if dir == (0, 0, 0, 0)
                || n >= 4
                    && (up, right, down, left) != dir
                    && (up, right, down, left) != (dir.2, dir.3, dir.0, dir.1)
            {
                if row.checked_sub(up).is_some()
                    && down.checked_add(row - up).is_some()
                    && (row - up + down) < grid.len()
                    && col.checked_sub(left).is_some()
                    && right.checked_add(col - left).is_some()
                    && (col - left + right) < grid[0].len()
                {
                    let nrow = row - up + down;
                    let ncol = col - left + right;
                    queue.push(Reverse((
                        heat + grid[nrow][ncol],
                        nrow,
                        ncol,
                        (up, right, down, left),
                        1,
                    )));
                }
            }
        }
    }

    return 0;
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
        assert_eq!(super::part_1(INPUT), 102);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 94);
    }
}
