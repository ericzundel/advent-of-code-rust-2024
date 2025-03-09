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

    fn count_vertical_edges(&self, id: u64) -> u64 {
        let mut result: u64 = 0;
        for x in 0..self.plots[0].len() {
            let mut found_west_edge = false;
            let mut found_east_edge = false;
            for y in 0..self.plots.len() {
                let plot = &self.plots[y][x];
                if plot.id != Some(id) {
                    found_west_edge = false;
                    found_east_edge = false;
                    continue;
                }
                if self.is_plant_west(x, y, id) {
                    if found_west_edge {
                        found_west_edge = false;
                    }
                } else {
                    if !found_west_edge {
                        found_west_edge = true;
                        result += 1;
                    }
                }
                if self.is_plant_east(x, y, id) {
                    if found_east_edge {
                        found_east_edge = false;
                    }
                } else {
                    if !found_east_edge {
                        found_east_edge = true;
                        result += 1;
                    }
                }
            }
        }
        result
    }
    fn count_horizontal_edges(&self, id: u64) -> u64 {
        let mut result: u64 = 0;
        for y in 0..self.plots.len() {
            let mut found_north_edge = false;
            let mut found_south_edge = false;

            for x in 0..self.plots[0].len() {
                let plot = &self.plots[y][x];
                if plot.id != Some(id) {
                    found_north_edge = false;
                    found_south_edge = false;
                    continue;
                }
                if self.is_plant_north(x, y, id) {
                    if found_north_edge {
                        found_north_edge = false;
                    }
                } else {
                    if !found_north_edge {
                        found_north_edge = true;
                        result += 1;
                    }
                }
                if self.is_plant_south(x, y, id) {
                    if found_south_edge {
                        found_south_edge = false;
                    }
                } else {
                    if !found_south_edge {
                        found_south_edge = true;
                        result += 1;
                    }
                }
            }
        }
        result
    }
    pub(crate) fn calc_num_sides(&self, id: u64) -> u64 {
        // scan for vertical edges
        let mut result: u64 = self.count_vertical_edges(id);
        // scan for horizontal edges
        result += self.count_horizontal_edges(id);
        result
    }

    /// Used to calculate costs for part 2
    pub(crate) fn calculate_bulk_cost(&self) -> u64 {
        let mut result: u64 = 0;
        for id in 0..self.max_id {
            result += self.calc_area(id) * self.calc_num_sides(id);
        }
        result
    }

    fn is_plant_west(&self, x: usize, y: usize, id: u64) -> bool {
        if x > 0 && self.plots[y][x - 1].id == Some(id) {
            return true;
        }
        false
    }
    fn is_plant_east(&self, x: usize, y: usize, id: u64) -> bool {
        let max_x = self.plots[0].len() - 1;
        if x < max_x && self.plots[y][x + 1].id == Some(id) {
            return true;
        }
        false
    }
    fn is_plant_north(&self, x: usize, y: usize, id: u64) -> bool {
        if y > 0 && self.plots[y - 1][x].id == Some(id) {
            return true;
        }
        false
    }
    fn is_plant_south(&self, x: usize, y: usize, id: u64) -> bool {
        let max_y = self.plots.len() - 1;
        if y < max_y && self.plots[y + 1][x].id == Some(id) {
            return true;
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut garden = Garden::new(input);
    garden.assign_ids();

    // Answer from AOC data is 1477762
    Some(garden.calculate_cost())
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut garden = Garden::new(input);
    garden.assign_ids();
    
    // AOC answer to part 2 is 923480
    Some(garden.calculate_bulk_cost())
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
    fn test_example1_part_two() {
        let input: &str = "AAAA\n\
          BBCD\n\
          BBCC\n\
          EEEC\n";
        let mut garden = Garden::new(input);
        garden.assign_ids();

        assert_eq!(garden.calc_num_sides(0), 4);
        assert_eq!(garden.calc_num_sides(1), 4);
        assert_eq!(garden.calc_num_sides(2), 8);
        assert_eq!(garden.calc_num_sides(3), 4);
        assert_eq!(garden.calc_num_sides(4), 4);
        assert_eq!(garden.calc_num_sides(5), 0);

        assert_eq!(garden.calculate_bulk_cost(), 80);
    }

    #[test]
    fn test_example2_part_two() {
        let input: &str = "EEEEE\n\
        EXXXX\n\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n";
        let mut garden = Garden::new(input);
        garden.assign_ids();
        assert_eq!(garden.is_plant_south(0, 4, 0), false);
        assert_eq!(garden.is_plant_south(4, 4, 0), false);
        assert_eq!(garden.count_horizontal_edges(0), 6);

        assert_eq!(garden.is_plant_west(0, 0, 0), false);
        assert_eq!(garden.is_plant_west(0, 1, 0), false);
        assert_eq!(garden.is_plant_west(0, 2, 0), false);
        assert_eq!(garden.is_plant_west(0, 3, 0), false);
        assert_eq!(garden.is_plant_west(0, 4, 0), false);
        assert_eq!(garden.is_plant_east(0, 0, 0), true);
        assert_eq!(garden.is_plant_east(0, 1, 0), false);
        assert_eq!(garden.is_plant_east(0, 2, 0), true);
        assert_eq!(garden.is_plant_east(0, 3, 0), false);
        assert_eq!(garden.is_plant_east(0, 4, 0), true);

        assert_eq!(garden.is_plant_east(4, 0, 0), false);
        assert_eq!(garden.is_plant_east(4, 1, 0), false);
        assert_eq!(garden.is_plant_east(4, 2, 0), false);
        assert_eq!(garden.is_plant_east(4, 3, 0), false);
        assert_eq!(garden.is_plant_east(4, 4, 0), false);

        assert_eq!(garden.count_vertical_edges(0), 6);

        assert_eq!(garden.calc_num_sides(0), 12);
        assert_eq!(garden.calc_area(0), 17);
        assert_eq!(garden.calc_num_sides(1), 4);
        assert_eq!(garden.calc_area(1), 4);
        assert_eq!(garden.calc_num_sides(2), 4);
        assert_eq!(garden.calc_area(2), 4);

        assert_eq!(garden.calculate_bulk_cost(), 236);
    }

    #[test]
    fn test_example_3_part_two() {
        let input: &str = "AAAAAA\n\
          AAABBA\n\
          AAABBA\n\
          ABBAAA\n\
          ABBAAA\n\
          AAAAAA\n";
        let mut garden = Garden::new(input);
        garden.assign_ids();
        assert_eq!(garden.calculate_bulk_cost(), 368);
    }
    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
