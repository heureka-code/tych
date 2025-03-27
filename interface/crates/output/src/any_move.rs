use crate::PrettyPrint;

impl PrettyPrint for model::NormalMove {
    fn pretty(&self) -> String {
        format!("{:?}", self)
    }
}

impl PrettyPrint for model::AnyMove {
    fn pretty(&self) -> String {
        match self {
            Self::Normal(n) => n.pretty(),
            Self::Castling(c) => c.castling_kind().as_short_move_str().to_string(),
            Self::ColoredCastling(cc) => cc
                .castling_move()
                .castling_kind()
                .as_short_move_str()
                .to_string(),
        }
    }
}
