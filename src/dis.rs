use std::collections::HashMap;

use crate::lexer::{Lexer, Token};

const MEM_SIZE: usize = 4096;

pub struct DIS {
    registers: HashMap<String, u16>,
    memory: [u16; MEM_SIZE],
    program: Vec<Token>,
}

impl DIS {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        for i in 0..8 {
            registers.insert(i.to_string(), 0);
        }

        DIS {
            registers,
            memory: [0; MEM_SIZE],
            program: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.registers.iter_mut().for_each(|(_, v)| *v = 0);
        self.memory.iter_mut().for_each(|v| *v = 0);
        self.program.clear();
    }

    pub fn load<T>(&mut self, source_path: T)
    where
        T: Into<String>,
    {
        self.reset();
        let mut lexer = Lexer::new(source_path.into());

        while let Some(token) = lexer.next_token() {
            self.program.push(token);
        }
    }
}
