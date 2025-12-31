use std::str::Chars;

advent_of_code::solution!(6);

fn input_to_arrays(input: &str) -> (Vec<Vec<u64>>, Vec<u8>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut numbers = Vec::with_capacity(lines.len() - 1);

    for index in 0..(lines.len() - 1) {
        let line = lines[index];
        numbers.push(
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
        );
    }

    (
        numbers,
        lines
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.as_bytes()[0])
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let (numbers, operators) = input_to_arrays(input);
    Some((0..operators.len()).fold(0u64, |acc, index| {
        let operator = operators[index];
        acc + numbers.iter().fold(
            match operator {
                b'+' => 0,
                b'*' => 1,
                _ => panic!("Invalid operator"),
            },
            |equation, next_val| match operator {
                b'+' => equation + next_val[index],
                b'*' => equation * next_val[index],
                _ => panic!("Invalid operator"),
            },
        )
    }))
}

enum Operator {
    Add,
    Multiply,
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<Chars> = lines[0..lines.len() - 1]
        .iter()
        .map(|l| l.chars())
        .collect();
    let mut operators = lines[lines.len() - 1].chars();

    let mut running_total = 0u64;
    let mut equation_numbers: Vec<u64> = vec![];
    let mut operator = Operator::Add;

    loop {
        if let Some(next_operator) = operators.next() {
            match next_operator {
                '+' => operator = Operator::Add,
                '*' => operator = Operator::Multiply,
                ' ' => (),
                _ => panic!("Invalid operator"),
            }
        } else {
            break;
        }

        let new_num = numbers
            .iter_mut()
            .map(|chars| {
                if let Some(next_char) = chars.next()
                    && let Some(next_digit) = next_char.to_digit(10)
                {
                    Some(next_digit as i64)
                } else {
                    None
                }
            })
            .fold(-1, |acc, num| {
                if num.is_none() {
                    acc
                } else if acc < 0 {
                    num.unwrap()
                } else {
                    (acc * 10) + num.unwrap()
                }
            });

        if new_num < 0 {
            if equation_numbers.len() > 0 {
                // solve the previous equation
                running_total += equation_numbers.iter().fold(
                    match operator {
                        Operator::Add => 0,
                        Operator::Multiply => 1,
                    },
                    |acc, &num| match operator {
                        Operator::Add => acc + num,
                        Operator::Multiply => acc * num,
                    },
                );

                equation_numbers.clear();
            }

            continue;
        }

        equation_numbers.push(new_num as u64);
    }

    // do the final one
    running_total += equation_numbers.iter().fold(
        match operator {
            Operator::Add => 0,
            Operator::Multiply => 1,
        },
        |acc, &num| match operator {
            Operator::Add => acc + num,
            Operator::Multiply => acc * num,
        },
    );

    Some(running_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
    #[test]
    fn test_input_to_arrays() {
        let result = input_to_arrays(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            (
                vec![
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314]
                ],
                vec![b'*', b'+', b'*', b'+']
            )
        );
    }
}
