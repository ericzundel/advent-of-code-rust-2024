//! https://adventofcode.com/2024/day/16
//!
use std::cmp::PartialEq;

advent_of_code::solution!(16);

enum TileType {
    Column,
    Empty,
    End,
    Start,
}

struct Edge<'a> {
    cost: u16,
    min_cost: Option<u64>, // Minimum cost found to end
    tile: &'a Tile<'a>,   
}
struct Tile<'a> {
    tile_type: TileType,
    edges: Vec<Edge<'a>>
}

struct Maze<'a> {
    map: Vec<Vec<Tile<'a>>>,
    start: &'a Tile<'a>,
}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl <'a> Maze<'a> {
    fn find_start(map: &'a Vec<Vec<Tile<'a>>>) -> &'a Tile<'a> {
        for row in map.iter() {
            for tile in row.iter() {
                if tile.tile_type == TileType::Start {
                    return tile
                }
            }
        }
        panic!("Where's the damn start?");
    }
    
    fn load_input(input: &str) -> Vec<Vec<Tile>> {
        let map :  Vec<Vec< crate::Tile>> = Vec::new();
        let chars: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim_end().chars().collect())
            .collect();
        // Sanity check the input data
        let col_count = chars[0].len();
        for row in chars.iter() {
            assert_eq!(col_count, row.len());
        }
        todo!();
        map
    }
    pub fn new(input: &str) -> Maze<'a> {
        let map = Self::load_input(input);
        todo!("scan the input");
        let start = Self::find_start(&map);
        Maze {
            map,
            start,
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
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
