use std::collections::{HashMap, HashSet};
use std::mem;

use oxc_ast::ast::{
    Argument, AssignmentExpression, AssignmentOperator, AssignmentTarget, Class, ClassBody,
    ClassElement, Expression, MethodDefinition, MethodDefinitionKind, PropertyDefinition,
    PropertyKey, Statement,
};
use svelte_analyze::{ClassFieldSemantics, DeclaratorSemantics, DerivedKind};
use svelte_ast_builder::Arg;
use svelte_component_semantics::OxcNodeId;

use crate::model::ServerTransform;

#[derive(Default)]
struct ClassInfo {
    backing: HashMap<OxcNodeId, String>,
    ctor_synth_nodes: Vec<OxcNodeId>,
    ctor_placeholder_names: HashSet<String>,
    has_rune_field: bool,
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_server_class(&mut self, class: &mut Class<'a>) {
        let info = self.scan_class(&class.body);
        if !info.has_rune_field {
            return;
        }
        self.rewrite_server_class_body(&mut class.body, &info);
    }

    pub(crate) fn rewrite_private_derived_read(&self, expr: &mut Expression<'a>) -> bool {
        let Expression::PrivateFieldExpression(pfe) = &*expr else {
            return false;
        };
        if !matches!(
            self.analysis.class_field_semantics(pfe.node_id()),
            ClassFieldSemantics::Derived { .. }
        ) {
            return false;
        }
        let field = self.b.move_expr(expr);
        *expr = self.b.call_expr_callee(field, []);
        true
    }

