advent_of_code::solution!(3);

enum Token {
    M,
    U,
    L,
    LParen,
    Digit,
    Comma,
    RParen,
}
enum State {
    Start,
    M,
    U,
    L,
    LParen,
    Param1Digit1,
    Param1Digit2,
    Param1Digit3,
    Comma,
    Param2Digit1,
    Param2Digit2,
    Param2Digit3,
    Complete,
}

struct Entry {
    curr: State,
    tok: Token,
    next: State,
    /* Function that processes the current state, then returns the current list of parsed nodes */
    func: fn(Token, char, State, Box<Node>) -> Box<Node>,
}

struct Node {
    first_number: u64,
    second_number: u64,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            first_number: 0,
            second_number: 0,
            next: None,
        }
    }
}

/* TODO: This could be many fewer states if I were to write a real lexical analyzer */
static TRANSITIONS: [Entry; 16] = [
    Entry {
        curr: State::Start,
        tok: Token::M,
        next: State::M,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::M,
        tok: Token::U,
        next: State::U,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::U,
        tok: Token::L,
        next: State::L,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::L,
        tok: Token::LParen,
        next: State::LParen,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::LParen,
        tok: Token::Digit,
        next: State::Param1Digit1,
        func: StateMachine::process_param1,
    },
    Entry {
        curr: State::Param1Digit1,
        tok: Token::Digit,
        next: State::Param1Digit2,
        func: StateMachine::process_param1,
    },
    Entry {
        curr: State::Param1Digit1,
        tok: Token::Comma,
        next: State::Comma,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::Param1Digit2,
        tok: Token::Digit,
        next: State::Param1Digit3,
        func: StateMachine::process_param1,
    },
    Entry {
        curr: State::Param1Digit2,
        tok: Token::Comma,
        next: State::Comma,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::Param1Digit3,
        tok: Token::Comma,
        next: State::Comma,
        func: StateMachine::consume,
    },
    Entry {
        curr: State::Comma,
        tok: Token::Digit,
        next: State::Param2Digit1,
        func: StateMachine::process_param2,
    },
    Entry {
        curr: State::Param2Digit1,
        tok: Token::Digit,
        next: State::Param2Digit2,
        func: StateMachine::process_param2,
    },
    Entry {
        curr: State::Param2Digit1,
        tok: Token::RParen,
        next: State::Complete,
        func: StateMachine::complete,
    },
    Entry {
        curr: State::Param2Digit2,
        tok: Token::Digit,
        next: State::Param2Digit3,
        func: StateMachine::process_param2,
    },
    Entry {
        curr: State::Param2Digit2,
        tok: Token::RParen,
        next: State::Complete,
        func: StateMachine::complete,
    },
    Entry {
        curr: State::Param2Digit3,
        tok: Token::RParen,
        next: State::Complete,
        func: StateMachine::complete,
    },
];

struct StateMachine {
    transitions: &'static[Entry;16],
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            transitions: &TRANSITIONS,
        }
    }
    
    pub fn parse() -> Option<Box<Node>> {
        let mut head = Box::new(Node::new());
        
        Some(head)
    }
    /* consume - consume the token and proceed to the next state */
    fn consume(
        token: Token,
        character: char,
        state: State,
        node: Box<Node>,
    ) -> Box<Node> {
        node
    }

    fn complete<'a>(
        token: Token,
        character: char,
        state: State,
        node: Box<Node>,
    ) -> Box<Node> {
        print!("Got {} * {}", node.first_number, node.second_number);
        let mut new_node = Box::new(Node::new());
        new_node.next = Some(node);
        new_node
    }

    fn process_param1<'a>(
        token: Token,
        character: char,
        state: State,
        mut node: Box<Node>,
    ) -> Box<Node> {
        node.first_number *= 10;
        node.first_number += character.to_digit(10).unwrap() as u64;
        node
    }

    fn process_param2<'a>(
        token: Token,
        character: char,
        state: State,
        mut node: Box<Node>,
    ) -> Box<Node> {
        node.second_number *= 10;
        node.second_number += character.to_digit(10).unwrap() as u64;
        node
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
