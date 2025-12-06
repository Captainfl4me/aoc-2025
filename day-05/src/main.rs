fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-05/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let split_line_id = input.lines().position(|line| line.is_empty()).unwrap();
    let lines_vec = input.lines().collect::<Vec<&str>>();
    let ranges = lines_vec[0..split_line_id].iter().map(|line| line.split('-').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>()).map(|range| (range[0], range[1])).collect::<Vec<(u64, u64)>>();

    lines_vec[(split_line_id+1)..].iter().fold(0, |acc, n| {
        let n = n.parse::<u64>().unwrap();
        let mut add = 0;

        for (rmin, rmax) in ranges.iter() {
            if n >= *rmin && n <= *rmax {
                add = 1;
            }
        }

        acc + add
    })
}

fn part_2(input: &str) -> u64 {
    let split_line_id = input.lines().position(|line| line.is_empty()).unwrap();
    let lines_vec = input.lines().collect::<Vec<&str>>();
    let mut ranges = lines_vec[0..split_line_id].iter().map(|line| line.split('-').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>()).map(|range| (range[0], range[1])).collect::<Vec<(u64, u64)>>();

    let mut have_reduce_ranges = true;
    while have_reduce_ranges {
        have_reduce_ranges = false;

        for i in 0..ranges.len() {
            let rmin = ranges[i].0;
            let rmax = ranges[i].1;

            for j in 0..ranges.len() {
                if i != j {
                    if ranges[j].0 >= rmin && ranges[j].0 <= rmax {
                        if ranges[j].1 > ranges[i].1 {
                            ranges[i].1 = ranges[j].1;
                        }
                        ranges.remove(j);
                        have_reduce_ranges = true;
                        break;
                    }
                    if ranges[j].1 >= rmin && ranges[j].1 <= rmax {
                        if ranges[j].0 < ranges[i].0 {
                            ranges[i].0 = ranges[j].0;
                        }
                        ranges.remove(j);
                        have_reduce_ranges = true;
                        break;
                    }
                } 
            }

            if have_reduce_ranges {
                break;
            }
        }
    }

    let mut acc = 0;
    for (rmin, rmax) in ranges.iter() {
        acc += *rmax - *rmin + 1;
    }

    acc
}

#[cfg(test)]
mod tests_day_05 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-05/test.txt");
        assert_eq!(part_1(input), 3);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-05/test.txt");
        assert_eq!(part_2(input), 14);

        let input = "1-5\n5-10\n\n";
        assert_eq!(part_2(input), 10);
    }
}
