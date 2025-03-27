use derive_getters::Getters;

use crate::{LetterAToH, Number1to8, PlacedPiece, Square};

use super::Board;

#[derive(Debug, Getters)]
pub struct IterRow<'a> {
    board: &'a Board,
    row_number: Number1to8,
}

#[derive(Debug, Getters)]
pub struct IterFileOfRow<'a> {
    board: &'a Board,
    square: Square,
    piece_on_it: Option<PlacedPiece>,
}

impl Board {
    pub fn rows<'a>(&'a self) -> impl DoubleEndedIterator<Item = IterRow<'a>> {
        Number1to8::forward_iter().map(|row_number| IterRow {
            board: self,
            row_number,
        })
    }
}

impl<'a> IterRow<'a> {
    pub fn squares(&'a self) -> impl DoubleEndedIterator<Item = IterFileOfRow<'a>> {
        let row_number = self.row_number.clone();
        LetterAToH::forward_iter().map(move |file_letter| {
            let square = Square::new(file_letter.into(), row_number.into());
            IterFileOfRow {
                board: self.board,
                piece_on_it: self
                    .board
                    .pieces()
                    .iter()
                    .find(|pp| pp.square() == &square)
                    .cloned(),
                square,
            }
        })
    }
}

impl FromIterator<PlacedPiece> for Board {
    fn from_iter<T: IntoIterator<Item = PlacedPiece>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}
