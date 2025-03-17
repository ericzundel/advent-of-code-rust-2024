//!
//! Advent of code 2024 Day 14
//! https://adventofcode.com/2024/day/14
//!
//!

use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Vector {
    delta_x: i32,
    delta_y: i32,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Robot {
    initial_position: Position,
    velocity: Vector,
}

#[derive(Debug, Clone)]

enum Quadrant {
    NORTHWEST,
    NORTHEAST,
    SOUTHWEST,
    SOUTHEAST,
}

#[derive(Debug, Clone)]
struct Floor {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

struct Simulation {
    floor: Floor,
    robot_positions: HashMap<Robot, Position>,
}

impl Floor {
    pub fn new(input: &str, width: usize, height: usize) -> Floor {
        let mut robots: Vec<Robot> = Vec::new();
        for line in input.lines() {
            let re = Regex::new(r"p=(\d+),(\d+)\s+v=(\-?\d+),(\-?\d+)").unwrap();
            if let Some(captures) = re.captures(line.trim()) {
                let initial_position = Position {
                    x: captures[1].parse().unwrap(),
                    y: captures[2].parse().unwrap(),
                };
                let velocity = Vector {
                    delta_x: captures[3].parse().unwrap(),
                    delta_y: captures[4].parse().unwrap(),
                };
                robots.push(Robot {
                    initial_position,
                    velocity,
                });
            } else {
                panic!("Parse error. Expected p,v format, got {}", line);
            }
        }
        Floor {
            robots,
            width,
            height,
        }
    }
}

impl Simulation {
    pub fn new(floor: Floor) -> Simulation {
        let mut robot_positions: HashMap<Robot, Position> = HashMap::new();
        for robot in floor.robots.iter() {
            robot_positions.insert(robot.clone(), robot.initial_position.clone());
        }
        Simulation {
            floor,
            robot_positions,
        }
    }

    pub fn run(&mut self, seconds: i32) {
        for (robot, position) in &mut self.robot_positions {
            let x =
                (position.x as i32 + seconds * robot.velocity.delta_x) % self.floor.width as i32;
            let y =
                (position.y as i32 + seconds * robot.velocity.delta_y) % self.floor.height as i32;
            if x < 0 {
                position.x = (x + self.floor.width as i32) as usize;
            } else {
                position.x = x as usize;
            }
            if y < 0 {
                position.y = (y + self.floor.height as i32) as usize;
            } else {
                position.y = y as usize;
            }
        }
    }

    pub fn get_robots(
        &self,
        northwest_corner: Position,
        southeast_corner: Position,
    ) -> HashMap<Robot, Position> {
        let mut result: HashMap<Robot, Position> = HashMap::new();
        for (robot, position) in self.robot_positions.iter() {
            if position.x >= northwest_corner.x
                && position.x < southeast_corner.x
                && position.y >= northwest_corner.y
                && position.y < southeast_corner.y
            {
                result.insert(robot.clone(), position.clone());
            }
        }
        result
    }

    pub fn get_quadrant_robots(&self, quadrant: Quadrant) -> HashMap<Robot, Position> {
        match quadrant {
            Quadrant::NORTHWEST => self.get_robots(
                Position { x: 0, y: 0 },
                Position {
                    x: self.floor.width / 2,
                    y: self.floor.height / 2,
                },
            ),
            Quadrant::NORTHEAST => self.get_robots(
                Position {
                    x: self.floor.width / 2 + 1,
                    y: 0,
                },
                Position {
                    x: self.floor.width,
                    y: self.floor.height / 2,
                },
            ),
            Quadrant::SOUTHEAST => self.get_robots(
                Position {
                    x: self.floor.width / 2 + 1,
                    y: self.floor.height / 2 + 1,
                },
                Position {
                    x: self.floor.width,
                    y: self.floor.height,
                },
            ),
            Quadrant::SOUTHWEST => self.get_robots(
                Position {
                    x: 0,
                    y: self.floor.height / 2 + 1,
                },
                Position {
                    x: self.floor.width / 2,
                    y: self.floor.height,
                },
            ),
        }
    }

    fn compute_safety_factor(&self) -> u64 {
        let ne = self.get_quadrant_robots(Quadrant::NORTHEAST);
        let nw = self.get_quadrant_robots(Quadrant::NORTHWEST);
        let sw = self.get_quadrant_robots(Quadrant::SOUTHWEST);
        let se = self.get_quadrant_robots(Quadrant::SOUTHEAST);
        (ne.len() * nw.len() * sw.len() * se.len()) as u64
    }

    fn get_robot_tiles(&self) -> Vec<Vec<i32>> {
        let mut map: Vec<Vec<i32>> = Vec::new();
        for _i in 0..self.floor.height {
            let mut row: Vec<i32> = Vec::new();
            for _ in 0..self.floor.width {
                row.push(0);
            }
            map.push(row);
        }
        for position in self.robot_positions.values() {
            map[position.y][position.x] += 1;
        }
        map
    }
    fn print_robots(&self) {
        let map = self.get_robot_tiles();
        for y in 0..self.floor.height {
            for x in 0..self.floor.width {
                let count = map[y][x];
                if count > 0 {
                    print!("{:<2}", count);
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    pub fn check_christmas_tree_5(&self) -> bool {
        let map = self.get_robot_tiles();
        for y in 13..self.floor.height {
            let mut result: i32 = 0;
            let mut found = false;
            for x in 0..self.floor.width {
                if map[y][x] == 1 {
                    found = true;
                }
                if found {
                    if map[y][x] == 1 {
                        result += map[y][x];
                        if result >= 13 {
                            return true;
                        }
                    } else {
                        break; // go to the next line
                    }
                }
            }git 
        }
        false
    }
}

pub fn do_part_one(input: &str, width: usize, height: usize) -> Option<u64> {
    let floor = Floor::new(input, width, height);
    let mut simulation = Simulation::new(floor);
    simulation.run(100);
    simulation.print_robots();
    Some(simulation.compute_safety_factor())
}

pub fn part_one(input: &str) -> Option<u64> {
    // Answer with AOC data is 221655456
    return do_part_one(input, 101, 103);
}

pub fn part_two(input: &str) -> Option<u64> {
    let floor = Floor::new(input, 101, 103);
    let mut simulation = Simulation::new(floor);
    let mut count = 0;
    loop {
        count += 1;
        simulation.run(1);
        if simulation.check_christmas_tree_5() {
            simulation.print_robots();
            println!("{} seconds elapsed.", count);
            return Some(count)
        }
        if count % 10000 == 0 {
            println!("{}", count);
        }
        if count == 1000000 {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = do_part_one(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
