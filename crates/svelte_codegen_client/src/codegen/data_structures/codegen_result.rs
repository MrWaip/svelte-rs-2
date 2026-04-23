use oxc_ast::ast::Statement;

#[derive(Default)]
pub(crate) struct CodegenResult<'a> {
    pub hoisted: Vec<Statement<'a>>,

    pub body: Vec<Statement<'a>>,

    pub instance_snippets: Vec<Statement<'a>>,

    pub hoistable_snippets: Vec<Statement<'a>>,
}
