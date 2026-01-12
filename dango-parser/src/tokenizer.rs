//! The second pass of tokenization, which tokenizes the spans from the span tokenizer stage.

use super::span_tokenizer::{SpanToken, SpanKind};
use dango_errors::{CompileError, CompileErrorKind};

#[derive(Debug)]
pub enum TokenKind {
    // Dumplings
    Add,                        // (+)
    CharCodePoint,              // ('c)
    Divide,                     // (/)
    Equal,                      // (=)
    Fetch(usize),               // fetch 3
    Float(f64),                 // 39.040141421
    FunctionCall(String),       // (:math-pi)
    Greater,                    // (>)
    Int(i64),                   // (12345)
    Jump,                       // (j)
    Length,                     // (len)
    Less,                       // (<)
    Multiply,                   // (*)
    NotEqual,                   // (!=)
    Null,                       // ()
    RawText(String),            // (Hello, world!)
    Stringify,                  // (')
    Subtract,                   // (-)
    ToInt,                      // (`)
    ToFloat,                    // (;)
    While,                      // (while)

    // Misc
    Eat,                        // eat
    Remove,                     // remove
    Skewer(u8),                 // skewer 5

    // More misc
    Comment,
    Eof,
    Newline,
    Stick,                      // ----
}

pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self {
            kind,
            line,
            column,
        }
    }
}

pub fn tokenize(span_tokens: Vec<SpanToken<'_>>) -> Result<Vec<Token>, Vec<CompileError>> {
    let mut tokenizer = Tokenizer {
        span_tokens,
        index: 0,
    };

    let mut errors = vec![];

    let mut toks: Vec<Token> = std::iter::from_fn(|| {
        let tok = tokenizer.tokenize_span();

        match tok {
            Ok(tok) => if matches!(tok.kind, TokenKind::Eof) {
                None
            } else {
                Some(Ok(tok))
            }
            Err(err) => Some(Err(err)),
        }
    })
        .filter_map(|tok| {
            // did you forget that pattern matching exists?
            match tok {
                Ok(tok) => Some(tok),
                Err(err) => {
                    errors.push(err);
                    None
                }
            }
        })
        .collect();

    toks.push(Token::new(TokenKind::Eof, 0, 0));

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(toks)
    }
}

pub struct Tokenizer<'a> {
    span_tokens: Vec<SpanToken<'a>>,
    index: usize,
}

#[allow(unreachable_patterns, unused)] // make the compiler and analyzer shut up
impl<'a> Tokenizer<'a> {
    pub fn tokenize_span(&mut self) -> Result<Token, CompileError> {
        let tok = match self.current() {
            Some(span) => match span.kind {
                SpanKind::Dumpling => self.tokenize_dumpling(),
                SpanKind::NonDumpling => self.tokenize_operation(),
                SpanKind::Newline => Ok(Token::new(TokenKind::Newline, span.line, span.column)),
                SpanKind::Stick => Ok(Token::new(TokenKind::Stick, span.line, span.column)),
                SpanKind::Eof => Ok(Token::new(TokenKind::Eof, span.line, span.column)),
                _ => todo!(),
            },
            None => Ok(Token::new(TokenKind::Eof, 0, 0)),
        };
        self.index += 1;
        tok
    }

