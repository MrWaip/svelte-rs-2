use rustc_hash::FxHashSet;

use oxc_allocator::CloneIn;
use oxc_ast::NONE;
use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::RuneKind;

use crate::builder::Arg;

use super::{ClassStateField, ClassStateInfo, ScriptTransformer};

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    /// Expand destructured `$state`/`$state.raw` declarations into expanded form.
    /// Called from `exit_statements` after other transformations.
    pub(super) fn expand_state_destructuring(&mut self, stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>) {
        let mut i = 0;
        while i < stmts.len() {
            let should_expand = if let Statement::VariableDeclaration(decl) = &stmts[i] {
                decl.declarations.len() == 1
                    && !matches!(&decl.declarations[0].id, oxc_ast::ast::BindingPattern::BindingIdentifier(_))
                    && decl.declarations[0].init.as_ref().is_some_and(|init| {
                        Self::detect_state_rune_kind(init).is_some()
                    })
            } else {
                false
            };

            if !should_expand {
                i += 1;
                continue;
            }

            // Take ownership of the statement
            let stmt = stmts.remove(i);
            let Statement::VariableDeclaration(mut decl) = stmt else { unreachable!() };
            let mut declarator = decl.declarations.remove(0);
            let init = declarator.init.take().unwrap();
            let rune_kind = Self::detect_state_rune_kind(&init).unwrap();

            // Extract the rune call argument
            let value = if let Expression::CallExpression(mut call) = init {
                if call.arguments.is_empty() {
                    self.b.ast.expression_object(oxc_span::SPAN, self.b.ast.vec())
                } else {
                    let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                    std::mem::swap(&mut call.arguments[0], &mut dummy);
                    dummy.into_expression()
                }
            } else {
                unreachable!()
            };

            // Generate the expanded declaration
            let replacement = self.gen_state_destructuring(
                &declarator.id,
                value,
                rune_kind,
                decl.kind,
            );

            // Insert replacement statement
            stmts.insert(i, replacement);
            self.ident_counter += 1;
            i += 1;
        }
    }

    /// Detect if an expression is a `$state(...)` or `$state.raw(...)` call.
    pub(super) fn detect_state_rune_kind(expr: &Expression<'_>) -> Option<RuneKind> {
        if let Expression::CallExpression(call) = expr {
            match &call.callee {
                Expression::Identifier(id) if id.name.as_str() == "$state" => {
                    return Some(RuneKind::State);
                }
                Expression::StaticMemberExpression(member) => {
                    if let Expression::Identifier(obj) = &member.object {
                        if obj.name.as_str() == "$state" && member.property.name.as_str() == "raw" {
                            return Some(RuneKind::StateRaw);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Generate expanded variable declaration for destructured $state/$state.raw.
    fn gen_state_destructuring(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        value: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
    ) -> Statement<'a> {
        let tmp_name = self.gen_unique_name("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut declarators = Vec::new();

        // First declarator: tmp = value
        let tmp_declarator = self.b.ast.variable_declarator(
            oxc_span::SPAN,
            decl_kind,
            self.b.ast.binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(tmp_name_str)),
            NONE,
            Some(value),
            false,
        );
        declarators.push(tmp_declarator);

        // Walk pattern and generate remaining declarators
        let tmp_expr = self.b.rid_expr(tmp_name_str);
        self.gen_destructure_declarators(pattern, tmp_expr, rune_kind, decl_kind, &mut declarators);

        let decl = self.b.ast.variable_declaration(
            oxc_span::SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
    }

    /// Recursively generate declarators for destructured patterns.
    fn gen_destructure_declarators(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
        declarators: &mut Vec<oxc_ast::ast::VariableDeclarator<'a>>,
    ) {
        match pattern {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                let name = id.name.as_str();
                let sym_id = id.symbol_id.get();
                let is_mutated = sym_id.is_some_and(|s| self.component_scoping.is_mutated(s));

                let final_value = self.wrap_state_value(accessor, rune_kind, is_mutated);

                let declarator = self.b.ast.variable_declarator(
                    oxc_span::SPAN,
                    decl_kind,
                    self.b.ast.binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(name)),
                    NONE,
                    Some(final_value),
                    false,
                );
                declarators.push(declarator);
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj) => {
                // Collect property key names for rest element
                let mut key_names: Vec<String> = Vec::new();
                for prop in &obj.properties {
                    if let Some(name) = Self::property_key_name(&prop.key) {
                        key_names.push(name);
                    }
                }

                for prop in &obj.properties {
                    let member = self.build_object_member_access(accessor.clone_in(self.b.ast.allocator), &prop.key, prop.computed);
                    self.gen_destructure_declarators(&prop.value, member, rune_kind, decl_kind, declarators);
                }

                if let Some(rest) = &obj.rest {
                    // $.exclude_from_object(accessor, ["key1", "key2"])
                    let keys_array = self.b.array_expr(key_names.iter().map(|k| self.b.str_expr(k)));
                    let exclude_expr = self.b.call_expr("$.exclude_from_object", [
                        Arg::Expr(accessor),
                        Arg::Expr(keys_array),
                    ]);
                    self.gen_destructure_declarators(&rest.argument, exclude_expr, rune_kind, decl_kind, declarators);
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(arr) => {
                // Generate intermediate: $$array_N = $.derived(() => $.to_array(accessor, len))
                let array_name = self.gen_unique_name("$$array");
                let array_name_str: &str = self.b.alloc_str(&array_name);

                let len_arg = if arr.rest.is_some() {
                    vec![Arg::Expr(accessor)]
                } else {
                    vec![Arg::Expr(accessor), Arg::Num(arr.elements.len() as f64)]
                };

                let to_array_call = self.b.call_expr("$.to_array", len_arg);
                let thunk = self.b.arrow_expr(self.b.no_params(), [self.b.expr_stmt(to_array_call)]);
                let derived_call = self.b.call_expr("$.derived", [Arg::Expr(thunk)]);

                let array_declarator = self.b.ast.variable_declarator(
                    oxc_span::SPAN,
                    decl_kind,
                    self.b.ast.binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(array_name_str)),
                    NONE,
                    Some(derived_call),
                    false,
                );
                declarators.push(array_declarator);

                // Generate element declarators
                for (idx, elem) in arr.elements.iter().enumerate() {
                    let Some(elem) = elem else { continue };
                    // $.get($$array)[idx]
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let elem_access = self.b.computed_member_expr(get_array, self.b.num_expr(idx as f64));
                    self.gen_destructure_declarators(elem, elem_access, rune_kind, decl_kind, declarators);
                }

                if let Some(rest) = &arr.rest {
                    // $.get($$array).slice(idx)
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let slice = self.b.static_member_expr(get_array, "slice");
                    let slice_call = self.b.ast.expression_call(
                        oxc_span::SPAN,
                        slice,
                        NONE,
                        self.b.ast.vec_from_array([oxc_ast::ast::Argument::from(self.b.num_expr(arr.elements.len() as f64))]),
                        false,
                    );
                    self.gen_destructure_declarators(&rest.argument, slice_call, rune_kind, decl_kind, declarators);
                }
            }
            oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                // Default value: $.fallback(accessor, default)
                let default_expr = assign.right.clone_in(self.b.ast.allocator);
                let fallback = self.b.call_expr("$.fallback", [
                    Arg::Expr(accessor),
                    Arg::Expr(default_expr),
                ]);
                self.gen_destructure_declarators(&assign.left, fallback, rune_kind, decl_kind, declarators);
            }
        }
    }

    /// Wrap a value based on rune kind and mutation status.
    pub(super) fn wrap_state_value(
        &self,
        value: Expression<'a>,
        rune_kind: RuneKind,
        is_mutated: bool,
    ) -> Expression<'a> {
        match rune_kind {
            RuneKind::State => {
                let proxied = if svelte_transform::rune_refs::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                if is_mutated {
                    self.b.call_expr("$.state", [Arg::Expr(proxied)])
                } else {
                    proxied
                }
            }
            RuneKind::StateRaw => {
                if is_mutated {
                    self.b.call_expr("$.state", [Arg::Expr(value)])
                } else {
                    value
                }
            }
            _ => value,
        }
    }

    /// Generate a unique name with a given prefix.
    /// Each prefix has its own counter so `tmp` and `$$array` don't conflict.
    pub(super) fn gen_unique_name(&mut self, prefix: &str) -> String {
        // Use a simple scheme: first call for any prefix gets no suffix,
        // subsequent calls get _1, _2, etc. Track via ident_counter globally
        // but offset per-prefix using a simple convention.
        // For simplicity, just track the count of destructured statements.
        // The first destructuring gets tmp/$$array, second gets tmp_1/$$array_1.
        // We use ident_counter to count destructuring invocations.
        // gen_state_destructuring increments once, both tmp and $$array use same number.
        let n = self.ident_counter;
        if n == 0 {
            prefix.to_string()
        } else {
            let mut s = String::with_capacity(prefix.len() + 4);
            s.push_str(prefix);
            s.push('_');
            s.push_str(&n.to_string());
            s
        }
    }

    /// Extract property key name as a string.
    pub(super) fn property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<String> {
        match key {
            oxc_ast::ast::PropertyKey::StaticIdentifier(id) => Some(id.name.to_string()),
            oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(s.value.to_string()),
            _ => None,
        }
    }

    /// Build a member access expression for an object property key.
    pub(super) fn build_object_member_access(
        &self,
        object: Expression<'a>,
        key: &oxc_ast::ast::PropertyKey<'a>,
        computed: bool,
    ) -> Expression<'a> {
        if computed {
            if let Some(expr) = Self::property_key_to_expr(self.b, key) {
                self.b.computed_member_expr(object, expr)
            } else {
                object
            }
        } else {
            match key {
                oxc_ast::ast::PropertyKey::StaticIdentifier(id) => {
                    self.b.static_member_expr(object, self.b.alloc_str(id.name.as_str()))
                }
                oxc_ast::ast::PropertyKey::StringLiteral(s) => {
                    self.b.static_member_expr(object, self.b.alloc_str(s.value.as_str()))
                }
                _ => object,
            }
        }
    }

    fn property_key_to_expr<'c>(b: &'c crate::builder::Builder<'a>, key: &oxc_ast::ast::PropertyKey<'a>) -> Option<Expression<'a>> {
        match key {
            oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(b.str_expr(s.value.as_str())),
            oxc_ast::ast::PropertyKey::NumericLiteral(n) => Some(b.num_expr(n.value)),
            _ => None,
        }
    }

    // -----------------------------------------------------------------------
    // Class state field helpers
    // -----------------------------------------------------------------------

    /// Scan a class body for state fields and return info about them.
    pub(super) fn scan_class_state_fields(&self, body: &oxc_ast::ast::ClassBody<'a>) -> ClassStateInfo {
        let mut fields = Vec::new();

        // Collect existing private names to avoid conflicts when generating backing fields
        let mut existing_private: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(prop) = element {
                if let oxc_ast::ast::PropertyKey::PrivateIdentifier(id) = &prop.key {
                    existing_private.insert(id.name.to_string());
                }
            }
        }

        // Scan PropertyDefinitions for $state/$state.raw
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(prop) = element {
                let Some(value) = &prop.value else { continue };
                let Some(rune_kind) = Self::detect_state_rune_kind(value) else { continue };
                let is_state = rune_kind == RuneKind::State;

                match &prop.key {
                    oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => {
                        // Private field: #name = $state(...) → just rewrite callee
                        fields.push(ClassStateField {
                            public_name: None,
                            private_name: id.name.to_string(),
                            is_state,
                        });
                    }
                    oxc_ast::ast::PropertyKey::StaticIdentifier(id) if !prop.computed => {
                        // Public field: name = $state(...) → private backing + getter/setter
                        let name = id.name.to_string();
                        let mut backing = format!("#{}", name);
                        // Deconflict if private name already exists
                        while existing_private.contains(backing.trim_start_matches('#')) {
                            backing = format!("#_{}", backing.trim_start_matches('#'));
                        }
                        existing_private.insert(backing.trim_start_matches('#').to_string());
                        fields.push(ClassStateField {
                            public_name: Some(name),
                            private_name: backing.trim_start_matches('#').to_string(),
                            is_state,
                        });
                    }
                    _ => {}
                }
            }
        }

        // Scan constructor for `this.name = $state(...)` assignments
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::MethodDefinition(method) = element {
                if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                    if let Some(func_body) = &method.value.body {
                        for stmt in &func_body.statements {
                            if let Statement::ExpressionStatement(es) = stmt {
                                if let Expression::AssignmentExpression(assign) = &es.expression {
                                    if assign.operator == oxc_ast::ast::AssignmentOperator::Assign {
                                        if let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) = &assign.left {
                                            if let Expression::ThisExpression(_) = &member.object {
                                                if let Some(rune_kind) = Self::detect_state_rune_kind(&assign.right) {
                                                    let name = member.property.name.to_string();
                                                    let is_state = rune_kind == RuneKind::State;
                                                    let mut backing = format!("#{}", name);
                                                    while existing_private.contains(backing.trim_start_matches('#')) {
                                                        backing = format!("#_{}", backing.trim_start_matches('#'));
                                                    }
                                                    existing_private.insert(backing.trim_start_matches('#').to_string());
                                                    fields.push(ClassStateField {
                                                        public_name: Some(name),
                                                        private_name: backing.trim_start_matches('#').to_string(),
                                                        is_state,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        ClassStateInfo { fields }
    }

    /// Rewrite class body: replace state fields with private backing + getter/setter.
    pub(super) fn rewrite_class_body(
        &self,
        body: &mut oxc_ast::ast::ClassBody<'a>,
        info: &ClassStateInfo,
    ) {
        use oxc_ast::ast::ClassElement;

        // Build a lookup: field name → ClassStateField for quick matching
        let public_fields: std::collections::HashMap<&str, &ClassStateField> = info.fields.iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();
        let private_fields: FxHashSet<&str> = info.fields.iter()
            .filter(|f| f.public_name.is_none())
            .map(|f| f.private_name.as_str())
            .collect();

        let mut new_body: Vec<ClassElement<'a>> = Vec::new();
        // Track which public field names were handled from PropertyDefinition
        let mut handled_public: FxHashSet<String> = FxHashSet::default();

        // Take ownership of old body elements
        let old_elements: Vec<ClassElement<'a>> = {
            let mut temp = self.b.ast.vec();
            std::mem::swap(&mut body.body, &mut temp);
            temp.into_iter().collect()
        };

        for element in old_elements {
            match element {
                ClassElement::PropertyDefinition(mut prop) => {
                    // Check if it's a state field
                    let is_state_prop = prop.value.as_ref().is_some_and(|v| Self::detect_state_rune_kind(v).is_some());
                    if !is_state_prop {
                        new_body.push(ClassElement::PropertyDefinition(prop));
                        continue;
                    }

                    match &prop.key {
                        oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => {
                            let name = id.name.to_string();
                            if private_fields.contains(name.as_str()) {
                                // Private field: just rewrite $state(arg) → $.state(arg)
                                if let Some(Expression::CallExpression(call)) = &mut prop.value {
                                    call.callee = self.b.rid_expr("$.state");
                                }
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            } else {
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            }
                        }
                        oxc_ast::ast::PropertyKey::StaticIdentifier(id) if !prop.computed => {
                            let name = id.name.to_string();
                            if let Some(field_info) = public_fields.get(name.as_str()) {
                                handled_public.insert(name.clone());
                                // Extract the rune argument
                                let arg = if let Some(Expression::CallExpression(mut call)) = prop.value.take() {
                                    if call.arguments.is_empty() {
                                        None
                                    } else {
                                        let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                                        Some(dummy.into_expression())
                                    }
                                } else {
                                    None
                                };

                                // Generate: #backing = $.state(arg)
                                let state_call = if let Some(arg) = arg {
                                    self.b.call_expr("$.state", [Arg::Expr(arg)])
                                } else {
                                    self.b.call_expr("$.state", std::iter::empty::<Arg<'a, '_>>())
                                };
                                new_body.push(self.b.class_private_field(
                                    &field_info.private_name,
                                    Some(state_call),
                                ));

                                // Generate getter: get name() { return $.get(this.#backing); }
                                let get_call = self.b.call_expr("$.get", [Arg::Expr(
                                    self.b.this_private_member(&field_info.private_name),
                                )]);
                                let return_stmt = self.b.return_stmt(get_call);
                                new_body.push(self.b.class_getter(
                                    self.b.public_key(&name),
                                    vec![return_stmt],
                                ));

                                // Generate setter: set name(value) { $.set(this.#backing, value, true?); }
                                let mut set_args: Vec<Arg<'a, '_>> = vec![
                                    Arg::Expr(self.b.this_private_member(&field_info.private_name)),
                                    Arg::Ident("value"),
                                ];
                                if field_info.is_state {
                                    set_args.push(Arg::Bool(true));
                                }
                                let set_call = self.b.call_stmt("$.set", set_args);
                                new_body.push(self.b.class_setter(
                                    self.b.public_key(&name),
                                    "value",
                                    vec![set_call],
                                ));
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
                    if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                        // Insert #backing; + getter + setter for constructor-originating state fields
                        let ctor_fields: Vec<&ClassStateField> = info.fields.iter()
                            .filter(|f| f.public_name.is_some() && !handled_public.contains(f.public_name.as_deref().unwrap()))
                            .collect();
                        for field_info in &ctor_fields {
                            let name = field_info.public_name.as_deref().unwrap();
                            // #backing; (no init)
                            new_body.push(self.b.class_private_field(&field_info.private_name, None));
                            // getter
                            let get_call = self.b.call_expr("$.get", [Arg::Expr(
                                self.b.this_private_member(&field_info.private_name),
                            )]);
                            let return_stmt = self.b.return_stmt(get_call);
                            new_body.push(self.b.class_getter(self.b.public_key(name), vec![return_stmt]));
                            // setter
                            let mut set_args: Vec<Arg<'a, '_>> = vec![
                                Arg::Expr(self.b.this_private_member(&field_info.private_name)),
                                Arg::Ident("value"),
                            ];
                            if field_info.is_state {
                                set_args.push(Arg::Bool(true));
                            }
                            let set_call = self.b.call_stmt("$.set", set_args);
                            new_body.push(self.b.class_setter(self.b.public_key(name), "value", vec![set_call]));
                        }
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

    /// Rewrite constructor: replace `this.name = $state(...)` with `this.#backing = $.state(...)`.
    /// Also insert `#backing;` property definitions and getter/setter before the constructor.
    pub(super) fn rewrite_constructor(
        &self,
        method: &mut oxc_allocator::Box<'a, oxc_ast::ast::MethodDefinition<'a>>,
        info: &ClassStateInfo,
    ) {
        let Some(func_body) = &mut method.value.body else { return };

        // Build lookup for constructor-originating fields
        let ctor_fields: std::collections::HashMap<&str, &ClassStateField> = info.fields.iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();

        for stmt in func_body.statements.iter_mut() {
            if let Statement::ExpressionStatement(es) = stmt {
                if let Expression::AssignmentExpression(assign) = &mut es.expression {
                    if assign.operator == oxc_ast::ast::AssignmentOperator::Assign {
                        if let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) = &assign.left {
                            if let Expression::ThisExpression(_) = &member.object {
                                let name = member.property.name.to_string();
                                if let Some(field_info) = ctor_fields.get(name.as_str()) {
                                    // Rewrite: this.name = $state(arg) → this.#backing = $.state(arg)
                                    if let Expression::CallExpression(call) = &mut assign.right {
                                        call.callee = self.b.rid_expr("$.state");
                                    }
                                    // Change left side to this.#backing
                                    let new_left = self.b.this_private_member(&field_info.private_name);
                                    // We need to convert Expression to AssignmentTarget
                                    // For private field: use PrivateFieldExpression
                                    if let Expression::PrivateFieldExpression(pfe) = new_left {
                                        assign.left = oxc_ast::ast::AssignmentTarget::PrivateFieldExpression(pfe);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check if we're inside a class body that has a private state field with given name.
    pub(super) fn is_private_state_field(&self, name: &str) -> bool {
        self.class_state_stack.last().is_some_and(|info| {
            info.fields.iter().any(|f| f.public_name.is_none() && f.private_name == name)
        })
    }

    /// Returns `Some(is_state)` for a private state field: true = `$state`, false = `$state.raw`.
    pub(super) fn private_state_field_is_state(&self, name: &str) -> Option<bool> {
        self.class_state_stack.last().and_then(|info| {
            info.fields.iter().find(|f| f.public_name.is_none() && f.private_name == name)
                .map(|f| f.is_state)
        })
    }
}
