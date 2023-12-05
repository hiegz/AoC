const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn parse(input: &str) -> Option<(Vec<u64>, Vec<Vec<(u64, u64, u64)>>)> {
    let mut res: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let (seeds, input) = input.split_once("\n\n")?;
    let seeds = seeds
        .split_once(": ")?
        .1
        .split_whitespace()
        .map(|word| word.parse::<u64>().unwrap())
        .collect();
    for block in input.split("\n\n") {
        res.push(vec![]);
        for line in block.split_once('\n')?.1.split('\n') {
            let nums = line
                .split_whitespace()
                .map(|word| word.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            if nums.len() == 3 {
                res.last_mut()?.push((nums[0], nums[1], nums[2]));
            }
        }
    }
    return Some((seeds, res));
}

fn part_1(input: &str) -> u64 {
    let (mut seeds, ranges) = parse(input).unwrap();
    for seed in seeds.iter_mut() {
        ranges.iter().for_each(|range| {
            let buf = *seed;
            range.clone().into_iter().for_each(|(dest, src, len)| {
                if buf >= src && buf < src + len {
                    *seed = buf - src + dest;
                }
            });
        })
    }
    return *seeds.iter().min().unwrap();
}

fn part_2(input: &str) -> u64 {
    let (seeds, ranges) = parse(input).unwrap();
    let mut seeds: Vec<_> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect();
    for block in ranges.iter() {
        let mut buf: Vec<(u64, u64)> = Vec::new();
        'seeds: while seeds.len() > 0 {
            let (start, end) = seeds.remove(0);
            for &(dest, src, len) in block.iter() {
                let (overlap_start, overlap_end) =
                    (u64::max(start, src), u64::min(end, src + len - 1));
                if overlap_start < overlap_end {
                    buf.push(((overlap_start - src + dest), (overlap_end - src + dest)));
                    if start < overlap_start {
                        seeds.push((start, overlap_start));
                    }
                    if overlap_end < end {
                        seeds.push((overlap_end + 1, end));
                    }
                    continue 'seeds;
                }
            }
            buf.push((start, end));
        }
        seeds = buf;
    }
    return seeds.iter().min().unwrap().0;
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
        assert_eq!(super::part_1(INPUT), 35);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 46);
    }
}
