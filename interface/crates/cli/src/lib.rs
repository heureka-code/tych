use std::path::PathBuf;

pub use clap::{Args, Parser, ValueEnum};

#[derive(Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CalculateValidMoves {
    /// Calculate the valid moves for the next player after every executed step.
    /// This can slow down the application very heavily.
    Always,
    /// Calculate the valid moves only for the last executed step.
    Last,
    /// Never calculate the valid moves for a player.
    Never,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ShowBoard {
    /// Print only the initial board before any move is executed
    Initial,
    /// Print the initial board, the board after the last executed move and all inbetween
    Always,
    /// Never print the board
    Never,
    /// Print the initial board and the board after the last executed move
    Outer,
    /// Print everything but the initial board
    Steps,
    /// Print the board only after the last executed move
    Last,
}
impl ShowBoard {
    pub fn show_initial_board(&self) -> bool {
        matches!(self, Self::Always | Self::Outer | Self::Initial)
    }
    pub fn show_last_board(&self) -> bool {
        matches!(self, Self::Always | Self::Outer | Self::Last | Self::Steps)
    }
    pub fn show_intermediate_board(&self) -> bool {
        matches!(self, Self::Always | Self::Steps)
    }
}

// NOTE: The version is taken from the cli crate, not from the main binary!
#[derive(Parser, Debug)]
#[command(name="tych", version, about, long_about = None)]
pub struct Cli {
    // #[command(flatten)]
    // pub source: InputSource,
    /// Decides how often the graphical representation of the board should get printed.
    #[arg(long, short='b', value_enum, default_value_t = ShowBoard::Always)]
    pub print_board: ShowBoard,

    /// Decides for which steps the valid moves should be calculated.
    /// The default is highly recommended!
    #[arg(long, value_enum, default_value_t = CalculateValidMoves::Last)]
    pub valid_moves: CalculateValidMoves,

    /// If provided the file with the provided name will be read instead of using standard input
    /// as source for moves to execute
    #[arg(long, short = 'f')]
    pub file: Option<PathBuf>,
    #[arg(long, hide = true)]
    pub input_file_relative_to: Option<PathBuf>,

    /// File to write the raw output of the C++ compiler into
    #[arg(long, short = 'o')]
    pub write_raw_compiler_output: Option<PathBuf>,
    #[arg(long, hide = true)]
    pub raw_file_relative_to: Option<PathBuf>,
}

/*
#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct InputSource {
    #[arg(long)]
    pub moves: Option<String>,

    #[arg(long)]
    pub file: Option<PathBuf>,
}
*/
