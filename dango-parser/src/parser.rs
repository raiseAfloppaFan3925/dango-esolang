
use super::tokenizer::{Token, TokenKind};
use dango_errors::{CompileError, CompileErrorKind};
use dango_runtime::instructions::*;

// This function just checks if all dangos are attached to sticks.
fn validate_tokens(tokens: &Vec<Token>) -> Option<Vec<CompileError>> {
    let mut errors = vec![];

    let mut last_is_dumpling = false;
    let mut last_dumpling_position = (0, 0);
    for token in tokens {
        match token.kind {
            TokenKind::Add | TokenKind::CharCodePoint | TokenKind::Divide | TokenKind::Equal | TokenKind::Float(_) |
                TokenKind::FunctionCall(_) | TokenKind::Greater | TokenKind::Int(_) | TokenKind::Jump | TokenKind::Length |
                TokenKind::Less | TokenKind::Multiply | TokenKind::NotEqual | TokenKind::Null | TokenKind::RawText(_) |
                TokenKind::Stringify | TokenKind::Subtract | TokenKind::ToFloat | TokenKind::ToInt | TokenKind::While
                => {
                    last_is_dumpling = true;
                    last_dumpling_position = (token.line, token.column);
                },
            TokenKind::Stick => if !last_is_dumpling {
                errors.push(CompileError::new(CompileErrorKind::OrphanedStick, token.line, token.column));
                last_is_dumpling = false;
            } else {
                last_is_dumpling = false;
            },
            TokenKind::Newline => if last_is_dumpling {
                last_is_dumpling = false;
                errors.push(CompileError::new(CompileErrorKind::OrphanedDumpling, last_dumpling_position.0, last_dumpling_position.1));
            },
            TokenKind::Eof => if last_is_dumpling {
                errors.push(CompileError::new(CompileErrorKind::OrphanedDumpling, 0, 0));
                break;
            }
            _ => if last_is_dumpling {
                last_is_dumpling = false;
                errors.push(CompileError::new(CompileErrorKind::OrphanedDumpling, last_dumpling_position.0, last_dumpling_position.1));
            },
        }
    }

    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}

// Surprisingly easier than I thought, the tokenization was the hardest part.
pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<CompileError>> {
    let mut prog = Program::new();

    let mut line = vec![];

    if let Some(errors) = validate_tokens(&tokens) {
        return Err(errors);
    }

    for token in tokens {
        match token.kind {
            TokenKind::Add => line.push(Instruction::Add),
            TokenKind::CharCodePoint => line.push(Instruction::CharFromCodePoint),
            TokenKind::Divide => line.push(Instruction::Divide),
            TokenKind::Equal => line.push(Instruction::Equal),
            TokenKind::Eat => line.push(Instruction::Eat),
            TokenKind::Fetch(count) => line.push(Instruction::Fetch(count)),
            TokenKind::Float(val) => line.push(Instruction::Float(val)),
            TokenKind::FunctionCall(name) => line.push(Instruction::FnCall(name)),
            TokenKind::Greater => line.push(Instruction::Greater),
            TokenKind::Int(val) => line.push(Instruction::Int(val)),
            TokenKind::Jump => line.push(Instruction::Jump),
            TokenKind::Length => line.push(Instruction::Length),
            TokenKind::Less => line.push(Instruction::Less),
            TokenKind::Multiply => line.push(Instruction::Multiply),
            TokenKind::Null => line.push(Instruction::Null), // how did I forget this?
            TokenKind::NotEqual => line.push(Instruction::NotEqual),
            TokenKind::RawText(raw_text) => line.push(Instruction::Text(raw_text)),
            TokenKind::Remove => line.push(Instruction::Remove),
            TokenKind::Skewer(count) => line.push(Instruction::Skewer(count)),
            TokenKind::Stringify => line.push(Instruction::Stringify),
            TokenKind::Subtract => line.push(Instruction::Subtract),
            TokenKind::ToFloat => line.push(Instruction::ToFloat),
            TokenKind::ToInt => line.push(Instruction::ToInt),
            TokenKind::While => line.push(Instruction::While),

            // These tokens were only for syntax and should be ignored during code generation
            TokenKind::Comment | TokenKind::Stick => (),

            // Couldn't split a list by predicate so I decided to do this
            TokenKind::Newline | TokenKind::Eof => {
                line.reverse();
                prog.add_line(line.clone());
                line.clear();
            }
        }
    }

    prog.add_line(vec![Instruction::Nop]);

    Ok(prog)
}
