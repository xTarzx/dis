use crate::lexer::Token;

use crate::result::Result;

#[derive(Debug, Clone)]
pub enum Op {
    MOV(Token),
    ADD(Token),
    SUB(Token),
    CMP(Token),
    JLT(Token),
    JGT(Token),
    JEQ(Token),
    JNE(Token),
    JMP(Token),
    RUN(Token),
    RET(Token),
    DIE(Token),
    OUT(Token),
    PRT(Token),
    DBG(Token),
    INC(Token),
    RDN(Token),
    RDC(Token),
    RLN(Token),
    NOP,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub label: Option<Token>,
    pub op: Op,
    pub body: Vec<Token>,
}

impl Statement {
    pub fn parse(tokens: &mut Vec<Token>) -> Result<Option<Self>> {
        if tokens.is_empty() {
            return Ok(None);
        }

        let mut statement = Self {
            label: None,
            op: Op::NOP,
            body: Vec::new(),
        };

        let mut token = tokens.remove(0);

        if let Token::Label { .. } = &token {
            statement.label = Some(token.clone());
            token = tokens.remove(0);
        }

        match &token {
            Token::Keyword { value, loc } => match value.as_str() {
                "mov" => {
                    if tokens.len() < 2 {
                        eprintln!("{loc}: expected two operands for `mov`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;
                    let op2 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::MOV(token);
                    statement.body.push(op1);
                    statement.body.push(op2);

                    return Ok(Some(statement));
                }
                "add" => {
                    if tokens.len() < 2 {
                        eprintln!("{loc}: expected two operands for `add`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;
                    let op2 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::ADD(token);
                    statement.body.push(op1);
                    statement.body.push(op2);

                    return Ok(Some(statement));
                }
                "sub" => {
                    if tokens.len() < 2 {
                        eprintln!("{loc}: expected two operands for `sub`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;

                    let op2 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::SUB(token);
                    statement.body.push(op1);
                    statement.body.push(op2);

                    return Ok(Some(statement));
                }
                "cmp" => {
                    if tokens.len() < 2 {
                        eprintln!("{loc}: expected two operands for `cmp`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;
                    let op2 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::CMP(token);
                    statement.body.push(op1);
                    statement.body.push(op2);

                    return Ok(Some(statement));
                }
                "jlt" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `jlt`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected label identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::JLT(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "jgt" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `jgt`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected label identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::JGT(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "jeq" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `jeq`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected label identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::JEQ(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "jne" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `jne`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected label identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::JNE(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "jmp" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `jmp`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected label identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::JMP(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "run" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `run`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected label identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::RUN(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "ret" => {
                    statement.op = Op::RET(token);
                    return Ok(Some(statement));
                }
                "die" => {
                    statement.op = Op::DIE(token);
                    return Ok(Some(statement));
                }
                "out" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `out`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::OUT(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "prt" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `prt`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::PRT(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "dbg" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `dbg`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            Token::Char { .. } => Ok(op),
                            other => {
                                eprintln!("{loc}: expected number, register, memory or char, found `{other}`", loc = other.loc());
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::DBG(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "@" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `@`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Identifier { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected identifier, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::INC(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }
                "rdn" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `rdn`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::RDN(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }

                "rdc" => {
                    if tokens.len() < 1 {
                        eprintln!("{loc}: expected one operand for `rdc`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::RDC(token);
                    statement.body.push(op1);

                    return Ok(Some(statement));
                }

                "rln" => {
                    if tokens.len() < 2 {
                        eprintln!("{loc}: expected two operands for `rln`", loc = loc);
                        return Err(());
                    }

                    let op1 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    let op2 = {
                        let op = tokens.remove(0);
                        match op {
                            Token::Number { .. } => Ok(op),
                            Token::Register { .. } => Ok(op),
                            Token::Memory { .. } => Ok(op),
                            other => {
                                eprintln!(
                                    "{loc}: expected number, register or memory, found `{other}`",
                                    loc = other.loc()
                                );
                                Err(())
                            }
                        }
                    }?;

                    statement.op = Op::RLN(token);
                    statement.body.push(op1);
                    statement.body.push(op2);

                    return Ok(Some(statement));
                }

                _ => {
                    eprintln!("{loc}: unknown keyword `{value}`");
                    return Err(());
                }
            },
            other => {
                eprint!("{loc}: expected keyword", loc = other.loc());
                if statement.label.is_none() {
                    eprint!("or label")
                }
                eprint!(", found `{other}`");
                return Err(());
            }
        }
    }
}
