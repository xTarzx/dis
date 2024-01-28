use std::collections::{HashMap, VecDeque};

const REG_N: usize = 4;
const MEM_N: usize = 4096;
const STK_N: usize = 256;

#[derive(Debug, PartialEq)]
enum MemT {
    ADR(usize),
    REG(usize),
}

#[derive(Debug, PartialEq)]
enum ArgT {
    NUM(u8),
    REG(usize),
    MEM(MemT),
    CHR(char),
    LBL(String),
}

impl ArgT {
    pub fn parse(words: &mut VecDeque<&str>, arg_types: Vec<u8>) -> Result<ArgT, String> {
        let arg = words.pop_front().unwrap();

        if arg_types.contains(&(ArgT::LBL as u8)) {
            return Ok(ArgT::LBL(arg.to_string()));
        }

        if arg_types.contains(&(ArgT::REG as u8)) && arg.starts_with('#') {
            // register
            let r_n = match arg[1..].parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(format!("Expected a number after '#': {}", arg)),
            };

            if r_n >= REG_N {
                return Err(format!("Invalid register number: {}", r_n));
            }

            return Ok(ArgT::REG(r_n));
        }

        if arg_types.contains(&(ArgT::MEM as u8)) && arg.starts_with("&") {
            let mem_t: MemT;
            if arg[1..].starts_with("#") {
                let r_n = match arg[2..].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => return Err(format!("Expected a number after '#': {}", arg)),
                };

                if r_n >= REG_N {
                    return Err(format!("Invalid register number: {}", r_n));
                }

                mem_t = MemT::REG(r_n);
            } else {
                let m_n = match arg[1..].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => return Err(format!("Expected a number after '&': {}", arg)),
                };

                if m_n >= MEM_N {
                    return Err(format!("Invalid memory address: {}", m_n));
                }

                mem_t = MemT::ADR(m_n);
            }

            return Ok(ArgT::MEM(mem_t));
        }

        if arg_types.contains(&(ArgT::CHR as u8)) && arg.starts_with(".") {
            let c = arg.chars().nth(1).unwrap();

            return Ok(ArgT::CHR(c));
        }

        if arg_types.contains(&(ArgT::NUM as u8)) {
            // number
            let n = match arg.parse::<u8>() {
                Ok(n) => n,
                Err(_) => return Err(format!("Expected a number: {}", arg)),
            };

            return Ok(ArgT::NUM(n));
        }

        unreachable!("UNREACHABLE: {} {:?}", arg, arg_types)
    }
}

#[derive(Debug)]
enum IType {
    MOV(ArgT, ArgT),
    ADD(ArgT, ArgT),
    SUB(ArgT, ArgT),
    CMP(ArgT, ArgT),
    JLT(ArgT),
    JGT(ArgT),
    JEQ(ArgT),
    JNE(ArgT),
    JMP(ArgT),
    RUN(ArgT),
    RET,
    DIE,
    DBG(ArgT),
    OUT(ArgT),
    PRT(ArgT),
    RDN(ArgT),
    RDC(ArgT),

    NULL,
}

enum CMP {
    EQ = 0b001,
    LT = 0b010,
    GT = 0b100,
}

#[derive(Debug)]
pub struct Token {
    itype: IType,
    label: Option<String>,
}

pub struct DIS {
    registers: [u8; REG_N],
    memory: [u8; MEM_N],

    ret_stack: [usize; STK_N],
    sp: usize,

    cmp: u8,

    pub program: Vec<Token>,

    label_map: HashMap<String, usize>,

    pc: usize,
}

impl DIS {
    pub fn new() -> DIS {
        DIS {
            registers: [0; REG_N],
            memory: [0; MEM_N],
            pc: 0,

            ret_stack: [0; STK_N],
            sp: 0,

            program: Vec::new(),

            label_map: HashMap::new(),

            cmp: 0,
        }
    }

