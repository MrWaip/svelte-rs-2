use std::collections::HashMap;
use std::iter;
use std::mem;

use rustc_hash::FxHashSet;

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

use super::model::{ClassStateField, ClassStateInfo, ComponentTransformer};

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
            let ClassElement::PropertyDefinition(prop) = element else {
                continue;
            };
            match &prop.key {
                PropertyKey::PrivateIdentifier(id) => {
                    existing_private.insert(id.name.to_string());
                    body_private_field_names.insert(id.name.to_string());
                }
                PropertyKey::StaticIdentifier(id) if prop.value.is_none() => {
                    placeholder_public_names.insert(id.name.to_string());
                }
                _ => {}
            }
        }

        let mut fields = Vec::new();
        let mut ctor_synth_names = FxHashSet::default();
        let mut ctor_placeholder_names = FxHashSet::default();
        let mut seen_public: FxHashSet<String> = FxHashSet::default();
        let mut seen_private: FxHashSet<String> = FxHashSet::default();

        for declaration in declarations {
            let Some(declarator) = self.class_field_declarator(declaration.decl_node) else {
                continue;
            };
            let name = declaration.name.to_string();

            if declaration.is_private {
                if declaration.from_constructor && !body_private_field_names.contains(&name) {
                    continue;
                }
                if !seen_private.insert(name.clone()) {
                    continue;
                }
                fields.push(ClassStateField {
                    public_name: None,
                    private_name: name,
                    declarator,
                });
                continue;
            }

            if !seen_public.insert(name.clone()) {
                continue;
            }
            let mut backing = format!("#{}", name);
            while existing_private.contains(backing.trim_start_matches('#')) {
                backing = format!("#_{}", backing.trim_start_matches('#'));
            }
            existing_private.insert(backing.trim_start_matches('#').to_string());
            if declaration.from_constructor {
                ctor_synth_names.insert(name.clone());
                if placeholder_public_names.contains(&name) {
                    ctor_placeholder_names.insert(name.clone());
                }
            }
            fields.push(ClassStateField {
                public_name: Some(name),
                private_name: backing.trim_start_matches('#').to_string(),
                declarator,
            });
        }

        ClassStateInfo {
            fields,
            ctor_synth_names,
            ctor_placeholder_names,
        }
    }

    pub(crate) fn rewrite_class_body(&self, body: &mut ClassBody<'a>, info: &ClassStateInfo) {
        use ClassElement;

        let public_fields: HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();
        let private_fields: FxHashSet<&str> = info
            .fields
            .iter()
            .filter(|f| f.public_name.is_none())
            .map(|f| f.private_name.as_str())
            .collect();

        let old_elements: Vec<ClassElement<'a>> = {
            let mut temp = self.b.ast.vec();
            mem::swap(&mut body.body, &mut temp);
            temp.into_iter().collect()
        };

        let mut new_body: Vec<ClassElement<'a>> = Vec::new();

        for field_info in info.fields.iter().filter(|f| {
            f.public_name
                .as_deref()
                .is_some_and(|n| info.ctor_synth_names.contains(n))
        }) {
            let name = field_info
                .public_name
                .as_deref()
                .expect("field_info with public_name is required by caller filter");
            new_body.push(self.b.class_private_field(&field_info.private_name, None));
            self.emit_getter_setter(&mut new_body, field_info, name);
        }

        for element in old_elements {
            match element {
                ClassElement::PropertyDefinition(mut prop) => {
                    let is_rune_prop = prop.value.is_some()
                        && self.class_field_declarator(prop.node_id()).is_some();
                    if !is_rune_prop {
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
                    }

                    match &prop.key {
                        PropertyKey::PrivateIdentifier(id) => {
                            let name = id.name.to_string();
                            if private_fields.contains(name.as_str()) {
                                self.rewrite_private_field_callee(&mut prop);
                            }
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        PropertyKey::StaticIdentifier(id) if !prop.computed => {
                            let name = id.name.to_string();
                            if let Some(field_info) = public_fields.get(name.as_str()) {
                                self.emit_public_field_rewrite(
                                    &mut new_body,
                                    &mut prop,
                                    field_info,
                                    &name,
                                );
                            } else {
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            }
                        }
                        _ => {
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                    }
                }
                ClassElement::MethodDefinition(mut method) => {
                    if method.kind == MethodDefinitionKind::Constructor {
                        self.rewrite_constructor(&mut method, info);
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
        field_info: &ClassStateField,
        name: &str,
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

        let init_call = match &field_info.declarator {
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

        new_body.push(
            self.b
                .class_private_field(&field_info.private_name, Some(init_call)),
        );
        self.emit_getter_setter(new_body, field_info, name);
    }

    fn emit_getter_setter(
        &self,
        new_body: &mut Vec<ClassElement<'a>>,
        field_info: &ClassStateField,
        name: &str,
    ) {
        let get_call = self.b.call_expr(
            "$.get",
            [Arg::Expr(
                self.b.this_private_member(&field_info.private_name),
            )],
        );
        let return_stmt = self.b.return_stmt(get_call);
        new_body.push(
            self.b
                .class_getter(self.b.public_key(name), vec![return_stmt]),
        );

        let mut set_args: Vec<Arg<'a, '_>> = vec![
            Arg::Expr(self.b.this_private_member(&field_info.private_name)),
            Arg::Ident("value"),
        ];
        if field_info
            .declarator
            .class_field_state()
            .is_some_and(|state| state.kind == StateKind::State)
        {
            set_args.push(Arg::Bool(true));
        }
        let set_call = self.b.call_stmt("$.set", set_args);
        new_body.push(
            self.b
                .class_setter(self.b.public_key(name), "value", vec![set_call]),
        );
    }

    pub(crate) fn rewrite_constructor(
        &self,
        method: &mut OxcBox<'a, MethodDefinition<'a>>,
        info: &ClassStateInfo,
    ) {
        let Some(func_body) = &mut method.value.body else {
            return;
        };

        let ctor_fields: HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();
        let ctor_private_fields: HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter(|f| f.public_name.is_none())
            .map(|f| (f.private_name.as_str(), f))
            .collect();

        for stmt in func_body.statements.iter_mut() {
            if let Statement::ExpressionStatement(es) = stmt
                && let Expression::AssignmentExpression(assign) = &mut es.expression
                && assign.operator == AssignmentOperator::Assign
            {
                let (resolved_field, is_private_target, public_name) = match &assign.left {
                    AssignmentTarget::StaticMemberExpression(member)
                        if matches!(&member.object, Expression::ThisExpression(_)) =>
                    {
                        let name = member.property.name.to_string();
                        let field = ctor_fields.get(name.as_str()).copied();
                        (field, false, Some(name))
                    }
                    AssignmentTarget::PrivateFieldExpression(member)
                        if matches!(&member.object, Expression::ThisExpression(_)) =>
                    {
                        let name = member.field.name.to_string();
                        let field = ctor_private_fields.get(name.as_str()).copied();
                        (field, true, None)
                    }
                    _ => (None, false, None),
                };
                if let Some(field_info) = resolved_field
                    && let Expression::CallExpression(call) = &mut assign.right
                {
                    match &field_info.declarator {
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
                            let needs_proxy = *proxied;
                            if needs_proxy {
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
                        let label_name = public_name
                            .as_deref()
                            .unwrap_or(field_info.private_name.as_str());
                        let label = self.class_tag_label(label_name);
                        let rhs = self.b.move_expr(&mut assign.right);
                        assign.right = self.b.call_expr("$.tag", [Arg::Expr(rhs), Arg::Str(label)]);
                    }

                    if !is_private_target {
                        let new_left = self.b.this_private_member(&field_info.private_name);
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
