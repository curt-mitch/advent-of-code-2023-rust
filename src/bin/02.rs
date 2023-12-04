use std::collections::HashMap;

advent_of_code::solution!(2);

// store limits: 12 red, 13 green, and 14 blue
// part 1: read each row, if color exceeds limits, return 0, otherwise add game id to total
fn check_color_limits_part1(games: Vec<&str>) -> bool {
    let mut color_limits_map: HashMap<String, u32> = HashMap::with_capacity(3);

    color_limits_map.insert("red".to_string(), 12);
    color_limits_map.insert("green".into(), 13);
    color_limits_map.insert("blue".into(), 14);

    for round in games {
        let game_counts: Vec<&str> = round.split(',').map(|l| l.trim()).collect();
        for count in game_counts {
            let color_count: Vec<&str> = count.split(' ').collect();
            let count: u32 = color_count[0].parse::<u32>().unwrap();
            let color: String = color_count[1].to_string();
            let limit: u32 = color_limits_map.get(&color).unwrap().to_owned();
            if count > limit {
                return false;
            }
        }
    }

    return true;
}

fn get_game_id_part1(ip: &str) -> Option<&str> {
    let mut game_parts: Vec<&str> = ip.split(&[':', ';'][..]).collect();
    let game_id_string: &str = game_parts.remove(0);
    let mut game_id: Option<&str> = game_id_string.split(' ').last();

    if !check_color_limits_part1(game_parts) {
        game_id = Some("0");
    }

    return game_id;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let game_id: Option<&str> = get_game_id_part1(line);
        let game_value: u32 = game_id?.parse::<u32>().unwrap();

        result += game_value;
    }

    return Some(result);
}

// part 2: read each row, get max values for each color
fn check_color_limits_part2(games: Vec<&str>) -> u32 {
    let mut color_limits_map: HashMap<String, u32> = HashMap::with_capacity(3);
    let mut cube_value: u32 = 1;

    color_limits_map.insert("red".to_string(), 0);
    color_limits_map.insert("green".into(), 0);
    color_limits_map.insert("blue".into(), 0);

    for round in games {
        let game_counts: Vec<&str> = round.split(',').map(|l| l.trim()).collect();
        for count in game_counts {
            let color_count: Vec<&str> = count.split(' ').collect();
            let count: u32 = color_count[0].parse::<u32>().unwrap();
            let color: String = color_count[1].to_string();
            let limit: u32 = color_limits_map.get(&color).unwrap().to_owned();
            if count > limit {
                *color_limits_map.get_mut(&color).unwrap() = count;
            }
        }
    }

    for key in color_limits_map.keys() {
        let limit: u32 = color_limits_map.get(&key.to_string()).unwrap().to_owned();
        cube_value *= limit;
    }

    return cube_value;
}

fn get_game_id_part2(ip: &str) -> u32 {
    let mut game_parts: Vec<&str> = ip.split(&[':', ';'][..]).collect();

    game_parts.remove(0);

    return check_color_limits_part2(game_parts);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines() {
        let power_cube: u32 = get_game_id_part2(line);

        result += power_cube;
    }
    print!("result: {}", result);
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
