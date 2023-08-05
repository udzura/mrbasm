use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Symbol,
    StrLit,
    Number,
    Reg,
    OpCode,
    Bang,
    Colon,
    // keywords
    BinFormat,
    BlkBegin,
    BlkEnd,
    // others
    Ln,
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pos {
    pub token: Token,
    pub lexeme: String,
    pub line: usize,
}

impl Pos {
    pub fn new(token: Token, lexeme: impl Into<String>, line: usize) -> Self {
        Self {
            token,
            lexeme: lexeme.into(),
            line,
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'source> {
    pub source: &'source str,
    pub tokens: Vec<Pos>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Debug)]
pub struct LexError {}

impl LexError {
    pub fn raise() -> Self {
        Self {}
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanError.",)
    }
}

impl Error for LexError {}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        let tokens = Vec::new();
        Self {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<usize, LexError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Pos::new(Token::Eof, "", self.line));
        Ok(self.tokens.len())
    }

    fn scan_token(&mut self) -> Result<(), LexError> {
        use Token::*;
        let c = self.advance()?;

        match c {
            '(' => {
                // comment: ( ... )
                loop {
                    match self.peek()? {
                        '\n' => return Err(LexError::raise()),
                        ')' => {
                            self.advance()?;
                            return Ok(());
                        }
                        _ => {
                            self.advance()?;
                        }
                    }
                }
            }
            ';' => {
                // comment: ; ...
                while self.peek()? != '\n' && !self.is_at_end() {
                    self.advance()?;
                }
            }
            '!' => self.push(Bang),
            ':' => self.push(Colon),
            'R' => {
                let peek = self.peek_n(0)?;
                if is_digit(peek) {
                    self.register()?;
                } else {
                    self.opcode()?;
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace.
            }
            '\n' => {
                self.push(Ln);
                self.line += 1;
            }
            '"' => {
                self.string('"')?;
            }

            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_upper(c) {
                    self.opcode()?;
                } else if is_alpha(c) {
                    self.symbol()?;
                } else {
                    eprintln!("unexpected character: {}", c);
                    return Err(LexError::raise());
                }
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> Result<char, LexError> {
        let c = self.getchar(self.current as usize)?;
        self.current += 1;
        Ok(c)
    }

    #[allow(dead_code)]
    fn test(&mut self, expected: char) -> Result<bool, LexError> {
        if self.is_at_end() {
            return Ok(false);
        }
        let c = self.getchar(self.current)?;
        if c != expected {
            return Ok(false);
        }

        self.current += 1;
        Ok(true)
    }

    fn getchar(&mut self, nth: usize) -> Result<char, LexError> {
        self.source
            .chars()
            .nth(nth)
            .ok_or_else(|| LexError::raise())
    }

    fn peek(&mut self) -> Result<char, LexError> {
        self.peek_n(0)
    }

    fn peek_n(&mut self, off: usize) -> Result<char, LexError> {
        if self.current + off >= self.source.len() {
            Ok('\0')
        } else {
            self.getchar(self.current + off)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self, quote: char) -> Result<(), LexError> {
        while self.peek()? != quote && !self.is_at_end() {
            if self.peek()? == '\n' {
                eprintln!("cannot contain linebreak");
                return Err(LexError::raise());
            }
            self.advance()?;
        }

        if self.is_at_end() {
            eprintln!("unterminated string");
            return Err(LexError::raise());
        }

        // The closing quote
        self.advance()?;
        self.push(Token::StrLit);

        Ok(())
    }

    fn number(&mut self) -> Result<(), LexError> {
        while is_digit(self.peek()?) {
            self.advance()?;
        }
        self.push(Token::Number);
        Ok(())
    }
    fn register(&mut self) -> Result<(), LexError> {
        while is_digit(self.peek()?) {
            self.advance()?;
        }
        self.push(Token::Reg);
        Ok(())
    }

    fn opcode(&mut self) -> Result<(), LexError> {
        while is_opcode_valid(self.peek()?) {
            self.advance()?;
        }
        self.push(Token::OpCode);
        Ok(())
    }

    fn symbol(&mut self) -> Result<(), LexError> {
        while is_alphanumeric(self.peek()?) {
            self.advance()?;
        }

        self.push(Token::Symbol);
        Ok(())
    }

    fn push(&mut self, token: Token) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Pos::new(token, lexeme, self.line));
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_upper(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_opcode_valid(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
