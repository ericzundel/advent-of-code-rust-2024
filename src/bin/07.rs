use std::io;
use std::io::Write;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct Equation {
    computed: u64,
    operands: Vec<u64>,
    operators: Vec<Operator>,
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Plus,
    Multiply,
}
impl Equation {
    /// Recursive function to build a list of operands and test them
    pub fn try_operators(&mut self, try_operators: &Vec<Operator>) -> bool {
        // Base case - We have a list of operators long enough to try the computation.
        if try_operators.len() == self.operands.len() - 1 {
            let mut computed = self.operands[0];
            for index in 1..self.operands.len() {
                match try_operators[index - 1] {
                    Operator::Multiply => computed = computed.checked_mul(self.operands[index]).unwrap(),
                    Operator::Plus => computed = computed.checked_add(self.operands[index]).unwrap(),
                }
                
            }
            if self.computed == computed {
                self.operators.extend(try_operators.iter().cloned());
                return true;
            }
            return false;
        }

        // We need at least one more operator in try_operators to attempt the computation.
        for op in [Operator::Plus, Operator::Multiply].iter() {
            // NB: Instead of cloning the list of operators each time, it would
            // be more efficient to use the Vec as a stack, pushing and popping...
            let mut next_try: Vec<Operator> = try_operators.clone();
            next_try.extend_from_slice(&[op.clone()]);
            if self.try_operators(&next_try) {
                return true;
            }
        }
        false
    }
    /// Compute all possible sets of operator combinations and evaluate them. If one matches the
    /// result in the Equation, return true.
    /// 
    /// On success, the Equation will have the matching set of operators set in field operators.
    pub fn compute_operators(&mut self) -> bool {
        self.try_operators(&Vec::new())
    }
}

fn load_input(input: &str) -> Vec<Equation> {
    let mut result: Vec<Equation> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let parsed = line.split_once(":");
        match parsed {
            Some((computed_str, operands_str)) => {
                let computed: u64 = computed_str.parse().unwrap();
                let operands: Vec<u64> = operands_str
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                result.push(Equation {
                    computed,
                    operands,
                    operators: Vec::new(),
                })
            }
            None => panic!("Unable to parse input: {}", line),
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut equations = load_input(input);
    let mut sum : u64 = 0;
    for equation in equations.iter_mut() {
        if equation.compute_operators() {
            print!("*"); 
            sum += equation.computed;
        } else {
            print!(".");
        }
        io::stdout().flush().unwrap();
    }
    println!();
    
    // 882304362421 for sample input
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_mult_sum() {
        let mut expressions = load_input("2: 1 1\n8: 4 2\n12:2 4 4\n2: 1 2 3");
        assert!(expressions[0].compute_operators());
        assert_eq!(expressions[0].operators, vec![Operator::Plus]);
        assert!(expressions[1].compute_operators());
        assert_eq!(expressions[1].operators, vec![Operator::Multiply]);
        assert!(expressions[2].compute_operators());
        assert_eq!(expressions[2].operators, vec![Operator::Multiply, Operator::Plus]);
        assert!(!expressions[3].compute_operators());
        assert_eq!(expressions[3].operators, vec![]);
    }
    
    #[test]
    fn test_load_input() {
        let parsed_input = load_input("2: 1 1\n8: 4 2");
        assert_eq!(
            vec![
                Equation {
                    computed: 2,
                    operands: vec![1, 1],
                    operators: Vec::new()
                },
                Equation {
                    computed: 8,
                    operands: vec![4, 2],
                    operators: Vec::new()
                }
            ],
            parsed_input
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
