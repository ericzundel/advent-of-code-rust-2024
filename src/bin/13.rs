use regex::Regex;
use std::cmp;

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

impl Machine {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.split_terminator("\n").collect();
        let a = Button::parse(lines[0], "A");
        let b = Button::parse(lines[1], "B");
        let prize = Prize::parse(lines[2]);
        Machine { a, b, prize }
    }

    pub fn solve_naive(&self) -> Option<u64> {
        let max_a: u64 = cmp::min(self.prize.x / self.a.x, self.prize.y / self.a.y) + 1;
        let max_b: u64 = cmp::min(self.prize.x / self.b.x, self.prize.y / self.b.y) + 1;
        // let max_a= 100;
        // let max_b = 100;
        let mut min_solution: Option<u64> = None;
        for a_presses in 0..max_a {
            for b_presses in 0..max_b {
                let result: Option<u64> = self.try_solve(a_presses, b_presses);
                if result.is_some() {
                    let cost = result.clone().unwrap();
                    if min_solution.is_some() {
                        let min_cost: u64 = min_solution.clone().unwrap();
                        if cost < min_cost {
                            min_solution = result;
                        }
                    } else {
                        min_solution = result;
                    }
                }
            }
        }
        if min_solution.is_none() {
            println!("1: Prize: {:?} No solution found", self.prize);
        }
        min_solution
    }

    pub fn try_solve(&self, a_presses: u64, b_presses: u64) -> Option<u64> {
        if self.prize.x == self.a.x * a_presses + self.b.x * b_presses
            && self.prize.y == self.a.y * a_presses + self.b.y * b_presses
        {
            println!(
                "1: {:?} Got a_presses: {a_presses} b_presses: {b_presses}",
                self.prize
            );
            return Some(a_presses * 3 + b_presses);
        }

        None
    }

    /// Compute a number of b guesses to go along with the specified a guesses. Always guess
    /// on the high side so we will overshoot y to make the comparision deterministic.
    fn compute_b_guesses(&self, a_guess: u64) -> u64 {
        let remainder: u64 = self.prize.x.saturating_sub(a_guess * self.a.x);
        let result = remainder / self.b.x;
        if a_guess * self.a.x + result * self.b.x < self.prize.x {
            return result + 1;
        }
        result
    }

    /// Compute a number of a guesses to go along with the specified b guesses. Always guess
    /// on the high side so we will overshoot y to make the comparision deterministic.
    fn compute_a_guesses(&self, b_guess: u64) -> u64 {
        let remainder: u64 = self.prize.x.saturating_sub(b_guess * self.b.x);
        let result = remainder / self.a.x;
        if result * self.a.x + b_guess * self.b.x < self.prize.x {
            return result + 1;
        }
        result
    }
    fn midpoint(a: u64, b: u64) -> u64 {
        (a / 2) + (b / 2) + (a & b & 1)
    }

    /// Control the number of b guesses and let a guesses be the dependent variable
    fn binary_search_b(&self, low_b_guess: u64, high_b_guess: u64) -> Option<(u64, u64)> {
        // Try to lower the range of low and high until we are within shooting distance
        //dbg!(low_b_guess, high_b_guess);
        assert!(low_b_guess <= high_b_guess);
        if low_b_guess == high_b_guess {
            let b_guess = low_b_guess;
            let a_guess = self.compute_a_guesses(low_b_guess);
            if a_guess * self.a.x + b_guess * self.b.x == self.prize.x
                && a_guess * self.a.y + b_guess * self.b.y == self.prize.y
            {
                return Some((a_guess, b_guess));
            }
            return None;
        } else if high_b_guess - low_b_guess == 1 {
            let result = self.binary_search_b(low_b_guess, low_b_guess);
            if result.is_none() {
                return self.binary_search_b(high_b_guess, high_b_guess);
            }
            return result;
        }

        if high_b_guess.abs_diff(low_b_guess) < 5 {
            /*
            println!(
                "Y diff is {y_diff} between {low_b_guess} and {high_b_guess}. Trying Brute Force"
            );
             */
            return self.brute_force_solve_b(low_b_guess, high_b_guess);
        }
        let mid_b_guess = Self::midpoint(low_b_guess, high_b_guess);
        let mid_a_guess = self.compute_a_guesses(mid_b_guess);

        // Hmm, our outer guesses weren't even close. Let's try narrowing in a bit
        let mid_y = self.a.y * mid_a_guess + self.b.y * mid_b_guess;

        if mid_y > self.prize.y {
            if self.a.y > self.b.y {
                return self.binary_search_b(low_b_guess, Self::midpoint(low_b_guess, high_b_guess));
            } else {
                return self.binary_search_b(Self::midpoint(low_b_guess, high_b_guess), high_b_guess);
            }
        } else {
            if self.a.y > self.b.y {
                return self.binary_search_b(Self::midpoint(low_b_guess, high_b_guess), high_b_guess);
            } else {
                return self.binary_search_b(low_b_guess, Self::midpoint(low_b_guess, high_b_guess));
            }
        }
    }

    /// Control the number of a guesses and let b guesses be the dependent variable
    fn binary_search_a(&self, low_a_guess: u64, high_a_guess: u64) -> Option<(u64, u64)> {
        // Try to lower the range of low and high until we are within shooting distance
        //dbg!(low_a_guess, high_a_guess);
        assert!(low_a_guess <= high_a_guess);
        if low_a_guess == high_a_guess {
            let a_guess = low_a_guess;
            let b_guess = self.compute_b_guesses(low_a_guess);
            if a_guess * self.a.x + b_guess * self.b.x == self.prize.x
                && a_guess * self.a.y + b_guess * self.b.y == self.prize.y
            {
                return Some((a_guess, b_guess));
            }
            return None;
        } else if high_a_guess - low_a_guess == 1 {
            let result = self.binary_search_a(low_a_guess, low_a_guess);
            if result.is_none() {
                return self.binary_search_a(high_a_guess, high_a_guess);
            }
            return result;
        }

        if high_a_guess.abs_diff(low_a_guess) < 5 {
            /*
            println!(
                "Y diff is {y_diff} between {low_a_guess} and {high_a_guess}. Trying Brute Force"
            );
            */

            return self.brute_force_solve_a(low_a_guess, high_a_guess);


        }
        let mid_a_guess = Self::midpoint(low_a_guess, high_a_guess);
        let mid_b_guess = self.compute_b_guesses(mid_a_guess);

        // Hmm, our outer guesses weren't even close. Let's try narrowing in a bit
        let mid_y = self.a.y * mid_a_guess + self.b.y * mid_b_guess;

        if mid_y > self.prize.y {
            if self.a.y > self.b.y {
                return self.binary_search_a(low_a_guess, Self::midpoint(low_a_guess, high_a_guess));
            } else {
                return self.binary_search_a(Self::midpoint(low_a_guess, high_a_guess), high_a_guess);
            }
        } else {
            if self.a.y > self.b.y {
                return self.binary_search_a(Self::midpoint(low_a_guess, high_a_guess), high_a_guess);
            } else {
                return self.binary_search_a(low_a_guess, Self::midpoint(low_a_guess, high_a_guess));
            }
        }
    }

    fn brute_force_solve_a(&self, low_a_guess: u64, high_a_guess: u64) -> Option<(u64, u64)> {
        let fudge_factor: u64 =
            std::cmp::max(self.a.y.abs_diff(self.b.y), self.a.x.abs_diff(self.b.x));

        for a_presses in
            low_a_guess.saturating_sub(fudge_factor)..high_a_guess.saturating_add(fudge_factor)
        {
            let b_presses = self.compute_b_guesses(a_presses);
            let result = self.try_solve(a_presses, b_presses);
            if result.is_some() {
                /*
                println!(
                    "BRUTE FORCE Guessed a:{} b{} prize.x:{} prize.y:{}",
                    a_guess, b_guess, x, y
                );

                 */
                return Some((a_presses, b_presses));
            }
        }
        None
    }

    fn brute_force_solve_b(&self, low_b_guess: u64, high_b_guess: u64) -> Option<(u64, u64)> {
        let fudge_factor: u64 =
            std::cmp::max(self.a.y.abs_diff(self.b.y), self.a.x.abs_diff(self.b.x));

        for b_presses in
            low_b_guess.saturating_sub(fudge_factor)..high_b_guess.saturating_add(fudge_factor)
        {
            let a_presses = self.compute_a_guesses(b_presses);
            let result = self.try_solve(a_presses, b_presses);
            if result.is_some() {
                /*
                println!(
                    "BRUTE FORCE Guessed a:{} b{} prize.x:{} prize.y:{}",
                    a_guess, b_guess, x, y
                );

                 */
                return Some((a_presses, b_presses));
            }
        }
        None
    }

    pub(crate) fn solve_optimized(&self) -> Option<u64> {
        // println!("{:?}", self);
        let a_guess = self.prize.x / self.a.x;
        let mut result = self.binary_search_a(0, a_guess);
        if result.is_none() {
            let b_guess = self.prize.x / self.b.x;
            result = self.binary_search_b(0, b_guess);
            if result.is_some() {
                println!("A failed, suceeded with b")
            }
        }
        if result.is_some() {
            let unwrapped_result = result.unwrap();
            let a_guesses = unwrapped_result.0;
            let b_guesses = unwrapped_result.1;
            println!(
                "2: {:?} Got a_guesses: {a_guesses}, b_guesses: {b_guesses}",
                self.prize
            );
            return Some(a_guesses * 3 + b_guesses);
        }
        println!("2: {:?} No solution found", self.prize);
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
            let machine_result = machine.solve_naive();
            if machine_result.is_some() {
                result += machine_result.unwrap();
            }
        }
        result
    }

    pub(crate) fn solve_part_two(&self) -> u64 {
        let mut result: u64 = 0;
        for machine in self.machines.iter() {
            let machine_result = machine.solve_optimized();
            if machine_result.is_some() {
                result += machine_result.unwrap() as u64;
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

    pub fn new_part_two(input: &str) -> Self {
        let mut solver = Solver::new(input);
        for machine in &mut solver.machines {
            machine.prize.x += 10000000000000;
            machine.prize.y += 10000000000000;
        }
        solver
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let solver = Solver::new(input);
    let cost1 = solver.solve_part_one();
    let cost2 = solver.solve_part_two();
    assert_eq!(cost1, cost2);
    // Solution with AOC data is 31761
    Some(cost1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let solver = Solver::new_part_two(input);
    let cost = solver.solve_part_two();
    // Solution with AOC data is higher than 875318608908
    //                                       875318608908
    // This solution isn't right either      82041245827082
    // Still wrong:                          79678581085762
    // And still wrong:                      47638619110835
    // Not this either: 48120546821769 ?
    // It ain't                              82076474516082
    // THE ANSWER!!!! 90798500745591
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::Write;

    static EXAMPLE_MACHINE1: &str = "Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n";

    static EXAMPLE_MACHINE2: &str = "Button A: X+26, Y+66\n\
            Button B: X+67, Y+21\n\
            Prize: X=12748, Y=12176\n\
            \n";

    static EXAMPLE1: &str = "Button A: X+94, Y+34\n\
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

    static EXAMPLE_MACHINE3: &str = "Button A: X+59, Y+62
            Button B: X+11, Y+57
            Prize: X=4193, Y=6860\n\
            \n";

    static EXAMPLE_MACHINE4: &str = "Button A: X+56, Y+22\n\
            Button B: X+59, Y+99\n\
            Prize: X=7907, Y=10461\n\
            \n";

    static EXAMPLE_MACHINE5: &str = "Button A: X+31, Y+85\n\
            Button B: X+30, Y+24\n\
            Prize: X=3096, Y=8256\n\
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
        let solver = Solver::new(EXAMPLE_MACHINE1);
        assert_eq!(expected, solver);

        let machine = solver.machines.iter().next().unwrap();
        let solve_result = machine.try_solve(80, 40);
        assert_eq!(280, solve_result.unwrap());

        let cost = solver.solve_part_one();
        assert_eq!(280, cost);

        let cost = solver.solve_part_two();
        assert_eq!(280, cost);
    }

    #[test]
    fn test_example1_part_one() {
        let solver: Solver = Solver::new(EXAMPLE1);
        let result = solver.solve_part_one();
        assert_eq!(480, result);
        let result = solver.solve_part_two();
        assert_eq!(480, result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480))
    }

    #[test]
    fn test_example1_part_two() {
        let solver = Solver::new_part_two(EXAMPLE_MACHINE2);
        print!("{:?}", &solver);
        io::stdout().flush().unwrap();
        let cost = solver.solve_part_two();
        assert_eq!(cost, 459236326669);
    }

    #[test]
    fn test_example3_part_two() {
        let solver = Solver::new(EXAMPLE_MACHINE3);
        print!("{:?}", &solver);
        io::stdout().flush().unwrap();
        let cost = solver.solve_part_one();
        assert_eq!(cost, 61 * 3 + 54);
        let cost = solver.solve_part_two();
        assert_eq!(cost, 61 * 3 + 54);
    }

    #[test]
    fn test_example4_part_two() {
        let solver = Solver::new(EXAMPLE_MACHINE4);
        print!("{:?}", &solver);
        io::stdout().flush().unwrap();
        let cost1 = solver.solve_part_one();
        let cost2 = solver.solve_part_two();
        assert_eq!(cost1, cost2);
    }

    #[test]
    fn test_example5_part_two() {
        let solver = Solver::new(EXAMPLE_MACHINE5);
        print!("{:?}", &solver);
        io::stdout().flush().unwrap();
        let cost1 = solver.solve_part_one();
        let cost2 = solver.solve_part_two();
        assert_eq!(cost1, cost2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