    fn current(&self) -> Option<&SpanToken<'a>> {
        self.span_tokens.get(self.index)
    }

    fn first(&self) -> Option<char> {
        self.current().unwrap().text.chars().next()
    }

    fn second(&self) -> Option<char> {
        let mut iter = unsafe { self.current().unwrap_unchecked().text.chars() };
        iter.next();
        iter.next()
    }

    fn tokenize_dumpling(&mut self) -> Result<Token, CompileError> {
        // No need for if let because this function wouldn't have been called if there was no span
        let current = unsafe { self.current().unwrap_unchecked() };

        if current.text.len() == 0 { return Ok(Token::new(TokenKind::Null, current.line, current.column)); }

        if current.text.chars().next().unwrap() == ':' {
            let trimmed = &current.text[1..];
            return Ok(Token::new(TokenKind::FunctionCall(trimmed.to_string()), current.line, current.column));
        }

        match current.text {
            "+" => return Ok(Token::new(TokenKind::Add, current.line, current.column)),
            "/" => return Ok(Token::new(TokenKind::Divide, current.line, current.column)),
            "=" => return Ok(Token::new(TokenKind::Equal, current.line, current.column)),
            ">" => return Ok(Token::new(TokenKind::Greater, current.line, current.column)),
            "<" => return Ok(Token::new(TokenKind::Less, current.line, current.column)),
            "!=" => return Ok(Token::new(TokenKind::NotEqual, current.line, current.column)),
            "*" => return Ok(Token::new(TokenKind::Multiply, current.line, current.column)),
            "`" => return Ok(Token::new(TokenKind::ToInt, current.line, current.column)),
            ";" => return Ok(Token::new(TokenKind::ToFloat, current.line, current.column)),
            "'" => return Ok(Token::new(TokenKind::Stringify, current.line, current.column)),
            "'c" => return Ok(Token::new(TokenKind::CharCodePoint, current.line, current.column)),
            "-" => return Ok(Token::new(TokenKind::Subtract, current.line, current.column)),
            "j" => return Ok(Token::new(TokenKind::Jump, current.line, current.column)),
            "len" => return Ok(Token::new(TokenKind::Length, current.line, current.column)),
            "while" => return Ok(Token::new(TokenKind::While, current.line, current.column)),
            _ => (),
        }

        if let Ok(as_int) = current.text.parse::<i64>() {
            return Ok(Token::new(TokenKind::Int(as_int), current.line, current.column));
        }

        if let Ok(as_float) = current.text.parse::<f64>() {
            return Ok(Token::new(TokenKind::Float(as_float), current.line, current.column));
        }

        Ok(Token::new(TokenKind::RawText(current.text.to_string()), current.line, current.column))
    }

    fn tokenize_operation(&mut self) -> Result<Token, CompileError> {
        // Can't use self.current because the borrow checker isn't exactly sure what I'm using in there
        // `unwrap_unchecked` because this function would never be called if there wasn't a token here
        let current = unsafe { self.span_tokens.get(self.index).unwrap_unchecked() };

        match current.text {
            "eat" => Ok(Token::new(TokenKind::Eat, current.line, current.column)),
            "fetch" => {
                self.index += 1;
                let Some(count) = self.current() else {
                    return Err(CompileError::new(
                        CompileErrorKind::CustomError("expected number, found end of file".to_string()),
                        current.line,
                        current.column
                    ));
                };

                if count.kind != SpanKind::NonDumpling {
                    return Err(CompileError::new(
                        CompileErrorKind::CustomError("expected number for `fetch`".to_string()),
                        current.line,
                        current.column
                    ));
                }

                let Ok(count) = count.text.parse() else {
                    return Err(CompileError::new(
                        CompileErrorKind::CustomError("expected number for `fetch`".to_string()),
                        current.line,
                        current.column
                    ));
                };
                Ok(Token::new(TokenKind::Fetch(count), current.line, current.column))
            }
            "remove" => Ok(Token::new(TokenKind::Remove, current.line, current.column)),
            "skewer" => {
                self.index += 1;
                let Some(count) = self.current() else {
                    return Err(CompileError::new(
                        CompileErrorKind::CustomError("expected number, found end of file".to_string()),
                        current.line,
                        current.column
                    ));
                };

                if count.kind != SpanKind::NonDumpling {
                    return Err(CompileError::new(
                        CompileErrorKind::CustomError("expected number for `skewer`".to_string()),
                        current.line,
                        current.column
                    ));
                }

                let Ok(count) = count.text.parse() else {
                    return Err(CompileError::new(
                        CompileErrorKind::CustomError("expected number for `skewer`".to_string()),
                        current.line,
                        current.column
                    ));
                };
                Ok(Token::new(TokenKind::Skewer(count), current.line, current.column))
            }
            _ => Err(CompileError::new(CompileErrorKind::InvalidToken(current.text.to_string()), current.line, current.column)),
        }
    }
}
