const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

struct Card {
    id: usize,
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse(input: &str) -> Card {
    let mut card: Card = Card {
        id: 0,
        left: Vec::new(),
        right: Vec::new(),
    };

    let buf = input.strip_prefix("Card").unwrap().to_owned();
    let (id, buf) = buf.split_once(':').unwrap();
    card.id = id.trim().parse::<u32>().unwrap() as usize;

    let (left, right) = buf.split_once('|').unwrap();

    card.left = left
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    card.right = right
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return card;
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(parse)
        .collect::<Vec<Card>>()
        .iter()
        .map(|card| {
            card.right
                .iter()
                .filter(|&num| card.left.contains(num))
                .count() as u32
        })
        .filter(|&count| count > 0)
        .map(|count| u32::pow(2, count - 1))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let cards = input.lines().map(parse).collect::<Vec<Card>>();
    let mut copies = vec![1 as u32; cards.len()];
    for (index, card) in cards.into_iter().enumerate() {
        for i in index + 1
            ..=index
                + card
                    .right
                    .iter()
                    .filter(|num| card.left.contains(num))
                    .count()
        {
            copies[i] += copies[index];
        }
    }
    return copies.iter().sum::<u32>();
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
        assert_eq!(super::part_1(INPUT), 13);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT), 30);
    }
}
