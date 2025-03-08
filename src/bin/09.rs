use std::collections::HashMap;
use std::fmt::{Display, Formatter};

advent_of_code::solution!(9);

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Block {
    File(u64),
    Free,
}

#[derive(Clone, Debug, PartialEq)]
struct Disk {
    blocks: Vec<Block>,
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Input:  ")?;
        for block in self.blocks.iter() {
            match block {
                Block::Free => write!(f, ".")?,
                Block::File(id) => write!(f, "[{}]", id)?,
            }
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Disk {
    pub fn new(input: &str) -> Disk {
        let input = input.trim();
        let mut input_blocks: Vec<Block> = Vec::new();
        let mut id = 0;
        for i in (0..input.len()).step_by(2) {
            let file_blocks = input[i..i + 1].parse().unwrap();

            for _ in 0..file_blocks {
                input_blocks.push(Block::File(id));
            }
            if i + 1 < input.len() {
                let free_blocks = input[i + 1..i + 2].parse().unwrap();
                for _ in 0..free_blocks {
                    input_blocks.push(Block::Free);
                }
            }
            id += 1;
        }
        Disk {
            blocks: input_blocks,
        }
    }

    pub fn optimize(self) -> Disk {
        let mut start_index = 0;
        let mut end_index = self.blocks.len() - 1;
        let input_blocks = &self.blocks;
        let optimized_blocks: Vec<Block> = vec![Block::Free; input_blocks.len()];
        let mut optimized = Disk {
            blocks: optimized_blocks,
        };
        while start_index <= end_index {
            // println!("{}", optimized);
            // println!("Start: {}  End: {}", start_index, end_index);
            match input_blocks[start_index] {
                Block::File(_id) => {
                    optimized.blocks[start_index] = input_blocks[start_index].clone()
                }
                Block::Free => {
                    let (new_end_index, block) = self.find_last_nonfree_block(end_index);
                    if new_end_index > start_index {
                        optimized.blocks[start_index] = block.clone();
                        end_index = new_end_index;
                    }
                }
            }
            start_index += 1;
        }
        optimized
    }

    fn find_last_nonfree_block(&self, end_index: usize) -> (usize, &Block) {
        for i in (0..=end_index).rev() {
            match self.blocks[i] {
                Block::File(_) => return (i - 1, &self.blocks[i]),
                _ => continue,
            }
        }
        (0, &Block::Free)
    }

    pub fn checksum(&self) -> u64 {
        let mut result: u64 = 0;
        for i in 0..self.blocks.len() {
            match self.blocks[i] {
                Block::File(id) => {
                    let mul = id.checked_mul(i as u64).unwrap();
                    result = result.checked_add(mul).unwrap();
                },
                Block::Free => continue,
            }
        }
        result
    }

    pub fn build_hash(&self) -> HashMap<&Block, u64> {
        let mut result: HashMap<&Block, u64> = HashMap::new();
        for block in self.blocks.iter() {
            let found: Option<&mut u64> = result.get_mut(block);
            if found.is_none() {
                result.insert(block, 1);
            } else {
                *found.unwrap() += 1;
            }
        }
        result
    }
    
    /// Checks the two disks to see that they contain the same blocks (regardless of position)
    pub fn consistency_check(disk1: &Disk, disk2: &Disk) -> bool {
        if (disk1.blocks.len() != disk2.blocks.len()) {
            return false;
        }
        let hash1 = disk1.build_hash();
        let hash2 = disk2.build_hash();
        assert_eq!(hash1.len(), hash2.len());
        for (key, value) in hash1.iter() {
            let other_value = hash2.get(key);
            if other_value.is_none() { return false; }
            if value != other_value.unwrap() {
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let orig_disk = Disk::new(input);
    println!("Before Optimization:\n{}", orig_disk);
    let optimized = orig_disk.clone().optimize();
    println!("After Optimization:\n{}", optimized);
    assert!(Disk::consistency_check(&orig_disk, &optimized));
    // 6386640371050 is too high in AOC data
    Some(optimized.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_disk1() {
        let expected: Disk = Disk {
            blocks: vec![
                Block::File(0),
                Block::Free,
                Block::File(1),
                Block::File(1),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(3),
            ],
        };
        assert_eq!(expected, Disk::new("1120331"));
    }
    
    #[test]
    fn test_new_disk2() {
        let expected: Disk = Disk {
            blocks: vec![
                Block::File(0),
                Block::Free,
                Block::File(1),
                Block::File(1),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File(3),
                Block::Free,
            ],
        };
        assert_eq!(expected, Disk::new("11203311"));
    }

    #[test]
    fn test_optimized() {
        let expected: Disk = Disk {
            blocks: vec![
                Block::File(0),
                Block::File(3),
                Block::File(1),
                Block::File(1),
                Block::File(2),
                Block::File(2),
                Block::File(2),
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
            ],
        };
        let orig_disk = Disk::new("1120331");
        assert_eq!(expected, orig_disk.optimize());
    }

    #[test]
    fn test_checksum() {
        let optimized = Disk::new("1120331").optimize();
        let expected: u64 = 0 * 0 + 3 * 1 + 1 * 2 + 1 * 3 + 2 * 4 + 2 * 5 + 2 * 6;
        assert_eq!(expected, optimized.checksum())
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
