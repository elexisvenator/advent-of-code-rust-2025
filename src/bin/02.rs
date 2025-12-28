use std::collections::HashSet;

use advent_of_code::get_factors_unsorted;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .trim_end()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        // skip ranges with only odd length digits
        .filter(|(start, end)| !(start.len() % 2 == 1 && start.len() == end.len()))
        .flat_map(|(start, end)| {
            (start.len()..=end.len())
                .filter(|len| len % 2 == 0)
                .map(|len| {
                    (
                        len as u32,
                        if start.len() == len {
                            start.parse::<u64>().unwrap()
                        } else {
                            10u64.pow(len as u32 - 1)
                        },
                        if end.len() == len {
                            end.parse::<u64>().unwrap()
                        } else {
                            10u64.pow(len as u32) - 1
                        },
                    )
                })
        })
        .fold(0u64, |sum, (len, start, end)| {
            let factor = len / 2;
            let chunk_size = 10u64.pow(factor);
            let mut current_chunk = get_first_chunk(start, len, factor);
            let mut current_num = build_repeat_num(current_chunk, chunk_size, 2);
            if current_num < start {
                current_chunk += 1;
                current_num = build_repeat_num(current_chunk, chunk_size, 2);
            }

            let mut current_sum = sum;
            loop {
                if current_num > end {
                    break;
                }

                current_sum += current_num;
                current_chunk += 1;
                current_num = build_repeat_num(current_chunk, chunk_size, 2);
            }

            current_sum
        });

    Some(result)
}

fn build_repeat_num(repeat: u64, chunk_size: u64, repeat_times: u32) -> u64 {
    (0..repeat_times).fold(0, |acc, _| acc * chunk_size + repeat)
}

fn get_first_chunk(num: u64, len: u32, factor: u32) -> u64 {
    num / 10u64.pow(len - factor)
}

pub fn part_two(input: &str) -> Option<u64> {
    fn process((len, factor, start, end): (u32, u32, u64, u64)) -> Vec<u64> {
        let chunk_size = 10u64.pow(factor);
        let mut current_chunk = get_first_chunk(start, len, factor);
        let mut current_num = build_repeat_num(current_chunk, chunk_size, len / factor);
        if current_num < start {
            current_chunk += 1;
            current_num = build_repeat_num(current_chunk, chunk_size, len / factor);
        }

        let mut matches = Vec::new();
        loop {
            if current_num > end {
                break;
            }

            matches.push(current_num);
            current_chunk += 1;
            current_num = build_repeat_num(current_chunk, chunk_size, len / factor);
        }

        matches
    }

    let all_matches = input
        .trim_end()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .flat_map(|(start, end)| {
            (start.len()..=end.len()).map(|len| {
                (
                    len as u32,
                    if start.len() == len {
                        start.parse::<u64>().unwrap()
                    } else {
                        10u64.pow(len as u32 - 1)
                    },
                    if end.len() == len {
                        end.parse::<u64>().unwrap()
                    } else {
                        10u64.pow(len as u32) - 1
                    },
                )
            })
        })
        .map(|(len, start, end)| {
            get_factors_unsorted(len)
                .into_iter()
                .filter(move |factor| *factor < len)
                .map(move |factor| (len, factor, start, end))
                .flat_map(process)
                .collect::<HashSet<_>>()
                .iter()
                .sum::<u64>()
        })
        .sum::<u64>()
        .into();

    Some(all_matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_factors_of_one() {
        let mut factors = get_factors_unsorted(1);
        factors.sort_unstable();
        assert_eq!(factors, vec![1]);
    }

    #[test]
    fn test_factors_of_prime() {
        let mut factors = get_factors_unsorted(7);
        factors.sort_unstable();
        assert_eq!(factors, vec![1, 7]);
    }

    #[test]
    fn test_factors_of_six() {
        let mut factors = get_factors_unsorted(6);
        factors.sort_unstable();
        assert_eq!(factors, vec![1, 2, 3, 6]);
    }

    #[test]
    fn test_factors_of_twelve() {
        let mut factors = get_factors_unsorted(12);
        factors.sort_unstable();
        assert_eq!(factors, vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_factors_of_sixty() {
        let mut factors = get_factors_unsorted(60);
        factors.sort_unstable();
        assert_eq!(factors, vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
    }

    #[test]
    fn test_factors_of_zero() {
        let mut factors = get_factors_unsorted(0);
        factors.sort_unstable();
        assert_eq!(factors, vec![]);
    }
}
