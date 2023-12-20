const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

mod module;
use module::Module;
use num::integer::lcm;

fn part_1(input: &str) -> usize {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    let modules: Vec<Module> = input.lines().map(Module::from).collect();
    let mut flipflops: HashMap<_, _> = modules.iter().filter_map(Module::flipflop).collect();
    let mut conjunctions: HashMap<_, _> = modules.iter().filter_map(Module::conjunction).collect();
    for (module, memory) in conjunctions.iter_mut() {
        *memory = modules
            .iter()
            .filter(|m| m.dest.contains(&module.name.clone()))
            .map(|f| (f.name.clone(), false))
            .collect();
    }

    let mut hi = 0;
    let mut lo = 0;

    const N: usize = 1000;
    for _ in 0..N {
        lo += 1;
        let broadcaster = modules.iter().find(|p| p.ctg == 'b').unwrap();
        let mut q: VecDeque<(String, String, bool)> = broadcaster
            .dest
            .iter()
            .map(|d| (broadcaster.name.clone(), d.clone(), false))
            .collect();
        while let Some((src, dest, mut pulse)) = q.pop_front() {
            hi += if pulse == true { 1 } else { 0 };
            lo += if pulse == false { 1 } else { 0 };
            if let Some(dest) = modules.iter().find(|p| p.name == dest) {
                if let Some(flipflop) = flipflops.get_mut(dest) {
                    if pulse == false {
                        *flipflop = !*flipflop;
                        pulse = *flipflop;
                    } else {
                        continue;
                    }
                } else if let Some(conjunction) = conjunctions.get_mut(&dest) {
                    *conjunction.get_mut(&src).unwrap() = pulse;
                    pulse = conjunction.iter().any(|(_, f)| *f == false);
                }

                for d in dest.dest.iter() {
                    q.push_back((dest.name.clone(), d.clone(), pulse));
                }
            }
        }
    }

    return hi * lo;
}

fn part_2(input: &str) -> usize {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    let modules: Vec<Module> = input.lines().map(Module::from).collect();
    let mut flipflops: HashMap<_, _> = modules.iter().filter_map(Module::flipflop).collect();
    let mut conjunctions: HashMap<_, _> = modules.iter().filter_map(Module::conjunction).collect();
    for (module, memory) in conjunctions.iter_mut() {
        *memory = modules
            .iter()
            .filter(|m| m.dest.contains(&module.name.clone()))
            .map(|f| (f.name.clone(), false))
            .collect();
    }

    let target = modules
        .iter()
        .find(|p| p.dest.contains(&"rx".to_string()))
        .unwrap();
    let mut lcm_targets: Vec<usize> = Vec::new();
    const N: usize = usize::MAX;
    for i in 0..N {
        if lcm_targets.len() == 4 {
            return lcm_targets.into_iter().fold(1, lcm);
        }
        let broadcaster = modules.iter().find(|p| p.ctg == 'b').unwrap();
        let mut q: VecDeque<(String, String, bool)> = broadcaster
            .dest
            .iter()
            .map(|d| (broadcaster.name.clone(), d.clone(), false))
            .collect();
        while let Some((src, dest, mut pulse)) = q.pop_front() {
            if dest == target.name && pulse == true {
                lcm_targets.push(i + 1);
            }
            if let Some(dest) = modules.iter().find(|p| p.name == dest) {
                if let Some(flipflop) = flipflops.get_mut(dest) {
                    if pulse == false {
                        *flipflop = !*flipflop;
                        pulse = *flipflop;
                    } else {
                        continue;
                    }
                } else if let Some(conjunction) = conjunctions.get_mut(&dest) {
                    *conjunction.get_mut(&src).unwrap() = pulse;
                    pulse = conjunction.iter().any(|(_, f)| *f == false);
                }

                for d in dest.dest.iter() {
                    q.push_back((dest.name.clone(), d.clone(), pulse));
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
        assert_eq!(super::part_1(INPUT), 11687500);
    }
}
