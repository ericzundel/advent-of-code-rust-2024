use std::fmt::{Debug, Display, Formatter};

advent_of_code::solution!(15);

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
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
                        robot_position = Position { x, y: y };
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

    fn get_position(&self, pos: &Position, direction: &Move) -> Position {
        match direction {
            Move::UP => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Move::DOWN => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Move::LEFT => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Move::RIGHT => Position {
                x: pos.x + 1,
                y: pos.y,
            },
        }
    }

    pub(crate) fn try_move_box(&mut self, box_position: &Position, direction: &Move) -> Position {
        let curr_tile = &self.tiles[box_position.y][box_position.x];
        assert_eq!(curr_tile, &Tile::BOX);
        match direction {
            Move::UP => {}
            Move::DOWN => {}
            Move::LEFT => {}
            Move::RIGHT => {}
        }
        todo!()
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
            writeln!(f)?;
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

    pub fn run(&mut self) {
        for mv in self.moves.iter() {
            let robot_position = self.warehouse.robot_position.clone();

            let target_position = self.warehouse.get_position(&robot_position, &mv);
            let tile = &self.warehouse.tiles[target_position.y][target_position.x];
            match tile {
                Tile::WALL => break,
                Tile::EMPTY => {
                    self.warehouse.robot_position = target_position;
                }
                Tile::BOX => {
                    self.warehouse.robot_position =
                        self.warehouse.try_move_box(&target_position, mv)
                }
            }
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

    static SMALL_INPUT: &str = "########\n\
                                #..O.O.#\n\
                                ##@.O..#\n\
                                #...O..#\n\
                                #.#.O..#\n\
                                #...O..#\n\
                                #......#\n\
                                ########\n\
                                \n\
                                <^^>>>vv<v>>v<<\n";

    #[test]
    fn test_small() {
        let mut simulation = Simulation::new(SMALL_INPUT);
        simulation.run();
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
