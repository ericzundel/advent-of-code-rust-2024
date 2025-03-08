use std::collections::{HashMap, HashSet};
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
        for block in self.blocks.iter() {
            match block {
                Block::Free => write!(f, " . ")?,
                Block::File(id) => write!(f, "[{}]", id)?,
            }
        }
        writeln!(f)?;
        Ok(())
    }
}

impl From<Filesystem> for Disk {
    fn from(value: Filesystem) -> Self {
        let mut blocks = Vec::new();
        for file in value.files {
            for _ in 0..file.len {
                if file.id.is_none() {
                    blocks.push(Block::Free);
                } else {
                    blocks.push(Block::File(file.id.unwrap()));
                }
            }
        }
        Disk { blocks }
    }
}

impl Disk {
    pub fn new(input: &str) -> Disk {
        let input = input.trim();
        let mut input_blocks: Vec<Block> = Vec::new();
        let mut id = 0;
        let mut is_file = true;
        // Track the total number of blocks as a consistency check
        let mut block_count: usize = 0;
        for i in 0..input.len() {
            let val: u64 = input[i..i + 1].parse().unwrap();
            block_count += val as usize;
            if is_file {
                assert!(val > 0);
            }
            for _ in 0..val {
                if is_file {
                    input_blocks.push(Block::File(id));
                } else {
                    input_blocks.push(Block::Free);
                }
            }
            if is_file {
                id += 1;
            }
            is_file = !is_file;
        }
        assert_eq!(input_blocks.len(), block_count);

        Disk {
            blocks: input_blocks,
        }
    }

