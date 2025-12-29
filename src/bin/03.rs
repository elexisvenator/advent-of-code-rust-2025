use rayon::prelude::*;
use std::cmp::min;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .trim_end()
        .par_lines()
        .map(|bank| calculate_joltage(bank, 2))
        .sum();
    Some(result)
}

fn calculate_joltage(bank: &str, battery_count: usize) -> u64 {
    let mut max_joltage: Vec<u8> = bank.as_bytes()[0..battery_count].iter().copied().collect();
    let mut last_used_index = 0usize;
    let mut index_last_used_index_applied = 0usize;

    for (i, &digit) in bank.as_bytes().iter().enumerate().skip(1) {
        // Calculate the minimum position we can fill (greedy constraint)
        let min_pos = battery_count.saturating_sub(bank.len() - i);
        let max_pos = min(
            battery_count,
            index_last_used_index_applied + i - last_used_index,
        );

        // Find the first position where we can improve
        if let Some(pos) = (min_pos..max_pos).find(|&j| max_joltage[j] < digit) {
            // Update from position pos onwards with remaining digits
            last_used_index = i;
            index_last_used_index_applied = pos;
            for (offset, &val) in bank.as_bytes()[i..].iter().enumerate() {
                if pos + offset >= battery_count {
                    break;
                }
                max_joltage[pos + offset] = val;
            }
        }
    }

    max_joltage
        .iter()
        .fold(0u64, |acc, &elem| acc * 10 + (elem - b'0') as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .trim_end()
        .par_lines()
        .map(|bank| calculate_joltage(bank, 12))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    #[case("987654321111111", 12, 987654321111)]
    #[case("811111111111119", 12, 811111111119)]
    #[case("234234234234278", 12, 434234234278)]
    #[case("818181911112111", 12, 888911112111)]
    fn test_calculate_joltage(
        #[case] input: &str,
        #[case] battery_count: usize,
        #[case] expected: u64,
    ) {
        let joltage = calculate_joltage(input, battery_count);
        assert_eq!(joltage, expected);
    }
}
