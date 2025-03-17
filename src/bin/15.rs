use std::fmt::{Debug, Display, Formatter};

advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Tile {
    WALL,
    EMPTY,
    BOX,
}

#[derive(Debug)]
struct Warehouse {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    robot_position: Position,
}

impl Warehouse {
    pub fn new(input: &str) -> Warehouse {
        let mut tiles = Vec::new();
        let mut robot_position = Position { x: 0, y: 0 };
        let mut y = 0;
        let mut x = 0;
        for line in input.lines() {
            let mut row: Vec<Tile> = Vec::new();
            x = 0;
            for character in line.chars() {
                match character {
                    '#' => row.push(Tile::WALL),
                    'O' => row.push(Tile::BOX),
                    '.' => row.push(Tile::EMPTY),
                    '@' => {
                        row.push(Tile::EMPTY);
                        robot_position = Position { x: x, y: y };
                    }
                    _ => {}
                }
                x += 1;
            }
            tiles.push(row);
            y += 1;
        }
        Warehouse {
            width: x,
            height: y,
            tiles,
            robot_position,
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.robot_position.x == x && self.robot_position.y == y {
                    write!(f, "@")?;
                } else {
                    match self.tiles[y][x] {
                        Tile::WALL => write!(f, "#")?,
                        Tile::EMPTY => write!(f, ".")?,
                        Tile::BOX => write!(f, "O")?,
                    }
                }
            }
            writeln!(f);
        }
        Ok(())
    }
}
struct Simulation {
    warehouse: Warehouse,
    moves: Vec<Move>,
}

enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Simulation {
    fn new(input: &str) -> Self {
        let split_input: Vec<&str> = input.split_terminator("\n\n").collect();
        let warehouse_input: &str = split_input[0];
        let moves_input: &str = split_input[1];
        let mut moves = Vec::new();
        for character in moves_input.chars() {
            match character {
                '^' => moves.push(Move::UP),
                '>' => moves.push(Move::RIGHT),
                'v' => moves.push(Move::DOWN),
                '<' => moves.push(Move::LEFT),
                '\n' => continue,
                _ => panic!("Unexpected character: {}", character),
            }
        }
        Simulation {
            warehouse: Warehouse::new(warehouse_input),
            moves,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let simulation = Simulation::new(input);
    println!("Warehouse is:\n{}", simulation.warehouse);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
