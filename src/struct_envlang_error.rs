use std::fmt;
use core::fmt::Display;
use failure::{Backtrace, Context, Fail};

// ------------------------------------------------------------------------------------
// Error
// ------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct EnvLangError {
    pub inner: Context<EnvLangErrorKind>,
}

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum EnvLangErrorKind {
    SpaceNotAllow,
    TooManyPoint,
    TooManyUnderscore,
    TooManyArobase,
}

impl fmt::Display for EnvLangErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnvLangErrorKind::SpaceNotAllow => { write!(f, "space not allow") },
            EnvLangErrorKind::TooManyPoint => { write!(f, "split by point return more than 2 chunks") },
            EnvLangErrorKind::TooManyUnderscore => { write!(f, "split by underscore return more than 2 chunks") },
            EnvLangErrorKind::TooManyArobase => { write!(f, "split by arobase return more than 2 chunks") },
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Fail for EnvLangError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for EnvLangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl EnvLangError {
    pub fn kind(&self) -> &EnvLangErrorKind {
        &*self.inner.get_context()
    }
}

impl From<EnvLangErrorKind> for EnvLangError {
    fn from(kind: EnvLangErrorKind) -> EnvLangError {
        EnvLangError { inner: Context::new(kind)}
    }
}
