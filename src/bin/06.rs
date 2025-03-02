use std::fmt::{Display, Formatter};

advent_of_code::solution!(6);

#[derive(Clone, Debug)]
struct LabMap {
    tiles: Vec<Vec<Tile>>,
    guard: Option<Guard>,
}

impl LabMap {
    pub(crate) fn add_column(&mut self, pos: &Position) {
        self.tiles[pos.y][pos.x] = Tile::Column;
    }

    pub(crate) fn move_guard(&mut self, new_position: &Position) {
        if self.guard.is_none() {
            return;
        }
        let guard = self.guard.take().unwrap();

        if new_position.x >= self.tiles[0].len() || new_position.y >= self.tiles.len() {
            self.guard = None;
            return;
        }
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
            Some(pos) => {
                if pos.x >= self.tiles[0].len() || pos.y >= self.tiles.len() {
                    return false;
                }
                match self.tiles[pos.y][pos.x] {
                    Tile::Column => true,
                    _ => false,
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Empty { visited: bool, turned: bool },
    Column,
}

impl Tile {
    fn new(character: &char) -> Tile {
        match character {
            '.' => Tile::Empty {
                visited: false,
                turned: false,
            },
            '>' | '<' | '^' | 'v' => Tile::Empty {
                visited: true,
                turned: false,
            },
            '#' => Tile::Column,
            _ => panic!("Unknown char in map {:?}", character),
        }
    }
    fn turn(&mut self) {
        match self {
            Tile::Column => panic!("Can't turn a column!"),
            Tile::Empty { visited: _, turned } => *turned = true,
        }
    }

    fn visit(&mut self) {
        match self {
            Tile::Column => panic!("Can't visit column!"),
            Tile::Empty { visited, turned: _ } => *visited = true,
        }
    }

    fn is_column(&self) -> bool {
        match self {
            Tile::Column => true,
            _ => false,
        }
    }

    fn is_visited(&self) -> bool {
        match self {
            Tile::Column => false,
            Tile::Empty { visited, turned: _ } => *visited,
        }
    }

    fn has_turned(&self) -> bool {
        match self {
            Tile::Column => false,
            Tile::Empty { visited: _, turned } => *turned,
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
        for y in 0..tiles.len() {
            for x in 0..tiles[y].len() {
                if Guard::is_guard_char(tiles[y][x]) {
                    return Some(Guard::new(x, y, tiles[y][x]));
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
        self.tiles[position.y][position.x].visit();
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
        let mut buf: String = String::with_capacity((2 * self.tiles.len()) ^ 2);
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[0].len() {
                let mut character = match self.tiles[row][col] {
                    Tile::Column => '#',
                    Tile::Empty { visited, turned: _ } => {
                        if visited {
                            'X'
                        } else {
                            '.'
                        }
                    }
                };
                if self.get_guard().is_some() {
                    let guard = self.get_guard().unwrap();
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

#[derive(PartialEq, Debug)]
enum SimulationStatus {
    InProgress,
    GuardExited,
    GuardCycle,
}

impl Simulation {
    pub fn new(map: LabMap) -> Self {
        Simulation { map }
    }

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.map.tiles
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.map.tiles[y][x]
    }

    /// returns true when simulation is complete
    fn tick(&mut self) -> SimulationStatus {
        let guard: Option<&Guard> = self.map.get_guard();
        if guard.is_none() {
            return SimulationStatus::GuardExited;
        }
        let guard = guard.unwrap().clone();
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
            return SimulationStatus::GuardExited;
        }

        // Would this new position hit a column? If so, then turn, else move to the new spot
        if self.map.is_column(new_position.clone()) {
            let curr_tile = &mut self.map.tiles[guard.position.y][guard.position.x];
            // Have we already turned here before? If so, we have detected a cycle
            if curr_tile.has_turned() {
                return SimulationStatus::GuardCycle;
            }
            curr_tile.turn();
            self.map.get_guard_mut().unwrap().turn();
        } else {
            self.map.move_guard(&new_position.unwrap());
        }
        SimulationStatus::InProgress
    }

    pub fn run(&mut self) -> SimulationStatus {
        loop {
            let result = self.tick();
            if result != SimulationStatus::InProgress {
                return result;
            }
        }
    }

    pub fn get_visit_count(&self) -> usize {
        self.map.get_visit_count()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lab_map = LabMap::new(input);
    print!("Start:\n{}\n", lab_map);

    assert!(!lab_map.get_guard().is_none());
    let mut simulation = Simulation::new(lab_map);
    simulation.run();
    let visit_count: u64 = simulation.get_visit_count() as u64;
    print!("End:\n{}\n", simulation.map);
    if visit_count > 0 {
        Some(visit_count)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    // Make a read-only version of the original map
    let original_lab_map = &LabMap::new(input);
    let first_run_lap_map = original_lab_map.clone();
    let mut original_simulation = Simulation::new(first_run_lap_map);
    let original_simulation_result = original_simulation.run();
    assert_eq!(original_simulation_result, SimulationStatus::GuardExited);

    let mut count: u64 = 0;
    let tiles = original_simulation.get_tiles();
    for row in 0..tiles.len() {
        for col in 0..tiles[row].len() {
            let tile = original_simulation.get_tile(row, col);
            let curr_position = &Position { x: col, y: row };
            // We can't put a column in the original guard position
            if &original_lab_map.get_guard().unwrap().position == curr_position {
                continue;
            }
            if !tile.is_column() && tile.is_visited() {
                // Try replacing this tile with a column in a new map
                let mut new_lab_map = original_lab_map.clone();
                new_lab_map.add_column(curr_position);
                let mut new_simulation = Simulation::new(new_lab_map);
                if new_simulation.run() == SimulationStatus::GuardCycle {
                    count += 1;
                }
            }
        }
    }
    // The value from the AOC test data is > 783
    Some(count)
}

#[cfg(test)]
mod simulation_tests {
    use super::*;
    #[test]
    fn test_simulation1() {
        let lab_map = LabMap::new(".#.\n.^.\n");
        let mut simulation = Simulation::new(lab_map.clone());
        print!("Original:\n{}\n", simulation.map);
        assert_eq!(
            simulation.map.get_guard_mut().unwrap().direction,
            Direction::Up
        );
        assert_eq!(
            simulation.map.get_guard_mut().unwrap().position,
            Position { x: 1, y: 1 }
        );
        assert_eq!(simulation.map.tiles[1][1].has_turned(), false);

        simulation.tick();
        print!("Tick1:\n{}\n", simulation.map);
        assert_eq!(
            simulation.map.get_guard_mut().unwrap().direction,
            Direction::Right
        );
        assert_eq!(
            simulation.map.get_guard_mut().unwrap().position,
            Position { x: 1, y: 1 }
        );
        assert_eq!(simulation.map.tiles[1][1].is_visited(), true);
        assert_eq!(simulation.map.tiles[1][1].has_turned(), true);
        assert_eq!(simulation.map.tiles[1][2].is_visited(), false);

        simulation.tick();
        print!("Tick 2:\n{}\n", simulation.map);
        assert_eq!(
            simulation.map.get_guard_mut().unwrap().direction,
            Direction::Right
        );
        assert_eq!(
            simulation.map.get_guard_mut().unwrap().position,
            Position { x: 2, y: 1 }
        );
        assert_eq!(simulation.map.tiles[1][1].is_visited(), true);
        assert_eq!(simulation.map.tiles[1][2].is_visited(), true);

        simulation.tick();
        print!("Tick 3:\n{}\n", simulation.map);
        assert!(simulation.map.get_guard().is_none());
    }

    #[test]
    fn test_detect_cycle() {
        // This pattern of columns will cause the guard to cycle infinitely
        let map_cycle_data = vec![".#....", ".^...#", "#.....", "....#."];
        let lab_map = LabMap::new(map_cycle_data.join("\n").as_str());
        let mut simulation = Simulation::new(lab_map);
        let simulation_status = simulation.run();
        print!("{}", simulation.map);
        assert_eq!(simulation_status, SimulationStatus::GuardCycle);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map() {
        let lab_map = LabMap::new(".#.\n.^.\n");
        let ex_row1: Vec<Tile> = vec![
            Tile::Empty {
                visited: false,
                turned: false,
            },
            Tile::Column,
            Tile::Empty {
                visited: false,
                turned: false,
            },
        ];
        let ex_row2: Vec<Tile> = vec![
            Tile::Empty {
                visited: false,
                turned: false,
            },
            Tile::Empty {
                visited: true,
                turned: false,
            },
            Tile::Empty {
                visited: false,
                turned: false,
            },
        ];
        assert_eq!(vec![ex_row1, ex_row2], lab_map.tiles);
        let expected_guard = Guard {
            position: Position { x: 1, y: 1 },
            direction: Direction::Up,
        };
        assert_eq!(lab_map.get_guard().unwrap(), &expected_guard);
        assert_eq!(1, lab_map.get_visit_count());
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
    fn test_add_column() {
        let mut lab_map = LabMap::new(".#.\n.^.\n");
        let position = &Position { x: 2, y: 0 };
        assert!(!lab_map.is_column(Some(position.clone())));
        lab_map.add_column(position);
        assert!(lab_map.is_column(Some(position.clone())));
    }
    #[test]
    fn test_lab_map_print() {
        let lab_map = LabMap::new(".#.\n.^.\n");
        assert_eq!(".#.\n.^.\n", format!("{}", lab_map));
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
