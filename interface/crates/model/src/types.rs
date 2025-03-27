mod any_move;
pub mod board;
mod boolean;
mod castling_move;
mod color;
mod container;
mod execution;
mod normal_move;
mod piece_kind;
mod pieces;
pub mod squares;
mod unit;
mod validation_failure;

#[allow(unused)]
pub use self::{
    any_move::{AnyMove, MoveContainer},
    board::{Board, Situation, SituationAnalysisResult},
    castling_move::{CastlingKind, CastlingMove, ColoredCastlingMove},
    color::Color,
    container::Container,
    execution::{
        execute_on_initial, MultiExecutionResult, SingleExecutionStep, SingleExecutionStepContainer,
    },
    normal_move::NormalMove,
    piece_kind::Kind,
    pieces::{ColoredPiece, PlacedPiece},
    squares::{File, LetterAToH, Number1to8, Row, Square},
    unit::Unit,
    validation_failure::ValidationFailure,
};

pub use abstraction::ToCppTypes;

use super::Rule;
