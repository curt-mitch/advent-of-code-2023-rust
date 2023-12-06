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

fn calc_num_overlap(ip: &str) -> u32 {
    let mut overlap_count: u32 = 0;
    let card_sets: Vec<&str> = strip_line_prefix(ip, ':').split('|').collect();
    // println!("card_sets: {:?}", card_sets);
    let first_card_set = card_sets[0].split(' ').collect::<Vec<&str>>();
    let second_card_set = card_sets[1].split(' ').collect::<Vec<&str>>();
    // println!("first card set: {:?}", first_card_set);
    // println!("second card set: {:?}", second_card_set);
    for number in first_card_set {
        println!("number: {}", number);
        if number.parse::<u32>().is_ok() && second_card_set.contains(&number) {
            println!("overlap number: {}", number);
            overlap_count += 1;
        }
    }

    println!("overlap count: {}", overlap_count);

    return overlap_count;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let winning_num_count: u32 = calc_num_overlap(line);
        let base: u32 = 2;
        if winning_num_count > 0 {
            result += base.pow(winning_num_count - 1);
        }
        println!("result: {}", result);
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
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
