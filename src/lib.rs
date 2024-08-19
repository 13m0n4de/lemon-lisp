#![feature(iterator_try_collect)]
#![feature(let_chains)]
#[warn(clippy::all, clippy::pedantic)]
#[allow(clippy::missing_errors_doc)]
pub mod evaluator;
pub mod internal;
pub mod interpreter;
pub mod lexer;
pub mod model;
pub mod optimizer;
pub mod parser;
