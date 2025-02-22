advent_of_code::solution!(3);

#[derive(PartialEq, Debug)]
enum Token {
    MUL,
    LParen,
    Number,
    Comma,
    RParen,
    UNKNOWN,
}

/* A state machine to parse MUL(<param1>,<param2>)

 Currently, I just parse into a list of MUL nodes.
 To do this the "right" way, you'd have the nodes as a tree with MUL as parent and parameters as
 child nodes.
 */
#[derive(PartialEq, Debug)]
enum State {
    Start,
    MUL,
    LParen,
    Param1,
    Comma,
    Param2,
    Complete,
}

struct Entry {
    curr: State,
    tok: Token,
    next: State,
    /* Function that processes the current state, then returns the current list of parsed nodes */
    func: fn(&Token, &str, &State, Box<Node>) -> Box<Node>,
}

/*
  Stores a list of parsed nodes.
  NB: In a traditional parser, the output is a tree, not a list.
 */
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
            'M' => {
                let lookahead = self.input.get(self.curr_pos-1..=self.curr_pos+1);
                // Look to see if there were enough characters
                if !lookahead.is_none() {
                    let lookahead = lookahead.unwrap();
                    if lookahead == "MUL" {
                        self.curr_pos += 2;
                        return (Some(Token::MUL), Some(lookahead))
                    }
                }
                (Some(Token::UNKNOWN), value)

            }
            '0'..='9' => {
                return (Some(Token::Number), Some(Self::get_digits(self)))
            },
            _ => { return (Some(Token::UNKNOWN), value)},
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
                        tok: Token::MUL,
                        next: State::MUL,
                        func: Self::consume,
                    },

                    Entry {
                        curr: State::MUL,
                        tok: Token::LParen,
                        next: State::LParen,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::LParen,
                        tok: Token::Number,
                        next: State::Param1,
                        func: Self::process_param,
                    },
                    Entry {
                        curr: State::Param1,
                        tok: Token::Comma,
                        next: State::Comma,
                        func: Self::consume,
                    },
                    Entry {
                        curr: State::Comma,
                        tok: Token::Number,
                        next: State::Param2,
                        func: Self::process_param,
                    },
                    Entry {
                        curr: State::Param2,
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
            if entry.curr == *state && entry.tok == *token {
                return Some(&entry)
            }
        }
        None
    }
    pub fn parse(input: &str) -> Option<Box<Node>> {
        let mut tokenizer = Tokenizer::new(input);
        let parser = Self::new();
        let mut head = Box::new(Node::new());
        let mut curr_state = &State::Start;
       loop {
           let (next_token, next_value) = tokenizer.get();
           if next_token.is_none() {
               break
           }
           let next_value = next_value.unwrap();
           let next_token = next_token.unwrap();
           let entry = parser.find_entry(&curr_state, &next_token);
           // Parser returns None when it reaches the end of input
           if entry.is_none() {
               break
           }
           let entry = entry.unwrap();
           // Handle this transition with the function specified in the table
           // The function will return the new head of the list of nodes.
           head = (entry.func)(&next_token, next_value, &curr_state, head);
           curr_state = &entry.next;
        }
        Some(head)
    }
    /* consume - consume the token and proceed to the next state */
    fn consume(
        _token: &Token,
        _value: &str,
        state: &State,
        node: Box<Node>,
    ) -> Box<Node> {
        node
    }

    fn complete<'a>(
        _token: &Token,
        value: &str,
        _state: &State,
        node: Box<Node>,
    ) -> Box<Node> {
        print!("Got {} * {}", node.first_number, node.second_number);
        let mut new_node = Box::new(Node::new());
        new_node.next = Some(node);
        new_node
    }

    fn process_param<'a>(
        _token: &Token,
        value: &str,
        state: &State,
        mut node: Box<Node>,
    ) -> Box<Node> {
        match state {
            State::Param1 => node.first_number = value.parse().unwrap(),
            State::Param2 => node.second_number = value.parse().unwrap(),
            _ => panic!("Unknown state {}", state)
        }
        
        node
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let nodes = Parser::parse(input);
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let _nodes = Parser::parse(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        assert_eq!((Some(Token::LParen), Some("(")), Tokenizer::new("(").get());
        assert_eq!((Some(Token::Comma), Some(",",)), Tokenizer::new(",").get());
        assert_eq!((Some(Token::RParen), Some(")")), Tokenizer::new(")").get());
        assert_eq!((Some(Token::MUL), Some("MUL")), Tokenizer::new("MUL").get(), "MUL valid");
        assert_eq!((Some(Token::UNKNOWN), Some("M")), Tokenizer::new("MU").get(), "MU invalid");
        assert_eq!((Some(Token::UNKNOWN), Some("M")), Tokenizer::new("MUD").get(), "MUD invalid");

    }
    #[test]
    fn validate_transitions() {
        let _p = Parser::new();
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
