//! This is the main application code for the Rust interface of this project.
//!
//! It uses the [cli] submodule to parse command line arguments.
//!
//! Then it will read a list of moves (separated by `,` (comma), `\n` (newline), or ` ` (space))
//! from standard input (or from the file provided with the [cli::Cli::file] flag)
//! and will parse each of the list elements as [model::AnyMove].
//! The invalid parse results get removed and the valid ones get allocated into [model::MoveContainer].
//! This container will then get passed into an [model::execute_on_initial] on which
//! [abstraction::ToCppTypes::to_cpp_types] is called.
//!
//! The resulting string then gets passed to the C++ compiler and then the compiler error output will be
//! parsed by [compiler::Compiler::parsed] into an object of type [model::MultiExecutionResult].
//! The generated structure then will get printed by [output::print_multiple_steps] where the flags from
//! [cli::Cli] control the exact behaviour of the output.
//!
//! ## Crates
//!
//! The functionality is divided into multiple submodules (crates):
//!
//! * [abstraction]: Contains the interfaces (traits) a type needs to implement to be equivalent to a
//! C++ type
//! * [abstraction_derive]: Implementing the traits from [abstraction] requires a lot of
//! boilerplate. To simplify the implementation this crate defines macros which write the needed
//! implementations considering the structure of the types.
//! * [model]: Here the different types that represent C++ counterparts are actually defined.
//! Also with [pest](https://docs.rs/pest/latest/pest/) a grammar file is used to construct a
//! parser for C++ template type expressions.
//! * [compiler]: This abstracts the needed interactions with the C++ compiler and also uses the
//! traits from [abstraction] to allow extracting of output from compiler errors and parsing it as
//! types from [model].
//! * [colorterm]: Here some constants for terminal style and color manipulation are defined. Those
//! will act like blank strings when the application is used in non-interactive context so no weird
//! characters will show up while piping.
//! * [cli]: Here a very basic command line interface is defined using [clap](https://docs.rs/clap/latest/clap/).
//! * [output]: Via additional functions this submodule defines how the structure from [model]
//! should get printed depending on the flags set in [cli::Cli]

use std::{io::Read, path::PathBuf, process::exit};

use compiler::{Compiler, Invoke, ManageCompilerFlags};
use model::{self, execute_on_initial, MoveContainer, ToCppTypes};
use output::print_multiple_steps;

fn get_lib_path() -> Option<std::path::PathBuf> {
    Some(std::env::current_dir().ok()?.parent()?.join("src").into())
}

fn get_compiler_flags(c: &cli::CalculateValidMoves) -> Vec<&'static str> {
    use cli::CalculateValidMoves as C;
    match c {
        C::Last => vec![],
        C::Never => vec![compiler::REMOVE_VALID_MOVES_INFORMATION_FOR_LAST_STEP],
        C::Always => vec![compiler::ALWAYS_ADD_VALID_MOVES_INFORMATION],
    }
}

fn construct_path(relative_to: &Option<PathBuf>, filename: &Option<PathBuf>) -> Option<PathBuf> {
    if let Some(filename) = filename {
        Some(if filename.is_absolute() {
            filename.clone()
        } else {
            let relative_to = relative_to
                .clone()
                .unwrap_or_else(|| std::env::current_dir().ok().expect("A cwd to be valid"));
            relative_to.join(filename)
        })
    } else {
        None
    }
}

fn main() {
    use cli::Parser;
    let args = cli::Cli::parse();

    let lib_path = get_lib_path().unwrap();
    let comp = Compiler::new()
        .include(lib_path)
        .define_multiple(get_compiler_flags(&args.valid_moves));

    let mut data = String::new();
    if let Some(ref input_file) = construct_path(&args.input_file_relative_to, &args.file) {
        match std::fs::read_to_string(input_file) {
            Ok(content) => data = content,
            Err(err) => {
                eprintln!("There was an error reading the provided file: {err:?}");
                exit(39);
            }
        }
    } else {
        let mut stdin = std::io::stdin();
        let _ = stdin.read_to_string(&mut data);
    }

    let moves: MoveContainer = data
        .split([' ', '\n', ','])
        .flat_map(|s| s.parse())
        .collect();
    let expr = execute_on_initial::new(moves).to_cpp_types();

    let invocation: Invoke = comp.invoke(expr).expect("Working g++ compiler and interaction");

    if let Some(ref path) =
        construct_path(&args.raw_file_relative_to, &args.write_raw_compiler_output)
    {
        let out = std::fs::write(path, invocation.raw_text().as_bytes());
        if let Err(err) = out {
            eprintln!("Failed to write raw compiler output into {path:?}: {err:?}");
        }
    }

    let parsed: model::MultiExecutionResult = invocation.parsed().expect("Valid grammar expression for parsing");
    let output = print_multiple_steps(&args, &parsed);
    println!("{output}");
}
