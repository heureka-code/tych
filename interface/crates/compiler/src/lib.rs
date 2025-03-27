//! ```
//! use compiler::{Compiler, ManageCompilerFlags, ALWAYS_ADD_VALID_MOVES_INFORMATION};
//!
//! // Get path to sibling directory of interface named src
//! fn get_lib_path() -> Option<std::path::PathBuf> {
//!     Some(std::env::current_dir().ok()?.parent()?.join("src").into())
//! }
//!
//! fn main() {
//!     // Get path of C++ source libraries (folder containing core/ and chess/)
//!     let lib_path = get_lib_path().unwrap();
//!     let comp = Compiler::new()
//!         .include(lib_path)
//!         // This will be set as preprocessor macro and enables the computation of valid moves
//!         // after each step. Be carful, this extremly slows down the application
//!         .define(ALWAYS_ADD_VALID_MOVES_INFORMATION);
//! }
//! ```
//!
//! `comp.parsed` can then be used to invoke the compiler with a String that represents a C++
//! template type expression. The output type is generic and decides how parsing should interprete
//! the output of the compiler.

mod error;
mod extract;
mod flags;
mod invoke;
mod run_command;

#[allow(unused)]
pub use self::{
    error::Error,
    invoke::{Compiler, Invoke},
};

// pub use model::types::*;

pub use extract::ExtractedOutput;
use flags::ExtraCompilerFlags;
pub use flags::ManageCompilerFlags;
use model::ChessParser;
pub use model::Rule;
use run_command::run_gpp;

pub use flags::{ALWAYS_ADD_VALID_MOVES_INFORMATION, REMOVE_VALID_MOVES_INFORMATION_FOR_LAST_STEP};
