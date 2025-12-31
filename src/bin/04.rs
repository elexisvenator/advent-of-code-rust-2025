use std::cmp::max;

advent_of_code::solution!(4);

const PAPER_ROLL: u8 = b'@';
const TOO_MANY_ROLLS: u8 = 4;

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let height = lines.len();

    if height == 0 {
        return Some(0);
    }

    let mut below: &[u8] = lines[0].as_bytes();
    let width = below.len();
    let zero_line = vec![b'.'; width];
    let mut middle: &[u8] = zero_line.as_slice();

    let mut total_moveable_rolls = 0;
    for y in 0..height {
        let above = middle;
        middle = below;
        below = if y + 1 < height {
            lines[y + 1].as_bytes()
        } else {
            zero_line.as_slice()
        };

        total_moveable_rolls += (0..width).fold(0, |matches, x| {
            if middle[x] != PAPER_ROLL {
                return matches;
            }

            let neighbours = if x == 0 {
                0
            } else {
                (above[x - 1] == PAPER_ROLL) as u8
                    + (middle[x - 1] == PAPER_ROLL) as u8
                    + (below[x - 1] == PAPER_ROLL) as u8
            } + (above[x] == PAPER_ROLL) as u8
                + (below[x] == PAPER_ROLL) as u8
                + if x == width - 1 {
                    0
                } else {
                    (above[x + 1] == PAPER_ROLL) as u8
                        + (middle[x + 1] == PAPER_ROLL) as u8
                        + (below[x + 1] == PAPER_ROLL) as u8
                };

            return if neighbours < TOO_MANY_ROLLS {
                matches + 1
            } else {
                matches
            };
        });
    }

    Some(total_moveable_rolls)
}

pub fn get_grid_height_width_spacer(input: &str) -> (usize, usize, usize) {
    let spacer = if input.contains("\r\n") { 2 } else { 1 };
    let width = input.find('\n').unwrap_or(input.len()) - (spacer - 1);
    let height = max(input.len() / (width + spacer), 1);
    (height, width, spacer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (width, height, spacer) = get_grid_height_width_spacer(input);

    let input_bytes = input.as_bytes();

    // has a 1 cell padding all round, removes the need for bounds checks
    let mut neighbourhood = vec![0u8; (width + 2) * (height + 2)];
    let mut next_removals: Vec<usize> = vec![];

    // build the neighbourhood
    for x in 0..width {
        for y in 0..height {
            let pos = (y * (width + spacer)) + x;
            if input_bytes[pos] != PAPER_ROLL {
                continue;
            }
            //let actual_index = (y * width) + x;

            let top_left = if x == 0 || y == 0 {
                false
            } else {
                input_bytes[pos - (width + spacer) - 1] == PAPER_ROLL
            };
            let top_middle = if y == 0 {
                false
            } else {
                input_bytes[pos - (width + spacer)] == PAPER_ROLL
            };
            let top_right = if x == width - 1 || y == 0 {
                false
            } else {
                input_bytes[pos - (width + spacer) + 1] == PAPER_ROLL
            };
            let left = if x == 0 {
                false
            } else {
                input_bytes[pos - 1] == PAPER_ROLL
            };
            let right = if x == width - 1 {
                false
            } else {
                input_bytes[pos + 1] == PAPER_ROLL
            };
            let bottom_left = if x == 0 || y == height - 1 {
                false
            } else {
                input_bytes[pos + (width + spacer) - 1] == PAPER_ROLL
            };
            let bottom_middle = if y == height - 1 {
                false
            } else {
                input_bytes[pos + (width + spacer)] == PAPER_ROLL
            };
            let bottom_right = if x == width - 1 || y == height - 1 {
                false
            } else {
                input_bytes[pos + (width + spacer) + 1] == PAPER_ROLL
            };
            let neighbours = top_left as u8
                + top_middle as u8
                + top_right as u8
                + left as u8
                + right as u8
                + bottom_left as u8
                + bottom_middle as u8
                + bottom_right as u8;

            let neighbourhood_index = ((y + 1) * (width + 2)) + 1 + x;
            neighbourhood[neighbourhood_index] = neighbours;
            if neighbours < 4 {
                next_removals.push(neighbourhood_index);
            }
        }
    }

    // move rolls out of the neighbourhood
    let mut total_removals = next_removals.len();
    while next_removals.len() > 0 {
        next_removals = remove_neighbours(&mut neighbourhood, &next_removals, width);
        total_removals += next_removals.len();
    }

    Some(total_removals as u64)
}

fn remove_neighbours(
    neighbourhood: &mut Vec<u8>,
    removals: &Vec<usize>,
    width: usize,
) -> Vec<usize> {
    let mut next_removals: Vec<usize> = vec![];

    fn decrement_neighbour_count(
        next_removals: &mut Vec<usize>,
        neighbourhood: &mut Vec<u8>,
        index: usize,
    ) {
        let val = neighbourhood[index];
        if val == 0 {
            return;
        }

        // 4 neighbours before removing one
        if val == 4 {
            next_removals.push(index);
        }

        neighbourhood[index] = val - 1;
    }

    for &index in removals {
        // remove
        neighbourhood[index] = 0;

        // update the neighbours
        decrement_neighbour_count(&mut next_removals, neighbourhood, index - (width + 2) - 1);
        decrement_neighbour_count(&mut next_removals, neighbourhood, index - (width + 2));
        decrement_neighbour_count(&mut next_removals, neighbourhood, index - (width + 2) + 1);
        decrement_neighbour_count(&mut next_removals, neighbourhood, index - 1);
        decrement_neighbour_count(&mut next_removals, neighbourhood, index + 1);
        decrement_neighbour_count(&mut next_removals, neighbourhood, index + (width + 2) - 1);
        decrement_neighbour_count(&mut next_removals, neighbourhood, index + (width + 2));
        decrement_neighbour_count(&mut next_removals, neighbourhood, index + (width + 2) + 1);
    }

    next_removals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }

    #[test]
    fn test_get_grid_height_width_spacer() {
        let expected_spacer = if input.contains("\r\n") { 2 } else { 1 };
        let result =
            get_grid_height_width_spacer(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (10, 10, expected_spacer));
    }
}
