/// https://adventofcode.com/2024/day/8
///
/// Part 1, compute antinodes
advent_of_code::solution!(8);

#[derive(Debug, Clone, PartialEq)]
struct Distance {
    name: char,
    x_distance: usize,
    y_distance: usize,
}

#[derive(Debug, PartialEq)]
struct Antenna {
    name: char,
    x_position: usize,
    y_position: usize,
}
impl Distance {
    pub(crate) fn new(name: char, x_distance: usize, y_distance: usize) -> Distance {
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
}

impl Node {
    pub(crate) fn get_antenna_name(&self) -> Option<char> {
        return self.antenna;
    }
}

impl Node {
    fn has_antinode(&self) -> bool {
        for i in 0..self.antennae_distances.len() - 1 {
            let distance = &self.antennae_distances[i];
            let rest: &[Distance] = &self.antennae_distances[(i + 1)..];
            for j in 0..rest.len() {
                let other_distance = &rest[j];

                if distance.name == other_distance.name &&
                    ((other_distance.x_distance == 2 * distance.x_distance
                        && other_distance.y_distance == 2 * distance.y_distance) || (2 * other_distance.x_distance == distance.x_distance
                    && 2 * other_distance.y_distance == distance.y_distance)) {
                    return true;
                }
            }
        }
        false
    }
}

struct CityMap {
    antennae: Vec<Antenna>,
    node_map: Vec<Vec<Node>>,
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
                    },
                    'a'..='z' | 'A'..='Z' | '0'..='9' => Node {
                        antenna: Some(character),
                        antennae_distances: vec![],
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

    fn get_node_mut(&mut self, x: usize, y: usize) -> &mut Node {
        return &mut self.node_map[y][x];
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
                        x_position: x,
                        y_position: y,
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
                        antenna.x_position.abs_diff(x),
                        antenna.y_position.abs_diff(y),
                    ));
                }
            }
        }
    }

    pub fn count_antinodes(&self) -> u64 {
        let mut count: u64 = 0;
        for y in 0..self.node_map.len() {
            for x in 0..self.node_map[y].len() {
                let node = &self.node_map[y][x];
                if node.has_antinode() {
                    count += 1;
                }
            }
        }
        count
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut map = CityMap::new(input);
    map.compute_distances();
    Some(map.count_antinodes())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
                antennae_distances: vec![]
            },
            map.get_node(0, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('a'),
                antennae_distances: vec![]
            },
            map.get_node(1, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('A'),
                antennae_distances: vec![]
            },
            map.get_node(2, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('0'),
                antennae_distances: vec![]
            },
            map.get_node(3, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('z'),
                antennae_distances: vec![]
            },
            map.get_node(4, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('Z'),
                antennae_distances: vec![]
            },
            map.get_node(5, 0)
        );
        assert_eq!(
            &Node {
                antenna: Some('9'),
                antennae_distances: vec![]
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
        let node = Node {
            antenna: None,
            antennae_distances: vec![Distance::new('a', 1, 1), Distance::new('a', 2, 2)],
        };
        assert!(node.has_antinode());
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
