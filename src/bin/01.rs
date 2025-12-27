advent_of_code::solution!(1);

pub fn part_one_for_loop(input: &str) -> Option<u64> {
    let mut dial = 50u8;
    let mut password = 0u64;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (direction, distance_str) = line.split_at(1);
        let distance = (distance_str.parse::<u64>().ok()? % 100) as u8;
        dial = match direction.chars().next()? {
            'L' => (dial + 100 - distance) % 100,
            'R' => (dial + distance) % 100,
            _ => return None,
        };

        if dial == 0 {
            password += 1;
        }
    }

    Some(password)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, password) = input.lines().filter(|l| !l.is_empty()).try_fold(
        (50u8, 0u64),
        |(dial, password), line| {
            let (direction, distance_str) = line.split_at(1);
            let distance = (distance_str.parse::<u64>().ok()? % 100) as u8;
            let new_dial = match direction.chars().next()? {
                'L' => (dial + 100 - distance) % 100,
                'R' => (dial + distance) % 100,
                _ => return None,
            };

            let new_password = password + (new_dial == 0) as u64;
            Some((new_dial, new_password))
        },
    )?;

    Some(password)
}

pub fn part_two_for_loop(input: &str) -> Option<u64> {
    let mut dial = 50u8;
    let mut password = 0u64;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (direction, distance_str) = line.split_at(1);
        let distance = distance_str.parse::<u64>().ok()?;
        password += distance / 100;
        let distance_to_apply = (distance % 100) as u8;
        dial = match direction.chars().next()? {
            'L' => {
                if dial != 0 && dial <= distance_to_apply {
                    password += 1;
                }
                (dial + 100 - distance_to_apply) % 100
            }
            'R' => {
                if dial + distance_to_apply >= 100 {
                    password += 1;
                }
                (dial + distance_to_apply) % 100
            }
            _ => return None,
        };
    }

    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, password) = input.lines().filter(|l| !l.is_empty()).try_fold(
        (50u8, 0u64),
        |(dial, password), line| {
            let (direction, distance_str) = line.split_at(1);
            let distance = distance_str.parse::<u64>().ok()?;
            let loops = distance / 100;
            let distance_to_apply = (distance % 100) as u8;
            let (new_dial, passed_zero) = match direction.chars().next()? {
                'L' => (
                    (dial + 100 - distance_to_apply) % 100,
                    dial != 0 && dial <= distance_to_apply,
                ),
                'R' => (
                    (dial + distance_to_apply) % 100,
                    dial + distance_to_apply >= 100,
                ),
                _ => return None,
            };

            let new_password = password + loops + passed_zero as u64;
            Some((new_dial, new_password))
        },
    )?;

    Some(password)
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
        assert_eq!(result, Some(6));
    }
}
