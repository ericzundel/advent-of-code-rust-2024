//! https://adventofcode.com/2024/day/4
//! part1 find the string XMAS horizontally, vertically, diagonally both forward and backward
//! part 2 find M A S x M A S in 3x3 grid
advent_of_code::solution!(4);

/// Load the input data into a 2D array of characters
/// Sanity check the data to make sure each row has the same # of columns
fn load_data(input: &str) -> Vec<Vec<char>> {
    let result : Vec<Vec<char>> = input
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

/// Transforms the vector into a string forward and backward and checks it
fn count_xmas_in_vec(input: Vec<char>) -> u64 {
    // first forward
    let forward = input.iter().collect::<String>();
    let reverse = input.iter().rev().collect::<String>();
    count_xmas_in_str(forward.as_str()) + count_xmas_in_str(reverse.as_str())
}

/// Count the number of times the string XMAS appears in a string
fn count_xmas_in_str(line: &str) -> u64 {
    let mut start = 0;
    let mut count = 0u64;
    loop {
        let tail = &line[start..];
        let pos = tail.find("XMAS");
        if pos.is_none() {
            break;
        }
        count += 1;
        start += pos.unwrap() + 4;
        assert!(count < line.len() as u64);
    }
    count
}

/// Rotates the matrix 90 degrees.
fn change_rows_to_columns(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Sanity check the input data
    let col_count = input[0].len();
    let row_count = input.len();

    let mut result: Vec<Vec<char>> = vec![vec![' '; row_count]; col_count];
    for i in 0..row_count {
        for j in 0..col_count {
            result[j][i] = input[i][j];
        }
    }
    result
}

/// Transforms the puzzle so that the diagonals become horizontal rows so we 
/// can use count_xmas_in_vec()
fn change_diag1_to_rows(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Initialize a matrix that is twice as wide as the original
    let row_count = input.len();
    let orig_column_count = input[0].len();
    let column_count = orig_column_count * 2;
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; column_count]; row_count];

    // Copy the data from row0 going straight across
    // Then copy data from row1 but shift over 1 position
    // Then row 2 is shifted 2 positions... and so on.
    //   1  2  3        1  2  3  .  .
    //   4  5  6  -->   .  4  5  6  .
    //   7  8  9        .  .  7  8  9
    // Scanning the columns will be the same as evaluating the diagonals from bottom left to upper right
    for i in 0..row_count {
        for j in 0..orig_column_count {
            matrix[i][i + j] = input[i][j];
        }
    }
    //print!("diag1 matrix is {:?}\n", matrix);

    change_rows_to_columns(matrix)
}

/// Transforms the puzzle so that the diagonals in the other direction become horizontal rows so we 
/// can use count_xmas_in_vec()
fn change_diag2_to_rows(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Initialize a matrix that is twice as wide as the original
    let row_count = input.len();
    let orig_column_count = input[0].len();

    let column_count = orig_column_count * 2;
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; column_count]; row_count];

    // Copy the data from row0 going straight across
    // Then copy data from row1 but shift over 1 position
    // Then row 2 is shifted 2 positions... and so on.
    //   1  2  3        1  2  3  .  .
    //   4  5  6  -->   .  4  5  6  .
    //   7  8  9        .  .  7  8  9
    // Scanning the columns will be the same as evaluating the diagonals from bottom left to upper right
    for i in 0..row_count {
        for j in 0..orig_column_count {
            matrix[i][j + (orig_column_count - i)] = input[i][j];
        }
    }
    //print!("diag2 matrix is {:?}\n", matrix);

    change_rows_to_columns(matrix)
}

/// Count instances of "XMAS" in the entire puzzle
fn count_xmas_in_puzzle(input: Vec<Vec<char>>) -> u64 {
    let mut count = 0u64;

    // Count horizontal matches
    count += input
        .iter()
        .map(|row| count_xmas_in_vec(row.to_vec()))
        .sum::<u64>();

    // Count vertical matches
    count += change_rows_to_columns(input.clone())
        .iter()
        .map(|row| count_xmas_in_vec(row.to_vec()))
        .sum::<u64>();

    // Count bottom left to upper right diagonal matches
    count += change_diag1_to_rows(input.clone())
        .iter()
        .map(|row| count_xmas_in_vec(row.to_vec()))
        .sum::<u64>();

    // Count top left to bottom right diagonal matches
    count += change_diag2_to_rows(input)
        .iter()
        .map(|row| count_xmas_in_vec(row.to_vec()))
        .sum::<u64>();
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(count_xmas_in_puzzle(load_data(input)))
}

