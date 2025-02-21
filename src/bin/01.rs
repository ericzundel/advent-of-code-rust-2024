use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(1);

fn load_data(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut column1: Vec<u64> = Vec::new();
    let mut column2: Vec<u64> = Vec::new();
    for line in input.split_terminator("\n") {
        let mut values = line.split_whitespace();
        let first = values.next().unwrap();
        let second = values.next().unwrap();
        column1.push(u64::from_str(first).unwrap());
        column2.push(u64::from_str(second).unwrap());
    }
    (column1, column2)
}

pub fn part_one(input: &str) -> Option<u64> {
    // print!("Got input one: {}", input);
    let (mut column1, mut column2) = load_data(input);

    // Now, need to sort in ascending order
    column1.sort();
    column2.sort();

    let mut total: u64 = 0;
    for i in 0..column1.len() {
        total += column1[i].abs_diff(column2[i])
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut column1, mut column2) = load_data(input);

    let mut frequency_table: HashMap<u64, u64> = HashMap::new();
    column2
        .into_iter()
        .for_each(|key| match frequency_table.get_mut(&key) {
            Some(entry) => *entry += 1,
            None => {
                let _ = frequency_table.insert(key, 1);
            }
        });

    let zero: u64 = 0;
    Some(
        column1
            .into_iter()
            .map(|key| key * frequency_table.get(&key).unwrap_or(&zero))
            .sum(),
    )
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
