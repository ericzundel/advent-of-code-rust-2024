advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Garden {
    plots: Vec<Vec<char>>,
}

impl Garden {
    pub fn new(input: &str) -> Self {
        let mut plots = Vec::new();
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            let line = line.trim();
            for c in line.chars() {
                row.push(c);
            }
            plots.push(row);
        }
        Garden { plots }
    }

    fn find_adjacent_fence(&self, curr: Position, value: char) -> u64 {
        if self.plots[curr.y][curr.x] != value {
            return 0;
        }

        let mut result: u64 = 0;
        let max_x = self.plots[0].len() - 1;
        let max_y = self.plots.len() - 1;
        if curr.x > 0 && self.plots[curr.y][curr.x - 1] != value {
            // west
            result += 1;
        }
        if curr.y > 0 && self.plots[curr.y - 1][curr.x] != value {
            // north
            result += 1;
        }
        if curr.y < max_y && self.plots[curr.y + 1][curr.x] != value {
            // south
            result += 1;
        }
        if curr.x < max_x && self.plots[curr.y][curr.x + 1] != value {
            // east
            result += 1;
        }
        if curr.x == 0 {
            result += 1;
        }
        if curr.y == 0 {
            result += 1;
        }
        if curr.x == max_x {
            result += 1;
        }
        if curr.y == max_y {
            result += 1;
        }
        result
    }

    pub fn calc_area(&self, plot_name: char) -> u64 {
        let mut result = 0;
        for y in 0..self.plots.len() {
            for x in 0..self.plots[0].len() {
                if self.plots[y][x] == plot_name {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn calc_perimeter(&self, plot_name: char) -> u64 {
        let mut result = 0;
        for y in 0..self.plots.len() {
            for x in 0..self.plots[0].len() {
                result += self.find_adjacent_fence(Position { x, y }, plot_name);
            }
        }
        result
    }

    pub fn incorrect_calculate_cost(&self) -> u64 {
        let mut result: u64 = 0;
        for plot_name in 'A'..'Z' {
            result += self
                .calc_area(plot_name)
                .checked_mul(self.calc_perimeter(plot_name))
                .unwrap();
        }
        result
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let garden = Garden::new(input);
    Some(garden.incorrect_calculate_cost())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example1() {
        let input: &str = "AAAA\n\
          BBCD\n\
          BBCC\n\
          EEEC\n";
        let garden = Garden::new(input);
        assert_eq!(garden.calc_area('F'), 0);
        assert_eq!(garden.calc_perimeter('F'), 0);
        assert_eq!(garden.calc_area('A'), 4);
        assert_eq!(garden.calc_perimeter('A'), 10);
        assert_eq!(garden.calc_area('B'), 4);
        assert_eq!(garden.calc_perimeter('B'), 8);
        assert_eq!(garden.calc_area('C'), 4);
        assert_eq!(garden.calc_perimeter('C'), 10);
        assert_eq!(garden.calc_area('D'), 1);
        assert_eq!(garden.calc_perimeter('D'), 4);
        assert_eq!(garden.calc_area('E'), 3);
        assert_eq!(garden.calc_perimeter('E'), 8);
        assert_eq!(garden.incorrect_calculate_cost(), 4*10 + 4*8 + 4*10 + 1*4+ 3*8);
        assert_eq!(garden.incorrect_calculate_cost(), 140);
    }

    #[test]
    fn test_example2() {
        let input: &str = "OOOOO\n\
          OXOXO\n\
          OOOOO\n\
          OXOXO\n\
          OOOOO\n";
        let garden = Garden::new(input);
        assert_eq!(garden.calc_area('F'), 0);
        assert_eq!(garden.calc_perimeter('F'), 0);
        assert_eq!(garden.calc_area('X'), 4);
        assert_eq!(garden.calc_perimeter('X'), 16);
        assert_eq!(garden.calc_area('O'), 21);
        assert_eq!(garden.calc_perimeter('O'), 36);
        assert_eq!(garden.incorrect_calculate_cost(), 772);
        
    }
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
