pub mod svelte_table;
pub mod visitor;

use std::mem::{replace, take};

use oxc_ast::{
    ast::{BindingPatternKind, Expression, IdentifierReference, VariableDeclarator},
    visit::walk::{walk_assignment_expression, walk_call_expression, walk_update_expression},
    Visit,
};
use oxc_semantic::{ReferenceFlags, SemanticBuilder};
use svelte_table::{ExpressionFlags, RuneKind, SvelteTable};
use visitor::TemplateVisitor;

use ast::Ast;

pub struct Analyzer {}

pub struct AnalyzeResult<'a> {
    pub svelte_table: SvelteTable<'a>,
}

impl Analyzer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn analyze<'a, 'link>(&self, ast: &'link Ast<'a>) -> AnalyzeResult<'a> {
        let svelte_table = if let Some(script) = &ast.script {
            let ret = SemanticBuilder::new().build(&script.program);

            if !ret.errors.is_empty() {
                todo!();
            }

            let (symbols, scopes) = ret.semantic.into_symbol_table_and_scope_tree();
            let mut svelte_table = SvelteTable::new(symbols, scopes);

            let mut script_visitor = ScriptVisitorImpl {
                svelte_table: &mut svelte_table,
            };

            script_visitor.visit_program(&script.program);

            let mut template_visitor = TemplateVisitorImpl {
                current_reference_flags: ReferenceFlags::empty(),
                current_expression_flags: ExpressionFlags::empty(),
                svelte_table: script_visitor.svelte_table,
            };

            template_visitor.visit_template(&ast.template);

            svelte_table
        } else {
            SvelteTable::default()
        };

        return AnalyzeResult { svelte_table };
    }
}

pub struct ScriptVisitorImpl<'link, 'a> {
    pub svelte_table: &'link mut SvelteTable<'a>,
}

pub struct TemplateVisitorImpl<'link, 'a> {
    pub svelte_table: &'link mut SvelteTable<'a>,
    current_reference_flags: ReferenceFlags,
    current_expression_flags: ExpressionFlags,
}

impl<'a, 'link> Visit<'a> for ScriptVisitorImpl<'link, 'a> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(Expression::CallExpression(call)) = &declarator.init {
            if call.callee_name() == Some("$state") {
                if let BindingPatternKind::BindingIdentifier(id) = &declarator.id.kind {
                    let symbol_id = id.symbol_id();

                    self.svelte_table.add_rune(symbol_id, RuneKind::State);
                }
            }
        }
    }
}

impl<'a, 'link> TemplateVisitor<'a> for TemplateVisitorImpl<'link, 'a> {
    fn visit_expression(&mut self, it: &Expression<'a>) {
        Visit::visit_expression(self, it);

        let flags = replace(&mut self.current_expression_flags, ExpressionFlags::empty());

        self.svelte_table.add_expression_flag(it, flags);
    }
}

impl<'a, 'link> Visit<'a> for TemplateVisitorImpl<'link, 'a> {
    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        self.reference_identifier(it);
    }

    fn visit_update_expression(&mut self, it: &oxc_ast::ast::UpdateExpression<'a>) {
        self.current_reference_flags = ReferenceFlags::read_write();
        walk_update_expression(self, it);
    }

    fn visit_assignment_expression(&mut self, it: &oxc_ast::ast::AssignmentExpression<'a>) {
        self.current_reference_flags = ReferenceFlags::write();
        walk_assignment_expression(self, it);
    }

    fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
        self.current_expression_flags.has_call = true;
        walk_call_expression(self, it);
    }
}

impl<'link, 'a> TemplateVisitorImpl<'link, 'a> {
    fn reference_identifier(&mut self, ident: &IdentifierReference<'a>) {
        let flags = self.resolve_reference_usages();

        let option = self
            .svelte_table
            .add_root_scope_reference(&ident.name, flags);

        if let Some(reference_id) = option {
            ident.reference_id.set(Some(reference_id));

            if self.svelte_table.is_rune_reference(reference_id) {
                self.current_expression_flags.has_state = true;
            }
        }
    }

    fn resolve_reference_usages(&mut self) -> ReferenceFlags {
        if self.current_reference_flags.is_empty() {
            ReferenceFlags::Read
        } else {
            take(&mut self.current_reference_flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::Node;
    use oxc_allocator::Allocator;

    use parser::Parser;

    use super::*;

    #[test]
    fn analyze_smoke() {
        let allocator = Allocator::default();
        let analyzer = Analyzer::new();
        let mut parser = Parser::new(
            "<script>let rune_var = $state(10); onMount(() => rune_var = 0);</script>",
            &allocator,
        );
        let ast = parser.parse().unwrap();
        let result = analyzer.analyze(&ast);

        assert!(!result.svelte_table.runes.is_empty());

        for (id, _rune) in result.svelte_table.runes.iter() {
            assert_eq!(result.svelte_table.symbols.get_name(id.clone()), "rune_var");
        }
    }

    #[test]
    fn svelte_table_smoke() {
        let allocator = Allocator::default();
        let analyzer = Analyzer::new();
        let mut parser = Parser::new(
            "<script>let rune_var = $state(10); onMount(() => rune_var = 0);</script>{goto(rune_var)}",
            &allocator,
        );
        let ast = parser.parse().unwrap();
        let result = analyzer.analyze(&ast);

        let Node::Interpolation(interpolation) = &*ast.template[0].borrow() else {
            unreachable!()
        };

        let flags = result
            .svelte_table
            .get_expression_flag(&interpolation.expression)
            .unwrap();

        assert_eq!(flags.has_state, true);
        assert_eq!(flags.has_call, true);
    }
}
