advent_of_code::solution!(4);

/*
Part 1 solution:
1. split each line by bar |
2. create a set for each collection of numbers
3. check for membership overlap in pair of sets
4. # of points per row is 2^(n-1), where n is amount of numbers in both sets

*/

fn strip_line_prefix(input: &str, character: char) -> &str {
    match input.find(character) {
        Some(index) => &input[index + 1..].trim(),
        None => input,
    }
}

fn calc_num_overlap_part1(ip: &str) -> u32 {
    let mut overlap_count: u32 = 0;
    let card_sets: Vec<&str> = strip_line_prefix(ip, ':').split('|').collect();
    let first_card_set = card_sets[0].split(' ').collect::<Vec<&str>>();
    let second_card_set = card_sets[1].split(' ').collect::<Vec<&str>>();

    for number in first_card_set {
        if number.parse::<u32>().is_ok() && second_card_set.contains(&number) {
            overlap_count += 1;
        }
    }

    return overlap_count;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let winning_num_count: u32 = calc_num_overlap_part1(line);
        let base: u32 = 2;
        if winning_num_count > 0 {
            result += base.pow(winning_num_count - 1);
        }
    }

    return Some(result);
}

/*
Part 2 solution:
1. split each line by bar |
2. create a set for each collection of numbers
3. check for membership overlap in pair of sets
4. create a vector of 1's equal to the number of rows
5. at each row, if the row has n overlapping numbers, add n times the copy to the following n rows.
    For example, if row 3 has 2 copies and contains 3 overlapping numbers, add 6 to the indicies for cards 4 and 5.
*/

fn calc_num_overlap_part2(ip: &str) -> u32 {
    let mut overlap_count: u32 = 0;
    let card_sets: Vec<&str> = strip_line_prefix(ip, ':').split('|').collect();
    let first_card_set = card_sets[0].split(' ').collect::<Vec<&str>>();
    let second_card_set = card_sets[1].split(' ').collect::<Vec<&str>>();

    for number in first_card_set {
        if number.parse::<u32>().is_ok() && second_card_set.contains(&number) {
            overlap_count += 1;
        }
    }

    return overlap_count;
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.lines().count();
    let mut card_count_vec: Vec<u32> = vec![1; line_count];
    let mut set_count_vec: Vec<u32> = vec![0; line_count];

    for (i, line) in input.lines().enumerate() {
        let overlap_set_count: u32 = calc_num_overlap_part2(line);
        set_count_vec[i] = overlap_set_count;
    }

    for (i, &count) in set_count_vec.iter().enumerate() {
        let mut look_ahead_index = i as u32 + 1;
        let mut current_count = count;
        while current_count > 0 && look_ahead_index < line_count as u32 {
            card_count_vec[look_ahead_index as usize] += card_count_vec[i];

            look_ahead_index += 1;
            current_count -= 1;
        }
    }

    return Some(card_count_vec.iter().sum::<u32>());
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
