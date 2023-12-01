use std::{collections::HashMap, vec};

advent_of_code::solution!(1);

fn find_int_pair_part1(ip: &str) -> String {
    const RADIX: u32 = 10;
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    let mut result: String = String::new();
    for c in ip.chars() {
        if c.is_digit(RADIX) {
            if first == None {
                first = Some(c);
            }
            last = Some(c);
        }
    }
    result.push(first.unwrap());
    result.push(last.unwrap());
    return result;
}

fn find_int_pair_part2(ip: &str) -> String {
    const RADIX: u32 = 10;
    let num_list = vec![
        "one".to_string(),
        "two".into(),
        "three".into(),
        "four".into(),
        "five".into(),
        "six".into(),
        "seven".into(),
        "eight".into(),
        "nine".into(),
    ];
    let mut number_map: HashMap<String, char> = HashMap::with_capacity(9);

    number_map.insert("one".to_string(), "1".chars().nth(0).unwrap());
    number_map.insert("two".into(), "2".chars().nth(0).unwrap());
    number_map.insert("three".into(), "3".chars().nth(0).unwrap());
    number_map.insert("four".into(), "4".chars().nth(0).unwrap());
    number_map.insert("five".into(), "5".chars().nth(0).unwrap());
    number_map.insert("six".into(), "6".chars().nth(0).unwrap());
    number_map.insert("seven".into(), "7".chars().nth(0).unwrap());
    number_map.insert("eight".into(), "8".chars().nth(0).unwrap());
    number_map.insert("nine".into(), "9".chars().nth(0).unwrap());
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    let mut result: String = String::new();
    for (i, el) in ip.chars().enumerate() {
        if el.is_digit(RADIX) {
            if first == None {
                first = Some(el);
            }
            last = Some(el);
        }
        if check_numeral_string(ip, i, &num_list).0 {
            let num_string = check_numeral_string(ip, i, &num_list).1;
            let current_num: Option<char> = number_map.get(&num_string).copied();
            if first == None {
                first = current_num;
            }
            last = current_num;
        }
    }

    result.push(first.unwrap());
    result.push(last.unwrap());

    return result;
}

fn check_numeral_string(ip: &str, index: usize, num_map: &Vec<String>) -> (bool, String) {
    for num in num_map {
        if ip[index..].to_string().contains(num) && ip[index..].to_string().find(num) == Some(0) {
            return (true, num.to_string());
        }
    }
    return (false, ip.to_string());
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let pair: String = find_int_pair_part1(line);
        let pair_sum: u32 = pair.parse::<u32>().unwrap();

        result += pair_sum;
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let pair: String = find_int_pair_part2(line);
        let pair_sum: u32 = pair.parse::<u32>().unwrap();

        result += pair_sum;
    }

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
