use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(PartialEq)]
enum Direction {
    Increasing(i64),
    Decreasing(i64),
    Flat,
}

struct Report {
    values: Vec<i64>,
    defects: usize,
}

impl Report {
    const THRESHOLD: usize = 3;
    pub fn new(report_data: &str) -> Report {
        let values = report_data
            .split_whitespace()
            .map(|x| i64::from_str(x).unwrap())
            .collect();
        let defects = Self::compute_defects(&values);
        Report { values, defects }
    }

    pub fn is_safe(&self) -> bool {
        self.defects <= 1
    }
    
    pub fn is_strictly_safe(&self) -> bool {
        self.defects == 0
    }
    
    fn compute_direction(difference: i64) -> Direction {
        if difference == 0 {
            Direction::Flat
        } else if difference < 0 {
            Direction::Decreasing(difference.abs())
        } else {
            Direction::Increasing(difference.abs())
        }
    }

    fn compute_defects(values: &Vec<i64>) -> usize {
        let mut directions: Vec<Direction> = Vec::new();
        for i in 1..values.len() {
            let difference = values[i].saturating_sub(values[i - 1]);
            directions.push(Self::compute_direction(difference));
        }
        
        let mut num_defects: usize = 0;
        let mut num_increasing: usize = 0;
        let mut num_decreasing: usize = 0;
        
        for direction in directions {
            match direction {
                Direction::Flat => num_defects += 1,
                Direction::Increasing(val) => {
                    if val as usize > Self::THRESHOLD {
                        num_defects += 1;
                    } else {
                        num_increasing += 1;
                    }
                }
                Direction::Decreasing(val) => {
                    if val as usize > Self::THRESHOLD {
                        num_defects += 1;
                    } else {
                        num_decreasing += 1;
                    }
                }
            }
        }
        if num_increasing > num_decreasing {
            num_defects += num_decreasing;
        } else {
            num_defects += num_increasing;
        }
        num_defects
    }
}
fn load_data(input: &str) -> Vec<Report> {
    input
        .split_terminator("\n")
        .map(|report_data| Report::new(report_data))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = load_data(input);
    let count: u64 = reports
        .iter()
        .filter(|x| x.is_strictly_safe())
        .count()
        .try_into()
        .unwrap();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = load_data(input);
    let count: u64 = reports
        .iter()
        .filter(|x| x.is_safe())
        .count()
        .try_into()
        .unwrap();
    Some(count)
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
