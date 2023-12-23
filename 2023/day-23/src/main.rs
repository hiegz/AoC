use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Coord = (usize, usize);

const P: &[Coord] = &[(usize::MAX, 0), (1, 0), (0, 1), (0, usize::MAX)];
const U: &[Coord] = &[(usize::MAX, 0)];
const L: &[Coord] = &[(0, 1)];
const D: &[Coord] = &[(1, 0)];
const R: &[Coord] = &[(0, usize::MAX)];

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn compress(
    start: Coord,
    end: Coord,
    grid: &Vec<Vec<char>>,
    actions: &HashMap<char, &[Coord]>,
) -> HashMap<Coord, HashMap<Coord, usize>> {
    let mut points: Vec<Coord> = Vec::from([start, end]);
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' {
                continue;
            }

            let mut neighbors = 0;
            for (dr, dc) in actions[&grid[r][c]].iter().copied() {
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if (0..grid.len()).contains(&nr)
                    && (0..grid[nr].len()).contains(&nc)
                    && grid[nr][nc] != '#'
                {
                    neighbors += 1;
                }
            }

            if neighbors >= 3 {
                points.push((r, c));
            }
        }
    }

    let mut adjacency_list: HashMap<Coord, HashMap<Coord, usize>> = HashMap::new();
    for (sr, sc) in points.iter().copied() {
        adjacency_list.insert((sr, sc), HashMap::new());

        let mut stack: VecDeque<(usize, usize, usize)> = VecDeque::from([(0, sr, sc)]);
        let mut seen: HashSet<(usize, usize)> = HashSet::from([(sr, sc)]);

        while let Some((n, r, c)) = stack.pop_back() {
            if n != 0 && points.contains(&(r, c)) {
                adjacency_list.get_mut(&(sr, sc)).unwrap().insert((r, c), n);
                continue;
            }

            for (dr, dc) in actions[&grid[r][c]].iter().copied() {
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if (0..grid.len()).contains(&nr)
                    && (0..grid[nr].len()).contains(&nc)
                    && !seen.contains(&(nr, nc))
                    && grid[nr][nc] != '#'
                {
                    stack.push_back((n + 1, nr, nc));
                    seen.insert((nr, nc));
                }
            }
        }
    }

    return adjacency_list;
}

fn solve(input: &str, actions: &HashMap<char, &[Coord]>) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let (start, end) = ((0, 1), (grid.len() - 1, grid[0].len() - 2));
    let adjacency_list = compress(start, end, &grid, &actions);
    let mut deque: BinaryHeap<(usize, Coord, Vec<Coord>)> =
        BinaryHeap::from([(0, start, Vec::from([]))]);
    let mut res = 0;
    while let Some((n, (r, c), mut seen)) = deque.pop() {
        seen.push((r, c));
        if (r, c) == end {
            res = usize::max(n, res);
            continue;
        }

        for (&(nr, nc), nn) in adjacency_list[&(r, c)].iter() {
            if !seen.contains(&(nr, nc)) {
                deque.push((n + nn, (nr, nc), seen.clone()));
            }
        }
    }

    return res;
}
fn main() {
    println!(
        "Part 1: {:?}",
        solve(
            INPUT,
            &HashMap::from([('.', P), ('^', U), ('v', D), ('>', L), ('<', R)])
        )
    );

    println!(
        "Part 2: {:?}",
        solve(
            INPUT,
            &HashMap::from([('.', P), ('^', P), ('v', P), ('>', P), ('<', P)])
        )
    );
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/example.txt"));

    #[test]
    fn part_1() {
        assert_eq!(
            super::solve(
                INPUT,
                &super::HashMap::from([
                    ('.', super::P),
                    ('^', super::U),
                    ('v', super::D),
                    ('>', super::L),
                    ('<', super::R)
                ])
            ),
            94
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            super::solve(
                INPUT,
                &super::HashMap::from([
                    ('.', super::P),
                    ('^', super::P),
                    ('v', super::P),
                    ('>', super::P),
                    ('<', super::P)
                ])
            ),
            154
        );
    }
}
