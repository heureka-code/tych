use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
use derive_getters::Getters;
use derive_more::Debug;

/// Contains information about the board's meaning after a specific move got executed or rejected.
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    ParsingRulesDecl,
    FromCppTypes,
    Getters,
    ToCppTypes,
)]
pub struct SituationAnalysisResult {
    /// if the white player is in check after the step this analysis is for has finished.
    white_in_check: bool,
    /// if the black player is in check after the step this analysis is for has finished.
    black_in_check: bool,
    /// List of all moves the player could do after this step this analysis is for has finished.
    /// If the move was considered valid the other player's moves are listed.
    /// If it was invalid the same player's moves are listed.
    ///
    /// This field is an [Option] because this information doesn't always exist.
    /// As it is very costly to compute there are compiler flags (preprocessor macros) which
    /// control the generation of this data. The standard behaviour is that this field will be
    /// empty for all moves but the last. Therefore it can be used for deciding the end result of a
    /// game (stalemate, checkmate or none of these) without slowing down the application very
    /// heavily.
    possible_after_move: Option<crate::MoveContainer>,
}
