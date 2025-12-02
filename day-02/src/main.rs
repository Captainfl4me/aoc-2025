fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-02/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    // dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let res = input.trim_end().split(',').fold(0, |acc, range_str| {
        let range_min = range_str.split('-').next().unwrap().parse::<i64>().unwrap();
        let range_max = range_str.split('-').nth(1).unwrap().parse::<i64>().unwrap();
        let mut invalid_id = 0;

        for i in range_min..=range_max {
            let i_string = i.to_string();
            let i_vec = i_string.chars().collect::<Vec<char>>();

            let chunk_size = i_vec.len() / 2;
            if chunk_size > 0 && (i_vec.len() % 2 == 0) {
                let num_iter = i_vec.chunks_exact(chunk_size).map(|x| x.iter().collect::<String>().parse::<i64>().unwrap()).collect::<Vec<i64>>();
                if (num_iter.len() == 2) && (num_iter[1] == num_iter[0]) {
                    invalid_id += i;
                }
            }
        }

        acc + invalid_id
    });

    res.try_into().unwrap()
}

fn part_2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests_day_02 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-02/test.txt");
        assert_eq!(part_1(input), 1227775554);

        let input = "1-19";
        assert_eq!(part_1(input), 11);

        let input = "100-120";
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-02/test.txt");
        // assert_eq!(part_2(input), 0);
    }
}
