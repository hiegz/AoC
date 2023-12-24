const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt"));

fn part_1(input: &str, area: (f64, f64)) -> usize {
    let input: Vec<(Vec<f64>, Vec<f64>)> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" @ ").unwrap();
            return (
                l.split(", ")
                    .map(|w| w.trim_start().parse::<_>().unwrap())
                    .collect(),
                r.split(", ")
                    .map(|w| w.trim_start().parse::<_>().unwrap())
                    .collect(),
            );
        })
        .collect();

    let mut counter = 0;

    for i in 0..input.len() {
        let h1 = &input[i];
        let ((x1, y1), (vx1, vy1)) = ((h1.0[0], h1.0[1]), (h1.1[0], h1.1[1]));
        for j in i + 1..input.len() {
            if i == j {
                continue;
            }

            let h2 = &input[j];
            let ((x2, y2), (vx2, vy2)) = ((h2.0[0], h2.0[1]), (h2.1[0], h2.1[1]));
            if vx1 * vy2 - vx2 * vy1 == 0.0 {
                continue;
            }

            let x = ((vx1 * y1 - vy1 * x1) * vx2 - (vx2 * y2 - vy2 * x2) * vx1)
                / (vy2 * vx1 - vx2 * vy1);
            let y = ((vx1 * y1 - vy1 * x1) * vy2 - (vx2 * y2 - vy2 * x2) * vy1)
                / (vy2 * vx1 - vx2 * vy1);

            if (area.0..=area.1).contains(&x)
                && (area.0..=area.1).contains(&y)
                && (x - x1) / vx1 >= 0.0
                && (y - y1) / vy1 >= 0.0
                && (x - x2) / vx2 >= 0.0
                && (y - y2) / vy2 >= 0.0
            {
                counter += 1;
            }
        }
    }

    return counter;
}

fn main() {
    println!(
        "Part 1: {:?}",
        part_1(INPUT, (200000000000000.0, 400000000000000.0))
    );
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/example.txt"));

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT, (7.0, 27.0)), 2);
    }
}
