use compact_str::CompactString;
use oxc_ast::ast::{
    AssignmentTarget, Class, ClassElement, Expression, MethodDefinitionKind, PropertyKey, Statement,
};
use oxc_syntax::node::NodeId as OxcNodeId;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct ClassFieldDecl {
    pub name: CompactString,
    pub is_private: bool,
    pub decl_node: OxcNodeId,
    pub from_constructor: bool,
}

pub(crate) struct ClassFieldAccess {
    pub node: OxcNodeId,
    pub name: CompactString,
    pub is_private: bool,
}

#[derive(Debug, Default)]
pub struct ClassTable {
    fields_by_class: FxHashMap<OxcNodeId, Vec<ClassFieldDecl>>,
    access_target: FxHashMap<OxcNodeId, OxcNodeId>,
}

impl ClassTable {
    pub fn field_access_target(&self, access_node: OxcNodeId) -> Option<OxcNodeId> {
        self.access_target.get(&access_node).copied()
    }

    pub fn class_fields(&self, class_node: OxcNodeId) -> &[ClassFieldDecl] {
        self.fields_by_class
            .get(&class_node)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(crate) fn record_class(&mut self, class: &Class<'_>, accesses: &[ClassFieldAccess]) {
        let fields = collect_fields(class);

        let mut by_name: FxHashMap<(&str, bool), OxcNodeId> = FxHashMap::default();
        for field in &fields {
            by_name
                .entry((field.name.as_str(), field.is_private))
                .or_insert(field.decl_node);
        }
        for access in accesses {
            if let Some(decl_node) = by_name.get(&(access.name.as_str(), access.is_private)) {
                self.access_target.insert(access.node, *decl_node);
            }
        }

        self.fields_by_class.insert(class.body.node_id(), fields);
    }
}

fn collect_fields(class: &Class<'_>) -> Vec<ClassFieldDecl> {
    let mut fields: Vec<ClassFieldDecl> = Vec::new();

    for element in &class.body.body {
        let ClassElement::PropertyDefinition(prop) = element else {
            continue;
        };
        if prop.value.is_none() {
            continue;
        }
        let Some((name, is_private)) = key_name(&prop.key) else {
            continue;
        };
        fields.push(ClassFieldDecl {
            name,
            is_private,
            decl_node: prop.node_id(),
            from_constructor: false,
        });
    }

    let mut seen_ctor: FxHashMap<(CompactString, bool), ()> = FxHashMap::default();
    for assignment in constructor_field_assignments(class) {
        let key = (assignment.name.clone(), assignment.is_private);
        if seen_ctor.insert(key, ()).is_some() {
            continue;
        }
        fields.push(assignment);
    }

    fields
}

fn constructor_field_assignments(class: &Class<'_>) -> Vec<ClassFieldDecl> {
    let mut assignments: Vec<ClassFieldDecl> = Vec::new();
    for element in &class.body.body {
        let ClassElement::MethodDefinition(method) = element else {
            continue;
        };
        if !matches!(method.kind, MethodDefinitionKind::Constructor) {
            continue;
        }
        let Some(body) = &method.value.body else {
            continue;
        };
        for statement in &body.statements {
            let Statement::ExpressionStatement(stmt) = statement else {
                continue;
            };
            let Expression::AssignmentExpression(assign) = &stmt.expression else {
                continue;
            };
            let Some((name, is_private)) = assignment_target_field(&assign.left) else {
                continue;
            };
            assignments.push(ClassFieldDecl {
                name,
                is_private,
                decl_node: assign.node_id(),
                from_constructor: true,
            });
        }
    }
    assignments
}

fn assignment_target_field(target: &AssignmentTarget<'_>) -> Option<(CompactString, bool)> {
    match target {
        AssignmentTarget::StaticMemberExpression(member) => {
            if !matches!(&member.object, Expression::ThisExpression(_)) {
                return None;
            }
            Some((CompactString::from(member.property.name.as_str()), false))
        }
        AssignmentTarget::PrivateFieldExpression(member) => {
            if !matches!(&member.object, Expression::ThisExpression(_)) {
                return None;
            }
            Some((CompactString::from(member.field.name.as_str()), true))
        }
        _ => None,
    }
}

fn key_name(key: &PropertyKey<'_>) -> Option<(CompactString, bool)> {
    match key {
        PropertyKey::PrivateIdentifier(ident) => {
            Some((CompactString::from(ident.name.as_str()), true))
        }
        PropertyKey::StaticIdentifier(ident) => {
            Some((CompactString::from(ident.name.as_str()), false))
        }
        _ => None,
    }
}
