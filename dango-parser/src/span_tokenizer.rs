//! The first pass of the tokenization phase. Since Dango is pretty weird syntactically, we first have to isolate
//! the stuff that we can actually tokenize.

use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum SpanKind {
    Dumpling,               // (Hello, world!)
    NonDumpling,            // eat
    Stick,                  // ----
    Eof,
}

#[derive(Debug)]
pub struct SpanToken<'a> {
    pub kind: SpanKind,
    pub text: &'a str,
}

impl<'a> SpanToken<'a> {
    pub fn new(kind: SpanKind, text: &'a str) -> Self {
        Self { kind, text }
    }
}

pub fn tokenize_into_spans(source: &str) -> Vec<SpanToken<'_>> {
    let mut tokenizer = SpanTokenizer {
        source: source,
        chars: source.chars(),
        start: 0,
        current: 0,
    };

    let mut spans: Vec<SpanToken<'_>> = std::iter::from_fn(|| {
        let span = tokenizer.consume_span();
        if span.kind == SpanKind::Eof {
            None
        } else {
            Some(span)
        }
    }).collect();

    spans.push(SpanToken::new(SpanKind::Eof, ""));

    spans
}

pub struct SpanTokenizer<'a> {
    source: &'a str,
    chars: Chars<'a>,

    start: usize,
    current: usize,
}

impl<'a> SpanTokenizer<'a> {
    pub fn consume_span(&mut self) -> SpanToken<'a> {
        self.skip_whitespace();
        self.start = self.current;

        match self.first() {
            Some(c) => match c {
                '(' => self.consume_dumpling(),
                '-' => self.consume_stick(),
                _ => self.consume_misc(),
            }
            None => SpanToken::new(SpanKind::Eof, ""),
        }
    }

    fn first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn second(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    fn third(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next()
    }

    fn fourth(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next();
        iter.next()
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
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.first() {
            if !c.is_whitespace() { break; }
            self.advance();
        }
    }

    // AAA WHY IS THIS SUCH A FUNNY NAME FOR A FUNCTION
    fn consume_dumpling(&mut self) -> SpanToken<'a> {
        while let Some(c) = self.first() {
            if c == '\n' {
                panic!("Dumplings cannot span multiple lines");
            }
            if c == ')' { break; }

            self.advance();
        }

        self.advance();

        SpanToken::new(SpanKind::Dumpling, &self.source[self.start + 1..self.current - 1])
    }

    fn consume_stick(&mut self) -> SpanToken<'a> {
        if !self.matches_string("----") {
            panic!("Invalid token");
        }
        self.advance();
        self.advance();
        self.advance();
        self.advance();
        SpanToken::new(SpanKind::Stick, &self.source[self.start..self.current])
    }

    fn consume_misc(&mut self) -> SpanToken<'a> {
        while let Some(c) = self.first() {
            match c {
                '(' | '-' => break,
                _ => (),
            }
            if c.is_whitespace() { break; }

            self.advance();
        }

        SpanToken::new(SpanKind::NonDumpling, &self.source[self.start..self.current])
    }
}
