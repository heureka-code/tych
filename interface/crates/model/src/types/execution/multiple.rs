use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::{Container, SingleExecutionStep, Situation};

/// Combines all information generated when a list of moves gets executed on a provided `Situation`
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    FromCppTypes,
    ToCppTypes,
    Getters,
    ParsingRulesDecl,
)]
pub struct MultiExecutionResult {
    /// Container of all single steps, each representing an executed or failed move
    steps: Container<SingleExecutionStep>,
    /// The `Situation` after all (valid) moves got executed
    final_situation: Situation,
}

impl MultiExecutionResult {
    /// The situation the execution started with.
    ///
    /// If there are steps it uses the first one's situation.
    /// If there are no steps (empty) the `final_situation` is used.
    pub fn initial_situation(&self) -> &Situation {
        self.steps()
            .items()
            .first()
            .map(|step| step.before())
            .unwrap_or(self.final_situation())
    }
}
