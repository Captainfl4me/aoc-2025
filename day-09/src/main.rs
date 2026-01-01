use geo::{coord, Contains, LineString, Polygon, Rect};
use itertools::Itertools;

fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-09/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let area = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        })
        .combinations(2)
        .map(|comb| {
            let width = (comb[0].0 - comb[1].0).abs() + 1;
            let height = (comb[0].1 - comb[1].1).abs() + 1;
            width * height
        });

    area.max().unwrap() as u64
}

fn part_2(input: &str) -> u64 {
    let segments: Vec<(f64, f64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<f64>().unwrap())
                .collect_tuple::<(f64, f64)>()
                .unwrap()
        })
        .collect();

    let polygon = Polygon::new(LineString::from(segments), vec![]);

    let create_rect = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        })
        .combinations(2)
        .map(|comb| {
            Rect::new(
                coord! { x: comb[0].0 as f64, y: comb[0].1 as f64 },
                coord! { x: comb[1].0 as f64, y: comb[1].1 as f64 },
            )
        }).collect_vec();

    let sorted_rect = create_rect.iter().sorted_by_key(|rect| ((rect.width() + 1.0) * (rect.height() + 1.0)) as u64).collect_vec();
    let mean_area = sorted_rect.iter().map(|rect| ((rect.width() + 1.0) * (rect.height() + 1.0)) as u64).sum::<u64>() / (sorted_rect.len() as u64);
    let mean_index = sorted_rect.iter().map(|rect| ((rect.width() + 1.0) * (rect.height() + 1.0)) as u64).find_position(|&x| x > mean_area).unwrap();

    let filtered_rect = sorted_rect.iter().skip(mean_index.0).filter(|&&rect| polygon.contains(rect)).collect_vec();

    let area =
        filtered_rect.iter().map(|rect| ((rect.width() + 1.0) * (rect.height() + 1.0)) as u64);

    area.max().unwrap()
}

#[cfg(test)]
mod tests_day_09 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-09/test.txt");
        assert_eq!(part_1(input), 50);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-09/test.txt");
        assert_eq!(part_2(input), 24);
    }
}
