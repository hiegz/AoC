const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn part_1(input: &str) -> u64 {
    let mut grid: Vec<Vec<u32>> = Vec::new();

    let mut counter = 0;
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            let mut curr = 0;
            if char == '#' {
                counter = counter + 1;
                curr = counter;
            }
            row.push(curr);
        }
        grid.push(row);
    }

    let empty_rows: Vec<_> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&el| el == 0))
        .map(|(i, _)| i)
        .collect();

    let empty_cols: Vec<_> = (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect::<Vec<_>>())
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&el| el == 0))
        .map(|(i, _)| i)
        .collect();

    let galaxies: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &col)| {
                return if col != 0 { Some((i, j)) } else { None };
            })
        })
        .collect();

    let mut total = 0;
    let scale = 2;

    for (i, &(r1, c1)) in galaxies.iter().enumerate() {
        for &(r2, c2) in galaxies[i..].iter() {
            (usize::min(r1, r2)..usize::max(r1, r2))
                .for_each(|r| total += if empty_rows.contains(&r) { scale } else { 1 });
            (usize::min(c1, c2)..usize::max(c1, c2))
                .for_each(|c| total += if empty_cols.contains(&c) { scale } else { 1 });
        }
    }

    return total;
}

fn part_2(input: &str) -> u64 {
    let mut grid: Vec<Vec<u32>> = Vec::new();

    let mut counter = 0;
    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            let mut curr = 0;
            if char == '#' {
                counter = counter + 1;
                curr = counter;
            }
            row.push(curr);
        }
        grid.push(row);
    }

    let empty_rows: Vec<_> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&el| el == 0))
        .map(|(i, _)| i)
        .collect();

    let empty_cols: Vec<_> = (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect::<Vec<_>>())
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&el| el == 0))
        .map(|(i, _)| i)
        .collect();

    let galaxies: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &col)| {
                return if col != 0 { Some((i, j)) } else { None };
            })
        })
        .collect();

    let mut total = 0;
    let scale = 1000000;

    for (i, &(r1, c1)) in galaxies.iter().enumerate() {
        for &(r2, c2) in galaxies[i..].iter() {
            (usize::min(r1, r2)..usize::max(r1, r2))
                .for_each(|r| total += if empty_rows.contains(&r) { scale } else { 1 });
            (usize::min(c1, c2)..usize::max(c1, c2))
                .for_each(|c| total += if empty_cols.contains(&c) { scale } else { 1 });
        }
    }

    return total;
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
        assert_eq!(super::part_1(INPUT), 374);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 82000210);
    }
}
