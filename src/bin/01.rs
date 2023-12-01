advent_of_code::solution!(1);

fn find_int_pair(ip: &str) -> String {
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let pair: String = find_int_pair(line);
        let pair_sum: u32 = pair.parse::<u32>().unwrap();

        result += pair_sum;
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
