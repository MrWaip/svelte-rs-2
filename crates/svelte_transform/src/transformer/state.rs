use std::iter;
use std::mem;

use rustc_hash::{FxHashMap, FxHashSet};

use oxc_allocator::Box as OxcBox;
use oxc_ast::ast::{
    Argument, AssignmentOperator, AssignmentTarget, BindingPattern, ClassBody, ClassElement,
    Expression, MethodDefinition, MethodDefinitionKind, PropertyDefinition, PropertyKey, Statement,
};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{
    ClassFieldDerivedSemantics, ClassFieldStateSemantics, DeclaratorSemantics, DerivedKind,
    StateKind,
};

use svelte_ast_builder::Arg;
use svelte_component_semantics::ClassFieldDecl;

use super::model::{ClassStateInfo, ComponentTransformer};

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn state_destructure_dev_label(
        pattern: &BindingPattern<'a>,
        state_kind: StateKind,
    ) -> Option<&'static str> {
        if !matches!(state_kind, StateKind::State | StateKind::StateRaw) {
            return None;
        }

        match pattern {
            BindingPattern::ArrayPattern(_) => Some("[$state iterable]"),
            BindingPattern::ObjectPattern(_) => Some("[$state object]"),
            _ => None,
        }
    }

    pub(crate) fn class_field_declarator(&self, node: OxcNodeId) -> Option<DeclaratorSemantics> {
        let analysis = self.analysis.as_ref()?;
        match analysis.declarator_semantics(node) {
            declarator @ (DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_)) => Some(declarator),
            _ => None,
        }
    }

    pub(crate) fn scan_class_state_fields(&self, body: &ClassBody<'a>) -> ClassStateInfo {
        let Some(analysis) = self.analysis else {
            return ClassStateInfo::default();
        };
        let declarations = analysis.scoping.semantics().class_fields(body.node_id());

        let mut existing_private: FxHashSet<String> = FxHashSet::default();
        let mut body_private_field_names: FxHashSet<String> = FxHashSet::default();
        let mut placeholder_public_names: FxHashSet<String> = FxHashSet::default();
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

        let mut info = ClassStateInfo::default();
        let mut seen_public: FxHashSet<String> = FxHashSet::default();
        let mut seen_private: FxHashSet<String> = FxHashSet::default();

        for declaration in declarations {
            if self.class_field_declarator(declaration.decl_node).is_none() {
                continue;
            }
            let name = declaration.name.to_string();

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
            let mut backing = fold_to_private_identifier(&name);
            while existing_private.contains(&backing) {
                backing = format!("_{}", backing);
            }
            existing_private.insert(backing.clone());
            if declaration.from_constructor {
                info.ctor_synth_nodes.push(declaration.decl_node);
                if placeholder_public_names.contains(&name) {
                    info.ctor_placeholder_names.insert(name);
                }
            }
            info.has_rune_field = true;
            info.backing.insert(declaration.decl_node, backing);
        }

        info
    }

    pub(crate) fn rewrite_class_body(&self, body: &mut ClassBody<'a>, info: &ClassStateInfo) {
        use ClassElement;

        let Some(analysis) = self.analysis else {
            return;
        };
        let decls: FxHashMap<OxcNodeId, &ClassFieldDecl> = analysis
            .scoping
            .semantics()
            .class_fields(body.node_id())
            .iter()
            .map(|decl| (decl.decl_node, decl))
            .collect();

        let old_elements: Vec<ClassElement<'a>> = {
            let mut temp = self.b.ast.vec();
            mem::swap(&mut body.body, &mut temp);
            temp.into_iter().collect()
        };

        let mut new_body: Vec<ClassElement<'a>> = Vec::new();

        for node in &info.ctor_synth_nodes {
            let (Some(decl), Some(backing), Some(declarator)) = (
                decls.get(node),
                info.backing.get(node),
                self.class_field_declarator(*node),
            ) else {
                continue;
            };
            let get_key = self.b.key(decl.name.as_str());
            let set_key = self.b.key(decl.name.as_str());
            new_body.push(self.b.class_private_field(backing, None));
            self.emit_getter_setter(&mut new_body, backing, declarator, get_key, set_key);
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
                        let is_ctor_placeholder = prop.value.is_none()
                            && match &prop.key {
                                PropertyKey::StaticIdentifier(id) if !prop.computed => {
                                    info.ctor_placeholder_names.contains(id.name.as_str())
                                }
                                _ => false,
                            };
                        if !is_ctor_placeholder {
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        continue;
                    };

                    if matches!(&prop.key, PropertyKey::PrivateIdentifier(_)) {
                        self.rewrite_private_field_callee(&mut prop);
                        new_body.push(ClassElement::PropertyDefinition(prop));
                        continue;
                    }

                    let Some(backing) = info.backing.get(&node) else {
                        new_body.push(ClassElement::PropertyDefinition(prop));
                        continue;
                    };
                    let name = decls
                        .get(&node)
                        .map(|decl| decl.name.as_str())
                        .unwrap_or("");
                    let get_key = self.b.clone_key(&prop.key);
                    let set_key = self.b.clone_key(&prop.key);
                    self.emit_public_field_rewrite(
                        &mut new_body,
                        &mut prop,
                        name,
                        backing,
                        declarator,
                        get_key,
                        set_key,
                    );
                }
                ClassElement::MethodDefinition(mut method) => {
                    if method.kind == MethodDefinitionKind::Constructor {
                        self.rewrite_constructor(&mut method, info, &decls);
                    }
                    new_body.push(ClassElement::MethodDefinition(method));
                }
                other => {
                    new_body.push(other);
                }
            }
        }

        body.body = self.b.ast.vec_from_iter(new_body);
    }

    fn rewrite_private_field_callee(&self, prop: &mut PropertyDefinition<'a>) {
        let declarator = self.class_field_declarator(prop.node_id());
        if let Some(value) = prop.value.take() {
            prop.value = Some(value.into_inner_expression());
        }
        if let Some(Expression::CallExpression(call)) = &mut prop.value {
            match &declarator {
                Some(DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics {
                    kind: StateKind::State,
                    proxied,
                })) => {
                    call.callee = self.b.rid_expr("$.state");
                    if !call.arguments.is_empty() {
                        let mut dummy = Argument::from(self.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg = dummy.into_expression();
                        let wrapped = if *proxied {
                            self.b.call_expr("$.proxy", [Arg::Expr(arg)])
                        } else {
                            arg
                        };
                        call.arguments[0] = Argument::from(wrapped);
                    }
                }
                Some(DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics {
                    kind: StateKind::StateRaw,
                    ..
                })) => {
                    call.callee = self.b.rid_expr("$.state");
                }
                Some(DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics {
                    kind: DerivedKind::Derived,
                    ..
                })) => {
                    call.callee = self.b.rid_expr("$.derived");
                    if !call.arguments.is_empty() {
                        let mut dummy = Argument::from(self.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let thunked = self.b.thunk(dummy.into_expression());
                        call.arguments[0] = Argument::from(thunked);
                    }
                }
                Some(DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics {
                    kind: DerivedKind::DerivedBy,
                    ..
                })) => {
                    call.callee = self.b.rid_expr("$.derived");
                }
                _ => {}
            }
            if self.dev && declarator.is_some() {
                let field_name = match &prop.key {
                    PropertyKey::PrivateIdentifier(id) => format!("#{}", id.name),
                    _ => String::new(),
                };
                let label = self.class_tag_label(&field_name);
                let value = self.b.move_expr(
                    prop.value
                        .as_mut()
                        .expect("rune property definitions always carry an initializer"),
                );
                prop.value = Some(
                    self.b
                        .call_expr("$.tag", [Arg::Expr(value), Arg::Str(label)]),
                );
            }
        }
    }

    fn emit_public_field_rewrite(
        &self,
        new_body: &mut Vec<ClassElement<'a>>,
        prop: &mut PropertyDefinition<'a>,
        name: &str,
        backing: &str,
        declarator: DeclaratorSemantics,
        get_key: PropertyKey<'a>,
        set_key: PropertyKey<'a>,
    ) {
        let arg = if let Some(Expression::CallExpression(mut call)) =
            prop.value.take().map(|v| v.into_inner_expression())
        {
            if call.arguments.is_empty() {
                None
            } else {
                let mut dummy = Argument::from(self.b.cheap_expr());
                mem::swap(&mut call.arguments[0], &mut dummy);
                Some(dummy.into_expression())
            }
        } else {
            None
        };

        let init_call = match &declarator {
            DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics {
                kind: DerivedKind::Derived,
                ..
            }) => {
                let thunked = self.b.thunk(arg.unwrap_or_else(|| self.b.cheap_expr()));
                self.b.call_expr("$.derived", [Arg::Expr(thunked)])
            }
            DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics {
                kind: DerivedKind::DerivedBy,
                ..
            }) => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.derived", [Arg::Expr(arg)])
                } else {
                    self.b.call_expr("$.derived", iter::empty::<Arg<'a, '_>>())
                }
            }
            DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics {
                kind: StateKind::State,
                proxied,
            }) => {
                if let Some(arg) = arg {
                    let wrapped = if *proxied {
                        self.b.call_expr("$.proxy", [Arg::Expr(arg)])
                    } else {
                        arg
                    };
                    self.b.call_expr("$.state", [Arg::Expr(wrapped)])
                } else {
                    self.b.call_expr("$.state", iter::empty::<Arg<'a, '_>>())
                }
            }
            _ => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.state", [Arg::Expr(arg)])
                } else {
                    self.b.call_expr("$.state", iter::empty::<Arg<'a, '_>>())
                }
            }
        };

        let init_call = if self.dev {
            let label = self.class_tag_label(name);
            self.b
                .call_expr("$.tag", [Arg::Expr(init_call), Arg::Str(label)])
        } else {
            init_call
        };

        new_body.push(self.b.class_private_field(backing, Some(init_call)));
        self.emit_getter_setter(new_body, backing, declarator, get_key, set_key);
    }

    fn emit_getter_setter(
        &self,
        new_body: &mut Vec<ClassElement<'a>>,
        backing: &str,
        declarator: DeclaratorSemantics,
        get_key: PropertyKey<'a>,
        set_key: PropertyKey<'a>,
    ) {
        let get_call = self
            .b
            .call_expr("$.get", [Arg::Expr(self.b.this_private_member(backing))]);
        let return_stmt = self.b.return_stmt(get_call);
        new_body.push(self.b.class_getter(get_key, vec![return_stmt]));

        let mut set_args: Vec<Arg<'a, '_>> = vec![
            Arg::Expr(self.b.this_private_member(backing)),
            Arg::Ident("value"),
        ];
        if declarator
            .class_field_state()
            .is_some_and(|state| state.kind == StateKind::State)
        {
            set_args.push(Arg::Bool(true));
        }
        let set_call = self.b.call_stmt("$.set", set_args);
        new_body.push(self.b.class_setter(set_key, "value", vec![set_call]));
    }

    pub(crate) fn rewrite_constructor(
        &self,
        method: &mut OxcBox<'a, MethodDefinition<'a>>,
        info: &ClassStateInfo,
        decls: &FxHashMap<OxcNodeId, &ClassFieldDecl>,
    ) {
        let Some(func_body) = &mut method.value.body else {
            return;
        };

        let name_to_backing: FxHashMap<&str, &str> = info
            .backing
            .iter()
            .filter_map(|(node, backing)| {
                decls
                    .get(node)
                    .map(|decl| (decl.name.as_str(), backing.as_str()))
            })
            .collect();

        for stmt in func_body.statements.iter_mut() {
            if let Statement::ExpressionStatement(es) = stmt
                && let Expression::AssignmentExpression(assign) = &mut es.expression
                && assign.operator == AssignmentOperator::Assign
            {
                let node = assign.node_id();
                let (Some(decl), Some(declarator)) =
                    (decls.get(&node).copied(), self.class_field_declarator(node))
                else {
                    continue;
                };
                if let Expression::CallExpression(call) = &mut assign.right {
                    match &declarator {
                        DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics {
                            kind: DerivedKind::Derived,
                            ..
                        }) => {
                            call.callee = self.b.rid_expr("$.derived");
                            if !call.arguments.is_empty() {
                                let mut dummy = Argument::from(self.b.cheap_expr());
                                mem::swap(&mut call.arguments[0], &mut dummy);
                                let thunked = self.b.thunk(dummy.into_expression());
                                call.arguments[0] = Argument::from(thunked);
                            }
                        }
                        DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics {
                            kind: DerivedKind::DerivedBy,
                            ..
                        }) => {
                            call.callee = self.b.rid_expr("$.derived");
                        }
                        DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics {
                            kind: StateKind::State,
                            proxied,
                        }) => {
                            call.callee = self.b.rid_expr("$.state");
                            if *proxied {
                                let mut dummy = Argument::from(self.b.cheap_expr());
                                mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = Argument::from(proxied);
                            }
                        }
                        DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics {
                            kind: StateKind::StateRaw,
                            ..
                        }) => {
                            call.callee = self.b.rid_expr("$.state");
                        }
                        _ => {
                            call.callee = self.b.rid_expr("$.state");
                        }
                    }
                    if self.dev {
                        let label = self.class_tag_label(decl.name.as_str());
                        let rhs = self.b.move_expr(&mut assign.right);
                        assign.right = self.b.call_expr("$.tag", [Arg::Expr(rhs), Arg::Str(label)]);
                    }

                    if !decl.is_private
                        && let Some(backing) = name_to_backing.get(decl.name.as_str())
                    {
                        let new_left = self.b.this_private_member(backing);
                        if let Expression::PrivateFieldExpression(pfe) = new_left {
                            assign.left = AssignmentTarget::PrivateFieldExpression(pfe);
                        }
                    }
                }
            }
        }
    }

    fn class_tag_label(&self, field_name: &str) -> String {
        let class_name = self
            .class_name_stack
            .last()
            .and_then(|n| n.as_deref())
            .unwrap_or("[class]");
        format!("{}.{}", class_name, field_name)
    }
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

#[cfg(test)]
mod tests {
    use super::fold_to_private_identifier;

    #[track_caller]
    fn assert_folds(cases: &[(&str, &str)]) {
        for &(input, expected) in cases {
            let got = fold_to_private_identifier(input);
            assert_eq!(
                got, expected,
                "fold_to_private_identifier({input:?}): expected {expected:?}, got {got:?}"
            );
        }
    }

    #[test]
    fn folds_invalid_identifier_chars_to_underscore() {
        assert_folds(&[
            ("0", "_"),
            ("1", "_"),
            ("aria-pressed", "aria_pressed"),
            ("tree", "tree"),
            ("$x", "$x"),
            ("_y", "_y"),
            ("0abc", "_abc"),
        ]);
    }
}
