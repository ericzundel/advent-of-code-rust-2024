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

impl<'a> SafetyManual {
    fn is_valid(&self, page: u64, updates: &[u64]) -> bool {
        if updates.len() == 0 {
            return true;
        }
        for rule in self.rules.iter() {
            if rule.post == page {
                if updates.contains(&rule.pre) {
                    return false;
                }
            }
        }
        // Recursively search
        return self.is_valid(updates[0], &updates[1..]);
    }

    fn is_valid_update(&self, update: &Update) -> bool {
        self.is_valid(*update.pages.get(0).unwrap(), update.pages.as_slice())
    }
    
    // Helper function for valid_updates() also re-used for repairing updates
    fn calc_valid_updates(&'a self, updates: &'a Vec<Update>) -> Vec<&'a Update> {
        let mut results: Vec<&'a Update> = Vec::new();
        for update in updates.iter() {
            if self.is_valid_update(update) {
                results.push(update);
            }
        }
        results
    }

    // Return the list of valid updates from the SafetyManual
    fn valid_updates(&'a self) -> Vec<&'a Update> {
        self.calc_valid_updates(&self.updates)
    }
    
    // Return the list of invalid updates from the SafetyManual
    fn invalid_updates(&'a self) -> Vec<&'a Update> {
        let mut results: Vec<&'a Update> = Vec::new();
        for update in self.updates.iter() {
            if !self.is_valid(*update.pages.get(0).unwrap(), update.pages.as_slice()) {
                results.push(update);
            }
        }
        results
    }

    fn repair_pages(&'a self, pages: &'a Vec<u64>) -> Vec<u64> {
        // Recursively sort the vector using quicksort
        if pages.len() <= 1 {
            return pages.clone();
        }
        let split = pages.split_at(1);
        let partition = split.0[0];
        let remainder = split.1;
        let mut pre_list: Vec<u64> = Vec::new();
        let mut post_list: Vec<u64> = Vec::new();
        for page in remainder {
            if self.is_valid(partition, &[*page]) {
                post_list.push(*page);
            } else {
                pre_list.push(*page);
            }
        }

        let mut result = Vec::new();
        let pre_repaired: Vec<u64> = self.repair_pages(&pre_list);
        result.extend(pre_repaired);
        result.push(partition);
        let post_repaired: Vec<u64> = self.repair_pages(&post_list);
        result.extend(post_repaired);
        result
    }
    
    /// Given an update line, try to repair it by reordering the pages.
    /// If it can't be repaired, return None.
    fn repair_update(&self, update_to_repair: &Update) -> Option<Update> {
        let pages_to_repair = update_to_repair.clone().pages;
        let repared_pages = self.repair_pages(&pages_to_repair);
        let result = Update {
            pages: repared_pages.clone(),
        };
        if self.is_valid_update(&result) {
            Some(result)
        } else {
            None
        }
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
    let invalid_updates = manual.invalid_updates();
    let mut repaired_updates: Vec<Update> = Vec::new();
    for update in invalid_updates {
        match manual.repair_update(update) {
            Some(update) => repaired_updates.push(update),
            None => continue,
        }
    }
    if repaired_updates.len() == 0 {
        None
    } else {
        Some(
            repaired_updates
                .into_iter()
                .map(|x| x.clone().mid())
                .sum::<u64>(),
        )
    }
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

    fn example_safety_manual() -> SafetyManual {
        let expected_rules: Vec<Rule> = vec![
            Rule { pre: 12, post: 23 },
            Rule { pre: 34, post: 45 },
            Rule { pre: 23, post: 56 },
        ];
        let expected_updates: Vec<Update> = vec![
            Update {
                pages: vec![12, 23, 56],
            },
            Update {
                pages: vec![23, 56, 12],
            },
        ];
        SafetyManual {
            rules: expected_rules,
            updates: expected_updates,
        }
    }

    #[test]
    fn test_load_data() {
        let expected = example_safety_manual();
        assert_eq!(
            expected,
            load_data("12|23\n34|45\n23|56\n\n12,23,56\n23,56,12")
        )
    }

    #[test]
    fn test_valid_updates() {
        let safety_manual = example_safety_manual();
        assert_eq!(safety_manual.is_valid(12, &[12u64, 23u64, 56u64]), true);
        assert_eq!(safety_manual.is_valid(23, &[23u64, 56u64, 12u64]), false);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_repair_update() {
        let safety_manual = example_safety_manual();
        let invalid_update = Update {
            pages: vec![23u64, 56u64, 12u64],
        };
        let valid_update = Update {
            pages: vec![12u64, 23u64, 56u64],
        };
        assert_eq!(safety_manual.is_valid_update(&valid_update), true);
        assert_eq!(safety_manual.is_valid_update(&invalid_update), false);
        let result = safety_manual.repair_update(&invalid_update).unwrap();
        assert_eq!(result, valid_update)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
