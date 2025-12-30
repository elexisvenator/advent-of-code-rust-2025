advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, mut ids) = parse_input(input, false);
    if ranges.is_empty() {
        return Some(ids.len() as u64);
    }
    if ids.is_empty() {
        return Some(0);
    }
    let ranges = optimise_ranges(ranges);
    let mut ranges_enum = ranges.iter();
    ids.sort_unstable();
    let mut ids_enum = ids.iter();

    let mut match_count = 0;
    let mut current_range = ranges_enum.next().unwrap();
    let mut current_id = ids_enum.next().unwrap();
    loop {
        if *current_id > current_range.max {
            if let Some(next_range) = ranges_enum.next() {
                current_range = next_range;
                continue;
            } else {
                // no more ranges, remaining numbers are not matches
                break;
            }
        }

        if *current_id >= current_range.min {
            match_count += 1;
        }

        if let Some(next_id) = ids_enum.next() {
            current_id = next_id;
        } else {
            // no more ids
            break;
        }
    }

    return Some(match_count);
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input, true);
    let ranges = optimise_ranges(ranges);

    Some(
        ranges
            .iter()
            .fold(0, |acc, range| acc + (range.max - range.min) + 1),
    )
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct IdRange {
    min: u64,
    max: u64,
}

fn parse_input(input: &str, skip_ids: bool) -> (Vec<IdRange>, Vec<u64>) {
    let mut ranges: Vec<IdRange> = vec![];
    let mut ids: Vec<u64> = vec![];
    let mut processing_ranges = true;
    for line in input.lines() {
        if line.is_empty() {
            if skip_ids {
                break;
            }
            processing_ranges = false;
            continue;
        }

        if processing_ranges {
            let (min, max) = line.split_once('-').unwrap();
            ranges.push(IdRange {
                min: min.parse::<u64>().unwrap(),
                max: max.parse::<u64>().unwrap(),
            });
            continue;
        }

        ids.push(line.parse::<u64>().unwrap());
    }

    (ranges, ids)
}

fn optimise_ranges(ranges: Vec<IdRange>) -> Vec<IdRange> {
    let mut ranges = ranges;
    ranges.sort_by_key(|range| range.min);
    let mut merged_ranges = vec![];
    let mut current_range = ranges[0];

    for range in ranges.iter().skip(1) {
        if range.min <= current_range.max + 1 {
            current_range.max = current_range.max.max(range.max);
        } else {
            merged_ranges.push(current_range);
            current_range = *range;
        }
    }
    merged_ranges.push(current_range);
    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY), false);
        assert_eq!(
            result,
            (
                vec![
                    IdRange { min: 3, max: 5 },
                    IdRange { min: 10, max: 14 },
                    IdRange { min: 16, max: 20 },
                    IdRange { min: 12, max: 18 },
                ],
                vec![1, 5, 8, 11, 17, 32],
            )
        );
    }

    #[test]
    fn test_optimise_ranges() {
        let (ranges, _) = parse_input(&advent_of_code::template::read_file("examples", DAY), true);
        let result = optimise_ranges(ranges);
        assert_eq!(
            result,
            vec![IdRange { min: 3, max: 5 }, IdRange { min: 10, max: 20 },]
        );
    }
}
