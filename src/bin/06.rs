use std::fmt::{Display, Formatter, Write};

advent_of_code::solution!(6);

#[derive(Clone, Debug)]
struct LabMap {
    tiles: Vec<Vec<Tile>>,
    guard: Option<Guard>,
}

impl LabMap {
    pub(crate) fn move_guard(&mut self, new_position: &Position) {
        if self.guard.is_none() {
            return;
        }
        let guard = self.guard.take().unwrap();

        // This is sloppy. I'm not sure if the tile has been visited yet
        // when the map is first created.
        self.visit(&guard.position);
        self.visit(&new_position);

        self.guard = Some(Guard::new(
            new_position.x,
            new_position.y,
            guard.direction.to_char(),
        ));
    }

    pub(crate) fn is_column(&self, position: Option<Position>) -> bool {
        match position {
            None => false,
            Some(pos) => match self.tiles[pos.y][pos.x] {
                Tile::Column => true,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Empty { visited: bool },
    Column,
}

impl Tile {
    fn new(character: &char) -> Tile {
        match character {
            '.' | '>' | '<' | '^' | 'v' => Tile::Empty { visited: false },
            '#' => Tile::Column,
            _ => panic!("Unknown char in map {:?}", character),
        }
    }
    fn visit(&mut self) {
        match self {
            Tile::Column => panic!("Can't visit column!"),
            Tile::Empty { visited } => *visited = true,
        }
    }

    fn is_visited(&self) -> bool {
        match self {
            Tile::Column => false,
            Tile::Empty { visited } => *visited,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Down => 'v',
        }
    }

    fn from_char(character: &char) -> Direction {
        match character {
            '^' => Direction::Up,
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("Not a guard character! Check with is_guard_char() first!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn new(x: usize, y: usize, character: char) -> Guard {
        let direction = Direction::from_char(&character);
        Guard {
            position: Position { x, y },
            direction,
        }
    }
    fn is_guard_char(character: char) -> bool {
        match character {
            '^' | '>' | '<' | 'v' => true,
            _ => false,
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}
impl LabMap {
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

    fn get_guard(&self) -> Option<&Guard> {
        self.guard.as_ref()
    }

    fn get_guard_mut(&mut self) -> Option<&mut Guard> {
        self.guard.as_mut()
    }

    /// Returns the Guard structure if the character is found.
    /// Assumes there is at most one guard one the map.
    fn find_guard(tiles: &Vec<Vec<char>>) -> Option<Guard> {
        for x in 0..tiles.len() {
            for y in 0..tiles[x].len() {
                if Guard::is_guard_char(tiles[x][y]) {
                    return Some(Guard::new(x, y, tiles[x][y]));
                }
            }
        }
        None
    }

    fn make_tiles(input: Vec<Vec<char>>) -> Vec<Vec<Tile>> {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for input_row in input {
            let mut row: Vec<Tile> = Vec::new();
            for input_column in input_row.iter() {
                row.push(Tile::new(&input_column));
            }
            tiles.push(row);
        }
        tiles
    }
    fn new(input: &str) -> Self {
        let tile_data = Self::load_map_data(input);
        let guard = Self::find_guard(&tile_data);
        LabMap {
            tiles: Self::make_tiles(tile_data),
            guard,
        }
    }
    fn visit(&mut self, position: &Position) {
        self.tiles[position.x][position.y].visit();
    }

    fn get_visit_count(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|col| col.is_visited()).count())
            .sum()
    }
}

impl Display for LabMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf: String = String::new();
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[0].len() {
                let mut character = match self.tiles[row][col] {
                    Tile::Column => '#',
                    Tile::Empty { visited } => {
                        if visited {
                            'X'
                        } else {
                            '.'
                        }
                    }
                };
                if self.clone().get_guard().is_some() {
                    let guard = self.clone().guard.unwrap();
                    if row == guard.position.y && col == guard.position.x {
                        character = guard.direction.to_char();
                    }
                }
                buf.push(character);
            }
            buf.push('\n');
        }
        f.write_str(buf.as_str())
    }
}

struct Simulation {
    map: LabMap,
}

impl Simulation {
    pub fn new(map: LabMap) -> Self {
        Simulation { map }
    }

    /// returns true when simulation is complete
    fn tick(&mut self) -> bool {
        let mut binding = self.map.clone();
        let guard: Option<&mut Guard> = binding.get_guard_mut();
        if guard.is_none() {
            return true;
        }
        let guard: &mut Guard = guard.unwrap();
        let direction = &guard.direction;
        let max_col = self.map.tiles[0].len();
        let max_row = self.map.tiles.len();

        let new_position: Option<Position> = match direction {
            Direction::Down => {
                if guard.position.y >= max_row {
                    None
                } else {
                    Some(Position {
                        x: guard.position.x,
                        y: guard.position.y + 1,
                    })
                }
            }
            Direction::Up => {
                if guard.position.y == 0 {
                    None
                } else {
                    Some(Position {
                        x: guard.position.x,
                        y: guard.position.y - 1,
                    })
                }
            }
            Direction::Right => {
                if guard.position.x >= max_col {
                    None
                } else {
                    Some(Position {
                        x: guard.position.x + 1,
                        y: guard.position.y,
                    })
                }
            }
            Direction::Left => {
                if guard.position.x == 0 {
                    None
                } else {
                    Some(Position {
                        x: guard.position.x - 1,
                        y: guard.position.y,
                    })
                }
            }
        };

        if new_position.is_none() {
            self.map.guard = None;
            return true;
        }

        // Would this new position hit a column? If so, then turn, else move to the new spot
        if self.map.is_column(new_position.clone()) {
            guard.turn();
        } else {
            self.map.move_guard(&new_position.unwrap());
        }
        false
    }

    pub fn run(&mut self) {
        while !self.tick() {}
    }

    pub fn get_visit_count(&self) -> usize {
        self.map.get_visit_count()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lab_map = LabMap::new(input);
    assert!(!lab_map.get_guard().is_none());
    let mut simulation = Simulation::new(lab_map);
    simulation.run();
    let visit_count: u64 = simulation.get_visit_count() as u64;
    if visit_count > 0 {
        Some(visit_count)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let _lab_map = LabMap::load_map_data(input);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map() {
        let lab_map = LabMap::new(".#.\n.^.\n");
        let ex_row1: Vec<Tile> = vec![
            Tile::Empty { visited: false },
            Tile::Column,
            Tile::Empty { visited: false },
        ];
        let ex_row2: Vec<Tile> = vec![
            Tile::Empty { visited: false },
            Tile::Empty { visited: false },
            Tile::Empty { visited: false },
        ];
        assert_eq!(vec![ex_row1, ex_row2], lab_map.tiles);
        let expected_guard = Guard {
            position: Position { x: 1, y: 1 },
            direction: Direction::Up,
        };
        assert_eq!(lab_map.get_guard().unwrap(), &expected_guard);
        assert_eq!(0, lab_map.get_visit_count());
    }

    #[test]
    fn test_is_column() {
        let lab_map = LabMap::new(".#.\n.^.\n");
        assert_eq!(lab_map.is_column(Some(Position { x: 0, y: 0 })), false);
        assert_eq!(lab_map.is_column(Some(Position { x: 1, y: 0 })), true);
        assert_eq!(lab_map.is_column(Some(Position { x: 2, y: 0 })), false);
        assert_eq!(lab_map.is_column(Some(Position { x: 0, y: 1 })), false);
        assert_eq!(lab_map.is_column(Some(Position { x: 1, y: 1 })), false);
        assert_eq!(lab_map.is_column(Some(Position { x: 2, y: 1 })), false);
    }
    #[test]
    fn test_lab_map_print() {
        let lab_map = LabMap::new(".#.\n.^.\n");
        print!("{}\n", lab_map);
    }
    #[test]
    fn test_turn() {
        let mut guard = Guard::new(1, 1, '^');
        assert_eq!(Direction::Up, guard.direction);
        guard.turn();
        assert_eq!(Direction::Right, guard.direction);
        guard.turn();
        assert_eq!(Direction::Down, guard.direction);
        guard.turn();
        assert_eq!(Direction::Left, guard.direction);
        guard.turn();
        assert_eq!(Direction::Up, guard.direction);
        assert_eq!(Position { x: 1, y: 1 }, guard.position);
    }

    #[test]
    fn test_move_guard() {
        let mut lab_map = LabMap::new("...\n.^.\n");
        lab_map.move_guard(&Position { x: 1, y: 0 });
        assert_eq!(
            lab_map.get_guard().unwrap().position,
            Position { x: 1, y: 0 }
        );
    }

    #[test]
    fn test_simulation1() {
        let lab_map = LabMap::new("...\n.^.\n");
        let mut simulation = Simulation::new(lab_map.clone());
        print!("Original:\n{}\n", simulation.map);
        simulation.tick();
        print!("Tick1:\n{}\n", simulation.map);
        simulation.tick();
        print!("Tick 2:\n{}\n", simulation.map);
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
