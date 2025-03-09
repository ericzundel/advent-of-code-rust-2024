advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Plot {
    plant: char,
    id: Option<u64>,
}

struct Garden {
    plots: Vec<Vec<Plot>>,
    max_id: u64,
}

impl Garden {
    pub fn new(input: &str) -> Self {
        let mut plots: Vec<Vec<Plot>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<Plot> = Vec::new();
            let line = line.trim();
            for c in line.chars() {
                row.push(Plot { plant: c, id: None });
            }
            plots.push(row);
        }
        Garden { plots, max_id: 0 }
    }

    fn find_adjacent_fence(&self, curr: Position, id: u64) -> u64 {
        if self.plots[curr.y][curr.x].id != Some(id) {
            return 0;
        }

        let mut result: u64 = 0;
        let max_x = self.plots[0].len() - 1;
        let max_y = self.plots.len() - 1;
        if curr.x > 0 && self.plots[curr.y][curr.x - 1].id != Some(id) {
            // west
            result += 1;
        }
        if curr.y > 0 && self.plots[curr.y - 1][curr.x].id != Some(id) {
            // north
            result += 1;
        }
        if curr.y < max_y && self.plots[curr.y + 1][curr.x].id != Some(id) {
            // south
            result += 1;
        }
        if curr.x < max_x && self.plots[curr.y][curr.x + 1].id != Some(id) {
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

    pub fn calc_area(&self, id: u64) -> u64 {
        let mut result = 0;
        for y in 0..self.plots.len() {
            for x in 0..self.plots[0].len() {
                if self.plots[y][x].id == Some(id) {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn calc_perimeter(&self, id: u64) -> u64 {
        let mut result = 0;
        for y in 0..self.plots.len() {
            for x in 0..self.plots[0].len() {
                result += self.find_adjacent_fence(Position { x, y }, id);
            }
        }
        result
    }

    pub fn calculate_cost(&self) -> u64 {
        let mut result: u64 = 0;
        for id in 0..self.max_id {
            result += self
                .calc_area(id)
                .checked_mul(self.calc_perimeter(id))
                .unwrap();
        }
        result
    }
    // Scan the plots. Adjacent plots with the same plant type get the same id
    pub fn assign_ids(&mut self) {
        let mut next_id = 0;
        for y in 0..self.plots.len() {
            for x in 0..self.plots[0].len() {
                if self.plots[y][x].id.is_none() {
                    self.mark_plots(Position { x, y }, next_id);
                    next_id += 1;
                }
            }
        }
        self.max_id = next_id;
    }

    fn mark_plots(&mut self, curr: Position, id: u64) {
        self.plots[curr.y][curr.x].id = Some(id);
        for adj in self.adjacent_plots(curr) {
            self.mark_plots(adj, id);
        }
    }

    fn adjacent_plots(&mut self, curr: Position) -> Vec<Position> {
        let value = self.plots[curr.y][curr.x].plant;
        let mut result: Vec<Position> = Vec::new();
        let max_x = self.plots[0].len() - 1;
        let max_y = self.plots.len() - 1;
        let plots = &self.plots;
        if curr.x > 0 {
            let plot = &plots[curr.y][curr.x - 1];
            if plot.id.is_none() && plot.plant == value {
                // west
                result.push(Position {
                    x: curr.x - 1,
                    y: curr.y,
                });
            }
        }
        if curr.y > 0 {
            let plot = &plots[curr.y - 1][curr.x];
            if plot.id.is_none() && plot.plant == value {
                // north
                result.push(Position {
                    x: curr.x,
                    y: curr.y - 1,
                });
            }
        }
        if curr.y < max_y {
            let plot = &plots[curr.y + 1][curr.x];
            if plot.id.is_none() && plot.plant == value {
                // south
                result.push(Position {
                    x: curr.x,
                    y: curr.y + 1,
                });
            }
        }
        if curr.x < max_x {
            let plot: &Plot = &plots[curr.y][curr.x + 1];
            if plot.id.is_none() && plot.plant == value {
                // east
                result.push(Position {
                    x: curr.x + 1,
                    y: curr.y,
                });
            }
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut garden = Garden::new(input);
    garden.assign_ids();
    
    // Answer from AOC data is 1477762
    Some(garden.calculate_cost())
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
        let mut garden = Garden::new(input);
        garden.assign_ids();
        assert_eq!(garden.calc_area(5), 0);
        assert_eq!(garden.calc_perimeter(5), 0);
        assert_eq!(garden.calc_area(0), 4);
        assert_eq!(garden.calc_perimeter(0), 10);
        assert_eq!(garden.calc_area(1), 4);
        assert_eq!(garden.calc_perimeter(1), 8);
        assert_eq!(garden.calc_area(2), 4);
        assert_eq!(garden.calc_perimeter(2), 10);
        assert_eq!(garden.calc_area(3), 1);
        assert_eq!(garden.calc_perimeter(3), 4);
        assert_eq!(garden.calc_area(4), 3);
        assert_eq!(garden.calc_perimeter(4), 8);
        assert_eq!(
            garden.calculate_cost(),
            4 * 10 + 4 * 8 + 4 * 10 + 1 * 4 + 3 * 8
        );
        assert_eq!(garden.calculate_cost(), 140);
    }

    #[test]
    fn test_example2() {
        let input: &str = "OOOOO\n\
          OXOXO\n\
          OOOOO\n\
          OXOXO\n\
          OOOOO\n";
        let mut garden = Garden::new(input);
        garden.assign_ids();

        assert_eq!(garden.calc_area(6), 0);
        assert_eq!(garden.calc_perimeter(6), 0);
        
        assert_eq!(garden.calc_area(0), 21);
        assert_eq!(garden.calc_perimeter(0), 36);

        for id in 1..5 {
            assert_eq!(garden.calc_perimeter(id), 4);
            assert_eq!(garden.calc_area(id), 1);
        }
        assert_eq!(garden.calculate_cost(), 772);
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