    fn class_field_declarator(&self, node: OxcNodeId) -> Option<DeclaratorSemantics> {
        match self.analysis.declarator_semantics(node) {
            declarator @ (DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_)) => Some(declarator),
            _ => None,
        }
    }

    fn scan_class(&self, body: &ClassBody<'a>) -> ClassInfo {
        let declarations = self
            .analysis
            .scoping
            .semantics()
            .class_fields(body.node_id());

        let mut existing_private: HashSet<String> = HashSet::default();
        let mut body_private_field_names: HashSet<String> = HashSet::default();
        let mut placeholder_public_names: HashSet<String> = HashSet::default();
        for element in &body.body {
            match element {
                ClassElement::PropertyDefinition(prop) => match &prop.key {
                    PropertyKey::PrivateIdentifier(id) => {
                        existing_private.insert(id.name.to_string());
                        body_private_field_names.insert(id.name.to_string());
                    }
                    PropertyKey::StaticIdentifier(id) if prop.value.is_none() => {
                        placeholder_public_names.insert(id.name.to_string());
                    }
                    _ => {}
                },
                ClassElement::MethodDefinition(method) => {
                    if let PropertyKey::PrivateIdentifier(id) = &method.key {
                        existing_private.insert(id.name.to_string());
                    }
                }
                _ => {}
            }
        }

        let mut info = ClassInfo::default();
        let mut seen_public: HashSet<String> = HashSet::default();
        let mut seen_private: HashSet<String> = HashSet::default();

        for declaration in declarations {
            let Some(declarator) = self.class_field_declarator(declaration.decl_node) else {
                continue;
            };
            let name = declaration.name.to_string();
            let is_derived = matches!(declarator, DeclaratorSemantics::ClassFieldDerived(_));

            if declaration.is_private {
                if declaration.from_constructor && !body_private_field_names.contains(&name) {
                    continue;
                }
                if !seen_private.insert(name) {
                    continue;
                }
                info.has_rune_field = true;
                continue;
            }

            if !seen_public.insert(name.clone()) {
                continue;
            }
            info.has_rune_field = true;
            if !is_derived {
                continue;
            }
            let mut backing = fold_to_private_identifier(&name);
            while existing_private.contains(&backing) {
                backing = format!("_{backing}");
            }
            existing_private.insert(backing.clone());
            if declaration.from_constructor {
                info.ctor_synth_nodes.push(declaration.decl_node);
                if placeholder_public_names.contains(&name) {
                    info.ctor_placeholder_names.insert(name);
                }
            }
            info.backing.insert(declaration.decl_node, backing);
        }

        info
    }

    fn rewrite_server_class_body(&mut self, body: &mut ClassBody<'a>, info: &ClassInfo) {
        let names: HashMap<OxcNodeId, String> = self
            .analysis
            .scoping
            .semantics()
            .class_fields(body.node_id())
            .iter()
            .map(|decl| (decl.decl_node, decl.name.to_string()))
            .collect();

        let old_elements: Vec<ClassElement<'a>> = {
            let mut temp = self.b.ast.vec();
            mem::swap(&mut body.body, &mut temp);
            temp.into_iter().collect()
        };

        let mut new_body: Vec<ClassElement<'a>> = Vec::new();

        for node in &info.ctor_synth_nodes {
            let (Some(name), Some(backing), Some(declarator)) = (
                names.get(node),
                info.backing.get(node),
                self.class_field_declarator(*node),
            ) else {
                continue;
            };
            let _ = declarator;
            new_body.push(self.b.class_private_field(backing, None));
            self.emit_derived_accessors(&mut new_body, backing, self.b.key(name), self.b.key(name));
        }

        for element in old_elements {
            match element {
                ClassElement::PropertyDefinition(mut prop) => {
                    let node = prop.node_id();
                    let declarator = if prop.value.is_some() {
                        self.class_field_declarator(node)
                    } else {
                        None
                    };
                    let Some(declarator) = declarator else {
                        let is_placeholder = prop.value.is_none()
                            && matches!(&prop.key, PropertyKey::StaticIdentifier(id) if !prop.computed && info.ctor_placeholder_names.contains(id.name.as_str()));
                        if !is_placeholder {
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        continue;
                    };

                    if matches!(&prop.key, PropertyKey::PrivateIdentifier(_)) {
                        self.rewrite_private_field_value(&mut prop, &declarator);
                        new_body.push(ClassElement::PropertyDefinition(prop));
                        continue;
                    }

                    match &declarator {
                        DeclaratorSemantics::ClassFieldState(_) => {
                            self.rewrite_state_field_value(&mut prop);
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        DeclaratorSemantics::ClassFieldDerived(_) => {
                            let Some(backing) = info.backing.get(&node) else {
                                new_body.push(ClassElement::PropertyDefinition(prop));
                                continue;
                            };
                            let get_key = self.b.clone_key(&prop.key);
                            let set_key = self.b.clone_key(&prop.key);
                            let init = self.derived_field_init(&mut prop, &declarator);
                            new_body.push(self.b.class_private_field(backing, Some(init)));
                            self.emit_derived_accessors(&mut new_body, backing, get_key, set_key);
                        }
                        _ => new_body.push(ClassElement::PropertyDefinition(prop)),
                    }
                }
                ClassElement::MethodDefinition(mut method) => {
                    if method.kind == MethodDefinitionKind::Constructor {
                        self.rewrite_constructor(&mut method, info, &names);
                    }
                    new_body.push(ClassElement::MethodDefinition(method));
                }
                other => new_body.push(other),
            }
        }

        body.body = self.b.ast.vec_from_iter(new_body);
    }

    fn rewrite_state_field_value(&self, prop: &mut PropertyDefinition<'a>) {
        let Some(Expression::CallExpression(mut call)) =
            prop.value.take().map(|v| v.into_inner_expression())
        else {
            return;
        };
        if call.arguments.is_empty() {
            prop.value = None;
            return;
        }
        let mut dummy = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut dummy);
        prop.value = Some(dummy.into_expression().into_inner_expression());
    }

    fn rewrite_private_field_value(
        &self,
        prop: &mut PropertyDefinition<'a>,
        declarator: &DeclaratorSemantics,
    ) {
        match declarator {
            DeclaratorSemantics::ClassFieldState(_) => self.rewrite_state_field_value(prop),
            DeclaratorSemantics::ClassFieldDerived(_) => {
                let init = self.derived_field_init(prop, declarator);
                prop.value = Some(init);
            }
            _ => {}
        }
    }

    fn derived_field_init(
        &self,
        prop: &mut PropertyDefinition<'a>,
        declarator: &DeclaratorSemantics,
    ) -> Expression<'a> {
        let arg = prop
            .value
            .take()
            .map(|v| v.into_inner_expression())
            .and_then(|value| match value {
                Expression::CallExpression(mut call) if !call.arguments.is_empty() => {
                    let mut dummy = Argument::from(self.b.cheap_expr());
                    mem::swap(&mut call.arguments[0], &mut dummy);
                    Some(dummy.into_expression())
                }
                _ => None,
            });
        self.build_derived_init(declarator, arg)
    }

    fn build_derived_init(
        &self,
        declarator: &DeclaratorSemantics,
        arg: Option<Expression<'a>>,
    ) -> Expression<'a> {
        let derived_arg = match declarator {
            DeclaratorSemantics::ClassFieldDerived(d) if matches!(d.kind, DerivedKind::Derived) => {
                self.b.thunk(arg.unwrap_or_else(|| self.b.void_zero_expr()))
            }
            _ => arg.unwrap_or_else(|| self.b.void_zero_expr()),
        };
        self.b.call_expr("$.derived", [Arg::Expr(derived_arg)])
    }

    fn emit_derived_accessors(
        &self,
        new_body: &mut Vec<ClassElement<'a>>,
        backing: &str,
        get_key: PropertyKey<'a>,
        set_key: PropertyKey<'a>,
    ) {
        let get_call = self
            .b
            .call_expr_callee(self.b.this_private_member(backing), []);
        new_body.push(
            self.b
                .class_getter(get_key, vec![self.b.return_stmt(get_call)]),
        );

        let set_call = self
            .b
            .call_expr_callee(self.b.this_private_member(backing), [Arg::Ident("$$value")]);
        new_body.push(
            self.b
                .class_setter(set_key, "$$value", vec![self.b.return_stmt(set_call)]),
        );
    }

    fn rewrite_constructor(
        &self,
        method: &mut MethodDefinition<'a>,
        info: &ClassInfo,
        names: &HashMap<OxcNodeId, String>,
    ) {
        let Some(func_body) = &mut method.value.body else {
            return;
        };
        let name_to_backing: HashMap<&str, &str> = info
            .backing
            .iter()
            .filter_map(|(node, backing)| {
                names
                    .get(node)
                    .map(|name| (name.as_str(), backing.as_str()))
            })
            .collect();

        for stmt in func_body.statements.iter_mut() {
            let Statement::ExpressionStatement(es) = stmt else {
                continue;
            };
            let Expression::AssignmentExpression(assign) = &mut es.expression else {
                continue;
            };
            if assign.operator != AssignmentOperator::Assign {
                continue;
            }
            let node = assign.node_id();
            let Some(declarator) = self.class_field_declarator(node) else {
                continue;
            };
            let target_is_private =
                matches!(&assign.left, AssignmentTarget::PrivateFieldExpression(_));

            match &declarator {
                DeclaratorSemantics::ClassFieldState(_) => {
                    let value = take_state_assignment_value(self, assign);
                    assign.right = value;
                }
                DeclaratorSemantics::ClassFieldDerived(_) => {
                    let arg = match &mut assign.right {
                        Expression::CallExpression(call) if !call.arguments.is_empty() => {
                            let mut dummy = Argument::from(self.b.cheap_expr());
                            mem::swap(&mut call.arguments[0], &mut dummy);
                            Some(dummy.into_expression())
                        }
                        _ => None,
                    };
                    assign.right = self.build_derived_init(&declarator, arg);
                    if !target_is_private
                        && let Some(name) = names.get(&node)
                        && let Some(backing) = name_to_backing.get(name.as_str())
                    {
                        let new_left = self.b.this_private_member(backing);
                        if let Expression::PrivateFieldExpression(pfe) = new_left {
                            assign.left = AssignmentTarget::PrivateFieldExpression(pfe);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn take_state_assignment_value<'a>(
    transform: &ServerTransform<'_, 'a>,
    assign: &mut AssignmentExpression<'a>,
) -> Expression<'a> {
    let Expression::CallExpression(call) = &mut assign.right else {
        return transform.b.move_expr(&mut assign.right);
    };
    if call.arguments.is_empty() {
        return transform.b.void_zero_expr();
    }
    let mut dummy = Argument::from(transform.b.cheap_expr());
    mem::swap(&mut call.arguments[0], &mut dummy);
    dummy.into_expression().into_inner_expression()
}

fn fold_to_private_identifier(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    for (index, ch) in name.chars().enumerate() {
        let valid = if index == 0 {
            ch.is_ascii_alphabetic() || ch == '_' || ch == '$'
        } else {
            ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'
        };
        result.push(if valid { ch } else { '_' });
    }
    result
}
