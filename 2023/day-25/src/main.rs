use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn freq(adjacency_list: &HashMap<String, HashSet<String>>, n: usize) -> Vec<(String, String)> {
    let mut edges: HashMap<(String, String), usize> = HashMap::new();

    for (vertex, _) in adjacency_list.iter() {
        let mut seen: HashSet<String> = HashSet::from([vertex.clone()]);
        let mut deque: VecDeque<String> = VecDeque::from([vertex.clone()]);
        while let Some(v) = deque.pop_front() {
            for neighbor in adjacency_list[&v].iter() {
                if !seen.contains(neighbor) {
                    seen.insert(neighbor.clone());
                    deque.push_back(neighbor.clone());
                    *(edges
                        .entry((
                            String::min(v.clone(), neighbor.clone()),
                            String::max(v.clone(), neighbor.clone()),
                        ))
                        .or_default()) += 1;
                }
            }
        }
    }

    let mut res: Vec<(String, String)> = Vec::new();
    for _ in 0..n {
        let max = edges
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k.clone())
            .unwrap();
        res.push(max.clone());
        edges.remove(&max);
    }

    return res;
}

fn solve(input: &str) -> usize {
    let mut adjacency_list: HashMap<String, HashSet<String>> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            return (l.to_string(), r.split(' ').map(|x| x.to_string()).collect());
        })
        .collect();

    for (vertex, neighbors) in adjacency_list.clone().iter() {
        for neighbor in neighbors.iter() {
            adjacency_list
                .entry(neighbor.clone())
                .or_default()
                .insert(vertex.clone());
        }
    }

    let edge = freq(&adjacency_list, 3);
    for (u, w) in edge.iter() {
        adjacency_list.get_mut(u).unwrap().remove(w);
        adjacency_list.get_mut(w).unwrap().remove(u);
    }

    let mut seen: HashSet<String> = HashSet::from([edge[0].0.clone()]);
    let mut deque: VecDeque<String> = VecDeque::from([edge[0].0.clone()]);
    while let Some(v) = deque.pop_front() {
        for neighbor in adjacency_list[&v].iter() {
            if !seen.contains(neighbor) {
                seen.insert(neighbor.clone());
                deque.push_back(neighbor.clone());
            }
        }
    }

    return (adjacency_list.len() - seen.len()) * seen.len();
}

fn main() {
    println!("Part 1: {:?}", solve(INPUT));
}
