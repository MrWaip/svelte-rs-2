use oxc_allocator::Box as OxcBox;
use oxc_ast::ast::{Expression, FormalParameterKind, FormalParameters, Statement};
use oxc_span::{SPAN, Span};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{BlockSemantics, SnippetPlacement};
use svelte_ast::{Node, NodeId};
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn snippet_placement(&self, id: NodeId) -> Option<SnippetPlacement> {
        match self.analysis.block_semantics(id) {
            BlockSemantics::Snippet(sem) => Some(sem.placement),
            _ => None,
        }
    }

    pub(crate) fn route_snippet(
        &mut self,
        id: NodeId,
        local_out: &mut Vec<Statement<'a>>,
    ) -> Result<()> {
        match self.snippet_placement(id) {
            Some(SnippetPlacement::ModuleLevel) => {
                let mut decls = Vec::new();
                self.emit_snippet_into(id, &mut decls)?;
                self.hoisted.append(&mut decls);
            }
            Some(SnippetPlacement::InstanceLevel | SnippetPlacement::Local) => {
                self.emit_snippet_into(id, local_out)?;
            }
            None => {}
        }
        Ok(())
    }

    pub(crate) fn emit_snippet_into(
        &mut self,
        id: NodeId,
        out: &mut Vec<Statement<'a>>,
    ) -> Result<()> {
        let sem = match self.analysis.block_semantics(id) {
            BlockSemantics::Snippet(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(id, "snippet block")),
        };
        let name: &'a str = self
            .b
            .alloc_str(self.analysis.scoping.symbol_name(sem.name));
        let function = self.build_snippet_function(id, name)?;
        if self.dev {
            out.push(
                self.b
                    .call_stmt("$.prevent_snippet_stringification", [Arg::Ident(name)]),
            );
        }
        out.push(function);
        Ok(())
    }

    fn build_snippet_function(&mut self, block_id: NodeId, name: &'a str) -> Result<Statement<'a>> {
        let Node::SnippetBlock(block) = self.component.store.get(block_id) else {
            return Err(CodegenError::Unsupported(block_id, "snippet block node"));
        };
        let body_fragment = block.body;
        let params = self.take_snippet_params(block.decl.id());

        let mut body = self.child_statements(|codegen| {
            codegen.fragment(body_fragment, FragmentParent::Snippet, false)
        })?;

        if self.dev {
            let validate = self
                .b
                .call_stmt("$.validate_snippet_args", [Arg::Ident("$$renderer")]);
            body.insert(0, validate);
        }

        let function = self
            .b
            .function_decl(self.b.bid(name), body, params, Span::default());
        Ok(Statement::FunctionDeclaration(self.b.alloc(function)))
    }

    fn take_snippet_params(&mut self, decl_id: OxcNodeId) -> FormalParameters<'a> {
        let mut items = self.b.ast.vec();
        items.push(self.b.formal_parameter_from_str("$$renderer"));

        let mut rest = None;
        if let Some(stmt) = self.js_arena.take_stmt(decl_id)
            && let Some(mut arrow_params) = extract_arrow_params(stmt)
        {
            for param in arrow_params.items.drain(..) {
                items.push(param);
            }
            rest = arrow_params.rest.take();
        }

        self.b
            .ast
            .formal_parameters(SPAN, FormalParameterKind::FormalParameter, items, rest)
    }
}

fn extract_arrow_params<'a>(stmt: Statement<'a>) -> Option<OxcBox<'a, FormalParameters<'a>>> {
    let init = match stmt {
        Statement::VariableDeclaration(decl) => decl
            .unbox()
            .declarations
            .into_iter()
            .next()
            .and_then(|d| d.init),
        Statement::ExpressionStatement(stmt) => match stmt.unbox().expression {
            Expression::AssignmentExpression(assign) => Some(assign.unbox().right),
            other => Some(other),
        },
        _ => None,
    }?;
    match init {
        Expression::ArrowFunctionExpression(arrow) => Some(arrow.unbox().params),
        _ => None,
    }
}
