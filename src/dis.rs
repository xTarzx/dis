use std::collections::HashMap;

use crate::lexer::{Lexer, Token};
use crate::statement::Statement;
use crate::Result;

const MEM_SIZE: usize = 4096;

pub struct DIS {
    registers: HashMap<String, u16>,
    memory: [u16; MEM_SIZE],
    label_map: HashMap<String, usize>,
    program: Vec<Statement>,
    pc: usize,

    die: bool,
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
            pc: 0,
            die: false,
        }
    }

    fn reset(&mut self) {
        self.registers.iter_mut().for_each(|(_, v)| *v = 0);
        self.memory.iter_mut().for_each(|v| *v = 0);
        self.program.clear();
    }

    fn index_labels(&mut self) -> Result<()> {
        for (idx, statement) in self.program.iter().enumerate() {
            if let Some(Token::Label { value, loc }) = &statement.label {
                if self.label_map.contains_key(value) {
                    eprintln!("{loc}: duplicate label `{value}`");

                    let first_loc = match &self.program[self.label_map[value]] {
                        Statement {
                            label: Some(Token::Label { loc, .. }),
                            ..
                        } => loc,
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

        let mut tokens = lexer.tokens()?;

        while !tokens.is_empty() {
            let statement = Statement::parse(&mut tokens)?;

            if let Some(statement) = statement {
                self.program.push(statement);
            }
        }

        self.index_labels()?;

        // dbg!(&self.program);

        Ok(())
    }
}
