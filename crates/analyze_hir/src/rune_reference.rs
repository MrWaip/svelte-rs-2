use std::mem::{replace, take};

use hir::{ConcatenationAttributePart, ConcatenationPart, ExpressionId, HirStore};
use oxc_ast::{
    Visit,
    ast::{Expression, IdentifierReference},
    visit::walk::{
        walk_arrow_function_expression, walk_assignment_expression, walk_call_expression,
        walk_function, walk_update_expression,
    },
};
use oxc_semantic::{ReferenceFlags, ScopeId};

use crate::{AnalyzeHir, HirAnalyses, bitflags::ExpressionFlags};

struct AnalyzeTemplateExpression<'hir> {
    analyses: &'hir mut HirAnalyses,
    current_reference_flags: ReferenceFlags,
    scope_id: ScopeId,
    expression_flags: ExpressionFlags,
}

impl<'hir> AnalyzeHir<'hir> {
    pub(crate) fn rune_reference_pass(
        &self,
        analyses: &mut HirAnalyses,
        store: &hir::HirStore<'hir>,
    ) {
        let root_scope_id = analyses.root_scope_id();

        let mut analyzer = AnalyzeTemplateExpression::new(analyses);

        for node in store.nodes.iter() {
            match node {
                hir::Node::Text(_) => continue,
                hir::Node::Interpolation(it) => {
                    let expression = store.get_expression(it.expression_id);
                    analyzer.analyze(
                        it.expression_id,
                        &expression,
                        ReferenceFlags::read(),
                        root_scope_id,
                    );
                }
                hir::Node::Element(it) => {
                    for attr in it.attributes.iter_all() {
                        match attr {
                            hir::AnyAttribute::Bind(it) => {
                                let expression = store.get_expression(it.expression_id);
                                analyzer.analyze(
                                    it.expression_id,
                                    &expression,
                                    ReferenceFlags::read_write(),
                                    root_scope_id,
                                );
                            }
                            hir::AnyAttribute::ExpressionAttribute(it) => {
                                let expression = store.get_expression(it.expression_id);
                                analyzer.analyze(
                                    it.expression_id,
                                    &expression,
                                    ReferenceFlags::read(),
                                    root_scope_id,
                                );
                            }
                            hir::AnyAttribute::SpreadAttribute(it) => {
                                let expression = store.get_expression(it.expression_id);
                                analyzer.analyze(
                                    it.expression_id,
                                    &expression,
                                    ReferenceFlags::read(),
                                    root_scope_id,
                                );
                            }
                            hir::AnyAttribute::ConcatenationAttribute(it) => {
                                for part in it.parts.iter() {
                                    let ConcatenationAttributePart::Expression(expression_id) =
                                        part
                                    else {
                                        continue;
                                    };

                                    let expression = store.get_expression(*expression_id);
                                    analyzer.analyze(
                                        *expression_id,
                                        &expression,
                                        ReferenceFlags::read(),
                                        root_scope_id,
                                    );
                                }
                            }
                            hir::AnyAttribute::StringAttribute(_) => continue,
                            hir::AnyAttribute::BooleanAttribute(_) => continue,
                            hir::AnyAttribute::Use(_) => todo!(),
                            hir::AnyAttribute::Animation(_) => todo!(),
                            hir::AnyAttribute::On(_) => todo!(),
                            hir::AnyAttribute::Transition(_) => todo!(),
                            hir::AnyAttribute::Class(_) => todo!(),
                            hir::AnyAttribute::Style(_) => todo!(),
                            hir::AnyAttribute::Let(_) => todo!(),
                        }
                    }
                }
                hir::Node::Concatenation(it) => {
                    for part in it.parts.iter() {
                        let ConcatenationPart::Expression(expression_id) = part else {
                            continue;
                        };

                        let expression = store.get_expression(*expression_id);
                        analyzer.analyze(
                            *expression_id,
                            &expression,
                            ReferenceFlags::read(),
                            root_scope_id,
                        );
                    }
                }
                hir::Node::IfBlock(it) => {
                    let expression = store.get_expression(it.test);
                    analyzer.analyze(it.test, &expression, ReferenceFlags::read(), root_scope_id);
                }
                hir::Node::Comment(_) => continue,
                hir::Node::Phantom => continue,
                hir::Node::EachBlock(it) => {
                    let each_scope_id = it.scope_id.get().unwrap();

                    let expression = store.get_expression(it.collection);
                    analyzer.analyze(
                        it.collection,
                        &expression,
                        ReferenceFlags::read(),
                        each_scope_id,
                    );

                    let expression = store.get_expression(it.item);
                    analyzer.analyze(it.item, &expression, ReferenceFlags::read(), each_scope_id);
                }
                hir::Node::Script => todo!(),
            };
        }
    }
}

impl<'hir> AnalyzeTemplateExpression<'hir> {
    pub(crate) fn new(analyses: &'hir mut HirAnalyses) -> Self {
        return Self {
            analyses,
            current_reference_flags: ReferenceFlags::read(),
            scope_id: ScopeId::new(0),
            expression_flags: ExpressionFlags::empty(),
        };
    }

    pub(crate) fn analyze(
        &mut self,
        expression_id: ExpressionId,
        expression: &Expression<'hir>,
        flags: ReferenceFlags,
        scope_id: ScopeId,
    ) {
        self.scope_id = scope_id;
        self.current_reference_flags = flags;

        self.visit_expression(expression);

        self.analyses.set_expression_flags(
            expression_id,
            replace(&mut self.expression_flags, ExpressionFlags::empty()),
        );
    }
}

impl<'hir> Visit<'hir> for AnalyzeTemplateExpression<'hir> {
    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'hir>) {
        let flags = replace(&mut self.current_reference_flags, ReferenceFlags::read());

        let reference_id = self.analyses.add_reference(&it.name, flags, self.scope_id);

        if let Some(reference_id) = reference_id {
            it.reference_id.set(Some(reference_id));

            if self.analyses.get_rune_by_reference(reference_id).is_some() {
                self.expression_flags.insert(ExpressionFlags::RuneReference);
            }
        }
    }

    fn visit_update_expression(&mut self, it: &oxc_ast::ast::UpdateExpression<'hir>) {
        self.current_reference_flags = ReferenceFlags::read_write();
        walk_update_expression(self, it);
    }

    fn visit_assignment_expression(&mut self, it: &oxc_ast::ast::AssignmentExpression<'hir>) {
        self.current_reference_flags = ReferenceFlags::write();
        walk_assignment_expression(self, it);
    }

    fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'hir>) {
        self.expression_flags.insert(ExpressionFlags::FunctionCall);
        walk_call_expression(self, it);
    }

    fn visit_arrow_function_expression(
        &mut self,
        it: &oxc_ast::ast::ArrowFunctionExpression<'hir>,
    ) {
        let scope_id = self.analyses.add_scope();
        it.set_scope_id(scope_id);

        walk_arrow_function_expression(self, it);
    }

    fn visit_function(
        &mut self,
        it: &oxc_ast::ast::Function<'hir>,
        flags: oxc_semantic::ScopeFlags,
    ) {
        let scope_id = self.analyses.add_scope();
        it.set_scope_id(scope_id);

        walk_function(self, it, flags);
    }
}
