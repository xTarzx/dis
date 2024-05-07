use std::collections::HashMap;

use crate::lexer::{Lexer, Token};
use crate::statement::{Op, Statement};
use crate::Result;

const MEM_SIZE: usize = 4096;

enum CMP {
    EQ = 0b001,
    LT = 0b010,
    GT = 0b100,
}

pub struct DIS {
    registers: HashMap<String, u16>,
    memory: [u16; MEM_SIZE],
    label_map: HashMap<String, usize>,
    program: Vec<Statement>,
    pc: usize,
    cmp: u8,

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
            cmp: 0,
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

    fn get_value(&self, src_token: &Token) -> Result<u16> {
        match src_token {
            Token::Number { value, .. } => Ok(*value),
            Token::Register { value: reg_id, .. } => {
                let value = self.registers.get(reg_id).unwrap();
                Ok(*value)
            }
            Token::Memory { value: mem_id, .. } => {
                let mem_addr = {
                    if mem_id.starts_with("#") {
                        let reg_val = self.registers.get(&mem_id[1..].to_string()).unwrap();
                        *reg_val as usize
                    } else {
                        mem_id.parse::<usize>().unwrap()
                    }
                };
                let value = self.memory.get(mem_addr).unwrap();
                Ok(*value)
            }
            Token::Char { value, .. } => Ok(*value as u16),
            other => {
                eprintln!(
                    "{loc}: expected number, register, memory or char, found `{src_token}`",
                    loc = other.loc()
                );
                Err(())
            }
        }
    }

    fn set_value(&mut self, dst_token: &Token, value: u16) -> Result<()> {
        match dst_token {
            Token::Register { value: reg_id, .. } => {
                let reg = self.registers.get_mut(reg_id).unwrap();
                *reg = value;
                Ok(())
            }
            Token::Memory { value: mem_id, .. } => {
                let mem_addr = mem_id.parse::<usize>().unwrap();
                let mem = self.memory.get_mut(mem_addr).unwrap();
                *mem = value;
                Ok(())
            }
            other => {
                eprintln!(
                    "{loc}: expected register or memory, found `{dst_token}`",
                    loc = other.loc()
                );
                Err(())
            }
        }
    }

    pub fn step(&mut self) {
        if self.die {
            return;
        }

        let statement = self.program.get(self.pc);

        if statement.is_none() {
            self.die = true;
            return;
        }

        let statement = statement.unwrap().clone();

        match &statement.op {
            Op::MOV(_) => {
                let src = &statement.body[0];
                let dst = &statement.body[1];

                let val = self.get_value(src).unwrap();
                self.set_value(dst, val).unwrap();
            }
            Op::ADD(_) => {
                let src = &statement.body[0];
                let dst = &statement.body[1];

                let val = self.get_value(src).unwrap();
                let dst_val = self.get_value(dst).unwrap();

                self.set_value(dst, val + dst_val).unwrap();
            }
            Op::CMP(_) => {
                let src = &statement.body[0];
                let dst = &statement.body[1];

                let src_val = self.get_value(src).unwrap();
                let dst_val = self.get_value(dst).unwrap();

                self.cmp = 0;

                if src_val == dst_val {
                    self.cmp |= CMP::EQ as u8;
                }

                if src_val < dst_val {
                    self.cmp |= CMP::LT as u8;
                }

                if src_val > dst_val {
                    self.cmp |= CMP::GT as u8;
                }
            }
            Op::JNE(_) => {
                let target_token = &statement.body[0];

                let target_label = match target_token {
                    Token::Identifier { value, .. } => value,
                    _ => unreachable!(),
                };

                let target_idx = self.label_map.get(target_label).unwrap();

                if self.cmp & CMP::EQ as u8 == 0 {
                    self.pc = *target_idx - 1;
                }
            }
            Op::OUT(_) => {
                let src = &statement.body[0];
                let val = self.get_value(src).unwrap();
                print!("{}", val as u8 as char);
            }
            Op::NOP => {
                todo!()
            }
        }

        self.pc += 1;
    }

    pub fn run(&mut self) {
        while !self.die {
            self.step();
        }
    }
}
