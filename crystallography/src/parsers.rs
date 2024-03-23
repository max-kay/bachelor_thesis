pub(crate) mod oplist {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammars/op_list.pest"]
    pub struct Parser;
}
