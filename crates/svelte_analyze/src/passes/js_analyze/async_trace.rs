use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, AssignmentTarget, CallExpression, Expression,
    Function, FunctionBody, IdentifierReference, Program, ReturnStatement, SimpleAssignmentTarget,
    Statement, UpdateExpression, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk;
use oxc_semantic::ScopeFlags;
use rustc_hash::FxHashMap;
use svelte_component_semantics::{ComponentSemantics, SymbolId};

pub(super) struct AssignedValues {
    deps: FxHashMap<SymbolId, Vec<SymbolId>>,
}

impl AssignedValues {
    pub(super) fn collect<'a>(
        scoping: &ComponentSemantics<'a>,
        program: &Program<'a>,
    ) -> AssignedValues {
        let mut collector = AssignedValuesCollector {
            scoping,
            deps: FxHashMap::default(),
        };
        collector.visit_program(program);
        AssignedValues {
            deps: collector.deps,
        }
    }

    fn get(&self, sym: SymbolId) -> &[SymbolId] {
        match self.deps.get(&sym) {
            Some(deps) => deps.as_slice(),
            None => &[],
        }
    }
}

struct AssignedValuesCollector<'s, 'a> {
    scoping: &'s ComponentSemantics<'a>,
    deps: FxHashMap<SymbolId, Vec<SymbolId>>,
}

impl<'s, 'a> AssignedValuesCollector<'s, 'a> {
    fn record(&mut self, targets: Vec<SymbolId>, value: &Expression<'a>) {
        let mut refs = ReferencedSymbols {
            scoping: self.scoping,
            symbols: Vec::new(),
        };
        refs.visit_expression(value);
        for target in targets {
            let deps = self.deps.entry(target).or_default();
            for sym in &refs.symbols {
                if !deps.contains(sym) {
                    deps.push(*sym);
                }
            }
        }
    }
}

impl<'a> Visit<'a> for AssignedValuesCollector<'_, 'a> {
    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        if let Some(id) = func.id.as_ref()
            && let Some(sym) = id.symbol_id.get()
            && let Some(body) = func.body.as_ref()
        {
            let mut refs = ReferencedSymbols {
                scoping: self.scoping,
                symbols: Vec::new(),
            };
            refs.visit_function_body(body);
            self.deps.entry(sym).or_default().extend(refs.symbols);
        }
        walk::walk_function(self, func, flags);
    }

    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'a>) {
        if let Some(init) = declarator.init.as_ref() {
            let mut syms = Vec::new();
            svelte_component_semantics::walk_bindings(&declarator.id, |v| syms.push(v.symbol));
            self.record(syms, init);
        }
        walk::walk_variable_declarator(self, declarator);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        if let AssignmentTarget::AssignmentTargetIdentifier(ident) = &expr.left
            && let Some(sym) = self.scoping.symbol_for_identifier_reference(ident)
        {
            self.record(vec![sym], &expr.right);
        }
        walk::walk_assignment_expression(self, expr);
    }
}

struct ReferencedSymbols<'s, 'a> {
    scoping: &'s ComponentSemantics<'a>,
    symbols: Vec<SymbolId>,
}

impl<'a> Visit<'a> for ReferencedSymbols<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(sym) = self.scoping.symbol_for_identifier_reference(ident) else {
            return;
        };
        if self.symbols.contains(&sym) {
            return;
        }
        self.symbols.push(sym);
    }
}

pub(super) struct TouchedSymbols<'s, 'a> {
    scoping: &'s ComponentSemantics<'a>,
    assigned: &'s AssignedValues,
    pub(super) writes: Vec<SymbolId>,
    pub(super) reads: Vec<SymbolId>,
}

impl<'s, 'a> TouchedSymbols<'s, 'a> {
    pub(super) fn new(scoping: &'s ComponentSemantics<'a>, assigned: &'s AssignedValues) -> Self {
        Self {
            scoping,
            assigned,
            writes: Vec::new(),
            reads: Vec::new(),
        }
    }

    pub(super) fn visit_init(&mut self, expr: &Expression<'a>) {
        self.visit_expression(expr);
    }

    pub(super) fn visit_body(&mut self, body: &FunctionBody<'a>) {
        walk::walk_function_body(self, body);
    }

    pub(super) fn visit_function_like(&mut self, expr: &Expression<'a>) {
        match expr {
            Expression::ArrowFunctionExpression(arrow) => {
                walk::walk_function_body(self, &arrow.body);
            }
            Expression::FunctionExpression(func) => {
                if let Some(body) = func.body.as_ref() {
                    walk::walk_function_body(self, body);
                }
            }
            _ => {}
        }
    }

