fn main() {
    let input = include_str!("../../aoc-2025-inputs/day-06/input.txt");
    let now = std::time::Instant::now();
    dbg!(part_1(input));
    dbg!(part_2(input));
    println!("Time: {:?}", now.elapsed());
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part_1(input: &str) -> u64 {
    let input_table = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();
    let problems: Vec<Vec<&str>> = transpose(input_table)
        .iter_mut()
        .map(|problem| {
            problem.reverse();
            problem.clone()
        })
        .collect();

    problems.iter().fold(0, |acc, problem| {
        let operator = problem[0];

        let res_init = {
            if operator == "*" {
                1
            } else {
                0
            }
        };

        acc + problem.iter().skip(1).fold(res_init, |res, n| {
            if operator == "*" {
                res * n.parse::<u64>().unwrap()
            } else {
                res + n.parse::<u64>().unwrap()
            }
        })
    })
}

#[derive(Debug)]
struct ProblemT {
    operation: char,
    numbers: Vec<u64>,
}

fn part_2(input: &str) -> u64 {
    let input_table: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let input_table_t = transpose(input_table);

    let mut problems: Vec<ProblemT> = Vec::new();
    let mut temp: Vec<Vec<char>> = Vec::new();
    for col in input_table_t.iter() {
        if col.iter().all(|x| *x == ' ') {
            let problem = ProblemT {
                operation: *temp[0].last().unwrap(),
                numbers: temp
                    .iter()
                    .map(|n| {
                        n.iter()
                            .filter(|c| c.is_ascii_digit())
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap()
                    })
                    .collect(),
            };
            problems.push(problem);
            temp = Vec::new();
        } else {
            temp.push(col.clone());
        }
    }
    if !temp.is_empty() {
        let problem = ProblemT {
            operation: *temp[0].last().unwrap(),
            numbers: temp
                .iter()
                .map(|n| {
                    n.iter()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect(),
        };
        problems.push(problem);
    }

    problems.iter().fold(0, |acc, problem| {
        let res_init = {
            if problem.operation == '*' {
                1
            } else {
                0
            }
        };

        acc + problem.numbers.iter().fold(res_init, |res, n| {
            if problem.operation == '*' {
                res * n
            } else {
                res + n
            }
        })
    })
}

#[cfg(test)]
mod tests_day_06 {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../aoc-2025-inputs/day-06/test.txt");
        assert_eq!(part_1(input), 4277556);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../aoc-2025-inputs/day-06/test.txt");
        assert_eq!(part_2(input), 3263827);
    }
}
