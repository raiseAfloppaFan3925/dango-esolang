//! The second pass of tokenization, which tokenizes the spans from the span tokenizer stage.

use super::span_tokenizer::{SpanToken, SpanKind};

#[derive(Debug)]
pub enum Token {
    // Dumplings
    Float(f64),                 // 39.040141421
    FunctionCall(String),       // (:math-pi)
    Int(i64),                   // (12345)
    Jump,                       // (j)
    Null,                       // ()
    RawText(String),            // (Hello, world!)
    Stringify,                  // (')
    StringifyRawUtf32,          // ('b)

    // Misc
    Eat,                        // eat

    // More misc
    Stick,                      // ----
}

pub fn tokenize(span_tokens: Vec<SpanToken<'_>>) -> Vec<Token> {
    let mut tokenizer = Tokenizer {
        span_tokens,
        index: 0,
    };

    std::iter::from_fn(|| {
        tokenizer.tokenize_span()
    }).collect()
}

pub struct Tokenizer<'a> {
    span_tokens: Vec<SpanToken<'a>>,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize_span(&mut self) -> Option<Token> {
        let tok = match self.current() {
            Some(span) => match span.kind {
                SpanKind::Dumpling => Some(self.tokenize_dumpling()),
                SpanKind::NonDumpling => Some(self.tokenize_operation()),
                SpanKind::Stick => Some(Token::Stick),
                SpanKind::Eof => None,
                _ => todo!(),
            },
            None => None,
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
        let mut iter = self.current().unwrap().text.chars();
        iter.next();
        iter.next()
    }

    fn tokenize_dumpling(&mut self) -> Token {
        let Some(current) = self.current() else {
            panic!();
        };

        if current.text.len() == 0 { return Token::Null; }

        if current.text.chars().next().unwrap() == ':' {
            let trimmed = &current.text[1..];
            return Token::FunctionCall(trimmed.to_string());
        }

        if current.text == "j" { return Token::Jump; }

        if let Ok(as_int) = current.text.parse::<i64>() {
            return Token::Int(as_int);
        }

        if let Ok(as_float) = current.text.parse::<f64>() {
            return Token::Float(as_float);
        }

        Token::RawText(current.text.to_string())
    }

    fn tokenize_operation(&mut self) -> Token {
        let Some(current) = self.current() else {
            panic!();
        };

        match current.text {
            "eat" => Token::Eat,
            _ => todo!(),
        }
    }
}
