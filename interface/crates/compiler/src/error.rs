use std::{string::FromUtf8Error, sync::Arc};

use crate::extract::ExtractedOutput;

use super::Rule;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error writing tempfiles: {0}")]
    TempfileGeneration(std::io::Error),
    #[error("unable to invoke compiler or receive output: {0}")]
    CompilerInvocation(std::io::Error),
    #[error("compiler produced non utf-8 output {0}")]
    CompilerOutputIsNoUtf8(#[from] FromUtf8Error),
    #[error("parsing failed {0}")]
    Parsing(pest::error::Error<Rule>),
    #[error("abstracting the parse result failed")]
    ParsingAbstraction,
    #[error("the compiler provided the wrong number of output types, either too few or too many")]
    WrongNumberOfOutputs(Arc<str>, Vec<ExtractedOutput>),
}
