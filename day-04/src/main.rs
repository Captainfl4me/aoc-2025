fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-04/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn part_1(input: &str) -> u64 {
    let mut map = input.lines().map(|line| line.chars().map(|char| char == '@').collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    // Create borders
    for line in map.iter_mut() {
        line.insert(0, false);
        line.insert(line.len(), false);
    }
    map.insert(0, vec![false; map[0].len()]);
    map.insert(map.len(), vec![false; map[0].len()]);

    let mut accessible_paper_roll = 0;
    for line_id in 1..(map.len()-1) {
        for col_id in 1..(map[0].len()-1) {
            if map[line_id][col_id] {
                let mut busy_neighboors = 0;

                for offset_line in -1..=1 {
                    for offset_col in -1..=1 {
                        if map[((line_id as i64) + offset_line) as usize][((col_id as i64) + offset_col) as usize] {
                            busy_neighboors += 1;
                        }
                    }
                }

                if busy_neighboors <= 4 {
                    accessible_paper_roll += 1;
                }
            }
        }
    }
    
    accessible_paper_roll
}

fn part_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests_day_04 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-04/test.txt");
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-04/test.txt");
        assert_eq!(part_2(input), 0);
    }
}
