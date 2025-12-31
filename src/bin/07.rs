advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().into_iter();

    let mut total_splits = 0u64;
    let mut beams: Vec<u8> = vec![];

    // first, find the start and set that as the first beam
    beams.push(lines.next().unwrap().find('S').unwrap() as u8);

    loop {
        // skip the blank line
        if let None = lines.next() {
            // no more lines
            break;
        }

        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };

        let mut new_beams: Vec<u8> = vec![];

        let mut old_beams = beams.iter();
        let mut old_beam = old_beams.next();
        for (i, char) in line.char_indices() {
            if let Some(&b) = old_beam
                && b == i as u8
            {
                if char == '^' {
                    total_splits += 1;
                    if new_beams.len() == 0 || new_beams[new_beams.len() - 1] < b - 1 {
                        new_beams.push(b - 1);
                    }
                    new_beams.push(b + 1);
                } else if new_beams[new_beams.len() - 1] < b {
                    new_beams.push(b);
                }

                old_beam = old_beams.next();
            }
        }

        beams = new_beams;
    }

    Some(total_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().into_iter();

    let first_line = lines.next().unwrap();
    let mut beams: Vec<u64> = vec![0; first_line.len()];

    // first, find the start and set that as the first beam
    beams[first_line.find('S').unwrap()] = 1;

    loop {
        // skip the blank line
        if let None = lines.next() {
            // no more lines
            break;
        }

        let line = match lines.next() {
            Some(l) => l,
            // no more lines
            None => break,
        };

        let mut new_beams: Vec<u64> = vec![0; beams.len()];
        for (i, char) in line.char_indices() {
            if char == '^' {
                new_beams[i - 1] += beams[i];
                new_beams[i + 1] += beams[i];
            } else {
                new_beams[i] += beams[i];
            }
        }

        beams = new_beams;
    }

    Some(beams.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
