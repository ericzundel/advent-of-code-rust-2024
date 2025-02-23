use std::fmt;

advent_of_code::solution!(3);

#[derive(PartialEq, Debug, Hash, Eq)]
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
#[derive(PartialEq, Debug, Hash, Eq)]
enum State {
    Start,
    MUL,
    LParen,
    Param1,
    Comma,
    Param2,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/* TODO: modify the parser to load the state transitions into a .
    Currently, I'm just doing that in a test to make sure I didn't
    screw anything up.
*/
#[derive(Hash, Eq, PartialEq, Debug)]
struct EntryKey {
    curr: State,
    tok: Token,
}

struct Entry {
    curr: State,
    tok: Token,
    next: State,
    /* Function that processes the current state, then returns the current list of parsed nodes */
    func: fn(&Token, &str, &State, &mut Node) -> bool,
}

/*
 Stores a list of parsed nodes.
 NB: In a traditional parser, the output is a tree, not a list.
*/
#[derive(PartialEq, Debug, Clone)]
struct Node {
    first_number: u64,
    second_number: u64,
}

impl Node {
    pub fn new() -> Node {
        Node {
            first_number: 0,
            second_number: 0,
        }
    }
}

#[derive(Clone)]
struct Tokenizer<'a> {
    input: &'a str,
    curr_pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, curr_pos: 0 }
    }

    /* Intended to be called from get to consume all digits starting from curr_pos -1.
      Assumes we already know that self.curr_pos is a valid digit.

      Returns all of the consecutive digits in the stream
    */
    fn get_digits(&mut self) -> &'a str {
        let start_pos = self.curr_pos - 1;
        let mut len = self.input[start_pos..]
            .chars()
            .take_while(|c| c.is_digit(10))
            .count();
        // consume all of these digits
        if len > 3 {
            len = 3;
        }
        self.curr_pos = start_pos + len;
        &self.input[start_pos..self.curr_pos]
    }

    /* Returns the next token in the input stream with a slice pointing to it's value */
    pub fn get(&mut self) -> (Option<Token>, Option<&'a str>) {
        let value = self.input.get(self.curr_pos..self.curr_pos + 1);
        if value == None {
            return (None, None);
        }
        self.curr_pos += 1;
        let next_char = value.unwrap().chars().next().unwrap();
        match next_char {
            '(' => (Some(Token::LParen), value),
            ',' => (Some(Token::Comma), value),
            ')' => (Some(Token::RParen), value),
            'm' => {
                let lookahead = self.input.get(self.curr_pos - 1..=self.curr_pos + 1);
                // Look to see if there were enough characters
                if !lookahead.is_none() {
                    let lookahead = lookahead.unwrap();
                    if lookahead == "mul" {
                        self.curr_pos += 2;
                        return (Some(Token::MUL), Some(lookahead));
                    }
                }
                (Some(Token::UNKNOWN), value)
            }
            '0'..='9' => return (Some(Token::Number), Some(Self::get_digits(self))),
            _ => return (Some(Token::UNKNOWN), value),
        }
    }
}

struct Parser {
    transitions: Vec<Entry>,
    nodes: Vec<Box<Node>>,
}

