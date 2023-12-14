const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect())
        .collect();
    grid.into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

fn tilt(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (rows, cols) = (grid.len(), grid[0].len());
    for col in 0..cols {
        let mut pos = 0;
        for row in 0..rows {
            if grid[row][col] == 'O' {
                grid[row][col] = '.';
                grid[pos][col] = 'O';
                pos += 1;
            } else if grid[row][col] == '#' {
                pos = row + 1;
            }
        }
    }
    return grid;
}

fn eval(grid: &Vec<Vec<char>>) -> u64 {
    grid.iter()
        .enumerate()
        .map(|(i, row): (usize, &Vec<char>)| {
            return (row.into_iter().filter(|&&space| space == 'O').count() * (grid.len() - i))
                as u64;
        })
        .sum::<u64>()
}

fn part_1(input: &str) -> u64 {
    return eval(&tilt(
        input.lines().map(|line| line.chars().collect()).collect(),
    ));
}

fn part_2(input: &str) -> u64 {
    let iterations = 1000000000;

    let mut seen: Vec<Vec<Vec<char>>> = Vec::new();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut iteration: usize = 0;

    while iteration < iterations {
        if !seen.contains(&grid) {
            seen.push(grid.clone());
        } else {
            break;
        }

        for _ in 0..4 {
            grid = tilt(grid);
            grid = rotate(grid);
        }

        iteration += 1;
    }

    let first = seen
        .iter()
        .enumerate()
        .find_map(|(index, g)| if *g == grid { Some(index) } else { None })
        .unwrap();

    return eval(&seen[(iterations - first) % (iteration - first) + first].clone());
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
        assert_eq!(super::part_1(INPUT), 136);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 64);
    }
}
