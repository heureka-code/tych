mod analysis_result;
mod board;
mod board_iteration;
mod castling_right;
mod flags;
mod situation;

pub use analysis_result::SituationAnalysisResult;
pub use board::Board;
pub use board_iteration::{IterFileOfRow, IterRow};
pub use castling_right::CastlingRight;
pub use flags::{CastlingRightFlags, SituationFlags};
pub use situation::Situation;

use super::{Color, MoveContainer, PlacedPiece, Rule, Square};
