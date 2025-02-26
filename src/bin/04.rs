advent_of_code::solution!(4);

fn load_data(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim_end().chars().collect())
        .collect()
}

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
    for row in input.clone() {
        assert_eq!(col_count, row.len());
    }
    let row_count = input.len();

    let mut result: Vec<Vec<char>> = vec![vec![' '; row_count]; col_count];
    for i in 0..row_count {
        for j in 0..col_count {
            result[j][i] = input[i][j];
        }
    }
    result
}

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
            matrix[i][i+j] = input[i][j];
        }
    }
    print!("diag1 matrix is {:?}\n", matrix);

    change_rows_to_columns(matrix)
}

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
            matrix[i][j+(orig_column_count-i)] = input[i][j];
        }
    }
    print!("diag2 matrix is {:?}\n", matrix);

    change_rows_to_columns(matrix)
}

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

pub fn part_two(input: &str) -> Option<u64> {
    load_data(input);
    None
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
