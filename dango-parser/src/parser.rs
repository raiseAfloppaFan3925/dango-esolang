
use super::tokenizer::Token;
use dango_runtime::{instructions::*, runtime::Runtime};

// This function just checks if all dangos are attached to sticks.
fn validate_tokens(tokens: &Vec<Token>) -> bool {
    let mut last_is_dumpling = false;
    for token in tokens {
        match token {
            Token::Float(_) | Token::FunctionCall(_) | Token::Int(_) |
                Token::Jump | Token::Null | Token::RawText(_) | Token::Stringify |
                Token::StringifyRawUtf32 => last_is_dumpling = true,
            Token::Stick => {
                if last_is_dumpling {
                    last_is_dumpling = false;
                } else {
                    panic!("Sticks must hold dango");
                }
            },
            _ => last_is_dumpling = false,
        }
    }

    if last_is_dumpling {
        panic!("Dumplings must be held together by sticks")
    }

    true
}

// Surprisingly easier than I thought, the tokenization was the hardest part.
pub fn parse(tokens: Vec<Token>) -> Program {
    let mut prog = Program::new();

    let mut line = vec![];

    if !validate_tokens(&tokens) {
        panic!();
    }

    for token in tokens {
        match token {
            Token::Eat => line.push(Instruction::Other(Operation::Eat)),
            Token::Float(val) => line.push(Instruction::Dumpling(Dumpling::Float(val))),
            Token::FunctionCall(name) => line.push(Instruction::Dumpling(Dumpling::FnCall(name))),
            Token::Int(val) => line.push(Instruction::Dumpling(Dumpling::Int(val))),
            Token::RawText(raw_text) => line.push(Instruction::Dumpling(Dumpling::Text(raw_text))),
            Token::Stringify => line.push(Instruction::Dumpling(Dumpling::Stringify)),

            Token::Stick => (), // These tokens were only for syntax and should be ignored during code generation

            // Couldn't split a list by predicate so I decided to do this
            Token::Newline | Token::Eof => {
                line.reverse();
                prog.add_line(line.clone());
                line.clear();
            },
            _ => todo!(),
        }
    }

    prog
}
