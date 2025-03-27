use abstraction_derive::ToCppTypes;
use abstraction_derive::{FromCppTypes, ParsingRulesDecl};
use derive_getters::Getters;
use derive_more::{Debug, Display};

use super::{Color, File, Row};

/// A square of a chess board, combination of `File` and `Row`
#[derive(
    Debug,
    Display,
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
#[display("{file}{row}")]
#[debug("{file}{row}")]
pub struct Square {
    file: File,
    row: Row,
}
impl Square {
    pub fn new(file: File, row: Row) -> Self {
        Self { file, row }
    }
    /// The color of the square, is calculated using file and row indices
    pub fn color(&self) -> Color {
        if (self.file.index() + self.row.index()) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SquareCantBeBuildFromTuple<EF: std::error::Error, ER: std::error::Error> {
    File(EF),
    Row(ER),
}

impl<F, R> TryFrom<(F, R)> for Square
where
    F: TryInto<File>,
    R: TryInto<Row>,
    <F as TryInto<File>>::Error: std::error::Error,
    <R as TryInto<Row>>::Error: std::error::Error,
{
    type Error =
        SquareCantBeBuildFromTuple<<F as TryInto<File>>::Error, <R as TryInto<Row>>::Error>;

    fn try_from((f, r): (F, R)) -> Result<Self, Self::Error> {
        let file: File = f.try_into().map_err(SquareCantBeBuildFromTuple::File)?;
        let row: Row = r.try_into().map_err(SquareCantBeBuildFromTuple::Row)?;
        Ok(Square { file, row })
    }
}
