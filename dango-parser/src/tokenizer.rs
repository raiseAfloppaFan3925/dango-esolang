
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Dumpling,               // (Hello, world!)
    NonDumpling,            // eat
    Stick,                  // ----
    Eof,
}

/// I'm calling it a token for a lack of a better word ("Span" exists but what would the tokenizer be named?)
/// This thing just stores the span so that the actual tokenizer knows what to do.
#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, text: &'a str) -> Self {
        Self { kind, text }
    }
}

pub fn tokenize(source: &str) -> Vec<Token<'_>> {
    let mut tokenizer = Tokenizer {
        source: source,
        chars: source.chars(),
        start: 0,
        current: 0,
    };

    std::iter::from_fn(|| {
        let span = tokenizer.consume_span();
        if span.kind == TokenKind::Eof {
            None
        } else {
            Some(span)
        }
    }).collect()
}

pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Chars<'a>,

    start: usize,
    current: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn consume_span(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        match self.first() {
            Some(c) => match c {
                '(' => self.consume_dumpling(),
                '-' => self.consume_stick(),
                _ => self.consume_misc(),
            }
            None => Token::new(TokenKind::Eof, ""),
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
        for i in 0..string.len() {
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
    fn consume_dumpling(&mut self) -> Token<'a> {
        while let Some(c) = self.first() {
            if c == '\n' {
                panic!("Dumplings cannot span multiple lines");
            }
            if c == ')' { break; }

            self.advance();
        }

        self.advance();

        Token::new(TokenKind::Dumpling, &self.source[self.start + 1..self.current - 1])
    }

    fn consume_stick(&mut self) -> Token<'a> {
        if !self.matches_string("----") {
            panic!("Invalid token");
        }
        self.advance();
        self.advance();
        self.advance();
        self.advance();
        Token::new(TokenKind::Stick, &self.source[self.start..self.current])
    }

    fn consume_misc(&mut self) -> Token<'a> {
        while let Some(c) = self.first() {
            match c {
                '(' | '-' => break,
                _ => (),
            }
            if c.is_whitespace() { break; }

            self.advance();
        }

        Token::new(TokenKind::NonDumpling, &self.source[self.start..self.current])
    }
}
