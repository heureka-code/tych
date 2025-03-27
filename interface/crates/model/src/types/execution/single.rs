use abstraction::impl_res_rule;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

use super::{AnyMove, Container, Situation, SituationAnalysisResult, Unit, ValidationFailure};

impl_res_rule!(ResValidation, Unit, ValidationFailure);

/// Information about a single move being executed or rejected
///
/// The move and the Situation before and after execution are stored.
/// If the move was invalid these situations are equal.
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    FromCppTypes,
    ParsingRulesDecl,
    Getters,
    ToCppTypes,
)]
pub struct SingleExecutionStep {
    /// The `Situation` before the move got executed
    before: Situation,
    /// The move that should get executed in this step
    tried_move: AnyMove,
    /// The `Situation` after the (valid) move got executed. Equals `before` if the move was considered invalid
    after: Situation,
    /// Information generated about the board and it's meaning (if any players are in check after
    /// the move)
    analysis: SituationAnalysisResult,
    /// If the move was considered valid
    validation: Result<Unit, ValidationFailure>,
}

pub type SingleExecutionStepContainer = Container<SingleExecutionStep>;
abstraction::impl_struct_main_rule!(SingleExecutionStepContainer);