/// Count number of MAS crossed in sub 3xx matrixes in the puzzle
fn count_x_mas_in_puzzle(input: Vec<Vec<char>>) -> u64 {
    let row_count = input.len();
    let col_count = input[0].len();
    let mut count = 0;
    for i in 0..row_count - 2 {
        for j in 0..col_count - 2 {
            // Isolate a 3x3 sub-matrix starting at this point
            let eval_x_mas: Vec<Vec<char>> = vec![
                vec![input[i][j], input[i + 1][j], input[i + 2][j]],
                vec![input[i][j + 1], input[i + 1][j + 1], input[i + 2][j + 1]],
                vec![input[i][j + 2], input[i + 1][j + 2], input[i + 2][j + 2]],
            ];
            if is_x_mas(eval_x_mas) {
                count += 1;
            }
        }
    }
    count
}

/// Look for 'MAS' crossed in a 3x3 matrix
///   M . M      M . S     S . M     S . S
///   . A .  or  . A . or  . A . or  . A .
///   S . S      M . S     S . M     M . M
fn is_x_mas(eval_x_mas: Vec<Vec<char>>) -> bool {
    assert_eq!(eval_x_mas.len(), 3);
    assert_eq!(eval_x_mas[0].len(), 3);
    if eval_x_mas[1][1] != 'A' {
        return false;
    }
    // Check diagonal 1
    if eval_x_mas[0][0] == 'M' && eval_x_mas[2][2] == 'S'
        || eval_x_mas[0][0] == 'S' && eval_x_mas[2][2] == 'M'
    {
        // check diagonal 2
        if eval_x_mas[0][2] == 'M' && eval_x_mas[2][0] == 'S'
            || eval_x_mas[0][2] == 'S' && eval_x_mas[2][0] == 'M'
        {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(count_x_mas_in_puzzle(load_data(input)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xmas_in_vec() {
        let input = vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'M', 'A', 'S'];
        assert_eq!(3, count_xmas_in_vec(input));
    }
    #[test]
    fn test_rows_to_columns() {
        let input = vec![vec!['1', '2', '3'], vec!['4', '5', '6']];
        let expected = vec![vec!['1', '4'], vec!['2', '5'], vec!['3', '6']];
        assert_eq!(expected, change_rows_to_columns(input));
    }
    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas_in_str("XMAS"), 1);
        assert_eq!(count_xmas_in_str("XMASXMAS"), 2);
        assert_eq!(count_xmas_in_str("XMASXMASXMA"), 2);
        assert_eq!(count_xmas_in_str("ABCXMASSSX"), 1);
        assert_eq!(count_xmas_in_str("123456789"), 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // Answer to full input is 2578
        assert_eq!(result, Some(18));
    }
    
    #[test]
    fn test_is_x_mas() {
        let eval_x_mas: Vec<Vec<char>> = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];
        assert_eq!(true, is_x_mas(eval_x_mas));
        let eval_x_mas: Vec<Vec<char>> = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];
        assert_eq!(true, is_x_mas(eval_x_mas));
        let eval_x_mas: Vec<Vec<char>> = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];
        assert_eq!(true, is_x_mas(eval_x_mas));
        let eval_x_mas: Vec<Vec<char>> = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];
        assert_eq!(true, is_x_mas(eval_x_mas));
        
        // Negative Cases
        let eval_x_mas: Vec<Vec<char>> = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', '.'],
        ];
        assert_eq!(false, is_x_mas(eval_x_mas));
        let eval_x_mas: Vec<Vec<char>> = vec![
            vec!['S', '.', 'S'],
            vec!['.', '.', '.'],
            vec!['M', '.', 'M'],
        ];
        assert_eq!(false, is_x_mas(eval_x_mas));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Answer to full puzzle is 1972
        assert_eq!(result, Some(9));
    }
}
