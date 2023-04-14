use crate::stream::Token;
use kotlin_ast::Span;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UnexpectedToken {
        expect: Token,
        actual: Token,
        span: Span,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken {
                expect,
                actual,
                span,
            } => {
                write!(
                    f,
                    "expected token {:?}, found {:?} (at {} to {})",
                    expect,
                    actual,
                    span.start(),
                    span.end()
                )?;
            }
        }

        Ok(())
    }
}