    fn record_write(&mut self, sym: SymbolId) {
        if !self.writes.contains(&sym) {
            self.writes.push(sym);
        }
    }

    fn record_read(&mut self, sym: SymbolId) {
        if !self.reads.contains(&sym) {
            self.reads.push(sym);
        }
    }

    fn resolve(&self, ident: &IdentifierReference<'a>) -> Option<SymbolId> {
        self.scoping.symbol_for_identifier_reference(ident)
    }

    fn expand(&self, seeds: Vec<SymbolId>) -> Vec<SymbolId> {
        let mut touched = seeds;
        let mut index = 0;
        while index < touched.len() {
            let sym = touched[index];
            index += 1;
            for &dep in self.assigned.get(sym) {
                if !touched.contains(&dep) {
                    touched.push(dep);
                }
            }
        }
        touched
    }

    fn touch_call(&self, call: &CallExpression<'a>) -> Vec<SymbolId> {
        let mut refs = ReferencedSymbols {
            scoping: self.scoping,
            symbols: Vec::new(),
        };
        refs.visit_call_expression(call);
        self.expand(refs.symbols)
    }

    fn touch_expression(&self, expr: &Expression<'a>) -> Vec<SymbolId> {
        let mut refs = ReferencedSymbols {
            scoping: self.scoping,
            symbols: Vec::new(),
        };
        refs.visit_expression(expr);
        self.expand(refs.symbols)
    }

    fn write_target(&mut self, target: &AssignmentTarget<'a>) {
        match target {
            AssignmentTarget::AssignmentTargetIdentifier(ident) => {
                if let Some(sym) = self.resolve(ident) {
                    self.record_write(sym);
                }
            }
            AssignmentTarget::StaticMemberExpression(member) => {
                if let Some(sym) = member_root(&member.object).and_then(|i| self.resolve(i)) {
                    self.record_write(sym);
                }
            }
            AssignmentTarget::ComputedMemberExpression(member) => {
                if let Some(sym) = member_root(&member.object).and_then(|i| self.resolve(i)) {
                    self.record_write(sym);
                }
            }
            other => {
                let mut refs = ReferencedSymbols {
                    scoping: self.scoping,
                    symbols: Vec::new(),
                };
                refs.visit_assignment_target(other);
                for sym in refs.symbols {
                    self.record_write(sym);
                }
            }
        }
    }
}

fn is_effect_call(expr: &CallExpression<'_>) -> bool {
    let Expression::Identifier(ident) = expr.callee.get_inner_expression() else {
        return false;
    };
    ident.name.as_str() == svelte_ast::RUNE_EFFECT
}

fn member_root<'b, 'a>(expr: &'b Expression<'a>) -> Option<&'b IdentifierReference<'a>> {
    match expr.get_inner_expression() {
        Expression::Identifier(ident) => Some(ident),
        Expression::StaticMemberExpression(member) => member_root(&member.object),
        Expression::ComputedMemberExpression(member) => member_root(&member.object),
        _ => None,
    }
}

impl<'a> Visit<'a> for TouchedSymbols<'_, 'a> {
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        self.write_target(&expr.left);
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        let root = match &expr.argument {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => self.resolve(ident),
            SimpleAssignmentTarget::StaticMemberExpression(member) => {
                member_root(&member.object).and_then(|ident| self.resolve(ident))
            }
            SimpleAssignmentTarget::ComputedMemberExpression(member) => {
                member_root(&member.object).and_then(|ident| self.resolve(ident))
            }
            _ => None,
        };
        if let Some(sym) = root {
            self.record_write(sym);
        }
        walk::walk_update_expression(self, expr);
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        if is_effect_call(expr) {
            return;
        }
        for sym in self.touch_call(expr) {
            self.record_write(sym);
        }
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement<'a>) {
        if let Some(argument) = stmt.argument.as_ref() {
            for sym in self.touch_expression(argument) {
                self.record_read(sym);
            }
        }
        walk::walk_return_statement(self, stmt);
    }

    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(sym) = self.resolve(ident) {
            self.record_read(sym);
        }
    }

    fn visit_arrow_function_expression(&mut self, _arrow: &ArrowFunctionExpression<'a>) {}

    fn visit_function(&mut self, _func: &Function<'a>, _flags: ScopeFlags) {}
}

pub(super) fn trace_statement<'s, 'a>(
    scoping: &'s ComponentSemantics<'a>,
    assigned: &'s AssignedValues,
    stmt: &Statement<'a>,
) -> TouchedSymbols<'s, 'a> {
    let mut tracer = TouchedSymbols::new(scoping, assigned);
    tracer.visit_statement(stmt);
    tracer
}