    pub fn optimize_part_one(self) -> Disk {
        let mut start_index = 0;
        let mut end_index = self.blocks.len() - 1;
        let start_len = self.blocks.len();
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
                    assert!(new_end_index < end_index);
                    if new_end_index > start_index {
                        optimized.blocks[start_index] = block.clone();
                        end_index = new_end_index;
                    } else {
                        // Tricky edge case!  There was a free block right before the end!
                        if optimized.blocks[start_index] == Block::Free {
                            optimized.blocks[start_index] = self.blocks[end_index].clone();
                        }
                        break;
                    }
                }
            }
            start_index += 1;
        }

        assert_eq!(self.blocks.len(), start_len);

        // Check the optimized list to make sure there are no free blocks in the middle.
        let mut is_free = false;
        for i in 0..optimized.blocks.len() {
            let block = &optimized.blocks[i];
            match block {
                Block::Free => is_free = true,
                Block::File(_) => {
                    if is_free {
                        println!("{}", optimized);
                        println!("Found free block in the middle at {i}");
                        assert!(false);
                    }
                }
            }
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
                }
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
        if disk1.blocks.len() != disk2.blocks.len() {
            dbg!(
                "Disks differ in block length",
                disk1.blocks.len(),
                disk2.blocks.len()
            );
            return false;
        }
        let hash1 = disk1.build_hash();
        let hash2 = disk2.build_hash();
        assert_eq!(hash1.len(), hash2.len());
        for (key, value) in hash1.iter() {
            let other_value = hash2.get(key);
            if other_value.is_none() {
                dbg!("different number of blocks for id {:?} (0)", key);
                return false;
            }
            if value != other_value.unwrap() {
                dbg!(
                    "for key {:?} value {:?} differs from {:?}",
                    key,
                    value,
                    other_value
                );
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let orig_disk = Disk::new(input);
    // println!("Before Optimization:\n{}", orig_disk);
    let optimized = orig_disk.clone().optimize_part_one();
    // println!("After Optimization:\n{}", optimized);
    assert!(Disk::consistency_check(&orig_disk, &optimized));
    // Answer with AOC data is 6386640365805
    Some(optimized.checksum())
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct File {
    id: Option<u64>,
    len: usize,
}

#[derive(Debug, Clone)]
struct Filesystem {
    files: Vec<File>,
}

impl Filesystem {
    pub fn new(input: &str) -> Filesystem {
        let mut files: Vec<File> = Vec::new();
        let mut id = 0;
        let mut is_file: bool = true;
        for character in input.trim().chars() {
            let val: u8 = format!("{}", character).parse().unwrap();
            if is_file {
                files.push(File {
                    id: Some(id),
                    len: val as usize,
                });
                id += 1;
            } else {
                files.push(File {
                    id: None,
                    len: val as usize,
                })
            }
            is_file = !is_file;
        }
        Filesystem { files }
    }

    pub(crate) fn optimize_part_two(&self) -> Filesystem {
        let mut optimized_files = Vec::new();
        let mut remaining_files: Vec<File> =self.files.clone();
        let mut copied_files_set: HashSet<File> = HashSet::with_capacity(self.files.len());
        let mut file_idx = 0;
        while file_idx < remaining_files.len() {
            let file :&mut File = &mut remaining_files[file_idx];
            // still needed?
            if !copied_files_set.contains(file) {
                if file.id.is_some() {
                    copied_files_set.insert(file.clone());
                    optimized_files.push(file.clone());
                    let file_to_copy = remaining_files.remove(file_idx);
                    remaining_files.insert(
                        file_idx,
                        File {
                            id: None,
                            len: file_to_copy.len,
                        },
                    );
                } else {
                    // We have a hole. We need to fill it.
                    let mut len = file.len;
                    'inner: while len > 0 {
                        let idx = Self::rev_find_from_idx(len, &remaining_files);
                        if idx.is_none() {
                            break 'inner;
                        }
                        let idx = idx.unwrap();
                        let file_to_copy = remaining_files.remove(idx);
                        remaining_files.insert(
                            idx,
                            File {
                                id: None,
                                len: file_to_copy.len,
                            },
                        );
                        copied_files_set.insert(file_to_copy.clone());
                        optimized_files.push(file_to_copy.clone());
                        len -= file_to_copy.len;
                    }
                    // Didn't fill the hole? Let's create a free spot on the optimized disk to match.
                    if len > 0 {
                        optimized_files.push(File { id: None, len: len });
                    }
                }
            }
            file_idx += 1;
        }

        Filesystem {
            files: optimized_files,
        }
    }

    fn rev_find_from_idx(len: usize, working_files: &Vec<File>) -> Option<usize> {
        for i in (0..working_files.len()).rev() {
            let file = &working_files[i];
            if file.id.is_some() && file.len <= len {
                return Some(i);
            }
        }
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let orig_disk = Disk::new(input);
    let orig_filesystem = Filesystem::new(input);
    let reconstituted_disk = orig_filesystem.clone().into();
    print!("Original Disk:  {}", orig_disk);
    assert!(Disk::consistency_check(&orig_disk, &reconstituted_disk));
    let optimized_filesystem = orig_filesystem.optimize_part_two();
    let reconstituted_disk = optimized_filesystem.into();
    print!("Optimized Disk: {}", reconstituted_disk);
    assert!(Disk::consistency_check(&orig_disk, &reconstituted_disk));
    
    // Answer from AOC data is 6423258376982
    Some(reconstituted_disk.checksum())
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
        assert_eq!(expected, orig_disk.optimize_part_one());
    }

    #[test]
    fn test_checksum() {
        let optimized = Disk::new("1120331").optimize_part_one();
        let expected: u64 = 0 * 0 + 3 * 1 + 1 * 2 + 1 * 3 + 2 * 4 + 2 * 5 + 2 * 6;
        assert_eq!(expected, optimized.checksum())
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_disk_from_filesystem() {
        let input = "1120331";
        let orig_disk = Disk::new(input);
        let orig_filesystem = Filesystem::new(input);
        let reconstituted_disk: Disk = orig_filesystem.into();
        assert_eq!(&orig_disk, &reconstituted_disk);
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
