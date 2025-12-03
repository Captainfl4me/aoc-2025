fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-03/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let digit_vec = line.chars().enumerate().map(|(i, char)| (i, char.to_digit(10).unwrap())).collect::<Vec<(usize, u32)>>();
        let mut tens_digit: (usize, u32) = (0, 0);
        for digit in digit_vec[0..(digit_vec.len()-1)].iter() {
            if digit.1 > tens_digit.1 {
                tens_digit = *digit;
            }
        }
        let last_digit = digit_vec[(tens_digit.0+1)..].iter().max_by_key(|x| x.1).unwrap();
        let max_joltage = (tens_digit.1*10 + last_digit.1) as u64;
        acc + max_joltage
    })
}

fn part_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests_day_03 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-03/test.txt");
        assert_eq!(part_1(input), 357);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-03/test.txt");
        assert_eq!(part_2(input), 0);
    }
}
