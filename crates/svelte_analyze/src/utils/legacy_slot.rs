use oxc_ast::ast::{BindingPattern, Statement, VariableDeclarationKind, VariableDeclarator};
use oxc_ast_visit::Visit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LegacySlotBindingKind {
    Direct,
    DestructuredLeaf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LegacySlotBinding {
    pub name: String,
    pub kind: LegacySlotBindingKind,
    pub has_default: bool,
}

pub(crate) fn legacy_slot_declarator<'a>(
    stmt: &'a Statement<'a>,
) -> Option<&'a VariableDeclarator<'a>> {
    let Statement::VariableDeclaration(decl) = stmt else {
        return None;
    };
    if decl.kind != VariableDeclarationKind::Const {
        return None;
    }
    decl.declarations.first()
}

pub(crate) fn legacy_slot_pattern<'a>(stmt: &'a Statement<'a>) -> Option<&'a BindingPattern<'a>> {
    Some(&legacy_slot_declarator(stmt)?.id)
}

pub(crate) fn collect_legacy_slot_bindings(stmt: &Statement<'_>) -> Vec<LegacySlotBinding> {
    let Some(pattern) = legacy_slot_pattern(stmt) else {
        return Vec::new();
    };

    if let BindingPattern::BindingIdentifier(id) = pattern {
        return vec![LegacySlotBinding {
            name: id.name.as_str().to_string(),
            kind: LegacySlotBindingKind::Direct,
            has_default: false,
        }];
    }

    let mut visitor = LegacySlotBindingVisitor {
        bindings: Vec::new(),
        in_default: false,
    };
    visitor.visit_binding_pattern(pattern);
    visitor.bindings
}

pub(crate) fn legacy_slot_is_destructured(stmt: &Statement<'_>) -> bool {
    matches!(
        legacy_slot_pattern(stmt),
        Some(BindingPattern::ObjectPattern(_)) | Some(BindingPattern::ArrayPattern(_))
    )
}

struct LegacySlotBindingVisitor {
    bindings: Vec<LegacySlotBinding>,
    in_default: bool,
}

impl<'a> Visit<'a> for LegacySlotBindingVisitor {
    fn visit_binding_identifier(&mut self, ident: &oxc_ast::ast::BindingIdentifier<'a>) {
        self.bindings.push(LegacySlotBinding {
            name: ident.name.as_str().to_string(),
            kind: LegacySlotBindingKind::DestructuredLeaf,
            has_default: self.in_default,
        });
    }

    fn visit_assignment_pattern(&mut self, pat: &oxc_ast::ast::AssignmentPattern<'a>) {
        let prev = self.in_default;
        self.in_default = true;
        self.visit_binding_pattern(&pat.left);
        self.in_default = prev;
    }

    fn visit_variable_declarator(&mut self, decl: &VariableDeclarator<'a>) {
        self.visit_binding_pattern(&decl.id);
    }
}
