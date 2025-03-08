use std::char::from_digit;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

advent_of_code::solution!(10);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone)]
struct Island {
    topo_map: Vec<Vec<char>>,
}

impl Island {
    pub fn find_trailheads(&self) -> Vec<Position> {
        let mut result = Vec::new();
        for y in 0..self.topo_map.len() {
            for x in 0..self.topo_map[0].len() {
                if self.topo_map[y][x] == '0' {
                    result.push(Position { x: x, y: y });
                }
            }
        }
        result
    }
    pub(crate) fn sum_scores(&self) -> u64 {
        let trailheads = self.find_trailheads();
        let mut score: u64 = 0;
        for trailhead in trailheads {
            score += self.walk_trailhead(trailhead);
        }
        score
    }
}

impl Display for Island {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.topo_map.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Island {
    pub fn new(input: &str) -> Island {
        let mut topo_map: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            let line = line.trim();
            for c in line.chars() {
                row.push(c);
            }
            topo_map.push(row);
        }
        Island { topo_map }
    }

    fn find_adjacent(&self, curr: Position, value: char) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        let max_x = self.topo_map[0].len() - 1;
        let max_y = self.topo_map.len() - 1;
        if curr.x > 0 && self.topo_map[curr.y][curr.x - 1] == value {
            // west
            result.push(Position {
                x: curr.x - 1,
                y: curr.y,
            });
        }
        if curr.y > 0 && self.topo_map[curr.y - 1][curr.x] == value {
            // north
            result.push(Position {
                x: curr.x,
                y: curr.y - 1,
            });
        }
        if curr.y < max_y && self.topo_map[curr.y + 1][curr.x] == value {
            // south
            result.push(Position {
                x: curr.x,
                y: curr.y + 1,
            });
        }
        if curr.x < max_x && self.topo_map[curr.y][curr.x + 1] == value {
            // east
            result.push(Position {
                x: curr.x + 1,
                y: curr.y,
            });
        }
        result
    }
    fn find_ends_recursive<'a>(
        &'a self,
        curr: Position,
        found: &'a mut HashSet<Position>,
    ) -> &'a mut HashSet<Position> {
        let value = self.topo_map[curr.y][curr.x];

        if value == '9' {
            // base case!
            found.insert(curr.clone());
            // dbg!("Found", curr);
        } else {
            // Search for the next one!
            let next_char = from_digit(value.to_digit(10).unwrap() + 1, 10).unwrap();
            let adjacent_positions: Vec<Position> = self.find_adjacent(curr, next_char);
            for adjacent_position in adjacent_positions {
                self.find_ends_recursive(adjacent_position, found);
            }
        }
        found
    }

    /// Returns the number of distinct trails found to distinct endpoints.
    pub fn walk_trailhead(&self, trailhead: Position) -> u64 {
        let mut found: HashSet<Position> = HashSet::new();
        let ends: &mut HashSet<Position> = self.find_ends_recursive(trailhead, &mut found);
        return ends.len() as u64;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let island = Island::new(input);
    // Answer is 698 using AOC data
    Some(island.sum_scores())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut island = Island::new(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1 trailhead with a score of 1
    static EXAMPLE1: &str = "0123\n1234\n8765\n9876\n";

    // 1 trailhead with a score of 2
    static EXAMPLE2: &str = "9990999\n\
        9991999\n\
        9992999\n\
        6543456\n\
        7111117\n\
        8111118\n\
        9111119\n";

    static EXAMPLE3: &str = "1190559\n\
        5551598\n\
        8882717\n\
        6543456\n\
        7651987\n\
        8761111\n\
        9871111\n";

    // 2 trailheads with a score of 1 and 2
    static EXAMPLE4: &str = "1055955\n\
        2555855\n\
        3111711\n\
        4567654\n\
        1118113\n\
        6669662\n\
        7777701\n";

    #[test]
    fn test_island1() {
        let island = Island::new(EXAMPLE1);
        assert_eq!(vec![Position { x: 0, y: 0 }], island.find_trailheads());
        let sum_score = island.sum_scores();
        assert_eq!(1, sum_score);
    }

    #[test]
    fn test_adjacent_positions() {
        let island = Island::new(EXAMPLE2);
        assert_eq!(
            vec![Position { x: 3, y: 1 }],
            island.find_adjacent(Position { x: 3, y: 0 }, '1')
        );
        assert_eq!(
            vec![Position { x: 6, y: 3 }],
            island.find_adjacent(Position { x: 5, y: 3 }, '6')
        );
        assert_eq!(
            vec![Position { x: 0, y: 3 }],
            island.find_adjacent(Position { x: 1, y: 3 }, '6')
        );
        assert_eq!(
            vec![Position { x: 0, y: 6 }],
            island.find_adjacent(Position { x: 0, y: 5 }, '9')
        );
        assert_eq!(
            vec![Position { x: 2, y: 3 }, Position { x: 4, y: 3 }],
            island.find_adjacent(Position { x: 3, y: 3 }, '4')
        );
    }
    #[test]
    fn test_island2() {
        dbg!("test_island2()");
        let island = Island::new(EXAMPLE2);
        assert_eq!(vec![Position { x: 3, y: 0 }], island.find_trailheads());
        assert_eq!(2, island.sum_scores())
    }
    #[test]
    fn test_island3() {
        dbg!("test_island3()");
        let island = Island::new(EXAMPLE3);
        assert_eq!(vec![Position { x: 3, y: 0 }], island.find_trailheads());
        assert_eq!(4, island.sum_scores())
    }
    #[test]
    fn test_island4() {
        let island = Island::new(EXAMPLE4);
        assert_eq!(
            vec![Position { x: 1, y: 0 }, Position { x: 5, y: 6 }],
            island.find_trailheads()
        );
        assert_eq!(3, island.sum_scores())
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
