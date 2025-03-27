use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::Deref, sync::Arc};

lazy_static! {
    static ref PATTERN: Regex =
        Regex::new("[{]aka [‘»]([^‘«»’]*)[’«][}] [^‘«»’]+ [‘»]([^«»’]*)[’«]").unwrap();
}

/// Represents one output that was extracted from the compiler error
///
/// Implements [Deref] so it can be used as the calcualated type (as [`Arc<str>`])
///
/// The compiler abstraction uses the following regular expression's matches for extracted outputs:
/// ```regex
/// [{]aka [‘»]([^‘«»’]*)[’«][}] [^‘«»’]+ [‘»]([^«»’]*)[’«]
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ExtractedOutput {
    label: Arc<str>,
    value: Arc<str>,
}
impl Deref for ExtractedOutput {
    type Target = Arc<str>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl ExtractedOutput {
    /// The target type of the failed conversion, mostly unused
    pub fn label(&self) -> Arc<str> {
        self.label.clone()
    }
    /// The source type of the tried conversion, contains the interesting result
    pub fn value(&self) -> Arc<str> {
        self.value.clone()
    }
}

/// Uses a regular expession to find the parts of the compiler error that represent the calcuated
/// result types.
///
/// Because each output is generated through a type missmatch error there is also a type that was
/// the tried target of the conversion. This type is considered a label and included in the output.
///
/// The function returns an iterator of all matches
pub(super) fn get_output_lines(comp: &str) -> impl Iterator<Item = ExtractedOutput> + '_ {
    PATTERN
        .captures_iter(comp)
        .map(|c| c.extract())
        .map(|(_, [value, name])| ExtractedOutput {
            label: name.into(),
            value: value.into(),
        })
}
