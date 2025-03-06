//! Advent of code 2024 Day 8
//! https://adventofcode.com/2024/day/8
//!
//! Part 1, compute antinodes just one distance away
//! Part 2, compute antinodes as lines drawn throut 2 antennae
//!
//!

use std::fmt::{Display, Formatter, Write};
advent_of_code::solution!(8);

#[derive(Debug, Clone, PartialEq)]
struct Distance {
    name: char,
    x_distance: i32,
    y_distance: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Antenna {
    name: char,
    x_position: i32,
    y_position: i32,
}

impl Distance {
    pub(crate) fn new(name: char, x_distance: i32, y_distance: i32) -> Distance {
        Distance {
            name,
            x_distance: x_distance,
            y_distance: y_distance,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    antenna: Option<char>,
    antennae_distances: Vec<Distance>,
    is_antinode: Option<bool>,
}

impl Node {
    pub(crate) fn get_antenna_name(&self) -> Option<char> {
        return self.antenna;
    }

    fn compute_antinode(&mut self) {
        for i in 0..self.antennae_distances.len() - 1 {
            let distance = &self.antennae_distances[i];
            let rest: &[Distance] = &self.antennae_distances[(i + 1)..];
            for j in 0..rest.len() {
                let other_distance = &rest[j];

                if distance.name == other_distance.name
                    && ((other_distance.x_distance == 2 * distance.x_distance
                        && other_distance.y_distance == 2 * distance.y_distance)
                        || (2 * other_distance.x_distance == distance.x_distance
                            && 2 * other_distance.y_distance == distance.y_distance))
                {
                    self.is_antinode = Some(true);
                    return;
                }
            }
        }
        self.is_antinode = Some(false)
    }

    pub(crate) fn set_antinode(&mut self) {
        self.is_antinode = Some(true);
    }

    fn is_antinode(&self) -> bool {
        if self.is_antinode.is_none() {
            return false;
        }
        self.is_antinode.unwrap()
    }

    fn to_char(&self) -> char {
        let has_antinode = self.is_antinode();
        if self.antenna.is_none() {
            if has_antinode {
                return '#';
            }
            return '.';
        }
        if has_antinode {
            '*'
        } else {
            self.antenna.unwrap()
        }
    }
}

struct CityMap {
    antennae: Vec<Antenna>,
    node_map: Vec<Vec<Node>>,
}

impl Display for CityMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.node_map.iter() {
            for node in row.iter() {
                f.write_char(node.to_char())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl CityMap {
    fn load_map_data(input: &str) -> Vec<Vec<char>> {
        let result: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim_end().chars().collect())
            .collect();
        // Sanity check the input data
        let col_count = result[0].len();
        for row in result.iter() {
            assert_eq!(col_count, row.len());
        }
        result
    }

    fn new(input: &str) -> Self {
        let char_map = CityMap::load_map_data(input);
        let mut node_map: Vec<Vec<Node>> = Vec::new();

        for y in 0..char_map.len() {
            node_map.push(Vec::new());
            for x in 0..char_map[y].len() {
                let character = char_map[y][x];
                let new_node = match character {
                    '.' => Node {
                        antenna: None,
                        antennae_distances: vec![],
                        is_antinode: None,
                    },
                    'a'..='z' | 'A'..='Z' | '0'..='9' => Node {
                        antenna: Some(character),
                        antennae_distances: vec![],
                        is_antinode: None,
                    },
                    _ => panic!("unknown character '{}' in map ", character),
                };
                node_map[y].push(new_node);
            }
        }

        CityMap {
            antennae: Self::collect_antennae(&node_map),
            node_map,
        }
    }

    pub fn get_node(&self, x: usize, y: usize) -> &Node {
        return &self.node_map[y][x];
    }

    pub fn collect_antennae(node_map: &Vec<Vec<Node>>) -> Vec<Antenna> {
        let mut antennae: Vec<Antenna> = Vec::new();
        for y in 0..node_map.len() {
            for x in 0..node_map[y].len() {
                let node = &node_map[y][x];
                let antenna_name = node.get_antenna_name();
                if antenna_name.is_some() {
                    let antenna_name = antenna_name.unwrap();
                    // Add to the set of antennae
                    antennae.push(Antenna {
                        name: antenna_name,
                        x_position: x as i32,
                        y_position: y as i32,
                    });
                }
            }
        }
        antennae
    }

    pub fn compute_distances(&mut self) {
        for y in 0..self.node_map.len() {
            for x in 0..self.node_map[y].len() {
                for antenna in self.antennae.iter() {
                    let node: &mut Node = &mut self.node_map[y][x];
                    node.antennae_distances.push(Distance::new(
                        antenna.name,
                        antenna.x_position - x as i32,
                        antenna.y_position - y as i32,
                    ));
                }
            }
        }
    }

    pub fn compute_antinodes_part_one(&mut self) {
        self.compute_distances();
        for y in 0..self.node_map.len() {
            for x in 0..self.node_map[y].len() {
                let node: &mut Node = &mut self.node_map[y][x];
                node.compute_antinode()
            }
        }
    }

    pub fn count_antinodes(&self) -> u64 {
        let mut count: u64 = 0;
        for y in 0..self.node_map.len() {
            for x in 0..self.node_map[y].len() {
                let node = &self.node_map[y][x];
                if node.is_antinode() {
                    count += 1;
                }
            }
        }
        count
    }

    fn mark_antinodes(&mut self, antenna: &Antenna, x_dist: i32, y_dist: i32) {
        let mut curr_x = antenna.x_position;
        let mut curr_y = antenna.y_position;
        let max_x = self.node_map[0].len() as i32;
        let max_y = self.node_map.len() as i32;
        while curr_x >= 0 && curr_x < max_x && curr_y >= 0 && curr_y < max_y {
            self.node_map[curr_y as usize][curr_x as usize].set_antinode();
            curr_x += x_dist;
            curr_y += y_dist;
        }
    }

    fn compute_antinodes_recursive(&mut self, antenna: &Antenna, rest: Vec<&Antenna>) {
        if rest.len() == 0 {
            return;
        }

        for other_antenna in rest.iter() {
            if other_antenna.name == antenna.name {
                // mark the two antennae as antinodes
                self.node_map[antenna.y_position as usize][antenna.x_position as usize]
                    .set_antinode();
                self.node_map[other_antenna.y_position as usize][other_antenna.x_position as usize]
                    .set_antinode();
                
                // compute the distance between the two antennae
                let x_dist = antenna.x_position - other_antenna.x_position;
                let y_dist = antenna.y_position - other_antenna.y_position;
                self.mark_antinodes(&antenna, x_dist, y_dist);
                self.mark_antinodes(&antenna, -x_dist, -y_dist);

                // let x_dist = other_antenna.x_position - antenna.y_position;
                // let y_dist = other_antenna.y_position - antenna.x_position;
                // self.mark_antinodes(&antenna, x_dist, y_dist);
                // self.mark_antinodes(&antenna, -x_dist, -y_dist);
            }
        }
        let head = rest[0];
        let rest = rest[1..].to_vec();
        self.compute_antinodes_recursive(head, rest);
    }

    pub fn compute_antinodes_part_two(&mut self) {
        let antennae = self.antennae.clone();
        let head = &antennae[0];
        let rest : Vec<&Antenna>= antennae[1..].iter().collect();
        self.compute_antinodes_recursive(head, rest);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = CityMap::new(input);

    map.compute_antinodes_part_one();
    print!("{}", map);

    // Answer with input data from AOC is 308
    Some(map.count_antinodes())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = CityMap::new(input);
    map.compute_antinodes_part_two();
    print!("{}", map);

    // Answer with input data from AOC is ???
    Some(map.count_antinodes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map_data() {
        let map = super::CityMap::new(".aA0zZ9");
        assert_eq!(
            &Node {
                antenna: None,
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(0, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('a'),
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(1, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('A'),
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(2, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('0'),
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(3, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('z'),
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(4, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('Z'),
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(5, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('9'),
                antennae_distances: vec![],
                is_antinode: None,
            },
            map.get_node(6, 0)
        );

        assert_eq!(
            map.antennae,
            vec![
                Antenna {
                    name: 'a',
                    x_position: 1,
                    y_position: 0
                },
                Antenna {
                    name: 'A',
                    x_position: 2,
                    y_position: 0
                },
                Antenna {
                    name: '0',
                    x_position: 3,
                    y_position: 0
                },
                Antenna {
                    name: 'z',
                    x_position: 4,
                    y_position: 0
                },
                Antenna {
                    name: 'Z',
                    x_position: 5,
                    y_position: 0
                },
                Antenna {
                    name: '9',
                    x_position: 6,
                    y_position: 0
                },
            ]
        );
    }

    #[test]
    fn test_has_antinode() {
        let mut node = Node {
            antenna: None,
            antennae_distances: vec![Distance::new('a', 1, 1), Distance::new('a', 2, 2)],
            is_antinode: None,
        };
        node.compute_antinode();
        assert!(node.is_antinode());
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
