#[cfg(any(feature = "traceExecution"))]
use crate::disassembler;
use crate::{chunk, compiler::Parser, values};

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

const MAX_STACK_SIZE: usize = 255;

pub struct VM {
    ip: usize,
    stack: Vec<values::Value>,
}

impl VM {
     pub fn new() -> Self {
        Self { ip: 0 , stack: vec![] }
    }
    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let mut chunk = chunk::Chunk::new();
        let mut parser = Parser::new();
        if let Err(e) = parser.compile(source, &mut chunk) {
            return Err(e);
        }
        self.ip = 0;
        self.run(&chunk)?;
        Ok(())
    }
    fn read_byte(&mut self, chunk: &chunk::Chunk) -> chunk::Byte {
        let result = chunk.read_code(self.ip);
        self.ip += 1;
        result
    }
    fn read_const(&mut self, chunk: &chunk::Chunk) -> values::Value {
        let idx = usize::from(self.read_byte(chunk));
        chunk.read_constant(idx)
    }
    fn read_long_const(&mut self, chunk: &chunk::Chunk) -> values::Value {
        let i1 = usize::from(self.read_byte(chunk));
        let i2 = usize::from(self.read_byte(chunk));
        let i3 = usize::from(self.read_byte(chunk));
        let idx = i1 << 16 | i2 << 8 | i3;
        chunk.read_constant(idx)
    }
    fn run(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        loop {
            #[cfg(feature = "traceExecution")]
            self.traceStack();
            #[cfg(feature = "traceExecution")]
            disassembler::dump_instruction(chunk, self.ip);
            if let Ok(instruction) = chunk::OpCode::try_from(self.read_byte(chunk)) {
                match instruction {
                    chunk::OpCode::OpConstant => self.execute_constant(chunk)?,
                    chunk::OpCode::OpConstantLong => self.execute_long_constant(chunk)?,
                    chunk::OpCode::OpReturn => return self.execute_return(chunk),
                    chunk::OpCode::OpNegate => self.execute_negate(chunk)?,
                    chunk::OpCode::OpAdd => self.execute_add(chunk)?,
                    chunk::OpCode::OpSubtract => self.execute_subtract(chunk)?,
                    chunk::OpCode::OpMultiply => self.execute_multiply(chunk)?,
                    chunk::OpCode::OpDivide => self.execute_divide(chunk)?,
                    chunk::OpCode::OpNil => self.execute_nil(chunk)?,
                    chunk::OpCode::OpTrue => self.execute_true(chunk)?,
                    chunk::OpCode::OpFalse => self.execute_false(chunk)?,
                    chunk::OpCode::OpNot => self.execute_not(chunk)?,
                    chunk::OpCode::OpEqual => self.execute_equal(chunk)?,
                    chunk::OpCode::OpLess => self.execute_less(chunk)?,
                    chunk::OpCode::OpGreater => self.execute_greater(chunk)?,
                }    
            }
        }
    }
    fn execute_less(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        let b = self.pop()?;
        let a = self.pop()?;
        if !a.is_number() || !b.is_number() {
            self.runtime_error(chunk, "Operand must be a number.");
            return Err(InterpretError::RuntimeError);            
        }
        self.push(values::Value::create_boolean(a.is_less_than(&b)))?;
        Ok(())
    }
    fn execute_greater(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        let b = self.pop()?;
        let a = self.pop()?;
        if !a.is_number() || !b.is_number() {
            self.runtime_error(chunk, "Operand must be a number.");
            return Err(InterpretError::RuntimeError);            
        }
        self.push(values::Value::create_boolean(a.is_greater_than(&b)))?;
        Ok(())
    }
    fn execute_equal(&mut self, _chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(values::Value::create_boolean(a.is_equal_to(&b)))?;
        Ok(())
    }
    fn execute_not(&mut self, _chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        let value = self.pop()?.is_falsey();
        self.push(values::Value::create_boolean(value))?;
        Ok(())
    }
    fn execute_nil(&mut self, _chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        self.push(values::Value::create_nil())?;
        Ok(())
    }
    fn execute_true(&mut self, _chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        self.push(values::Value::create_boolean(true))?;
        Ok(())
    }
    fn execute_false(&mut self, _chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        self.push(values::Value::create_boolean(false))?;
        Ok(())
    }
    fn execute_constant(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        let constant = self.read_const(chunk);
        self.push(constant)?;
        Ok(())
    }
    fn execute_long_constant(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        let constant = self.read_long_const(chunk);
        self.push(constant)?;
        Ok(())
    }
    fn execute_return(&mut self, _chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        self.pop()?.print();
        println!("");
        Ok(())
    }
    fn execute_negate(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        if self.peek_stack(0).is_number() {
            let value = self.pop()?;
            self.push(-value)?;
        } else {
            self.runtime_error(chunk, "Operand must be a number.");
            return Err(InterpretError::RuntimeError);
        }
        Ok(())
    }
    fn peek_stack(&self, idx: usize) -> values::Value {
        self.stack[self.stack.len() - idx - 1]
    }
    fn runtime_error(&mut self, chunk: &chunk::Chunk, msg: &str) {
        eprintln!("{}", msg);
        let instruction = self.ip - 1;
        let line = chunk.read_line(instruction);
        eprintln!("[line {}] in script\n", line);
        self.stack.clear();
    }

    fn execute_add(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        if !self.peek_stack(0).is_number() || !self.peek_stack(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(InterpretError::RuntimeError);
        }
        let arg1 = self.pop()?;
        let arg2 = self.pop()?;
        self.push(arg1 + arg2)?;
        Ok(())
    }
    fn execute_subtract(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        if !self.peek_stack(0).is_number() || !self.peek_stack(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(InterpretError::RuntimeError);
        }
        let arg2 = self.pop()?;
        let arg1 = self.pop()?;
        self.push(arg1 - arg2)?;
        Ok(())
    }
    fn execute_multiply(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        if !self.peek_stack(0).is_number() || !self.peek_stack(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(InterpretError::RuntimeError);
        }
        let arg1 = self.pop()?;
        let arg2 = self.pop()?;
        self.push(arg1 * arg2)?;
        Ok(())
    }
    fn execute_divide(&mut self, chunk: &chunk::Chunk) -> Result<(), InterpretError> {
        if !self.peek_stack(0).is_number() || !self.peek_stack(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(InterpretError::RuntimeError);
        }
        let arg2 = self.pop()?;
        let arg1 = self.pop()?;
        self.push(arg1 / arg2)?;
        Ok(())
    }
    fn push(&mut self, value: values::Value) -> Result<(), InterpretError> {
        if self.stack.len() >= MAX_STACK_SIZE {
            return Err(InterpretError::RuntimeError);
        }
        self.stack.push(value);
        Ok(())
    }
    fn pop(&mut self) -> Result<values::Value, InterpretError> {
        if let Some(value) = self.stack.pop() {
            Ok(value)
        } else {
            Err(InterpretError::RuntimeError)
        }
    }
    #[cfg(feature = "traceExecution")]
    fn traceStack(&mut self) {
        print!("        ");
        for &value in &self.stack {
            print!("[ ");
            value.print();
            print!(" ]");
        }
        println!("");
    }
}