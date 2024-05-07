use std::collections::HashMap;

use crate::lexer::{Lexer, Token};

type Result<T> = std::result::Result<T, ()>;

const MEM_SIZE: usize = 4096;

pub struct DIS {
    registers: HashMap<String, u16>,
    memory: [u16; MEM_SIZE],
    label_map: HashMap<String, usize>,
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
            label_map: HashMap::new(),
            program: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.registers.iter_mut().for_each(|(_, v)| *v = 0);
        self.memory.iter_mut().for_each(|v| *v = 0);
        self.program.clear();
    }

    fn index_labels(&mut self) -> Result<()> {
        for (idx, token) in self.program.iter().enumerate() {
            if let Token::Label { value, loc } = token {
                if self.label_map.contains_key(value) {
                    eprintln!("{loc}: duplicate label `{label}`", label = value);

                    let first_loc = match &self.program[self.label_map[value]] {
                        Token::Label { loc, .. } => loc,
                        _ => unreachable!(),
                    };

                    eprintln!("first defined here: {first_loc}");
                    return Err(());
                }
                self.label_map.insert(value.clone(), idx);
            }
        }
        Ok(())
    }

    pub fn load<T>(&mut self, source_path: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.reset();
        let mut lexer = Lexer::new(source_path.into());

        while let Some(token) = lexer.next_token() {
            self.program.push(token);
        }

        self.index_labels()?;

        Ok(())
    }
}
