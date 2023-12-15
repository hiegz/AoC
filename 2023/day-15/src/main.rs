const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn hash(input: &str) -> usize {
    return input
        .chars()
        .fold(0, |acc, char| ((acc + char as usize) * 17) % 256);
}

fn part_1(input: &str) -> u64 {
    input[..input.len() - 1].split(',').map(hash).sum::<usize>() as u64
}

fn part_2(input: &str) -> u64 {
    let mut boxes: Vec<Vec<(&str, u64)>> = vec![Vec::new(); 256];
    for step in input[..input.len() - 1].split(',') {
        if step.contains('=') {
            let (label, length) = step.split_once('=').unwrap();
            let length = length.parse::<u64>().unwrap();
            let b = &mut boxes[hash(label)];
            if let Some(slot) = b.iter_mut().find(|(key, _)| *key == label) {
                slot.1 = length;
            } else {
                b.push((label, length));
            }
        } else if step.contains('-') {
            let label = &step[..step.len() - 1];
            boxes[hash(label)].retain(|(key, _)| *key != label);
        }
    }

    return boxes
        .iter()
        .enumerate()
        .map(|(nbox, b)| {
            b.iter()
                .enumerate()
                .map(|(nslot, slot)| (nbox + 1) * (nslot + 1) * (slot.1 as usize))
                .sum::<usize>()
        })
        .sum::<usize>() as u64;
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
        assert_eq!(super::part_1(INPUT), 1320);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 145);
    }
}
