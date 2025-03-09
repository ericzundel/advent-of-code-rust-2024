use std::collections::HashMap;
use std::fmt::{Display, Formatter};

advent_of_code::solution!(11);

fn load_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn transform_stones(stones: &Vec<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::with_capacity(stones.len() / 2);
    for stone in stones {
        let stone_str = format!("{}", stone);
        if *stone == 0 {
            result.push(1);
        } else if stone_str.len() % 2 == 0 {
            let (head, tail) = stone_str.split_at(stone_str.len() / 2);
            result.push(head.parse().unwrap());
            result.push(tail.parse().unwrap());
        } else {
            result.push(stone.checked_mul(2024).unwrap());
        }
    }
    result
}

/// Keeping a list that doubles in size quickly gets out of hand
/// Looking at the output of shorter runs shows that we have many duplicate values.
/// Therefore, store the count of each value in a hash map so we can operate on each 
/// unique value once.
#[derive(Debug)]
struct StoneStore {
    stones: HashMap<u64, u64>,
}

impl Display for StoneStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stones)?;
        Ok(())
    }
}

impl StoneStore {
    pub fn new(input: &str) -> StoneStore {
        let mut stones: HashMap<u64, u64> = HashMap::new();

        let initial: Vec<u64> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        initial.iter().for_each(|x| {
            let result = stones.get_mut(x);
            if result.is_none() {
                stones.insert(x.clone(), 1);
            } else {
                let result: &mut u64 = result.unwrap();
                *result = *result + 1;
            }
        });
        StoneStore { stones }
    }

    fn incr_stones(stones: &mut  HashMap<u64,u64>, key : u64, value: u64) -> & mut HashMap<u64,u64> {
        let result = stones.get_mut(&key);
        if result.is_none() {
            stones.insert(key, value);
        } else {
            let result: &mut u64 = result.unwrap();
            *result = *result + value;
        }
        stones
    }

    fn decr_stones(stones: &mut  HashMap<u64,u64>, key : u64, value: u64) -> & mut HashMap<u64,u64>{
        let result = stones.get_mut(&key);
        if result.is_none() {
            panic!("Whatchootalkinboutwillis? key: {}", key)
        } else {
            let result: &mut u64 = result.unwrap();
            *result = result.checked_sub(value).unwrap();
        }
        stones
    }
    
    pub(crate) fn transform(&mut self) {
        let mut stones:& mut HashMap<u64,u64> = & mut self.stones.clone();
        for (key, value) in self.stones.iter_mut() {
            let stone_str = format!("{}", *key);
            if *key == 0 {
                stones = Self::incr_stones(stones, 1, *value);
                stones = Self::decr_stones(stones, 0, *value);
            } else if stone_str.len() % 2 == 0 {
                let (head, tail) = stone_str.split_at(stone_str.len() / 2);
                stones = Self::incr_stones(stones, head.parse().unwrap(), *value);
                stones = Self::incr_stones(stones, tail.parse().unwrap(), *value);
                stones = Self::decr_stones(stones, *key, *value);
            } else {
                stones = Self::incr_stones(stones, key.checked_mul(2024).unwrap(), *value);
                stones = Self::decr_stones(stones, *key, *value);
            }
        }
        self.stones = stones.clone();
    }

    pub fn len(&self) -> u64 {
        self.stones.iter().map(|(_key, value)| *value).sum::<u64>()
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = load_input(input);
    for _ in 0..25 {
        stones = transform_stones(&stones);
    }
    let stone_store: &mut StoneStore = &mut StoneStore::new(input);
    for _ in 0..25 {
        stone_store.transform();
    }
    assert_eq!(stones.len(), stone_store.len() as usize);
    
    // Answer from AOC data is 203457
    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stone_store: &mut StoneStore = &mut StoneStore::new(input);
    // 75 doublings is really really big.  Use the hash table version
    for _ in 0..75 {
        stone_store.transform();
    }
    // Answer from AOC data is 241394363462435
    Some(stone_store.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let stones = load_input("0 1 10 99 999");
        assert_eq!(vec![0, 1, 10, 99, 999], stones);
        let result = transform_stones(&stones);
        assert_eq!(vec![1, 2024, 1, 0, 9, 9, 2021976], result);
    }

    #[test]
    fn test_example2() {
        let mut stones = load_input("125 17");
        for _ in 0..6 {
            stones = transform_stones(&stones);
        }
        assert_eq!(22, stones.len());
    }

    #[test]
    fn test_example2_25() {
        let input = "125 17";
        let mut stones = load_input(input);
        for _ in 0..25 {
            stones = transform_stones(&stones);
        }
        println!("{:?}", stones);
        println!("Stone count: {}", stones.len());
        println!("Unique Values: ");
        let stone_store: &mut StoneStore = &mut StoneStore::new(input);
        for _ in 0..25 {
            stone_store.transform();
        }
        println!("{}", stone_store);
        println!("Stone count: {}", stone_store.len());

        assert_eq!(stone_store.len(), stones.len() as u64);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
