use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Clone, PartialEq)]
struct Button {
    x: u64,
    y: u64,
}

impl Button {
    pub fn parse(input: &str, expected: &str) -> Self {
        let re = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();

        if let Some(captures) = re.captures(input) {
            let found_label: &str = &captures[1];
            assert_eq!(expected, found_label);
            return Button {
                x: captures[2].parse().unwrap(),
                y: captures[3].parse().unwrap(),
            };
        }
        panic!("Parse error. Expected Button format, got {}", input)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Prize {
    x: u64,
    y: u64,
}

impl Prize {
    pub fn parse(input: &str) -> Self {
        let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        if let Some(captures) = re.captures(input) {
            return Prize {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
            };
        }
        panic!("Parse error. Expected Prize format, got {}", input)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

#[derive(Debug, Clone)]
struct ButtonPressSequence {
    a: u64,
    b: u64,
    cost: u64,
}

impl ButtonPressSequence {
    pub(crate) fn cost(&self) -> u64 {
        self.cost
    }
}

impl Machine {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.split_terminator("\n").collect();
        let a = Button::parse(lines[0], "A");
        let b = Button::parse(lines[1], "B");
        let prize = Prize::parse(lines[2]);
        Machine { a, b, prize }
    }

    pub fn solve(&self) -> Option<ButtonPressSequence> { 
        //let max_a: u64 = cmp::min(self.prize.x / self.a.x, self.prize.y / self.a.y);
        //let max_b: u64 = cmp::min(self.prize.x / self.b.x, self.prize.y / self.b.y);
        let max_a= 100;
        let max_b = 100;
        let mut min_solution: Option<ButtonPressSequence> = None;
        for a_presses in 0..max_a {
            for b_presses in 0..max_b {
                let result: Option<ButtonPressSequence> = self.try_solve(a_presses, b_presses);
                if result.is_some() {
                    let cost = result.clone().unwrap().cost;
                    if min_solution.is_some() {
                        let min_cost = min_solution.clone().unwrap().cost;
                        if cost < min_cost {
                            min_solution = result;
                        }
                    } else {
                        min_solution = result;
                    }
                }
            }
        }
        min_solution
    }

    pub fn try_solve(&self, a_presses: u64, b_presses: u64) -> Option<ButtonPressSequence> {
        if self.prize.x == self.a.x * a_presses + self.b.x * b_presses
            && self.prize.y == self.a.y * a_presses + self.b.y * b_presses
        {
            return Some(ButtonPressSequence {
                a: a_presses,
                b: b_presses,
                cost: a_presses * 3 + b_presses,
            });
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Solver {
    machines: Vec<Machine>,
}

impl Solver {
    pub(crate) fn solve_part_one(&self) -> u64 {
        let mut result: u64 = 0;
        for machine in self.machines.iter() {
            let machine_result = machine.solve();
            if machine_result.is_some() {
                result += machine_result.unwrap().cost();
            }
        }
        result
    }
}

impl Solver {
    pub fn new(input: &str) -> Self {
        let machines_str: Vec<&str> = input.split_terminator("\n\n").collect();
        let machines: Vec<Machine> = machines_str.iter().map(|x| Machine::new(x)).collect();
        Solver { machines }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let solver = Solver::new(input);
    let cost = solver.solve_part_one();
    // Solution with AOC data is 31761
    Some(cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let solver = Solver::new(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MACHINE : &str = "Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n";
    
    #[test]
    fn test_parse_solver() {
       
        let expected = Solver {
            machines: vec![Machine {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            }],
        };
        let solver = Solver::new(EXAMPLE_MACHINE);
        assert_eq!(expected, solver);
        
        let machine = solver.machines.iter().next().unwrap();
        let solve_result = machine.try_solve(80, 40);
        assert_eq!(280, solve_result.unwrap().cost);

        let cost = solver.solve_part_one();
        assert_eq!(280, cost);
    }

    #[test]
    fn test_example1_part_one() {
        let input: &str = "Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n\
            Button A: X+26, Y+66\n\
            Button B: X+67, Y+21\n\
            Prize: X=12748, Y=12176\n\
            \n\
            Button A: X+17, Y+86\n\
            Button B: X+84, Y+37\n\
            Prize: X=7870, Y=6450\n\
            \n\
            Button A: X+69, Y+23\n\
            Button B: X+27, Y+71\n\
            Prize: X=18641, Y=10279\n\
            \n";

        let solver: Solver = Solver::new(input);
        let result = solver.solve_part_one();
        assert_eq!(480, result);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480))
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
