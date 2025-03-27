use colorterm::*;
use itertools::Itertools;
use model::{Color, Kind, PlacedPiece};

fn format_square_with_color(square_color: Color, pp: Option<&PlacedPiece>) -> String {
    pp.map(|pp: &PlacedPiece| {
        let is_king = pp.piece().kind() == &Kind::King;
        let correct_color: &str = if pp.piece().color() == &Color::White {
            &COLOR_BRIGHT_YELLOW
        } else {
            &COLOR_BRIGHT_BLUE
        };

        format!(
            "{}{}{}{COLOR_RESET}{STYLE_RESET}",
            STYLE_BOLD.only_if(is_king),
            correct_color,
            pp.piece().short(),
        )
    })
    .unwrap_or_else(|| {
        if square_color == Color::White {
            format!("{COLOR_BRIGHT_BLACK} {COLOR_RESET}")
        } else {
            format!("{COLOR_BRIGHT_BLACK}#{COLOR_RESET}")
        }
    })
}

impl super::PrettyPrint for model::Board {
    fn pretty(&self) -> String {
        let file_labels = "   a b c d e f g h";

        let b = self
            .rows()
            .rev()
            .map(|row| {
                let number = row.row_number();
                let line = row
                    .squares()
                    .map(|file| {
                        format_square_with_color(
                            file.square().color(),
                            self.get_by_square(file.square()),
                        )
                    })
                    .join(" ");
                format!("{number} |{line}| {number}")
            })
            .join("\n");

        format!("{file_labels}\n{b}\n{file_labels}")
    }
}
