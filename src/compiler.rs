use crate::chunk::Byte;
use crate::chunk::OpCode;
use crate::chunk::Chunk;
use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::TokenType;
use crate::values::Value;
use crate::virtual_machine::InterpretError;
#[cfg(any(feature = "dumpChunk"))]
use crate::disassembler;

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: Token::create_dummy(),
            previous: Token::create_dummy(),
            had_error: false,
            panic_mode: false,
        }
    }
    pub fn had_error(&self) -> bool {
        self.had_error
    }
    pub fn set_error(&mut self, value: bool) {
        self.had_error = value;
    }
    pub fn compile(&mut self, source: &str, chunk: &mut Chunk) -> Result<(), InterpretError> {
        let mut scanner = Scanner::init(source);
        self.advance(&mut scanner);
        self.expression(chunk, &mut scanner);
        self.consume(&mut scanner, TokenType::EOF, "Expect end of expression");
        self.end_compiler(chunk);
        #[cfg(any(feature = "dumpChunk"))]
        if !self.had_error() {
            disassembler::dump_chunk(chunk, "code");
        }
        if self.had_error() {
            Err(InterpretError::CompileError)
        } else {
            Ok(())
        }
    }
    fn expression(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) {
        self.parse_precedence(chunk, scanner, Precedence::Assignment);
    }
    fn number(&mut self, chunk: &mut Chunk) {
        match self.previous.as_str().parse::<f64>() {
            Ok(value) => self.emit_constant(chunk, value),
            Err(_) => {
                self.error("Invalid float constant.");
            },
        }
    }
    fn grouping(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) {
        self.expression(chunk, scanner);
        self.consume(scanner, TokenType::RightParen, "Expect ')' after expression.");
    }
    fn unary(&mut self, chunk: &mut Chunk, scanner: &mut Scanner) {
        let op_type = self.previous.ttype();
        self.parse_precedence(chunk, scanner, Precedence::Unary);
        match op_type {
            TokenType::Minus => self.emit_byte(chunk, Byte::from(OpCode::OpNegate)),
            TokenType::Bang => self.emit_byte(chunk, Byte::from(OpCode::OpNot)),
            _ => {}
        }
    }
    fn binary(&mut self, scanner: &mut Scanner, chunk: &mut Chunk) {
        let op_type = self.previous.ttype();
        let prec = self.get_rule_precedence(op_type);
        self.parse_precedence(chunk, scanner, prec.higher());
        match op_type {
            TokenType::Plus => self.emit_byte(chunk, Byte::from(OpCode::OpAdd)),
            TokenType::Minus => self.emit_byte(chunk, Byte::from(OpCode::OpSubtract)),
            TokenType::Star => self.emit_byte(chunk, Byte::from(OpCode::OpMultiply)),
            TokenType::Slash => self.emit_byte(chunk, Byte::from(OpCode::OpDivide)),
            TokenType::BangEqual => self.emit_bytes(chunk, Byte::from(OpCode::OpEqual), Byte::from(OpCode::OpNot)),
            TokenType::EqualEqual => self.emit_byte(chunk, Byte::from(OpCode::OpEqual)),
            TokenType::Greater => self.emit_byte(chunk, Byte::from(OpCode::OpGreater)),
            TokenType::GreateEqual => self.emit_bytes(chunk, Byte::from(OpCode::OpLess), Byte::from(OpCode::OpNot)),
            TokenType::Less => self.emit_byte(chunk, Byte::from(OpCode::OpLess)),
            TokenType::LessEqual => self.emit_bytes(chunk, Byte::from(OpCode::OpGreater), Byte::from(OpCode::OpNot)),
            _ => {}
        }
    }
    fn get_rule_precedence(&self, ttype: TokenType) -> Precedence {
        match ttype {
            TokenType::Minus => Precedence::Term,
            TokenType::Plus => Precedence::Term,
            TokenType::Slash => Precedence::Factor,
            TokenType::Star => Precedence::Factor,
            TokenType::BangEqual => Precedence::Equality,
            TokenType::EqualEqual => Precedence::Equality,
            TokenType::Greater => Precedence::Comparison,
            TokenType::GreateEqual => Precedence::Comparison,
            TokenType::Less => Precedence::Comparison,
            TokenType::LessEqual => Precedence::Comparison,
            _ => Precedence::None,
        }
    }
    fn parse_precedence(&mut self, chunk: &mut Chunk, scanner: &mut Scanner, precedence: Precedence) {
        self.advance(scanner);
        if !self.call_rule_prefix(chunk, scanner, self.previous.ttype()) {
            self.error("Expect expression.");
        }
        while precedence <= self.get_rule_precedence(self.current.ttype()) {
            self.advance(scanner);
            if !self.call_rule_infix(chunk, scanner, self.previous.ttype()) {
                self.error("Strange: Missing infix rule!");
                break
            }
        }
    }
    fn call_rule_infix(&mut self, chunk: &mut Chunk, scanner: &mut Scanner, ttype: TokenType) -> bool {
        match ttype {
            TokenType::Minus => self.binary(scanner, chunk),
            TokenType::Plus => self.binary(scanner, chunk),
            TokenType::Slash => self.binary(scanner, chunk),
            TokenType::Star => self.binary(scanner, chunk),
            TokenType::BangEqual => self.binary(scanner, chunk),
            TokenType::EqualEqual => self.binary(scanner, chunk),
            TokenType::Greater => self.binary(scanner, chunk),
            TokenType::GreateEqual => self.binary(scanner, chunk),
            TokenType::Less => self.binary(scanner, chunk),
            TokenType::LessEqual => self.binary(scanner, chunk),
            _ => return false,
        }
        true
    }
    fn call_rule_prefix(&mut self, chunk: &mut Chunk, scanner: &mut Scanner, ttype: TokenType) -> bool {
        match ttype {
            TokenType::LeftParen => self.grouping(chunk, scanner),
            TokenType::Minus => self.unary(chunk, scanner),
            TokenType::Number => self.number(chunk),
            TokenType::Nil => self.literal(chunk),
            TokenType::True => self.literal(chunk),
            TokenType::False => self.literal(chunk),
            TokenType::Bang => self.unary(chunk, scanner),
            _ => return false,
        }
        true
    }
    fn literal(&self, chunk: &mut Chunk) {
        match self.previous.ttype() {
            TokenType::False => self.emit_byte(chunk, Byte::from(OpCode::OpFalse)),
            TokenType::True => self.emit_byte(chunk, Byte::from(OpCode::OpTrue)),
            TokenType::Nil => self.emit_byte(chunk, Byte::from(OpCode::OpNil)),
            _ => panic!("Strange literal!"),
        }
    }
    fn emit_constant(&self, chunk: &mut Chunk, value: f64) {
        chunk.write_const(Value::from(value), self.previous.line());
    }
    fn end_compiler(&self, chunk: &mut Chunk) {
        self.emit_return(chunk);
    }
    fn emit_byte(&self, chunk: &mut Chunk, byte: Byte) {
        chunk.write_code(byte, self.previous.line());
    }
    fn emit_bytes(&self, chunk: &mut Chunk, byte1: Byte, byte2: Byte) {
        chunk.write_code(byte1, self.previous.line());
        chunk.write_code(byte2, self.previous.line());
    }
    fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_byte(chunk, Byte::from(OpCode::OpReturn));
    }
    fn consume(&mut self, scanner: &mut Scanner, ttype: TokenType, msg: &str) {
        if self.current.ttype() == ttype {
            self.advance(scanner);
        } else {
            self.error_at_current(msg);
        }
    }
    fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();
        loop {
            self.current = scanner.scan_token();
            if self.current.ttype() != TokenType::Error {
                break;
            }
            let msg = self.current.as_str().to_string();
            self.error_at_current(&msg);
        }
    }
    fn error_at_current(&mut self, msg: &str) {
        let error_token = self.current.clone();
        self.error_at(&error_token, msg);
    }
    fn error(&mut self, msg: &str) {
        let error_token = self.previous.clone();
        self.error_at(&error_token, msg);
    }
    fn error_at(&mut self, token: &Token, msg: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}:{}] Error", token.line(), token.column() - token.as_str().len());
        if token.ttype() == TokenType::EOF {
            eprint!(" at end");
        } else if token.ttype() == TokenType::Error {
        } else {
            eprint!(" at '{}'", token.as_str());
        }
        eprintln!(": {}", msg);
        self.set_error(true);
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None = 0,
    Assignment = 1, // =
    Or = 2,         // or
    And = 3,        // and
    Equality = 4,   // == !=
    Comparison = 5, // < > <= >=
    Term = 6,       // + -
    Factor = 7,     // * /
    Unary = 8,      // ! -
    Call = 9,       // . ()
    Primary = 10,
}

impl Precedence {
    fn higher(&self) -> Precedence {
        let value = (*self as u8) + 1;
        Precedence::try_from(value).expect("Internal error, invalid precedence")
    }
}

impl Into<Precedence> for u8 {
    fn into(self) -> Precedence {
        match self {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            _ => Precedence::Primary,
        }
    }
}