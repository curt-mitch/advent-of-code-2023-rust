advent_of_code::solution!(6);

/*
Part 1 solution:
1. strip labels for each row and turn numerals into a vector of u64
2. for each pair of time and distance numbers, compute the number of winning possible combinations (see compute_winning_combos body)
*/

fn strip_line_prefix(input: Option<&str>, character: char) -> &str {
    match input.unwrap().find(character) {
        Some(index) => &input.unwrap()[index + 1..].trim(),
        None => input.unwrap(),
    }
}

fn string_to_digits(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn compute_winning_combos(time: &u64, distance: &u64) -> u64 {
    let mut winning_combo_count: u64 = 0;
    let mut current_ms: u64 = 1;
    let mut reached_record_range: bool = false;

    while current_ms < *time {
        let current_rate = time - current_ms;
        let reachable_distance = current_ms * current_rate;

        if reachable_distance > *distance {
            winning_combo_count += 1;
        } else if reachable_distance <= *distance && reached_record_range {
            break;
        }

        if winning_combo_count > 0 {
            reached_record_range = true;
        }

        current_ms += 1;
    }

    return winning_combo_count;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 1;
    let mut lines = input.lines();
    let time_line: Option<&str> = lines.next();
    let distance_line = lines.next();
    let time_string: &str = strip_line_prefix(time_line, ':');
    let distance_string: &str = strip_line_prefix(distance_line, ':');
    let time_values: Vec<u64> = string_to_digits(time_string);
    let distance_values: Vec<u64> = string_to_digits(distance_string);

    for (time, distance) in time_values.iter().zip(distance_values.iter()) {
        result *= compute_winning_combos(time, distance);
    }
    return Some(result);
}

fn string_to_single_digit(input: &str) -> Result<u64, std::num::ParseIntError> {
    let combined = input.split_whitespace().collect::<String>();
    combined.parse::<u64>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time_line: Option<&str> = lines.next();
    let distance_line = lines.next();
    let time_string: &str = strip_line_prefix(time_line, ':');
    let distance_string: &str = strip_line_prefix(distance_line, ':');
    let time_value: Result<u64, std::num::ParseIntError> = string_to_single_digit(time_string);
    let distance_value: Result<u64, std::num::ParseIntError> =
        string_to_single_digit(distance_string);
    let result = compute_winning_combos(&time_value.unwrap(), &distance_value.unwrap());
    // for (time, distance) in time_values.iter().zip(distance_values.iter()) {
    //     result *= compute_winning_combos(time, distance);
    // }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
