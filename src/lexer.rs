use std::fmt;
use std::fs;

use crate::Result;

const KEYWORDS: [&str; 19] = [
    "mov", "add", "sub", "cmp", "jmp", "jlt", "jgt", "jeq", "jne", "run", "ret", "die", "out",
    "prt", "@", "rdn", "rdc", "rln", "dbg",
];

#[derive(Debug, Clone)]
pub struct Location {
    file: String,
    line: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword { value: String, loc: Location },
    Label { value: String, loc: Location },
    Char { value: char, loc: Location },
    Number { value: u16, loc: Location },
    Register { value: String, loc: Location },
    Memory { value: String, loc: Location },
    Identifier { value: String, loc: Location },
}

impl Token {
    pub fn loc(&self) -> &Location {
        match self {
            Token::Keyword { loc, .. } => loc,
            Token::Label { loc, .. } => loc,
            Token::Char { loc, .. } => loc,
            Token::Number { loc, .. } => loc,
            Token::Register { loc, .. } => loc,
            Token::Memory { loc, .. } => loc,
            Token::Identifier { loc, .. } => loc,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Keyword { value, .. } => write!(f, "Keyword({})", value),
            Token::Label { value, .. } => write!(f, "Label({})", value),
            Token::Char { value, .. } => write!(f, "Char({})", value),
            Token::Number { value, .. } => write!(f, "Number({})", value),
            Token::Register { value, .. } => write!(f, "Register({})", value),
            Token::Memory { value, .. } => write!(f, "Memory({})", value),
            Token::Identifier { value, .. } => write!(f, "Identifier({})", value),
        }
    }
}

pub struct Lexer {
    source_path: String,
    source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(source_path: String) -> Result<Lexer> {
        let source = fs::read_to_string(&source_path);

        let source = match source {
            Ok(source) => source,
            Err(_) => {
                eprintln!("Failed to read file: {}", source_path);
                return Err(());
            }
        };

        Ok(Lexer {
            source_path: source_path,
            source: source,
            pos: 0,
        })
    }

    fn location(&self, pos: usize) -> Location {
        let mut line = 0;
        let mut column = 0;

        for (i, c) in self.source.chars().enumerate() {
            if i == pos {
                break;
            }

            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        Location {
            file: self.source_path.clone(),
            line: line + 1,
            column: column + 1,
        }
    }

    fn strip_whitespace(&mut self) {
        while let Some(c) = self.source.chars().nth(self.pos) {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    fn peek_word(&mut self) -> Option<String> {
        let mut word = String::new();
        let mut chars = self.source[self.pos..].chars().peekable();
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                break;
            }

            word.push(*c);
            chars.next();
        }

        if word.is_empty() {
            return None;
        }

        return Some(word);
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.strip_whitespace();
        let pos = self.pos;
        if let Some(word) = self.peek_word() {
            if word.starts_with("-") {
                while let Some(c) = self.source.chars().nth(self.pos) {
                    if c == '\n' {
                        break;
                    }
                    self.advance(1);
                }

                return self.next_token();
            }

            let loc = self.location(pos);
            self.advance(word.len());

            if word.starts_with(".") {
                return Some(Token::Char {
                    value: word.chars().nth(1).unwrap(),
                    loc: loc,
                });
            }

            if word.ends_with(":") {
                return Some(Token::Label {
                    value: word[..word.len() - 1].to_string(),
                    loc: loc,
                });
            }

            if word.starts_with("#") {
                return Some(Token::Register {
                    value: word[1..].to_string(),
                    loc: loc,
                });
            }
            if word.starts_with("&") {
                return Some(Token::Memory {
                    value: word[1..].to_string(),
                    loc: loc,
                });
            }

            if KEYWORDS.contains(&word.as_str()) {
                return Some(Token::Keyword {
                    value: word.to_string(),
                    loc: loc,
                });
            }

            if let Ok(number) = word.parse() {
                return Some(Token::Number {
                    value: number,
                    loc: loc,
                });
            }

            return Some(Token::Identifier {
                value: word,
                loc: loc,
            });
        };

        None
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        Ok(tokens)
    }
}