    fn tokenize(&mut self, source_dir: &str, source: String) -> Result<Vec<Token>, String> {
        let mut program: Vec<Token> = Vec::new();

        for line in source.lines() {
            if line.starts_with("-") {
                continue;
            }
            let mut words: VecDeque<&str> = line.split_whitespace().collect();

            if words.len() == 0 {
                continue;
            }

            let mut token = Token {
                itype: IType::NULL,
                label: None,
            };

            let mut word = words.pop_front().unwrap();

            if word.ends_with(':') {
                token.label = Some(word[..word.len() - 1].to_string());

                let next = words.pop_front();

                if next.is_none() {
                    program.push(token);
                    continue;
                }

                word = next.unwrap();
            }

            match word {
                "mov" => {
                    if words.len() != 2 {
                        return Err(format!("Invalid number of arguments for 'mov': {}", line));
                    }

                    let arg1 = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;
                    let arg2 = ArgT::parse(&mut words, [ArgT::REG as u8, ArgT::MEM as u8].into())?;

                    token.itype = IType::MOV(arg1, arg2);
                }
                "add" => {
                    if words.len() != 2 {
                        return Err(format!("Invalid number of arguments for add: {}", line));
                    }

                    let arg1 = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;

                    let arg2 = ArgT::parse(&mut words, [ArgT::REG as u8, ArgT::MEM as u8].into())?;

                    token.itype = IType::ADD(arg1, arg2);
                }
                "sub" => {
                    if words.len() != 2 {
                        return Err(format!("Invalid number of arguments for sub: {}", line));
                    }

                    let arg1 = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;

                    let arg2 = ArgT::parse(&mut words, [ArgT::REG as u8, ArgT::MEM as u8].into())?;

                    token.itype = IType::SUB(arg1, arg2);
                }
                "cmp" => {
                    if words.len() != 2 {
                        return Err(format!("Invalid number of arguments for cmp: {}", line));
                    }

                    let arg1 = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;

                    let arg2 = ArgT::parse(&mut words, [ArgT::REG as u8, ArgT::MEM as u8].into())?;

                    token.itype = IType::CMP(arg1, arg2);
                }
                "jlt" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for jlt: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::LBL as u8].into())?;

                    token.itype = IType::JLT(arg);
                }

