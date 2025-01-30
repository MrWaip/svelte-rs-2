pub mod visitor;

use std::{
    collections::HashMap,
    mem::{self, replace},
};

use oxc_ast::{
    ast::{BindingPatternKind, Expression, IdentifierReference, VariableDeclarator},
    visit::walk::{walk_assignment_expression, walk_update_expression},
    Visit,
};
use oxc_semantic::{
    NodeId, Reference, ReferenceFlags, ScopeTree, SemanticBuilder, SymbolId, SymbolTable,
};
use visitor::TemplateVisitor;

use crate::ast::Ast;

pub struct Analyzer {}

pub struct AnalyzeResult {
    pub runes: HashMap<SymbolId, Rune>,
    pub symbols: SymbolTable,
    pub scopes: ScopeTree,
}

#[derive(Debug)]
pub struct Rune {
    pub mutated: bool,
    pub kind: RuneKind,
}

#[derive(Debug)]
pub enum RuneKind {
    State,
}

impl Analyzer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn analyze<'a, 'link>(&self, ast: &'link Ast<'a>) -> AnalyzeResult {
        let (runes, symbols, scopes) = if let Some(script) = &ast.script {
            let ret = SemanticBuilder::new().build(&script.program);

            if !ret.errors.is_empty() {
                todo!();
            }

            let (mut symbols, mut scopes) = ret.semantic.into_symbol_table_and_scope_tree();

            let mut template_visitor = TemplateVisitorImpl {
                current_reference_flags: ReferenceFlags::empty(),
                scopes: &mut scopes,
                symbols: &mut symbols,
            };

            template_visitor.visit_template(&ast.template);

            let mut visitor = ScriptVisitorImpl {
                runes: HashMap::default(),
                scopes: template_visitor.scopes,
                symbols: template_visitor.symbols,
            };

            visitor.visit_program(&script.program);

            (
                replace(&mut visitor.runes, HashMap::default()),
                symbols,
                scopes,
            )
        } else {
            (
                HashMap::default(),
                SymbolTable::default(),
                ScopeTree::default(),
            )
        };

        return AnalyzeResult {
            runes,
            scopes,
            symbols,
        };
    }
}

pub struct ScriptVisitorImpl<'link> {
    pub runes: HashMap<SymbolId, Rune>,
    pub symbols: &'link mut SymbolTable,
    pub scopes: &'link mut ScopeTree,
}

impl<'a, 'link> Visit<'a> for ScriptVisitorImpl<'link> {
    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(Expression::CallExpression(call)) = &declarator.init {
            if call.callee_name() == Some("$state") {
                if let BindingPatternKind::BindingIdentifier(id) = &declarator.id.kind {
                    let symbol_id = id.symbol_id();

                    self.runes.insert(
                        symbol_id,
                        Rune {
                            kind: RuneKind::State,
                            mutated: self.symbols.symbol_is_mutated(symbol_id.clone()),
                        },
                    );
                }
            }
        }
    }
}

pub struct TemplateVisitorImpl<'link> {
    pub symbols: &'link mut SymbolTable,
    pub scopes: &'link mut ScopeTree,
    current_reference_flags: ReferenceFlags,
}

impl<'a, 'link> TemplateVisitor<'a> for TemplateVisitorImpl<'link> {
    fn visit_expression(&mut self, it: &Expression<'a>) {
        Visit::visit_expression(self, it);
    }
}

impl<'a, 'link> Visit<'a> for TemplateVisitorImpl<'link> {
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
}

impl<'link> TemplateVisitorImpl<'link> {
    fn reference_identifier<'a>(&mut self, ident: &IdentifierReference<'a>) {
        let flags = self.resolve_reference_usages();
        let mut reference = Reference::new(NodeId::DUMMY, flags);
        let symbol_id = self.scopes.get_root_binding(&ident.name);

        if let Some(symbol_id) = symbol_id {
            reference.set_symbol_id(symbol_id);
            let reference_id = self.symbols.create_reference(reference);
            ident.reference_id.set(Some(reference_id));
            self.symbols.add_resolved_reference(symbol_id, reference_id);
        }
    }

    fn resolve_reference_usages(&mut self) -> ReferenceFlags {
        if self.current_reference_flags.is_empty() {
            ReferenceFlags::Read
        } else {
            // Take the current reference flags so that we can reset it to empty
            mem::take(&mut self.current_reference_flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use crate::parser::Parser;

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

        assert!(!result.runes.is_empty());

        for (id, _rune) in result.runes.iter() {
            assert_eq!(result.symbols.get_name(id.clone()), "rune_var");
        }
    }
}
