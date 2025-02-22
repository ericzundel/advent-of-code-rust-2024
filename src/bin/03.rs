advent_of_code::solution!(3);

#[derive(PartialEq)]
enum Token {
    M,
    U,
    L,
    LParen,
    Digit,
    Comma,
    RParen,
    UNKNOWN,
}
#[derive(PartialEq)]
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
    func: fn(&Token, &str, &State, Box<Node>) -> Box<Node>,
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


struct Tokenizer<'a> {
    input: &'a str,
    curr_pos: usize,
}

impl <'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer {
        Tokenizer {
            input,
            curr_pos: 0,
        }
    }

    /* Intended to be called from get to consume all digits starting from curr_pos -1.
       Assumes we already know that self.curr_pos is a valid digit.

       Returns all of the consecutive digits in the stream
     */
    fn get_digits(&mut self) -> &str {
        let start_pos = self.curr_pos-1;
        let end_pos = start_pos
            + self.input[start_pos..]
            .chars()
            .take_while(|c| c.is_digit(10))
            .count();
        // consume all of these digits
        self.curr_pos = end_pos;
        &self.input[start_pos..end_pos]
    }

    /* Returns the next token in the input stream with a slice pointing to it's value */
    pub fn get(&mut self) -> (Option<Token>, Option<&str>) {

        let value = self.input.get(self.curr_pos..self.curr_pos+1);
        if value == None {
            return (None, None)
        }
        self.curr_pos += 1;
        let next_char = value.unwrap().chars().next().unwrap();
        match next_char {
            '(' => {return (Some(Token::LParen),value)},
            ',' => {return (Some(Token::Comma), value)},
            ')' => {return (Some(Token::RParen), value)},
            'M' => {return (Some(Token::M), value)},
            'U' => {return (Some(Token::U), value)},
            'L' => {return (Some(Token::L), value)},
            '0'..='9' => {
                return (Some(Token::Digit), Some(Self::get_digits(self)))
            },
            (_) => { return (Some(Token::UNKNOWN), value)},
        }
    }
}

struct Parser {
    transitions: Vec<Entry>,
}

impl Parser {
    
    /* TODO: This could be many fewer states if I were to write a real lexical analyzer */
    fn new() -> Parser {
        Parser {
            transitions: Vec::from(
                [
                    Entry {
                        curr: State::Start,
                        tok: Token::M,
                        next: State::M,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::M,
                        tok: Token::U,
                        next: State::U,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::U,
                        tok: Token::L,
                        next: State::L,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::L,
                        tok: Token::LParen,
                        next: State::LParen,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::LParen,
                        tok: Token::Digit,
                        next: State::Param1Digit1,
                        func: Self::process_param1,
                    },
                    Entry {
                        curr: State::Param1Digit1,
                        tok: Token::Digit,
                        next: State::Param1Digit2,
                        func: Self::process_param1,
                    },
                    Entry {
                        curr: State::Param1Digit1,
                        tok: Token::Comma,
                        next: State::Comma,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::Param1Digit2,
                        tok: Token::Digit,
                        next: State::Param1Digit3,
                        func: Self::process_param1,
                    },
                    Entry {
                        curr: State::Param1Digit2,
                        tok: Token::Comma,
                        next: State::Comma,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::Param1Digit3,
                        tok: Token::Comma,
                        next: State::Comma,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::Comma,
                        tok: Token::Digit,
                        next: State::Param2Digit1,
                        func: Self::process_param2,
                    },
                    Entry {
                        curr: State::Param2Digit1,
                        tok: Token::Digit,
                        next: State::Param2Digit2,
                        func: Self::process_param2,
                    },
                    Entry {
                        curr: State::Param2Digit1,
                        tok: Token::RParen,
                        next: State::Complete,
                        func: Self::complete,
                    },
                    Entry {
                        curr: State::Param2Digit2,
                        tok: Token::Digit,
                        next: State::Param2Digit3,
                        func: Self::process_param2,
                    },
                    Entry {
                        curr: State::Param2Digit2,
                        tok: Token::RParen,
                        next: State::Complete,
                        func: Self::complete,
                    },
                    Entry {
                        curr: State::Param2Digit3,
                        tok: Token::RParen,
                        next: State::Complete,
                        func: Self::complete,
                    },
                ]),
        }
    }

    /* Return the first entry in the transition table that matches the state and token */
    fn find_entry(&self, state: &State, token:&Token) -> Option<&Entry> {
        /*  NB: I looked at how to convert this into functional style and AI suggested
            .find (which finds all matches and is inefficient and .position which
            returns the index into the vector which requires a lot of mangling.
            Even though .position might end up being a "zero cost abstraction" at runtime,
            I consider it to be a lot more complex than the iterative style below.
         */
        for entry in self.transitions.iter() {
            if entry.curr == state && entry.tok == token {
                return Some(&entry)
            }
        }
        None
    }
    pub fn parse(input: &str) -> Option<Box<Node>> {
        let mut tokenizer = Tokenizer::new(input);
        let parser = Self::new();
        let mut head = Box::new(Node::new());
        let mut curr_state = State::Start;
       loop {
           let (next_token, next_value) = tokenizer.get();
           if next_token.is_none() {
               break
           }
           let next_value = next_value.unwrap();
           let next_token = next_token.unwrap();
           let entry = parser.find_entry(&curr_state, &next_token);
           if entry.is_none() {
               break
           }
           let entry = entry.unwrap();
           head = (entry.func(&next_token, next_value, &curr_state, head);
        }
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
        token: &Token,
        value: &str,
        state: &State,
        node: Box<Node>,
    ) -> Box<Node> {
        print!("Got {} * {}", node.first_number, node.second_number);
        let mut new_node = Box::new(Node::new());
        new_node.next = Some(node);
        new_node
    }

    fn process_param1<'a>(
        token: &Token,
        value: &str,
        state: &State,
        mut node: Box<Node>,
    ) -> Box<Node> {
        node.first_number = value.parse().unwrap();
        node
    }

    fn process_param2<'a>(
        token: &Token,
        value: &str,
        state: &State,
        mut node: Box<Node>,
    ) -> Box<Node> {
        node.second_number = value.parse().unwrap();
        node
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let nodes = Parser::parse(input);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_transitions() {
        let p = Parser::new();
        todo!("go thorugh all transitions and make sure there are no dups")
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
