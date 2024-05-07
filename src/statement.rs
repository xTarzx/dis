use crate::lexer::Token;
use crate::Result;

#[derive(Debug)]
pub struct Statement {
    pub label: Option<Token>,
    body: Vec<Token>,
}

impl Statement {
    pub fn parse(tokens: &mut Vec<Token>) -> Result<Option<Self>> {
        if tokens.is_empty() {
            return Ok(None);
        }

        let mut statement = Self {
            label: None,
            body: Vec::new(),
        };

        let mut token = tokens.remove(0);

        if let Token::Label { value, loc } = &token {
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

                    statement.body.push(token);
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

                    statement.body.push(token);
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

                    statement.body.push(token);
                    statement.body.push(op1);
                    statement.body.push(op2);

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

                    statement.body.push(token);
                    statement.body.push(op1);

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

                    statement.body.push(token);
                    statement.body.push(op1);

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
