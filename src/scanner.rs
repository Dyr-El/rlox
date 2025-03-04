pub struct Scanner {
    buffer: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {

    pub fn init(source: &str) -> Scanner {
        Scanner { 
            buffer: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }
        let c = self.advance();
        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::SemiColon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            },
            '=' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            },
            '<' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::GreateEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            }
            '"' => return self.string(),
            _ => {
                if c.is_digit(10) {
                    return self.number();
                }
                if Self::identifier_first(c) {
                    return self.identifier();
                }
                return self.error_token("Unexpected character.")
            },
        }
        // self.error_token("Unexpected character.")
    }

    fn identifier_first(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn identifier_rest(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn identifier(&mut self) -> Token {
        while Self::identifier_rest(self.peek()) {
            let _ = self.advance();
        }
        let tt = self.identifier_type();
        self.make_token(tt)
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.buffer[self.start] {
            'a' => self.check_keyword(1, 2, vec!['n', 'd'], TokenType::And),
            'c' => self.check_keyword(1, 4, vec!['l', 'a', 's', 's'], TokenType::Class),
            'e' => self.check_keyword(1, 3, vec!['l', 's', 'e'], TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.buffer[self.start + 1] {
                        'a' => self.check_keyword(2, 3, vec!['l', 's', 'e'], TokenType::False),
                        'o' => self.check_keyword(2, 1, vec!['r'], TokenType::For),
                        'u' => self.check_keyword(2, 1, vec!['n'], TokenType::Fun),
                        _ => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, vec!['f'], TokenType::If),
            'n' => self.check_keyword(1, 2, vec!['i', 'l'], TokenType::Nil),
            'o' => self.check_keyword(1, 1, vec!['r'], TokenType::Or),
            'p' => self.check_keyword(1, 4, vec!['r', 'i', 'n', 't'], TokenType::Print),
            'r' => self.check_keyword(1, 5, vec!['e', 't', 'u', 'r', 'n'], TokenType::Return),
            's' => self.check_keyword(1, 4, vec!['u', 'p', 'e', 'r'], TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.buffer[self.start + 1] {
                        'h' => self.check_keyword(2, 2, vec!['i', 's'], TokenType::This),
                        'r' => self.check_keyword(2, 2, vec!['u', 'e'], TokenType::True),
                        _ => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1 ,2, vec!['a', 'r'], TokenType::Var),
            'w' => self.check_keyword(1, 4, vec!['h', 'i', 'l', 'e'], TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&mut self, offset: usize, length: usize, chars: Vec<char>, ttype: TokenType) -> TokenType {
        if self.current - self.start != offset + length {
            return TokenType::Identifier
        }
        for idx in 0..length {
            if chars[idx] != self.buffer[self.start + offset + idx] {
                return TokenType::Identifier
            }
        }
        ttype
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            let _ = self.advance();
        }
        if self.peek() == '.' && self.peek_next(1).is_digit(10) {
            let _ = self.advance();
            while self.peek().is_digit(10) {
                let _ = self.advance();
            }    
        }
        return self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != '"' {
            let _ = self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        let _ = self.advance();
        self.make_token(TokenType::String)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.peek().is_whitespace() {
                let _ = self.advance();
            } else if self.peek() == '/' {
                if self.peek_next(1) == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        let _ = self.advance();
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.buffer[self.current]
        }
    }

    fn peek_next(&self, steps: usize) -> char {
        if self.current + steps >= self.buffer.len() {
            '\0'
        } else {
            self.buffer[self.current + steps]
        }
    }

    fn match_char(&mut self, the_char: char) -> bool {
        if self.peek() != the_char {
            return false;
        }
        let _ = self.advance();
        true
    }

    pub fn advance(&mut self) -> char {
        let c = self.buffer[self.current];
        match c {
            '\n' => {
                self.line += 1;
                self.column = 1;
            },
            '\t' => {
                self.column += 4;
            }
            _ => {
                self.column += 1;
            }
        }
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.buffer.len()
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        let mut s = String::new();
        for i in self.start..self.current {
            s.push(self.buffer[i]);
        }
        Token {
            token_type: ttype,
            the_string: s,
            line: self.line,
            column: self.column,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            the_string: message.to_string(),
            line: self.line,
            column: self.column,
        }
    }
}

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    the_string: String,
    line: usize,
    column: usize,
}

impl Token {
    pub fn ttype(&self) -> TokenType {
        self.token_type
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn as_str(&self) -> &str {
        &self.the_string
    }
    pub fn create_dummy() -> Token {
        Token {
            token_type: TokenType::Dummy,
            the_string: String::new(),
            line: 0,
            column: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreateEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Special.
    Error, EOF, Dummy,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN")?,
            TokenType::RightParen => write!(f, "RIGHT_PAREN")?,
            TokenType::LeftBrace => write!(f, "LEFT_BRACE")?,
            TokenType::RightBrace => write!(f, "RIGHT_BRACE")?,
            TokenType::Comma => write!(f, "COMMA")?,
            TokenType::Dot => write!(f, "DOT")?,
            TokenType::Minus => write!(f, "MINUS")?,
            TokenType::Plus => write!(f, "PLUS")?,
            TokenType::SemiColon => write!(f, "SEMI_COLON")?,
            TokenType::Slash => write!(f, "SLASH")?,
            TokenType::Star => write!(f, "STAR")?,
            TokenType::Bang => write!(f, "BANG")?,
            TokenType::BangEqual => write!(f, "BANG_EQUAL")?,
            TokenType::Equal => write!(f, "EQUAL")?,
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL")?,
            TokenType::Greater => write!(f, "GREATER")?,
            TokenType::GreateEqual => write!(f, "GREATER_EQUAL")?,
            TokenType::Less => write!(f, "LESS")?,
            TokenType::LessEqual => write!(f, "LESS_EQUAL")?,
            TokenType::Identifier => write!(f, "IDENTIFIER")?,
            TokenType::String => write!(f, "STRING")?,
            TokenType::Number => write!(f, "NUMBER")?,
            TokenType::And => write!(f, "AND")?,
            TokenType::Class => write!(f, "CLASS")?,
            TokenType::Else => write!(f, "ELSE")?,
            TokenType::False => write!(f, "FALSE")?,
            TokenType::For => write!(f, "FOR")?,
            TokenType::Fun => write!(f, "FUN")?,
            TokenType::If => write!(f, "IF")?,
            TokenType::Nil => write!(f, "NIL")?,
            TokenType::Or => write!(f, "OR")?,
            TokenType::Print => write!(f, "PRINT")?,
            TokenType::Return => write!(f, "RETURN")?,
            TokenType::Super => write!(f, "SUPER")?,
            TokenType::This => write!(f, "THIS")?,
            TokenType::True => write!(f, "TRUE")?,
            TokenType::Var => write!(f, "VAR")?,
            TokenType::While => write!(f, "WHILE")?,
            TokenType::Error => write!(f, "<ERROR>")?,
            TokenType::EOF => write!(f, "<EOF>")?,
            TokenType::Dummy => write!(f, "<DUMMY>")?,
        }
        Ok(())
    }
}