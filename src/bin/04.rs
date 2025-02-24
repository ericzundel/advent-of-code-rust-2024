advent_of_code::solution!(4);

fn load_data(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn count_xmas_in_strings(lines: Vec<Box<String>>) -> u64 {
    lines
        .iter()
        .map(|x| count_xmas_in_str(x.as_str()))
        .sum::<u64>()
}

/// Count the number of times the string XMAS appears in a string
fn count_xmas_in_str(line: &str) -> u64 {
    let mut start = 0;
    let mut count = 0u64;
    loop {
        let tail = &line[start..];
        let pos = tail.find("XMAS");
        if pos.is_none() {
            break;
        }
        count += 1;
        start += pos.unwrap() + 4;
        assert!(count < line.len() as u64);
    }
    count
}

fn get_forward_reversed_str(input: Vec<&str>) -> Vec<Box<String>> {
    let mut results: Vec<Box<String>> = Vec::new();
    for line in input {
        results.push(Box::new(line.to_string()));
        let mut reversed: Box<String> = Box::new("".to_string());
        for character in line.to_string().chars().rev() {
            reversed.push(character);
        }
        results.push(Box::new(reversed.to_string()));
    }
    results
}

fn count_xmas_in_puzzle(input: Vec<&str>) -> u64 {
    let mut count = 0u64;
    
    // Count horizontal matches
    let horiz_lines = get_forward_reversed_str(input);
    count += count_xmas_in_strings(horiz_lines);
 
    // Count vertical matches
    
    // Count bottom left to upper right diagonal matches
    
    // Count top left to bottom right diagonal matches
    
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(count_xmas_in_puzzle(load_data(input)))
}

pub fn part_two(input: &str) -> Option<u64> {
    load_data(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas_in_str("XMAS"), 1);
        assert_eq!(count_xmas_in_str("XMASXMAS"), 2);
        assert_eq!(count_xmas_in_str("XMASXMASXMA"), 2);
        assert_eq!(count_xmas_in_str("ABCXMASSSX"), 1);
        assert_eq!(count_xmas_in_str("123456789"), 0);
    }
    
    #[test]
    fn test_count_xmas_in_strings() {
        let input = vec!(Box::new("XMAS".to_string()), Box::new("XMASXMAS".to_string()));
        assert_eq!(3, count_xmas_in_strings(input));
    }

    #[test]
    fn test_horiz_str() {
        let mut expected: Vec<Box<String>> = Vec::new();
        expected.push(Box::new("SAMX".to_string()));
        expected.push(Box::new("XMAS".to_string()));

        let mut input: Vec<&str> = Vec::new();
        input.push("SAMX");

        assert_eq!(expected, get_forward_reversed_str(input));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(999));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
