//!
//! # Advent of Code 2024 Day 3
//! https://adventofcode.com/2024/day/3
//!
//!  I really wanted to learn Rust, so I decided to write a lexer and parser to solve this problem.
//!  The grammar for this language looks something like:
//! 
//!  MUL -> "mul" "(" NUMBER "," NUMBER ")"
//!  DO -> "do" "(" ")"
//!  DONT -> "don't" "(" ")"
//!  NUMBER -> [0-9]{1,3}
//!  DATA -> ((MUL | DO | DONT) ?*)*
//! 
//!  There are much simpler ways to do it:
//! 
//!  - Bespoke logic that just keeps track of state as you parse the string
//!  - Use a regular expression.  Very few lines could have solved this problem effectively.
//! 
//!  It wasn't the most efficient use of time to solve this problem, but I learned a lot
//!  by doing it the hard way. Part 2 was relatively easy to solve once I had my lexer and
//!  parser in place. Amazingly, I found and corrected at least one bug which could have stymied
//!  me in part one, but the input didn't contain those edge cases.
//! 
//!  Main components:
//!  Tokenizer - Creates a simple lexical analysis program
//!  Parser - A simple parser that outputs a list of nodes instead of a traditional parse tree
//!
use std::fmt;

advent_of_code::solution!(3);

/// Represents a token in the language
/// e.g. MUL(1,23) would return tokens: MUL LParen, Number, Comma, Number, RParen
#[derive(PartialEq, Debug, Hash, Eq, Clone)]
enum Token {
    MUL,
    DO,
    DONT,
    LParen,
    Number,
    Comma,
    RParen,
    UNKNOWN,
}

/// A state for use in the state machine embedded in the parser.
#[derive(PartialEq, Debug, Hash, Eq, Clone)]
enum State {
    Start,
    MUL,
    DO,
    DONT,
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

/// Key to use when loading transitions into a set. 
/// 
/// TODO: modify the parser to use it. Currently one used for a test.
#[derive(Hash, Eq, PartialEq, Debug)]
struct EntryKey {
    curr: State,
    tok: Token,
}

/// Represents an entry in the state table
struct Entry {
    /// Current state of the parser
    curr: State,
    /// Next token to analyze
    tok: Token,
    /// Next state to transition to
    next: State,
    /// Function that processes the current state
    func: fn(&Token, &str, &State, &mut Node) -> bool,
}

/// Stores the parsed representation of each statement.
/// 
/// NB: In a traditional parser, there nodes would have children to form a tree, This data structure
/// doesn't have a list of children because the final output of the parser is just a list.
#[derive(PartialEq, Debug, Clone)]
struct Node {
    /// The name of the function being parsed. I lazily reused the Token enum because it was already there.
    /// Can represent a MUL, DO, or DONT function in the language
    symbol_name: Token,
    /// First parameter to the MUL function if present
    first_number: u64,
    /// Second parameter to the MUL function if present
    second_number: u64,
}

impl Node {
    /// Instantiate an empty node of unknown type
    pub fn new() -> Node {
        Node {
            symbol_name: Token::UNKNOWN,
            first_number: 0,
            second_number: 0,
        }
    }
}

/// Lexical analyzer for our language
#[derive(Clone)]
struct Tokenizer<'a> {
    input: &'a str,
    curr_pos: usize,
}

impl<'a> Tokenizer<'a> {
    /// Instantiate a lexer for this problem. The lexical analyzer will keep state of the
    /// current position in the input after each call to get().
    ///
    /// # Arguments
    ///
    /// * `input` - the string data to parse.
    ///
    /// # Returns
    ///
    /// An instance of Tokenizer with the context set to the first character.
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, curr_pos: 0 }
    }

    /// Intenrnal function to parse a sequence of up to 3 digits and return it as a slice
    ///
    /// # Arguments
    ///
    /// * `self` - uses the input string and current position. Modifies the current position
    ///            on success.
    ///
    /// # Returns
    ///
    /// A slice containing the consecutive digits that represent the number.
    fn get_number(&mut self) -> &'a str {
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

    /// Internal function to look ahead for a token that matches the &str passed in `symbol`.
    /// Modifies the current position in the input on success.
    fn look_for(
        &mut self,
        symbol: &str,
        token: Token,
        value: Option<&'a str>,
    ) -> (Option<Token>, Option<&'a str>) {
        let symbol_len = symbol.len();
        let lookahead = self
            .input
            .get(self.curr_pos - 1..=self.curr_pos + symbol_len - 2);
        // Look to see if there were enough characters
        if !lookahead.is_none() {
            let lookahead = lookahead.unwrap();
            if lookahead == symbol {
                self.curr_pos += symbol_len - 1;
                return (Some(token), Some(lookahead));
            }
        }
        (Some(Token::UNKNOWN), value)
    }

    /// Retrieve the next token in the input
    ///
    /// # Arguments
    ///
    /// * self - Instance of the Tokenizer
    ///
    /// # Returns
    ///
    /// If there was data, returns the next token in the input stream and a slice pointing to its value.
    /// A value of Token::UNKNOWN is returned if it doesn't recognize the current character.
    /// Returns `(None, None)` at the end of the input.
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
            'd' => {
                let (tok_option, ret_value) = self.look_for("don't", Token::DONT, value);
                let test_ret_token = tok_option.unwrap();
                if test_ret_token == Token::UNKNOWN {
                    return self.look_for("do", Token::DO, value);
                }
                return (Some(test_ret_token), ret_value);
            }
            'm' => self.look_for("mul", Token::MUL, value),
            '0'..='9' => return (Some(Token::Number), Some(Self::get_number(self))),
            _ => return (Some(Token::UNKNOWN), value),
        }
    }
}

