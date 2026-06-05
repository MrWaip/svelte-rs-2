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
use svelte_analyze::{DerivedKind, RuneKind, StateKind};

use svelte_ast_builder::Arg;

use crate::rune_refs;

use super::model::{AsyncDerivedMode, ClassStateField, ClassStateInfo, ComponentTransformer};

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn state_destructure_dev_label(
        pattern: &BindingPattern<'a>,
        rune_kind: RuneKind,
    ) -> Option<&'static str> {
        if !matches!(rune_kind, RuneKind::State | RuneKind::StateRaw) {
            return None;
        }

        match pattern {
            BindingPattern::ArrayPattern(_) => Some("[$state iterable]"),
            BindingPattern::ObjectPattern(_) => Some("[$state object]"),
            _ => None,
        }
    }

    pub(crate) fn class_field_rune_kind(&self, node: OxcNodeId) -> Option<RuneKind> {
        use svelte_analyze::{
            ClassFieldDerivedSemantics, ClassFieldStateSemantics, DeclaratorSemantics,
        };
        let analysis = self.analysis.as_ref()?;
        match analysis.declarator_semantics(node) {
            DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics { kind, .. }) => {
                Some(match kind {
                    StateKind::State => RuneKind::State,
                    StateKind::StateRaw => RuneKind::StateRaw,
                    StateKind::StateEager => RuneKind::StateEager,
                })
            }
            DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics { kind, .. }) => {
                Some(match kind {
                    DerivedKind::Derived => RuneKind::Derived,
                    DerivedKind::DerivedBy => RuneKind::DerivedBy,
                })
            }
            DeclaratorSemantics::None
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem { .. }
            | DeclaratorSemantics::AwaitValue => None,
            DeclaratorSemantics::RuneProps | DeclaratorSemantics::LegacyProps => None,
            DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. } => None,
        }
    }


    pub(crate) fn wrap_state_value(
        &self,
        value: Expression<'a>,
        rune_kind: RuneKind,
        is_signal_source: bool,
    ) -> Expression<'a> {
        let value = value.into_inner_expression();
        match rune_kind {
            RuneKind::State => {
                let proxied = if rune_refs::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                if is_signal_source {
                    self.b.call_expr("$.state", [Arg::Expr(proxied)])
                } else {
                    proxied
                }
            }
            RuneKind::StateRaw => {
                if is_signal_source {
                    self.b.call_expr("$.state", [Arg::Expr(value)])
                } else {
                    value
                }
            }
            RuneKind::Derived | RuneKind::DerivedBy => {
                let thunk = self
                    .b
                    .arrow_expr(self.b.no_params(), [self.b.expr_stmt(value)]);
                self.b.seed_arrow_scope(&thunk, self.gen_arrow_scope);
                self.b.call_expr("$.derived", [Arg::Expr(thunk)])
            }
            _ => value,
        }
    }

    pub(crate) fn scan_class_state_fields(
        &self,
        body: &ClassBody<'a>,
    ) -> ClassStateInfo {
        let mut fields = Vec::new();

        let mut existing_private: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let ClassElement::PropertyDefinition(prop) = element
                && let PropertyKey::PrivateIdentifier(id) = &prop.key
            {
                existing_private.insert(id.name.to_string());
            }
        }

        let mut body_public_names: FxHashSet<String> = FxHashSet::default();
        let mut placeholder_public_names: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let ClassElement::PropertyDefinition(prop) = element {
                if let PropertyKey::StaticIdentifier(id) = &prop.key
                    && !prop.computed
                    && prop.value.is_none()
                {
                    placeholder_public_names.insert(id.name.to_string());
                }
                if prop.value.is_none() {
                    continue;
                }
                let Some(rune_kind) = self.class_field_rune_kind(prop.node_id()) else {
                    continue;
                };

                match &prop.key {
                    PropertyKey::PrivateIdentifier(id) => {
                        fields.push(ClassStateField {
                            public_name: None,
                            private_name: id.name.to_string(),
                            rune_kind,
                        });
                    }
                    PropertyKey::StaticIdentifier(id) if !prop.computed => {
                        let name = id.name.to_string();
                        let mut backing = format!("#{}", name);
                        while existing_private.contains(backing.trim_start_matches('#')) {
                            backing = format!("#_{}", backing.trim_start_matches('#'));
                        }
                        existing_private.insert(backing.trim_start_matches('#').to_string());
                        body_public_names.insert(name.clone());
                        fields.push(ClassStateField {
                            public_name: Some(name),
                            private_name: backing.trim_start_matches('#').to_string(),
                            rune_kind,
                        });
                    }
                    _ => {}
                }
            }
        }

        let mut ctor_synth_names = FxHashSet::default();
        let mut ctor_placeholder_names = FxHashSet::default();
        let mut ctor_private_names: FxHashSet<String> = FxHashSet::default();
        let body_private_field_names: FxHashSet<String> = body
            .body
            .iter()
            .filter_map(|el| {
                if let ClassElement::PropertyDefinition(prop) = el
                    && let PropertyKey::PrivateIdentifier(id) = &prop.key
                {
                    Some(id.name.to_string())
                } else {
                    None
                }
            })
            .collect();
        for element in &body.body {
            if let ClassElement::MethodDefinition(method) = element
                && method.kind == MethodDefinitionKind::Constructor
                && let Some(func_body) = &method.value.body
            {
                for stmt in &func_body.statements {
                    if let Statement::ExpressionStatement(es) = stmt
                        && let Expression::AssignmentExpression(assign) = &es.expression
                        && assign.operator == AssignmentOperator::Assign
                        && let Some(rune_kind) = self.class_field_rune_kind(assign.node_id())
                    {
                        match &assign.left {
                            AssignmentTarget::StaticMemberExpression(member)
                                if matches!(&member.object, Expression::ThisExpression(_)) =>
                            {
                                let name = member.property.name.to_string();
                                if body_public_names.contains(&name)
                                    || !ctor_synth_names.insert(name.clone())
                                {
                                    continue;
                                }
                                let mut backing = format!("#{}", name);
                                while existing_private.contains(backing.trim_start_matches('#')) {
                                    backing = format!("#_{}", backing.trim_start_matches('#'));
                                }
                                existing_private
                                    .insert(backing.trim_start_matches('#').to_string());
                                if placeholder_public_names.contains(&name) {
                                    ctor_placeholder_names.insert(name.clone());
                                }
                                fields.push(ClassStateField {
                                    public_name: Some(name),
                                    private_name: backing
                                        .trim_start_matches('#')
                                        .to_string(),
                                    rune_kind,
                                });
                            }
                            AssignmentTarget::PrivateFieldExpression(member)
                                if matches!(&member.object, Expression::ThisExpression(_)) =>
                            {
                                let name = member.field.name.to_string();
                                if !body_private_field_names.contains(&name) {
                                    continue;
                                }
                                if !ctor_private_names.insert(name.clone()) {
                                    continue;
                                }
                                if fields.iter().any(|f| {
                                    f.public_name.is_none() && f.private_name == name
                                }) {
                                    continue;
                                }
                                fields.push(ClassStateField {
                                    public_name: None,
                                    private_name: name,
                                    rune_kind,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        ClassStateInfo {
            fields,
            ctor_synth_names,
            ctor_placeholder_names,
        }
    }

    pub(crate) fn rewrite_class_body(
        &self,
        body: &mut ClassBody<'a>,
        info: &ClassStateInfo,
    ) {
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
                        && self.class_field_rune_kind(prop.node_id()).is_some();
                    if !is_rune_prop {
                        let is_ctor_placeholder = prop.value.is_none()
                            && match &prop.key {
                                PropertyKey::StaticIdentifier(id)
                                    if !prop.computed =>
                                {
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
        let rune_kind = self.class_field_rune_kind(prop.node_id());
        if let Some(value) = prop.value.take() {
            prop.value = Some(value.into_inner_expression());
        }
        if let Some(Expression::CallExpression(call)) = &mut prop.value {
            match rune_kind {
                Some(RuneKind::State) => {
                    call.callee = self.b.rid_expr("$.state");
                    if !call.arguments.is_empty() {
                        let mut dummy = Argument::from(self.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg = dummy.into_expression();
                        let wrapped = if rune_refs::should_proxy(&arg) {
                            self.b.call_expr("$.proxy", [Arg::Expr(arg)])
                        } else {
                            arg
                        };
                        call.arguments[0] = Argument::from(wrapped);
                    }
                }
                Some(RuneKind::StateRaw) => {
                    call.callee = self.b.rid_expr("$.state");
                }
                Some(RuneKind::Derived) => {
                    call.callee = self.b.rid_expr("$.derived");
                    if !call.arguments.is_empty() {
                        let mut dummy = Argument::from(self.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let thunked = self.b.thunk(dummy.into_expression());
                        call.arguments[0] = Argument::from(thunked);
                    }
                }
                Some(RuneKind::DerivedBy) => {
                    call.callee = self.b.rid_expr("$.derived");
                }
                _ => {}
            }
            if self.dev && rune_kind.is_some() {
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

        let init_call = match field_info.rune_kind {
            RuneKind::Derived => {
                let thunked = self.b.thunk(arg.unwrap_or_else(|| self.b.cheap_expr()));
                self.b.call_expr("$.derived", [Arg::Expr(thunked)])
            }
            RuneKind::DerivedBy => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.derived", [Arg::Expr(arg)])
                } else {
                    self.b
                        .call_expr("$.derived", iter::empty::<Arg<'a, '_>>())
                }
            }
            RuneKind::State => {
                if let Some(arg) = arg {
                    let wrapped = if rune_refs::should_proxy(&arg) {
                        self.b.call_expr("$.proxy", [Arg::Expr(arg)])
                    } else {
                        arg
                    };
                    self.b.call_expr("$.state", [Arg::Expr(wrapped)])
                } else {
                    self.b
                        .call_expr("$.state", iter::empty::<Arg<'a, '_>>())
                }
            }
            _ => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.state", [Arg::Expr(arg)])
                } else {
                    self.b
                        .call_expr("$.state", iter::empty::<Arg<'a, '_>>())
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
        if field_info.rune_kind == RuneKind::State {
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
                    match field_info.rune_kind {
                        RuneKind::Derived => {
                            call.callee = self.b.rid_expr("$.derived");
                            if !call.arguments.is_empty() {
                                let mut dummy = Argument::from(self.b.cheap_expr());
                                mem::swap(&mut call.arguments[0], &mut dummy);
                                let thunked = self.b.thunk(dummy.into_expression());
                                call.arguments[0] = Argument::from(thunked);
                            }
                        }
                        RuneKind::DerivedBy => {
                            call.callee = self.b.rid_expr("$.derived");
                        }
                        RuneKind::State => {
                            call.callee = self.b.rid_expr("$.state");
                            let needs_proxy = call
                                .arguments
                                .first()
                                .and_then(|a| a.as_expression())
                                .is_some_and(|e| rune_refs::should_proxy(e));
                            if needs_proxy {
                                let mut dummy = Argument::from(self.b.cheap_expr());
                                mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = Argument::from(proxied);
                            }
                        }
                        RuneKind::StateRaw => {
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

    pub(crate) fn is_private_state_field(&self, name: &str) -> bool {
        self.private_state_field_rune_kind(name).is_some()
    }

    pub(crate) fn private_state_field_rune_kind(&self, name: &str) -> Option<RuneKind> {
        self.class_state_stack.last().and_then(|info| {
            info.fields
                .iter()
                .find(|f| f.public_name.is_none() && f.private_name == name)
                .map(|f| f.rune_kind)
        })
    }

    pub(crate) fn in_constructor(&self) -> bool {
        self.function_info_stack
            .last()
            .is_some_and(|f| f.in_constructor)
    }

    pub(crate) fn async_derived_mode(&self) -> AsyncDerivedMode {
        if self.strip_exports && self.function_info_stack.len() > 1 {
            AsyncDerivedMode::Save
        } else {
            AsyncDerivedMode::Await
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

