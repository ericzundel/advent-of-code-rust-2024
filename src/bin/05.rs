advent_of_code::solution!(5);

#[derive(PartialEq, Debug)]
struct Rule {
    pre: i32,
    post: i32,
}

#[derive(PartialEq, Debug)]
struct Update {
    pages: Vec<i32>,
}

/// Parse a line that looks like "01|02" into a Rule struct
fn parse_rule_line(rule_line: &str) -> Rule {
    let numbers: Vec<&str> = rule_line.split('|').collect();
    Rule {
        pre: numbers[0].parse().expect("Not a number!"),
        post: numbers[1].parse().expect("Not a number!"),
    }
}

// Parse a line that looks like "01,02,03,..." into an Update struct
fn parse_update_line(update_line: &str) -> Update {
    let pages: Vec<i32> = update_line
        .split(',')
        .map(|y| {
            let val: i32 = y.parse().expect("Not a number!");
            val
        })
        .collect();
    Update { pages }
}

fn load_data(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let rules_lines: Vec<&str> = input.lines().filter(|x| x.contains("|")).collect();
    let updates_lines: Vec<&str> = input.lines().filter(|x| x.contains(',')).collect();
    let rules: Vec<Rule> = rules_lines.iter().map(|x| parse_rule_line(x)).collect();
    let updates: Vec<Update> = updates_lines.iter().map(|x| parse_update_line(x)).collect();

    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = load_data(input);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = load_data(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_line() {
        assert_eq!(Rule { pre: 12, post: 34 }, parse_rule_line("12|34"));
    }
    #[test]
    fn test_parse_update_line() {
        assert_eq!(
            Update {
                pages: vec![12, 34, 56]
            },
            parse_update_line("12,34,56"),
        );
    }
    
    #[test]
    fn test_load_data() {
        let expected_rules: Vec<Rule> = vec![Rule{pre:12,post:23}, Rule{pre:34,post:45}];
        let expected_updates: Vec<Update> = vec![Update{pages: vec![12, 34, 56]},Update{pages: vec![78, 90, 10]}];
        let expected = (expected_rules, expected_updates);
        assert_eq!(expected, load_data("12|23\n34|45\n\n12,34,56\n78,90,10"))
    }
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