/// This parser uses the Tokenizer to do lexical analysis, then uses a state machine to analyze
/// the tokens in sequence.
///
/// # Returns
/// ///
/// A list of Node structures to correspond to the valid data found in the input
struct Parser<'a> {
    transitions: Vec<Entry>,
    nodes: Vec<Box<Node>>,
    input: &'a str,
}

impl<'a> Parser<'a> {

    /// Initialize a new instance.
    fn new(input: &str) -> Parser {
        Parser {
            transitions: Vec::from([
                Entry {
                    curr: State::Start,
                    tok: Token::MUL,
                    next: State::MUL,
                    func: Self::process_symbol,
                },
                Entry {
                    curr: State::Start,
                    tok: Token::DO,
                    next: State::DO,
                    func: Self::process_symbol,
                },
                Entry {
                    curr: State::DO,
                    tok: Token::LParen,
                    next: State::LParen,
                    func: Self::consume,
                },
                Entry {
                    curr: State::Start,
                    tok: Token::DONT,
                    next: State::DONT,
                    func: Self::process_symbol,
                },
                Entry {
                    curr: State::DONT,
                    tok: Token::LParen,
                    next: State::LParen,
                    func: Self::consume,
                },
                Entry {
                    curr: State::MUL,
                    tok: Token::LParen,
                    next: State::LParen,
                    func: Self::consume,
                },

                // State to end do() and don't()
                Entry {
                    curr: State::LParen,
                    tok: Token::RParen,
                    next: State::Start,
                    func: Self::complete,
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
                // State to end mul(123,456)
                Entry {
                    curr: State::Param2,
                    tok: Token::RParen,
                    next: State::Start,
                    func: Self::complete,
                },
            ]),
            nodes: Vec::<Box<Node>>::new(),
            input,
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

    /// This function runs lexical analysis and the parser, returning a list of valid nodes.
    pub fn parse(&mut self) -> Vec<Box<Node>> {
        let mut curr_node = Box::new(Node::new());
        let mut curr_state = State::Start;
        let mut tokenizer = Tokenizer::new(self.input);
        loop {
            let (next_token, next_value) = tokenizer.get();
            if next_token.is_none() {
                break;
            }
            let next_value = next_value.unwrap();
            let next_token = next_token.unwrap();
            let mut entry = self.find_entry(&curr_state, &next_token).clone();
            // Can't find a valid state transition? Try again with START
            if entry.is_none() {
                // Try again assuming we are at the start state
                curr_state = State::Start;
                curr_node = Box::new(Node::new());
                entry = self.find_entry(&curr_state, &next_token).clone();
                if entry.is_none() {
                    continue;
                }
            }
            let entry = entry.unwrap();
            // Handle this transition with the function specified in the table
            let complete = (entry.func)(&next_token, next_value, &entry.next, &mut curr_node);
            curr_state = entry.next.clone();
            if complete {
                self.nodes.push(curr_node);
                curr_node = Box::new(Node::new());
            }
        }
        self.nodes.clone()
    }

    /// Consume the token.  The node is not complete yet.
    fn consume(_token: &Token, _value: &str, _state: &State, _node: &mut Node) -> bool {
        false
    }

    /// Consume the token.  The node is complete.
    fn complete(_token: &Token, _value: &str, _state: &State, _node: &mut Node) -> bool {
        true
    }

    /// This is a parameter to one of the symbols.  Record it in the node.
    fn process_param(_token: &Token, value: &str, state: &State, node: &mut Node) -> bool {
        match state {
            State::Param1 => node.first_number = value.parse().unwrap(),
            State::Param2 => node.second_number = value.parse().unwrap(),
            _ => panic!("Unknown state {}", state),
        }
        false
    }
    /// This is a token representing a function like `mul()`, `don't()` or `do()`. Record
    /// the token so we can process the node by type later.
    fn process_symbol(token: &Token, _: &str, _state: &State, node: &mut Node) -> bool {
        node.symbol_name = token.clone();
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let nodes = Parser::new(input).parse();
    let mut sum = 0u64;
    for node in nodes.iter().filter(|x| x.symbol_name == Token::MUL) {
        sum += node.first_number * node.second_number;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nodes = Parser::new(input).parse();
    let mut sum = 0u64;
    let mut disabled = false;
    for node in nodes.iter() {
        match node.symbol_name {
            Token::DO => disabled = false,
            Token::DONT => disabled = true,
            Token::MUL => {
                if !disabled {
                    sum += node.first_number * node.second_number
                }
            }
            _ => panic!("Unknown symbol {:?}", node.symbol_name),
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            (Some(Token::DO), Some("do")),
            Tokenizer::new("do").get(),
            "DO"
        );
        assert_eq!(
            (Some(Token::DONT), Some("don't")),
            Tokenizer::new("don't").get(),
            "DONT"
        );
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
        let mut parser = Parser::new("mul(123,456)");
        let result = *(parser.parse().get(0).unwrap()).clone();
        assert_eq!(
            Node {
                symbol_name: Token::MUL,
                first_number: 123,
                second_number: 456,
            },
            result
        );
    }

    #[test]
    fn validate_transitions() {
        let p = Parser::new("");
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
        assert_eq!(Some(39), result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(Some(37), result);
    }
}