/* This parser uses the Tokenizer to do lexical analysis,
  then creates a list of Node structures to correspond to the
  valid data found in the input
*/
impl Parser {
    fn new() -> Parser {
        Parser {
            transitions: Vec::from([
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
                    next: State::Start,
                    func: Self::complete,
                },
            ]),
            nodes: Vec::<Box<Node>>::new(),
        }
    }

    /* Return the first entry in the transition table that matches the state and token */
    fn find_entry(&self, state: &State, token: &Token) -> Option<&Entry> {
        /*  NB: I looked at how to convert this into functional style and AI suggested
           .find (which finds all matches and is inefficient and .position which
           returns the index into the vector which requires a lot of mangling.
           Even though .position might end up being a "zero cost abstraction" at runtime,
           I consider it to be a lot more complex than the iterative style below.
        */
        for entry in self.transitions.iter() {
            if entry.curr == *state && entry.tok == *token {
                return Some(&entry);
            }
        }
        None
    }
    fn add_node(&mut self, node: Box<Node>) {
        self.nodes.push(node);
    }
    pub fn parse(input: &str) -> Vec<Box<Node>> {
        let mut tokenizer = Tokenizer::new(input);
        let mut parser = Self::new();
        let mut curr_node = Box::new(Node::new());
        let mut curr_state = &State::Start;
        loop {
            let (next_token, next_value) = tokenizer.get();
            if next_token.is_none() {
                break;
            }
            let next_value = next_value.unwrap();
            let next_token = next_token.unwrap();
            let entry = {
                let t = parser.find_entry(&curr_state, &next_token).clone();
                t
            };
            // Can't find a valid state transistion? Go back to START
            if entry.is_none() {
                curr_state = &State::Start;
                continue;
            }
            let entry = entry.unwrap();
            // Handle this transition with the function specified in the table
            let complete = (entry.func)(&next_token, next_value, &entry.next, & mut curr_node);
            if complete {
                parser.add_node(curr_node);
                curr_node = Box::new(Node::new());
            }
            curr_state = &entry.next;
        }
        parser.nodes
    }

    /* consume - consume the token and proceed to the next state */
    fn consume(_token: &Token, _value: &str, _state: &State, _node: &mut Node) -> bool {
        false
    }

    fn complete<'a>(_token: &Token, _value: &str, _state: &State, node: &mut Node) -> bool {
        print!("Got {} * {}", node.first_number, node.second_number);
        true
    }

    fn process_param<'a>(_token: &Token, value: &str, state: &State, node: &mut Node) -> bool {
        match state {
            State::Param1 => node.first_number = value.parse().unwrap(),
            State::Param2 => node.second_number = value.parse().unwrap(),
            _ => panic!("Unknown state {}", state),
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let nodes = Parser::parse(input);
    let mut sum = 0u64;
    for node in nodes.iter() {
        sum += node.first_number * node.second_number;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let _nodes = Parser::parse(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            (Some(Token::MUL), Some("mul")),
            Tokenizer::new("mul").get(),
            "MUL valid"
        );
        assert_eq!((Some(Token::LParen), Some("(")), Tokenizer::new("(").get());
        assert_eq!((Some(Token::Comma), Some(",",)), Tokenizer::new(",").get());
        assert_eq!((Some(Token::RParen), Some(")")), Tokenizer::new(")").get());

        assert_eq!(
            (Some(Token::UNKNOWN), Some("m")),
            Tokenizer::new("mu").get(),
            "MU invalid"
        );
        assert_eq!(
            (Some(Token::UNKNOWN), Some("m")),
            Tokenizer::new("mud").get(),
            "MUD invalid"
        );
        assert_eq!(
            (Some(Token::Number), Some("1")),
            Tokenizer::new("1").get(),
            "1"
        );
        assert_eq!(
            (Some(Token::Number), Some("21")),
            Tokenizer::new("21").get(),
            "21"
        );
        assert_eq!(
            (Some(Token::Number), Some("219")),
            Tokenizer::new("219").get(),
            "219"
        );
        assert_eq!(
            (Some(Token::Number), Some("217")),
            Tokenizer::new("2179").get(),
            "2179"
        );
    }

    #[test]
    fn test_tokenizer_seq1() {
        // Test sequence of tokens
        let expected = [
            ((Some(Token::Number)), Some("292")),
            ((Some(Token::Number)), Some("7")),
            ((None, None)),
        ];
        let mut result = Vec::new();
        let t: &mut Tokenizer = &mut Tokenizer::new("2927");
        for _i in 0..3 {
            result.push(t.get());
        }
        assert_eq!(expected, result.as_slice());
    }

    #[test]
    fn test_tokenizer_seq2() {
        // Test sequence of tokens
        let expected = [
            ((Some(Token::MUL)), Some("mul")),
            ((Some(Token::LParen)), Some("(")),
            ((Some(Token::Number)), Some("123")),
            ((Some(Token::Comma)), Some(",")),
            ((Some(Token::Number)), Some("456")),
            ((Some(Token::RParen)), Some(")")),
            ((None, None)),
        ];
        let mut result = Vec::new();
        let t: &mut Tokenizer = &mut Tokenizer::new("mul(123,456)");
        for _i in 0..7 {
            result.push(t.get());
        }
        assert_eq!(expected, result.as_slice());
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Node {
                first_number: 123,
                second_number: 456,
            },
            *Parser::parse("mul(123,456)").get(0).unwrap()
        );
    }

    #[test]
    fn validate_transitions() {
        let p = Parser::new();
        let mut set: HashSet<EntryKey> = HashSet::new();
        for entry in p.transitions {
            let key = EntryKey {
                curr: entry.curr,
                tok: entry.tok,
            };
            assert!(!set.contains(&key));
            set.insert(key);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
        print!("Got result {}", result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