                "jgt" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for jgt: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::LBL as u8].into())?;

                    token.itype = IType::JGT(arg);
                }
                "jeq" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for jeq: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::LBL as u8].into())?;

                    token.itype = IType::JEQ(arg);
                }

                "jne" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for jne: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::LBL as u8].into())?;

                    token.itype = IType::JNE(arg);
                }

                "jmp" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for jmp: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::LBL as u8].into())?;

                    token.itype = IType::JMP(arg);
                }
                "run" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for run: {}", line));
                    }
                    let arg = ArgT::parse(&mut words, [ArgT::LBL as u8].into())?;
                    token.itype = IType::RUN(arg);
                }
                "ret" => {
                    if words.len() != 0 {
                        return Err(format!("Invalid number of arguments for ret: {}", line));
                    }

                    token.itype = IType::RET;
                }
                "die" => {
                    if words.len() != 0 {
                        return Err(format!("Invalid number of arguments for die: {}", line));
                    }

                    token.itype = IType::DIE;
                }
                "out" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for out: {}", line));
                    }

                    let arg = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;

                    token.itype = IType::OUT(arg);
                }
                "dbg" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for dbg: {}", line));
                    }

                    let arg = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;

                    token.itype = IType::DBG(arg);
                }
                "prt" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for prt: {}", line));
                    }

                    let arg = ArgT::parse(
                        &mut words,
                        [
                            ArgT::NUM as u8,
                            ArgT::REG as u8,
                            ArgT::MEM as u8,
                            ArgT::CHR as u8,
                        ]
                        .into(),
                    )?;

                    token.itype = IType::PRT(arg);
                }

                "@" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for @: {}", line));
                    }

                    let include_path =
                        source_dir.to_owned() + "/" + words.pop_front().unwrap() + ".dis";

                    let include_source = std::fs::read_to_string(&include_path);

                    if include_source.is_err() {
                        return Err(format!("Failed to read file: {}", include_path));
                    }
                    let include_source = include_source.unwrap();

                    let include_program = self.tokenize(source_dir, include_source)?;

                    program.extend(include_program);
                    continue;
                }

                "rdn" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for rdn: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::REG as u8, ArgT::MEM as u8].into())?;

                    token.itype = IType::RDN(arg);
                }

                "rdc" => {
                    if words.len() != 1 {
                        return Err(format!("Invalid number of arguments for rdc: {}", line));
                    }

                    let arg = ArgT::parse(&mut words, [ArgT::REG as u8, ArgT::MEM as u8].into())?;

                    token.itype = IType::RDC(arg);
                }

                _ => return Err(format!("Unknown instruction: {}", word)),
            }

            program.push(token);
        }

        Ok(program)
    }

    fn resolve_labels(&mut self) -> Result<(), String> {
        for (n, token) in self.program.iter().enumerate() {
            if let Some(label) = &token.label {
                if self.label_map.contains_key(label) {
                    return Err(format!("Duplicate label: {}", label));
                }
                self.label_map.insert(label.clone(), n);
            }
        }

        for token in self.program.iter_mut() {
            match &mut token.itype {
                IType::JLT(ArgT::LBL(l)) => {
                    if !self.label_map.contains_key(l) {
                        return Err(format!("Unknown label: {}", l));
                    }
                }
                IType::JGT(ArgT::LBL(l)) => {
                    if !self.label_map.contains_key(l) {
                        return Err(format!("Unknown label: {}", l));
                    }
                }
                IType::JEQ(ArgT::LBL(l)) => {
                    if !self.label_map.contains_key(l) {
                        return Err(format!("Unknown label: {}", l));
                    }
                }

                IType::JMP(ArgT::LBL(l)) => {
                    if !self.label_map.contains_key(l) {
                        return Err(format!("Unknown label: {}", l));
                    }
                }

                IType::RUN(ArgT::LBL(l)) => {
                    if !self.label_map.contains_key(l) {
                        return Err(format!("Unknown label: {}", l));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn load_program(&mut self, filename: &str) -> Result<(), ()> {
        let source = std::fs::read_to_string(filename);
        let source_dir = std::path::Path::new(filename)
            .parent()
            .unwrap()
            .to_str()
            .unwrap();

        if source.is_err() {
            println!("Error: Failed to read file: {}", filename);
            return Err(());
        }
        let source = source.unwrap();

        match self.tokenize(source_dir, source) {
            Ok(program) => {
                self.program = program;

                if let Ok(_) = self.resolve_labels() {
                    return Ok(());
                }

                self.program.clear();
                println!("Error: Failed to resolve labels!");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        return Err(());
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            let token = &self.program[self.pc];

            match &token.itype {
                IType::MOV(arg1, arg2) => {
                    let src = match arg1 {
                        ArgT::NUM(n) => *n,
                        ArgT::CHR(c) => *c as u8,
                        ArgT::REG(r_n) => self.registers[*r_n],
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => self.memory[*m_n],
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n]
                            }
                        },
                        other => unreachable!("{:?}", other),
                    };

                    match arg2 {
                        ArgT::REG(dst) => {
                            self.registers[*dst] = src;
                        }
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => {
                                self.memory[*m_n] = src;
                            }
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n] = src;
                            }
                        },
                        other => unreachable!("{:?}", other),
                    };
                }

                IType::ADD(arg1, arg2) => {
                    let src = match arg1 {
                        ArgT::NUM(n) => *n,
                        ArgT::CHR(c) => *c as u8,
                        ArgT::REG(r_n) => self.registers[*r_n],
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => self.memory[*m_n],
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n]
                            }
                        },
                        other => unreachable!("UNREACHABLE: {:?}", other),
                    };

                    match arg2 {
                        ArgT::REG(dst) => {
                            self.registers[*dst] = self.registers[*dst].overflowing_add(src).0;
                        }
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => {
                                self.memory[*m_n] = self.memory[*m_n].overflowing_add(src).0;
                            }
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n] = self.memory[m_n].overflowing_add(src).0;
                            }
                        },
                        other => unreachable!("UNREACHABLE: {:?}", other),
                    }
                }
                IType::SUB(arg1, arg2) => {
                    let src = match arg1 {
                        ArgT::NUM(n) => *n,
                        ArgT::CHR(c) => *c as u8,
                        ArgT::REG(r_n) => self.registers[*r_n],
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => self.memory[*m_n],
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n]
                            }
                        },
                        other => unreachable!("UNREACHABLE: {:?}", other),
                    };

                    match arg2 {
                        ArgT::REG(dst) => {
                            self.registers[*dst] = self.registers[*dst].overflowing_sub(src).0;
                        }
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => {
                                self.memory[*m_n] = self.memory[*m_n].overflowing_sub(src).0;
                            }
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n] = self.memory[m_n].overflowing_sub(src).0;
                            }
                        },
                        other => unreachable!("UNREACHABLE: {:?}", other),
                    }
                }

                IType::CMP(arg1, arg2) => {
                    let a = match arg1 {
                        ArgT::NUM(n) => *n,
                        ArgT::CHR(c) => *c as u8,
                        ArgT::REG(r_n) => self.registers[*r_n],
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => self.memory[*m_n],
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n]
                            }
                        },
                        other => unreachable!("UNREACHABLE: {:?}", other),
                    };

                    let b = match arg2 {
                        ArgT::NUM(n) => *n,
                        ArgT::CHR(c) => *c as u8,
                        ArgT::REG(r_n) => self.registers[*r_n],
                        ArgT::MEM(mem_t) => match mem_t {
                            MemT::ADR(m_n) => self.memory[*m_n],
                            MemT::REG(r_n) => {
                                let m_n = self.registers[*r_n] as usize;
                                self.memory[m_n]
                            }
                        },
                        other => unreachable!("UNREACHABLE: {:?}", other),
                    };

                    self.cmp = 0;
                    if a == b {
                        self.cmp |= CMP::EQ as u8;
                    }
                    if a < b {
                        self.cmp |= CMP::LT as u8;
                    }
                    if a > b {
                        self.cmp |= CMP::GT as u8;
                    }
                }

                IType::JLT(ArgT::LBL(l)) => {
                    if self.cmp & CMP::LT as u8 != 0 {
                        self.pc = self.label_map[l].overflowing_sub(1).0;
                    }
                }

                IType::JGT(ArgT::LBL(l)) => {
                    if self.cmp & CMP::GT as u8 != 0 {
                        self.pc = self.label_map[l].overflowing_sub(1).0;
                    }
                }

                IType::JEQ(ArgT::LBL(l)) => {
                    if self.cmp & CMP::EQ as u8 != 0 {
                        self.pc = self.label_map[l].overflowing_sub(1).0;
                    }
                }

                IType::JNE(ArgT::LBL(l)) => {
                    if self.cmp & CMP::EQ as u8 == 0 {
                        self.pc = self.label_map[l].overflowing_sub(1).0;
                    }
                }

                IType::JMP(ArgT::LBL(l)) => {
                    self.pc = self.label_map[l].overflowing_sub(1).0;
                }

                IType::RUN(ArgT::LBL(l)) => {
                    self.ret_stack[self.sp] = self.pc;
                    self.sp = self.sp.overflowing_add(1).0;
                    self.pc = self.label_map[l].overflowing_sub(1).0;
                }

                IType::RET => {
                    self.sp = self.sp.overflowing_sub(1).0;
                    self.pc = self.ret_stack[self.sp];
                }

                IType::DIE => {
                    return;
                }

                IType::OUT(arg) => match arg {
                    ArgT::NUM(n) => {
                        print!("{}", *n as char);
                    }

                    ArgT::CHR(c) => {
                        print!("{}", *c as char);
                    }

                    ArgT::REG(r_n) => {
                        print!("{}", self.registers[*r_n] as char);
                    }
                    ArgT::MEM(mem_t) => match mem_t {
                        MemT::ADR(m_n) => {
                            print!("{}", self.memory[*m_n] as char);
                        }
                        MemT::REG(r_n) => {
                            let m_n = self.registers[*r_n] as usize;
                            print!("{}", self.memory[m_n] as char);
                        }
                    },
                    other => unreachable!("UNREACHABLE: {:?}", other),
                },

                IType::DBG(arg) => match arg {
                    ArgT::NUM(n) => {
                        println!("DBG #: {}", *n);
                    }

                    ArgT::CHR(c) => {
                        println!("DBG #: {} ({})", *c, *c as u8);
                    }

                    ArgT::REG(r_n) => {
                        println!("DBG #{}: {}", *r_n, self.registers[*r_n]);
                    }
                    ArgT::MEM(mem_t) => match mem_t {
                        MemT::ADR(m_n) => {
                            println!("DBG &{}: {}", *m_n, self.memory[*m_n]);
                        }
                        MemT::REG(r_n) => {
                            let m_n = self.registers[*r_n] as usize;
                            println!("DBG &#{} (&{}): {}", *r_n, m_n, self.memory[m_n]);
                        }
                    },
                    other => unreachable!("UNREACHABLE: {:?}", other),
                },

                IType::PRT(arg) => match arg {
                    ArgT::NUM(n) => {
                        print!("{}", *n);
                    }

                    ArgT::CHR(c) => {
                        print!("{}", *c as u8);
                    }

                    ArgT::REG(r_n) => {
                        print!("{}", self.registers[*r_n]);
                    }
                    ArgT::MEM(mem_t) => match mem_t {
                        MemT::ADR(m_n) => {
                            print!("{}", self.memory[*m_n]);
                        }
                        MemT::REG(r_n) => {
                            let m_n = self.registers[*r_n] as usize;
                            print!("{}", self.memory[m_n]);
                        }
                    },
                    other => unreachable!("UNREACHABLE: {:?}", other),
                },

                IType::RDN(arg) => match arg {
                    ArgT::REG(r_n) => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();

                        let input = input.trim().parse::<u8>();

                        if input.is_err() {
                            println!("Error: Failed to parse input!");
                            return;
                        }

                        self.registers[*r_n] = input.unwrap();
                    }
                    ArgT::MEM(mem_t) => match mem_t {
                        MemT::ADR(m_n) => {
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            let input = input.trim().parse::<u8>();

                            if input.is_err() {
                                println!("Error: Failed to parse input!");
                                return;
                            }

                            self.memory[*m_n] = input.unwrap();
                        }
                        MemT::REG(r_n) => {
                            let m_n = self.registers[*r_n] as usize;

                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            let input = input.trim().parse::<u8>();

                            if input.is_err() {
                                println!("Error: Failed to parse input!");
                                return;
                            }

                            self.memory[m_n] = input.unwrap();
                        }
                    },
                    other => unreachable!("UNREACHABLE: {:?}", other),
                },

                IType::RDC(arg) => match arg {
                    ArgT::REG(r_n) => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();

                        let input = input.trim().chars().nth(0).unwrap() as u8;

                        self.registers[*r_n] = input;
                    }
                    ArgT::MEM(mem_t) => match mem_t {
                        MemT::ADR(m_n) => {
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();

                            let input = input.trim().chars().nth(0).unwrap() as u8;

                            self.memory[*m_n] = input;
                        }
                        MemT::REG(r_n) => {
                            let m_n = self.registers[*r_n] as usize;

                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();

                            let input = input.trim().chars().nth(0).unwrap() as u8;

                            self.memory[m_n] = input;
                        }
                    },
                    other => unreachable!("UNREACHABLE: {:?}", other),
                },

                IType::NULL => {}
                other => unreachable!("UNREACHABLE: {:?}", other),
            }

            self.pc = self.pc.overflowing_add(1).0;
        }
    }
}
