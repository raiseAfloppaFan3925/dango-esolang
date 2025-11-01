
#[derive(PartialEq)]
pub enum DangoError {
    Compile(Vec<CompileError>),
    Runtime(RuntimeError),
}

impl std::fmt::Display for DangoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compile(errors) => {
                for err in errors {
                    writeln!(f, "{err}")?;
                }
                Ok(())
            },
            Self::Runtime(err) => write!(f, "{err}"),
        }
    }
}

impl std::fmt::Debug for DangoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(PartialEq)]
pub enum CompileErrorKind {
    CustomError(String),
    InvalidToken(String),
    MultilineComment,
    MultilineDumpling,
    OrphanedDumpling,
    OrphanedStick,
    UnterminatedComment,
    UnterminatedDumpling,
}

#[derive(PartialEq)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub line: usize,
    pub column: usize,
}

impl CompileError {
    pub fn new(kind: CompileErrorKind, line: usize, column: usize) -> Self {
        Self {
            kind,
            line,
            column,
        }
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Display for CompileErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomError(msg) => write!(f, "{}", msg),
            Self::InvalidToken(tok) => write!(f, "invalid token '{}'", tok),
            Self::MultilineComment => write!(f, "due to complexities, multiline comments are not supported"),
            Self::MultilineDumpling => write!(f, "dumplings cannot span multiple lines"),
            Self::OrphanedDumpling => write!(f, "dumpling(s) must be connected to sticks"),
            Self::OrphanedStick => write!(f, "sticks must hold dango"),
            Self::UnterminatedComment => write!(f, "comments must be closed with a ':;'"),
            Self::UnterminatedDumpling => write!(f, "dumplings must be closed with a ')'"),
        }
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line > 0 && self.column > 0{
            write!(f, "error at {}:{}: {}", self.line, self.column, self.kind)
        } else {
            write!(f, "error at end: {}", self.kind)
        }
    }
}

#[derive(PartialEq)]
pub enum RuntimeError {
    CustomError(String),
    IncorrectOperationTypes(String),
    NonexistentFunction(String),
    NotACodePoint(Option<i64>),
    StackUnderflow,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomError(message) => write!(f, "{}", message),
            Self::IncorrectOperationTypes(op) => write!(f, "error: incorrect {} types", op),
            Self::NonexistentFunction(name) => write!(f, "error: function `:{}` does not exist", name),
            Self::NotACodePoint(val) => if let Some(val) = val {
                write!(f, "error: integer {} is not a code point", val)
            } else {
                write!(f, "error: value is not a code point")
            }
            Self::StackUnderflow => write!(f, "error: the stack cannot be popped from when it is already empty"),
        }
    }
}
