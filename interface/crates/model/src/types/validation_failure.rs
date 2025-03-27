use abstraction::impl_err_rule;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};

use super::Color;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, FromCppTypes, ParsingRulesDecl, ToCppTypes,
)]
pub enum ValidationFailure {
    CaptureDeclarationNotMatchingBoard,
    WrongPlayersTurn { player_that_should_move: Color },
    OnlyPawnsCanGetPromoted,
    ValueIsNoValidPromotionTargetKind,
    PromotionDeclarationDoesntMatchPieceMovement,
    PieceSquareCombinationNotFoundOnBoard,
    TargetIsOwnPiece,
    PieceCantMoveToThisSquare,
    MoveWouldLeavePlayerInCheck,
    CantCastleThroughCheck,
    CantCastleWhileInCheck,
    NoRookForCastlingFoundOnCorrectPosition,
    NoKingForCastlingFoundOnCorrectPosition,
    PlayerHasNotTheRightToCastleThisWay,
    NeededSquaresForCastlingAreNotFree,
}
impl_err_rule!(ValidationFailure, ErrValidationFailure);
