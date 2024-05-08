use std::collections::{HashMap, HashSet};
use std::io::Write;

use crate::lexer::{Lexer, Token};
use crate::statement::{Op, Statement};
use dis::Result;

const MEM_SIZE: usize = 4096;

enum CMP {
    EQ = 0b001,
    LT = 0b010,
    GT = 0b100,
}

pub struct DIS {
    registers: HashMap<String, u16>,
    memory: [u16; MEM_SIZE],
    return_stack: Vec<usize>,
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
            registers.insert("e".to_string(), 0);
        }

        DIS {
            registers,
            memory: [0; MEM_SIZE],
            return_stack: Vec::new(),
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

    fn ensure_labels(&mut self) -> Result<()> {
        for statement in &self.program {
            match &statement.op {
                Op::JEQ(_) | Op::JNE(_) | Op::JLT(_) | Op::JGT(_) | Op::JMP(_) | Op::RUN(_) => {
                    let (target_label, loc) = match &statement.body[0] {
                        Token::Identifier { value, loc } => (value, loc),
                        _ => unreachable!(),
                    };

                    if !self.label_map.contains_key(target_label) {
                        eprintln!("{loc}: undefined label `{target_label}`");
                        return Err(());
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }

    fn lex_and_parse_file<T>(
        source_file: T,
        include_map: &mut HashMap<String, HashSet<String>>,
        parent: Option<String>,
    ) -> Result<Vec<Statement>>
    where
        T: Into<String>,
    {
        let source_file = source_file.into();

        let mut lexer = Lexer::new(source_file.clone())?;
        let mut tokens = lexer.tokens()?;
        let mut statements = Vec::new();

        while !tokens.is_empty() {
            let statement = Statement::parse(&mut tokens)?;

            if let Some(statement) = statement {
                match statement.op {
                    Op::INC(token) => {
                        let filename = match &statement.body[0] {
                            Token::Identifier { value, .. } => value,
                            _ => unreachable!(),
                        };

                        let filename = format!("{}.dis", filename);

                        let source_path = std::path::Path::new(&source_file);
                        let source_dir = source_path.parent().unwrap();
                        let source_path = source_path.to_str().unwrap().to_string();
                        let include_filepath: String =
                            source_dir.join(&filename).to_str().unwrap().to_string();

                        // TODO: find a way to check for include conflicts
                        // the current implementation does not allow including the same file more than once

                        // check if the file we are including already includes the file we are including from

                        if include_map.contains_key(&include_filepath) {
                            let target_set = include_map.get(&include_filepath).unwrap();
                            if target_set.contains(&source_path) {
                                eprintln!(
                                    "{loc}: circular include detected: `{filename}`",
                                    loc = token.loc()
                                );
                                return Err(());
                            }
                        }

                        if !include_map.contains_key(&source_path) {
                            include_map.insert(source_path.clone(), HashSet::new());
                        }

                        // add import to set
                        let include_set = include_map.get_mut(&source_path).unwrap();
                        include_set.insert(include_filepath.clone());

                        // add import to parent set
                        if let Some(parent_path) = &parent {
                            let parent_set = include_map.get_mut(parent_path).unwrap();
                            parent_set.insert(include_filepath.clone());
                        }

                        let inc_statements = DIS::lex_and_parse_file(
                            include_filepath,
                            include_map,
                            Some(source_path),
                        )?;

                        statements.extend(inc_statements);
                    }

                    _ => {
                        statements.push(statement);
                    }
                }
            }
        }

        Ok(statements)
    }

    fn mem_addr_from_id(&self, mem_id: &str) -> usize {
        if mem_id.starts_with("#") {
            let reg_val = self.registers.get(&mem_id[1..].to_string()).unwrap();
            *reg_val as usize
        } else {
            mem_id.parse::<usize>().unwrap()
        }
    }

    fn get_value(&self, src_token: &Token) -> Result<u16> {
        match src_token {
            Token::Number { value, .. } => Ok(*value),
            Token::Register { value: reg_id, .. } => {
                let value = self.registers.get(reg_id).unwrap();
                Ok(*value)
            }
            Token::Memory { value: mem_id, .. } => {
                let mem_addr = self.mem_addr_from_id(mem_id);
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
                let mem_addr = self.mem_addr_from_id(mem_id);
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
            Op::SUB(_) => {
                let src = &statement.body[0];
                let dst = &statement.body[1];

                let val = self.get_value(src).unwrap();
                let dst_val = self.get_value(dst).unwrap();

                self.set_value(dst, dst_val - val).unwrap();
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
            Op::JLT(_) => {
                let target_token = &statement.body[0];

                let target_label = match target_token {
                    Token::Identifier { value, .. } => value,
                    _ => unreachable!(),
                };

                let target_idx = self.label_map.get(target_label).unwrap();

                if self.cmp & CMP::LT as u8 != 0 {
                    self.pc = *target_idx - 1;
                }
            }
            Op::JGT(_) => {
                let target_token = &statement.body[0];

                let target_label = match target_token {
                    Token::Identifier { value, .. } => value,
                    _ => unreachable!(),
                };

                let target_idx = self.label_map.get(target_label).unwrap();

                if self.cmp & CMP::GT as u8 != 0 {
                    self.pc = *target_idx - 1;
                }
            }
            Op::JEQ(_) => {
                let target_token = &statement.body[0];

                let target_label = match target_token {
                    Token::Identifier { value, .. } => value,
                    _ => unreachable!(),
                };

                let target_idx = self.label_map.get(target_label).unwrap();

                if self.cmp & CMP::EQ as u8 != 0 {
                    self.pc = *target_idx - 1;
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

            Op::JMP(_) => {
                let target_token = &statement.body[0];

                let target_label = match target_token {
                    Token::Identifier { value, .. } => value,
                    _ => unreachable!(),
                };

                let target_idx = self.label_map.get(target_label).unwrap();

                self.pc = *target_idx - 1;
            }
            Op::RUN(_) => {
                let target_token = &statement.body[0];

                let target_label = match target_token {
                    Token::Identifier { value, .. } => value,
                    _ => unreachable!(),
                };

                let target_idx = self.label_map.get(target_label).unwrap();

                self.return_stack.push(self.pc);

                self.pc = *target_idx - 1;
            }

            Op::RET(_) => {
                let return_idx = self.return_stack.pop().unwrap();
                self.pc = return_idx;
            }

            Op::DIE(_) => {
                self.die = true;
            }

            Op::OUT(_) => {
                let src = &statement.body[0];
                let val = self.get_value(src).unwrap();
                print!("{}", val as u8 as char);
                std::io::stdout().flush().unwrap();
            }

            Op::PRT(_) => {
                let src = &statement.body[0];
                let val = self.get_value(src).unwrap();
                print!("{}", val);
                std::io::stdout().flush().unwrap();
            }

            Op::DBG(_) => {
                let src = &statement.body[0];
                let val = self.get_value(src).unwrap();
                match src {
                    Token::Memory { value, .. } => {
                        if value.starts_with("#") {
                            let mem_addr = self.mem_addr_from_id(value);
                            println!("DBG {src} (&{mem_addr}): {val}");
                        } else {
                            println!("DBG {src}: {val}");
                        }
                    }

                    _ => println!("DBG {src}: {val}"),
                }
            }

            Op::INC(_) => {
                todo!()
            }

            Op::RDN(_) => {
                let dst = &statement.body[0];
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let val = input.trim().parse::<u16>();

                if val.is_err() {
                    self.registers.insert("e".to_string(), 1);
                } else {
                    self.registers.insert("e".to_string(), 0);
                    let val = val.unwrap();
                    self.set_value(dst, val).unwrap();
                }
            }

            Op::RDC(_) => {
                let dst = &statement.body[0];
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let val = input.trim().chars().nth(0);

                if val.is_none() {
                    self.registers.insert("e".to_string(), 1);
                } else {
                    self.registers.insert("e".to_string(), 0);

                    let val = val.unwrap() as u16;
                    self.set_value(dst, val).unwrap();
                }
            }

            Op::RLN(_) => {
                let dst = &statement.body[0];
                let max_c = &statement.body[1];
                let mut max_c = self.get_value(max_c).unwrap();

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let mut val = input.trim();
                if max_c != 0 {
                    max_c = max_c.min(val.len() as u16);
                    val = &val[..max_c as usize];
                }

                self.registers.insert("3".to_string(), val.len() as u16);

                let mem_addr = {
                    let mem_id = match dst {
                        Token::Memory { value, .. } => value,
                        _ => unreachable!(),
                    };

                    if mem_id.starts_with("#") {
                        let reg_val = self.registers.get(&mem_id[1..].to_string()).unwrap();
                        *reg_val as usize
                    } else {
                        mem_id.parse::<usize>().unwrap()
                    }
                };

                for (i, c) in val.chars().enumerate() {
                    self.memory[mem_addr + i] = c as u16;
                }
            }

            Op::NOP => {
                todo!()
            }
        }

        self.pc += 1;
    }

    pub fn load<T>(&mut self, source_path: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.reset();

        let mut include_map = HashMap::new();

        let statements = DIS::lex_and_parse_file(source_path, &mut include_map, None)?;

        self.program = statements;

        self.index_labels()?;
        self.ensure_labels()?;

        Ok(())
    }

    pub fn run(&mut self) {
        while !self.die {
            self.step();
        }
    }
}
