//! The first pass of the tokenization phase. Since Dango is pretty weird syntactically, we first have to isolate
//! the stuff that we can actually tokenize.

use std::str::Chars;

use dango_errors::{CompileError, CompileErrorKind};

#[derive(Debug, PartialEq)]
pub enum SpanKind {
    Dumpling,               // (Hello, world!)
    Newline,
    NonDumpling,            // eat
    Stick,                  // ----
    Eof,
}

#[derive(Debug)]
pub struct SpanToken<'a> {
    pub kind: SpanKind,
    pub text: &'a str,
    pub line: usize,
    pub column: usize,
}

impl<'a> SpanToken<'a> {
    pub fn new(kind: SpanKind, text: &'a str, line: usize, column: usize) -> Self {
        Self { kind, text, line, column }
    }
}

pub fn tokenize_into_spans(source: &str) -> Result<Vec<SpanToken<'_>>, Vec<CompileError>> {
    let mut tokenizer = SpanTokenizer {
        source: source,
        chars: source.chars(),
        start_pos: Position::new(1, 1),
        current_pos: Position::new(1, 1),
        start: 0,
        current: 0,
    };

    let mut errors = vec![];

    let mut spans: Vec<SpanToken<'_>> = std::iter::from_fn(|| {
        let span = tokenizer.consume_span();

        match span {
            Ok(span) => if span.kind == SpanKind::Eof {
                None
            } else {
                Some(Ok(span))
            },
            Err(err) => Some(Err(err)),
        }
    })
        .filter_map(|span| {
            if let Ok(span) = span {
                Some(span)
            } else {
                // we are 100% sure that span is an error
                // so this unsafe is actually safe
                errors.push(unsafe { span.unwrap_err_unchecked() });
                None
            }
        })
        .collect();

    spans.push(SpanToken::new(SpanKind::Eof, "", tokenizer.start_pos.line, tokenizer.start_pos.column));

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(spans)
    }
}

// convenience struct, not really useful
#[derive(Debug, Clone, Copy)]
pub(crate) struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

pub struct SpanTokenizer<'a> {
    source: &'a str,
    chars: Chars<'a>,

    start_pos: Position,
    current_pos: Position,

    start: usize,
    current: usize,
}

impl<'a> SpanTokenizer<'a> {
    pub fn consume_span(&mut self) -> Result<SpanToken<'a>, CompileError> {
        self.skip_whitespace();
        self.start = self.current;
        self.start_pos = self.current_pos;

        match self.first() {
            Some(c) => match c {
                '\n' => {
                    self.advance();
                    Ok(SpanToken::new(SpanKind::Newline, "", self.start_pos.line, self.start_pos.column))
                },
                '(' => self.consume_dumpling(),
                '-' => self.consume_stick(),
                _ => self.consume_misc(),
            }
            None => Ok(SpanToken::new(SpanKind::Eof, "", self.start_pos.line, self.start_pos.column)),
        }
    }

    fn first(&self) -> Option<char> {
        self.chars.clone().next()
    }
    
    fn matches_string(&self, string: &str) -> bool {
        let mut iter = self.chars.clone();
        let mut iter_str = string.chars();
        for _ in 0..string.len() {
            if iter.next() != iter_str.next() { return false; }
        }

        true
    }

    fn advance(&mut self) -> Option<char> {
        if self.current > self.source.len() {
            return None;
        }

        let c = self.chars.next();

        self.current += 1;
        self.current_pos.column += 1;

        if c == Some('\n') {
            self.newline();
        }

        c
    }

    fn newline(&mut self) {
        self.current_pos.line += 1;
        self.current_pos.column = 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.first() {
            if c == '\n' || c == ';' || !c.is_whitespace() { break; } 

            self.advance();
        }
    }

    // AAA WHY IS THIS SUCH A FUNNY NAME FOR A FUNCTION
    fn consume_dumpling(&mut self) -> Result<SpanToken<'a>, CompileError> {
        while let Some(c) = self.first() {
            if c == '\n' {
                return Err(CompileError::new(CompileErrorKind::MultilineDumpling, self.start_pos.line, self.start_pos.column));
            }
            if c == ')' { break; }

            self.advance();
        }

        if let Some(last) = self.first() && last == ')' {
            self.advance();
        } else {
            return Err(CompileError::new(CompileErrorKind::UnterminatedDumpling, self.start_pos.line, self.start_pos.column));
        }

        Ok(SpanToken::new(SpanKind::Dumpling, &self.source[self.start + 1..self.current - 1], self.start_pos.line, self.start_pos.column))
    }

    fn consume_stick(&mut self) -> Result<SpanToken<'a>, CompileError> {
        if !self.matches_string("----") {
            self.advance();
            return Err(CompileError::new(CompileErrorKind::InvalidToken("-".to_string()), self.start_pos.line, self.start_pos.column));
        }

        self.advance();
        self.advance();
        self.advance();
        self.advance();
        Ok(SpanToken::new(SpanKind::Stick, &self.source[self.start..self.current], self.start_pos.line, self.start_pos.column))
    }

    fn consume_misc(&mut self) -> Result<SpanToken<'a>, CompileError> {
        while let Some(c) = self.first() {
            match c {
                '(' | '-' => break,
                _ => (),
            }
            if c.is_whitespace() { break; }

            self.advance();
        }

        Ok(SpanToken::new(SpanKind::NonDumpling, &self.source[self.start..self.current], self.start_pos.line, self.start_pos.column))
    }
}
