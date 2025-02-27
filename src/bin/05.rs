//! https://https://adventofcode.com/2024/day/5
//!
advent_of_code::solution!(5);

use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
struct Rule {
    pre: u64,
    post: u64,
}

#[derive(PartialEq, Debug, Clone)]
struct Update {
    pages: Vec<u64>,
}

#[derive(PartialEq, Debug, Clone)]
struct SafetyManual {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Update {
    fn mid(self: Self) -> u64 {
        self.pages[self.pages.len() / 2]
    }
}

impl <'a>SafetyManual {
    fn is_valid(&self, page: u64, updates: &[u64]) -> bool {
        if updates.len() == 0 {
            return true;
        }
        let (head, remainder) = updates.split_at(1);
        for rule in self.rules.iter() {
            if rule.post == page {
                if remainder.contains(&rule.pre) {
                    return false;
                }
            }
        }
        // Recursively search
        return self.is_valid(head[0], remainder);
    }

    fn valid_updates(&'a self) -> Vec<&'a Update> {
        let mut results: Vec<&'a Update> = Vec::new();
        for update in self.updates.iter() {
            if self.is_valid(*update.pages.get(0).unwrap(), update.pages.as_slice()) {
                results.push(update);
            }
        }
        results
    }
}

/// Parse a line that looks like "01|02" into a Rule struct
fn parse_rule_line(rule_line: &str) -> Rule {
    let numbers: Vec<&str> = rule_line.split('|').collect();
    Rule {
        pre: numbers[0].parse().expect("Not a number!"),
        post: numbers[1].parse().expect("Not a number!"),
    }
}

/// Parse a line that looks like "01,02,03,..." into an Update struct
fn parse_update_line(update_line: &str) -> Update {
    let pages: Vec<u64> = update_line
        .split(',')
        .map(|y| {
            let val: u64 = y.parse().expect("Not a number!");
            val
        })
        .collect();
    // Unwritten rule of the input data.
    // Sanity check that there are always an odd number of pages
    assert_eq!(pages.len() % 2, 1);
    Update { pages }
}

/// Sanity checking on an unwritten rule of the dataset: all pages in the updates have
/// entries in the rule list. Useful to know if you want to walk a graph.
fn check_rules_and_updates(rules: &Vec<Rule>, updates: &Vec<Update>) {
    let mut set: HashSet<u64> = HashSet::new();
    rules.iter().for_each(|x| {
        set.insert(x.pre);
        set.insert(x.post);
    });
    updates.iter().for_each(|x| {
        x.pages
            .iter()
            .for_each(|y| assert!(set.contains(y), "Didn't find {}", y))
    });
}
fn load_data(input: &str) -> SafetyManual {
    let rules_lines: Vec<&str> = input.lines().filter(|x| x.contains("|")).collect();
    let updates_lines: Vec<&str> = input.lines().filter(|x| x.contains(',')).collect();
    let rules: Vec<Rule> = rules_lines.iter().map(|x| parse_rule_line(x)).collect();
    let updates: Vec<Update> = updates_lines.iter().map(|x| parse_update_line(x)).collect();

    // Sanity check: All pages in updates are listed somewhere in a rule
    check_rules_and_updates(&rules, &updates);
    SafetyManual { rules, updates }
}

pub fn part_one(input: &str) -> Option<u64> {
    let manual = load_data(input);
    let mut count = 0;
    for update in manual.valid_updates() {
        count += update.clone().mid();
    }
    // Answer to data from website is 6267
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let manual = load_data(input);
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
        let update = parse_update_line("12,34,56");
        assert_eq!(
            Update {
                pages: vec![12, 34, 56]
            },
            update
        );
        assert_eq!(34, update.mid());
    }

    #[test]
    fn test_load_data() {
        let expected_rules: Vec<Rule> = vec![
            Rule { pre: 12, post: 23 },
            Rule { pre: 34, post: 45 },
            Rule { pre: 56, post: 78 },
        ];
        let expected_updates: Vec<Update> = vec![
            Update {
                pages: vec![12, 34, 56],
            },
            Update {
                pages: vec![78, 23, 45],
            },
        ];
        let expected = SafetyManual {
            rules: expected_rules,
            updates: expected_updates,
        };
        assert_eq!(
            expected,
            load_data("12|23\n34|45\n56|78\n\n12,34,56\n78,23,45")
        )
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
