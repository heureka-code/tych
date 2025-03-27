use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "chess.pest"]
pub struct ChessParser;
