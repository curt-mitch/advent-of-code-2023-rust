advent_of_code::solution!(6);

/*
Part 1 solution:
1. strip labels for each row and turn numerals into a vector of u32
2. for each pair of time and distance numbers, compute the number of winning possible combinations (see compute_winning_combos body)
*/

fn strip_line_prefix(input: Option<&str>, character: char) -> &str {
    match input.unwrap().find(character) {
        Some(index) => &input.unwrap()[index + 1..].trim(),
        None => input.unwrap(),
    }
}

fn string_to_digits(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn compute_winning_combos(time: &u32, distance: &u32) -> u32 {
    let mut winning_combo_count: u32 = 0;
    let mut current_ms: u32 = 1;
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 1;
    let mut lines = input.lines();
    let time_line: Option<&str> = lines.next();
    let distance_line = lines.next();
    let time_string: &str = strip_line_prefix(time_line, ':');
    let distance_string: &str = strip_line_prefix(distance_line, ':');
    let time_values: Vec<u32> = string_to_digits(time_string);
    let distance_values: Vec<u32> = string_to_digits(distance_string);

    for (time, distance) in time_values.iter().zip(distance_values.iter()) {
        result *= compute_winning_combos(time, distance);
    }
    return Some(result);
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
