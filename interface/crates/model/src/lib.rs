//! ## Parsing a color
//!
//! ### C++ output
//! In the C++ part a color is one of two types (`White` or `Black`). `Color` is more of an
//! abstract concept that describes a relation between these two types.
//! When looking at the type missmatch error message the involved type names are reported.
//! For simplicity we assume that it was conversion of a single `Color`, so no nested structures
//! are involved. These would get parsed recursively by parsing each components of product types
//! and parsing sum types like `Color`, each variant of a sum type can be treated a product type,
//! maybe a unit one.
//!
//! So we got the output:
//! ```cpp
//! White
//! ```
//! from the C++ compiler
//!
//! ### The grammar
//! Here is the part of the `chess.pest` grammer file that is responsible for parsing a color.
//! The underscore indicates a silent rule so `Color` won't be included in the resulting ast like
//! structure returned by the parser. The [pest] crate will then create a parser and the [Rule]
//! enum from this definition file
//! ```peg
//! Color = _{ ColorWhite | ColorBlack }
//! ColorWhite = { "White" }
//! ColorBlack = { "Black" }
//! ```
//! Because it combines the two color variants without being included in the output `Color` can be
//! considered an abstract construct in this context.
//!
//! For our ouput the parser will generate one node with the associated [Rule] [Rule::ColorWhite]
//! and the string representation `White`.
//!
//! ### Rust
//! Because we know what we expect as output when we provide an input (normally the output would be
//! a specific result type created by executing multiple moves) we can assume that the Rust program
//! in this example can parse specifically with a `Color` equivalent.
//!
//! This is a simplified version of the [Color] definition using the derive macros from [abstraction_derive]:
//! ```
//! # use model::Rule;
//! #
//! # mod inner {
//! # use abstraction_derive::{FromCppTypes, ParsingRulesDecl, ToCppTypes};
//! #[derive(Clone, Copy, FromCppTypes, ParsingRulesDecl, ToCppTypes)]
//! pub enum Color {
//!     White,
//!     Black,
//! }
//! # }
//! # fn main() {}
//! ```
//!
//! These macros implement useful traits on `Color` so that it can be used by the compiler
//! abstraction type to parse values as a `Color`.
//!
//! When used on an enum [abstraction_derive::FromCppTypes] will create a function that takes a
//! node of the parse tree and checks the rule of the element. If it is one of the valid ones
//! declared by [abstraction::ParsingRulesDecl] parsing will contiune.
//! Then the correct variant for the found [Rule] will selected and a color gets created by parsing
//! this variant.
//!
//! The type names are not arbitrary and follow a specific convention assumed by the derive macros.
//! For each variant of an enum the derive macros will exepct an equivalent [Rule] named
//! `EnumVariant` and one silent rule called `Enum`. In this case there would be the two non-silent
//! rules [Rule::ColorWhite] and [Rule::ColorBlack] and the silent combination rule [Rule::Color].
//!
//! Because of the silent rule [Rule::Color] the Rust code can ignore possible alternatives in the
//! grammer when parsing and can simply say the grammar it should parse the abstract concept
//! [Rule::Color] and because it's silent it will not show up in the output and the root node is
//! guaranteed to be one of [Rule::ColorWhite] and [Rule::ColorBlack].
//!
//! For product types there is a non-silent rule expected to exist that is named exactly like the
//! Rust type. For structs or struct variants of enums parsing will consider fields in order.
//! So it will iterate over each field in the declaration of the struct (variant) from first to
//! last and will fetch one child node from the provided top level node for each.

pub mod types;
pub use types::*;
mod grammar;

pub use grammar::{ChessParser, Rule};
