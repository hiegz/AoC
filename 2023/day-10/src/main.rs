const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

//
// HINT
//
// '|' : 0x7C
// '-' : 0x2D
// 'L' : 0x4C
// 'J' : 0x4A
// 'F' : 0x46
// '7' : 0x37
// '.' : 0x2E
// 'S' : 0x53
//

fn part_1(input: &str) -> u64 {
    let input: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let find = |(i, row): (usize, &&[u8])| row.iter().position(|col| *col == 0x53).map(|j| (i, j));
    let (row, column) = input.iter().enumerate().find_map(find).unwrap();
    let mut seen: Vec<_> = Vec::from([(row, column)]);
    let mut queue: std::collections::VecDeque<_> =
        std::collections::VecDeque::from([(row, column)]);
    while queue.len() > 0 {
        let (r, c) = queue.pop_front().unwrap();
        let character = input[r][c];

        if r > 0
            && [0x53, 0x7C, 0x4A, 0x4C].contains(&character)
            && [0x7C, 0x37, 0x46].contains(&input[r - 1][c])
            && !seen.contains(&(r - 1, c))
        {
            seen.push((r - 1, c));
            queue.push_back((r - 1, c));
        }

        if r < input.len() - 1
            && [0x53, 0x7C, 0x37, 0x46].contains(&character)
            && [0x7C, 0x4A, 0x4C].contains(&input[r + 1][c])
            && !seen.contains(&(r + 1, c))
        {
            seen.push((r + 1, c));
            queue.push_back((r + 1, c));
        }

        if c > 0
            && [0x53, 0x2D, 0x4A, 0x37].contains(&character)
            && [0x2D, 0x4C, 0x46].contains(&input[r][c - 1])
            && !seen.contains(&(r, c - 1))
        {
            seen.push((r, c - 1));
            queue.push_back((r, c - 1));
        }

        if c < input[r].len() - 1
            && [0x53, 0x2D, 0x4C, 0x46].contains(&character)
            && [0x2D, 0x4A, 0x37].contains(&input[r][c + 1])
            && !seen.contains(&(r, c + 1))
        {
            seen.push((r, c + 1));
            queue.push_back((r, c + 1));
        }
    }

    return (seen.len() / 2) as u64;
}

fn part_2(input: &str) -> u64 {
    use std::collections::HashSet;
    let mut input: Vec<Vec<u8>> = input
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect();
    let find =
        |(i, row): (usize, &Vec<u8>)| row.iter().position(|col| *col == 0x53).map(|j| (i, j));
    let (row, column) = input.iter().enumerate().find_map(find).unwrap();
    let mut s: Vec<u8> = Vec::from([0x7C, 0x2D, 0x4A, 0x4C, 0x37, 0x46]);
    let mut seen: Vec<_> = Vec::from([(row, column)]);
    let mut queue: std::collections::VecDeque<_> =
        std::collections::VecDeque::from([(row, column)]);
    while queue.len() > 0 {
        let (r, c) = queue.pop_front().unwrap();
        let character = input[r][c];

        if r > 0
            && [0x53, 0x7C, 0x4A, 0x4C].contains(&character)
            && [0x7C, 0x37, 0x46].contains(&input[r - 1][c])
            && !seen.contains(&(r - 1, c))
        {
            seen.push((r - 1, c));
            queue.push_back((r - 1, c));
            if character == 0x53 {
                s = HashSet::from_iter(s.clone().into_iter())
                    .intersection(&HashSet::from([0x7C, 0x4A, 0x4C]))
                    .map(|v| *v)
                    .collect();
            }
        }

        if r < input.len() - 1
            && [0x53, 0x7C, 0x37, 0x46].contains(&character)
            && [0x7C, 0x4A, 0x4C].contains(&input[r + 1][c])
            && !seen.contains(&(r + 1, c))
        {
            seen.push((r + 1, c));
            queue.push_back((r + 1, c));
            if character == 0x53 {
                s = HashSet::from_iter(s.clone().into_iter())
                    .intersection(&HashSet::from([0x7C, 0x37, 0x46]))
                    .map(|v| *v)
                    .collect();
            }
        }

        if c > 0
            && [0x53, 0x2D, 0x4A, 0x37].contains(&character)
            && [0x2D, 0x4C, 0x46].contains(&input[r][c - 1])
            && !seen.contains(&(r, c - 1))
        {
            seen.push((r, c - 1));
            queue.push_back((r, c - 1));
            if character == 0x53 {
                s = HashSet::from_iter(s.clone().into_iter())
                    .intersection(&HashSet::from([0x2D, 0x4A, 0x37]))
                    .map(|v| *v)
                    .collect();
            }
        }

        if c < input[r].len() - 1
            && [0x53, 0x2D, 0x4C, 0x46].contains(&character)
            && [0x2D, 0x4A, 0x37].contains(&input[r][c + 1])
            && !seen.contains(&(r, c + 1))
        {
            seen.push((r, c + 1));
            queue.push_back((r, c + 1));
            if character == 0x53 {
                s = HashSet::from_iter(s.clone().into_iter())
                    .intersection(&HashSet::from([0x2D, 0x4C, 0x46]))
                    .map(|v| *v)
                    .collect();
            }
        }
    }

    for (i, row) in input.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if !seen.contains(&(i, j)) {
                *col = 0x2E;
            } else if *col == 0x53 {
                *col = s[0];
            }
        }
    }

    let mut outside: HashSet<(usize, usize)> = HashSet::new();

    for (r, row) in input.iter().enumerate() {
        let mut within = false;
        let mut up = false;
        for (c, &ch) in row.iter().enumerate() {
            if ch == 0x7C {
                within = !within;
            } else if [0x4C, 0x46].contains(&ch) {
                up = ch == 0x4C;
            } else if [0x37, 0x4A].contains(&ch) {
                if ch != (if up { 0x4A } else { 0x37 }) {
                    within = !within;
                }
                up = false;
            } else if ch == 0x2E {
                if !within {
                    outside.insert((r, c));
                }
            }
        }
    }

    (input.len() * input[0].len() - (seen.len() + outside.len())) as u64
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
        assert_eq!(super::part_1(INPUT), 8);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 10);
    }
}
