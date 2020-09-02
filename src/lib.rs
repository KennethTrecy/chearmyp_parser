#![no_std]

/// Contains the data structures used and/or returned by different lexers.
pub mod token;

/// Contains different characters needed to be recognized by the different lexers.
pub mod special_characters;

/// Contains `find_line_ending()`.
mod find_line_ending;

/// Contains `line_comment()` lexer.
pub mod line_comment;

pub use find_line_ending::find_line_ending;
