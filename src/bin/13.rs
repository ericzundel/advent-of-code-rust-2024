use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Button {
    x: u64,
    y: u64,
}

impl Button {
    pub fn parse(input: &str, expected: &str) -> Self {
        let re = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();

        if let Some(captures) = re.captures(input) {
            let found_label:&str = &captures[1];
            assert_eq!(expected, found_label);
            return Button {
                x: captures[2].parse().unwrap(),
                y: captures[3].parse().unwrap(),
            };
        }
        panic!("Parse error. Expected Button format, got {}", input)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl Machine {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.split_terminator("\n").collect();
        let a = Button::parse(lines[0], "A");
        let b = Button::parse(lines[1], "B");
        let prize = Prize::parse(lines[2]);
        Machine { a, b, prize }
    }
}

#[derive(Debug, Clone)]
struct Solver {
    machines: Vec<Machine>,
}

impl Solver {
    pub fn new(input: &str) -> Self {
        let machines_str: Vec<&str> = input.split_terminator("\n\n").collect();
        let machines: Vec<Machine> = machines_str.iter().map(|x| Machine::new(x)).collect();
        Solver { machines }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let Solver = Solver::new(input);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let Solver = Solver::new(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_solver() {
        let input : &str = "Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n";
        let expected = Solver{
            machines: vec![Machine {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            }],
        };
        let solver = Solver::new(input);
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
