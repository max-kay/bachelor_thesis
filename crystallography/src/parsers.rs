pub(crate) mod oplist {
    use pest_derive::Parser;

    /// Parser for lists of symmetry operations
    #[derive(Parser)]
    #[grammar = "grammars/op_list.pest"]
    pub struct Parser;
}
