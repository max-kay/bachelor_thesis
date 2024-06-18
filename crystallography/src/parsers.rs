use pest_derive::Parser;

/// Parser for lists of symmetry operations
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;
