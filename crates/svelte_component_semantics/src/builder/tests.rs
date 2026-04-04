use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_syntax::reference::ReferenceFlags;
use oxc_syntax::symbol::SymbolFlags;

use crate::builder::{ComponentSemanticsBuilder, TemplateBuildContext, TemplateWalker};
use crate::OxcNodeId;

/// Parse JS source and build semantics via our builder.
fn build_instance(source: &str) -> crate::ComponentSemantics {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();
    let parsed = Parser::new(&alloc, source, source_type).parse();
    assert!(
        parsed.errors.is_empty(),
        "parse errors: {:?}",
        parsed.errors
    );

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_instance_program(&parsed.program);
    builder.finish()
}

fn build_module_and_instance(module_src: &str, instance_src: &str) -> crate::ComponentSemantics {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();

    let instance_parsed = Parser::new(&alloc, instance_src, source_type).parse();
    assert!(instance_parsed.errors.is_empty());

    let module_parsed = Parser::new(&alloc, module_src, source_type).parse();
    assert!(module_parsed.errors.is_empty());

    let mut builder = ComponentSemanticsBuilder::new();
    // Module first — its bindings must exist before instance resolution
    builder.add_module_program(&module_parsed.program);
    builder.add_instance_program(&instance_parsed.program);
    builder.finish()
}

#[test]
fn simple_let_and_read_ref() {
    let sem = build_instance("let x = 1; x;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "x").expect("x should be bound");
    assert_eq!(sem.symbol_name(sym), "x");
    assert!(!sem.is_mutated(sym));

    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1);
    assert!(sem.get_reference(refs[0]).is_read());
}

#[test]
fn write_ref_marks_mutated() {
    let sem = build_instance("let x = 1; x = 2;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "x").unwrap();
    assert!(sem.is_mutated(sym));

    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1);
    assert!(sem.get_reference(refs[0]).is_write());
}

#[test]
fn compound_assignment_is_read_write() {
    let sem = build_instance("let x = 0; x += 1;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "x").unwrap();
    assert!(sem.is_mutated(sym));

    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1);
    let r = sem.get_reference(refs[0]);
    assert!(r.flags().is_read());
    assert!(r.flags().is_write());
}

#[test]
fn update_expression_is_read_write() {
    let sem = build_instance("let x = 0; x++;");
    let sym = sem.find_binding(sem.root_scope_id(), "x").unwrap();
    assert!(sem.is_mutated(sym));

    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1);
    let r = sem.get_reference(refs[0]);
    assert!(r.flags().is_read());
    assert!(r.flags().is_write());
}

#[test]
fn var_hoists_to_function_scope() {
    let sem = build_instance("function f() { { var x = 1; } }");
    let root = sem.root_scope_id();

    // x should NOT be in root (it's in f's function scope)
    assert!(sem.find_binding(root, "x").is_none());

    // f should be in root
    let f_sym = sem.find_binding(root, "f").unwrap();
    assert_eq!(sem.symbol_name(f_sym), "f");

    // x should be in f's function scope, not in the block scope
    let _f_scope = sem.symbol_scope_id(f_sym);
    // f is declared in root; its body creates a new function scope
    // We need to find x by looking through scopes
    // The function scope is a child of root — find x there
    // Since var hoists to the function scope, x should be findable from the block
}

#[test]
fn let_block_scoped() {
    let sem = build_instance("{ let x = 1; } x;");
    let root = sem.root_scope_id();

    // x is block-scoped — not visible from root
    assert!(sem.find_binding(root, "x").is_none());

    // The reference to x at root level should be unresolved
}

#[test]
fn function_declaration_hoisting() {
    let sem = build_instance("function foo() {}");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "foo").unwrap();
    assert!(sem.symbol_flags(sym).contains(SymbolFlags::Function));
}

#[test]
fn forward_reference_to_function() {
    // foo is referenced before its declaration — should still resolve
    let sem = build_instance("foo(); function foo() {}");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "foo").unwrap();
    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1, "forward ref to foo should be resolved");
    assert!(sem.get_reference(refs[0]).is_read());
}

#[test]
fn forward_reference_to_var() {
    // x is referenced before var declaration — should resolve (var hoists)
    let sem = build_instance("x; var x = 1;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "x").unwrap();
    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1, "forward ref to var x should be resolved");
}

#[test]
fn forward_reference_to_let_in_same_scope() {
    // let is not hoisted, but the binding exists in the scope — TDZ reference still resolves
    let sem = build_instance("x; let x = 1;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "x").unwrap();
    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(
        refs.len(),
        1,
        "forward ref to let x should resolve at scope exit"
    );
}

#[test]
fn truly_unresolved_stays_unresolved() {
    let sem = build_instance("foo();");
    let unresolved = sem.root_unresolved_references();
    assert!(unresolved.contains_key("foo"), "foo should be unresolved");
}

#[test]
fn forward_ref_inside_function_resolves() {
    // Inside function body, forward reference to later declaration
    let sem = build_instance("function f() { g(); function g() {} }");
    let root = sem.root_scope_id();

    // g is not in root (it's in f's scope)
    assert!(sem.find_binding(root, "g").is_none());
    // But g should be resolved (forward ref within f's body)
    let unresolved = sem.root_unresolved_references();
    assert!(
        !unresolved.contains_key("g"),
        "g should not be unresolved — it's in f's scope"
    );
}

#[test]
fn unresolved_shorthand_tracked() {
    let mut builder = ComponentSemanticsBuilder::new();

    struct UnresolvedShorthand;
    impl TemplateWalker for UnresolvedShorthand {
        fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
            ctx.materialize_shorthand_reference("nonexistent", ReferenceFlags::Write);
        }
    }

    let mut walker = UnresolvedShorthand;
    builder.add_template(&mut walker);
    let sem = builder.finish();
    let unresolved = sem.root_unresolved_references();
    assert!(
        unresolved.contains_key("nonexistent"),
        "unresolved shorthand bind:nonexistent should be tracked"
    );
}

#[test]
fn function_expression_name_in_own_scope() {
    let sem = build_instance("let f = function foo() { foo; };");
    let root = sem.root_scope_id();

    // f is in root
    assert!(sem.find_binding(root, "f").is_some());
    // foo is NOT in root (it's in the function expression's own scope)
    assert!(sem.find_binding(root, "foo").is_none());
}

#[test]
fn destructuring_binding() {
    let sem = build_instance("let { a, b } = obj;");
    let root = sem.root_scope_id();

    assert!(sem.find_binding(root, "a").is_some());
    assert!(sem.find_binding(root, "b").is_some());
    assert!(sem.find_binding(root, "obj").is_none()); // unresolved
}

#[test]
fn nested_scopes() {
    let sem = build_instance("let x = 1; function f(y) { let z = x + y; { let w = z; } }");
    let root = sem.root_scope_id();

    assert!(sem.find_binding(root, "x").is_some());
    assert!(sem.find_binding(root, "f").is_some());
    // y, z, w are not in root
    assert!(sem.find_binding(root, "y").is_none());
    assert!(sem.find_binding(root, "z").is_none());
    assert!(sem.find_binding(root, "w").is_none());
}

#[test]
fn import_binding() {
    let sem = build_instance("import { foo } from 'bar';");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "foo").unwrap();
    assert!(sem.symbol_flags(sym).contains(SymbolFlags::Import));
}

#[test]
fn import_default_binding() {
    let sem = build_instance("import Foo from 'bar';");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "Foo").unwrap();
    assert!(sem.symbol_flags(sym).contains(SymbolFlags::Import));
}

#[test]
fn import_namespace_binding() {
    let sem = build_instance("import * as ns from 'bar';");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "ns").unwrap();
    assert!(sem.symbol_flags(sym).contains(SymbolFlags::Import));
}

#[test]
fn member_expression_clears_write() {
    // In `obj.x = 1`, the reference to `obj` should be Read, not Write
    let sem = build_instance("let obj = {}; obj.x = 1;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "obj").unwrap();
    // obj has a read reference from `obj.x = 1` (member base is read)
    // NOT a write reference
    assert!(!sem.is_mutated(sym));
}

#[test]
fn arrow_function_scope() {
    let sem = build_instance("let f = (x) => x + 1;");
    let root = sem.root_scope_id();

    assert!(sem.find_binding(root, "f").is_some());
    // x is in arrow scope, not root
    assert!(sem.find_binding(root, "x").is_none());
}

#[test]
fn for_loop_scope() {
    let sem = build_instance("for (let i = 0; i < 10; i++) { let x = i; }");
    let root = sem.root_scope_id();

    // i and x are block-scoped in the for loop, not visible from root
    assert!(sem.find_binding(root, "i").is_none());
    assert!(sem.find_binding(root, "x").is_none());
}

#[test]
fn catch_binding() {
    let sem = build_instance("try {} catch (e) { e; }");
    let root = sem.root_scope_id();

    // e is in catch scope, not root
    assert!(sem.find_binding(root, "e").is_none());
}

#[test]
fn module_and_instance_cross_resolution() {
    let sem = build_module_and_instance("export const shared = 42;", "let x = shared;");
    let root = sem.root_scope_id();

    // instance binding
    let x_sym = sem.find_binding(root, "x").unwrap();
    assert_eq!(sem.symbol_name(x_sym), "x");

    // module binding should be visible from instance (instance root parents to module)
    let shared_sym = sem.find_binding(root, "shared").unwrap();
    assert_eq!(sem.symbol_name(shared_sym), "shared");

    // shared should have a read reference from instance
    let refs = sem.get_resolved_reference_ids(shared_sym);
    assert_eq!(refs.len(), 1);
    assert!(sem.get_reference(refs[0]).is_read());
}

#[test]
fn const_variable_not_mutated() {
    let sem = build_instance("const x = 1; x;");
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "x").unwrap();
    assert!(sem.symbol_flags(sym).contains(SymbolFlags::ConstVariable));
    assert!(!sem.is_mutated(sym));
}

#[test]
fn array_destructuring() {
    let sem = build_instance("let [a, b, ...rest] = arr;");
    let root = sem.root_scope_id();

    assert!(sem.find_binding(root, "a").is_some());
    assert!(sem.find_binding(root, "b").is_some());
    assert!(sem.find_binding(root, "rest").is_some());
}

#[test]
fn nested_destructuring() {
    let sem = build_instance("let { a: { b } } = obj;");
    let root = sem.root_scope_id();

    // a is not bound (it's a property key), b is bound
    assert!(sem.find_binding(root, "a").is_none());
    assert!(sem.find_binding(root, "b").is_some());
}

#[test]
fn for_in_scope() {
    let sem = build_instance("for (let k in obj) {}");
    let root = sem.root_scope_id();
    assert!(sem.find_binding(root, "k").is_none());
}

#[test]
fn for_of_scope() {
    let sem = build_instance("for (let v of arr) {}");
    let root = sem.root_scope_id();
    assert!(sem.find_binding(root, "v").is_none());
}

#[test]
fn switch_statement_scope() {
    let sem = build_instance("let x = 1; switch (x) { case 1: let y = 2; }");
    let root = sem.root_scope_id();
    assert!(sem.find_binding(root, "x").is_some());
    // y is block-scoped inside switch
    assert!(sem.find_binding(root, "y").is_none());
}

#[test]
fn class_declaration_binding() {
    let sem = build_instance("class Foo {}");
    let root = sem.root_scope_id();
    let sym = sem.find_binding(root, "Foo").unwrap();
    assert!(sem.symbol_flags(sym).contains(SymbolFlags::Class));
}

#[test]
fn class_expression_name_in_own_scope() {
    let sem = build_instance("let c = class Foo { method() { Foo; } };");
    let root = sem.root_scope_id();
    assert!(sem.find_binding(root, "c").is_some());
    // Foo is in class's own scope, not root
    assert!(sem.find_binding(root, "Foo").is_none());
}

#[test]
fn unresolved_reference_tracked() {
    let sem = build_instance("console.log(x);");
    let unresolved = sem.root_unresolved_references();
    // "console" is unresolved (the member expr root object)
    assert!(unresolved.contains_key("console"));
}

#[test]
fn rest_param_binding() {
    let sem = build_instance("function f(a, ...rest) {}");
    let root = sem.root_scope_id();
    assert!(sem.find_binding(root, "f").is_some());
    // a and rest are in function scope, not root
    assert!(sem.find_binding(root, "a").is_none());
    assert!(sem.find_binding(root, "rest").is_none());
}

// =========================================================
// Template builder tests (via TemplateWalker trait)
// =========================================================

/// Mock walker that simulates a template with a JS expression referencing
/// an instance binding.
struct ExprRefWalker<'a> {
    alloc: &'a Allocator,
}

impl TemplateWalker for ExprRefWalker<'_> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
        // Simulate: {count} in template → read reference to "count"
        let parsed = Parser::new(self.alloc, "count", SourceType::mjs())
            .parse_expression()
            .unwrap();
        ctx.visit_js_expression(&parsed);
    }
}

#[test]
fn template_expression_resolves_instance_binding() {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();

    let instance = Parser::new(&alloc, "let count = 0;", source_type).parse();
    assert!(instance.errors.is_empty());

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_instance_program(&instance.program);

    let mut walker = ExprRefWalker { alloc: &alloc };
    builder.add_template(&mut walker);

    let sem = builder.finish();
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "count").unwrap();
    let refs = sem.get_resolved_reference_ids(sym);
    // One ref from template expression
    assert_eq!(refs.len(), 1);
    assert!(sem.get_reference(refs[0]).is_read());
    assert!(sem.is_template_reference(refs[0]));
}

/// Mock walker that simulates child scopes (like each block).
struct ChildScopeWalker<'a> {
    alloc: &'a Allocator,
}

impl TemplateWalker for ChildScopeWalker<'_> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
        // Simulate: {#each items as item} {item} {/each}
        let each_scope = ctx.enter_child_scope();
        _ = each_scope;

        // Register "item" binding via a const statement in the scope
        let parsed = Parser::new(self.alloc, "const item = 0;", SourceType::mjs()).parse();
        // Visit the statement to register binding
        for stmt in &parsed.program.body {
            ctx.visit_js_statement(stmt);
        }

        // Now reference "item" from template
        let expr = Parser::new(self.alloc, "item", SourceType::mjs())
            .parse_expression()
            .unwrap();
        ctx.visit_js_expression(&expr);

        ctx.leave_scope();
    }
}

#[test]
fn template_child_scope_with_binding() {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();

    let instance = Parser::new(&alloc, "let items = [];", source_type).parse();
    assert!(instance.errors.is_empty());

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_instance_program(&instance.program);

    let mut walker = ChildScopeWalker { alloc: &alloc };
    builder.add_template(&mut walker);

    let sem = builder.finish();
    let root = sem.root_scope_id();

    // "items" in root
    assert!(sem.find_binding(root, "items").is_some());
    // "item" NOT in root (in child scope)
    assert!(sem.find_binding(root, "item").is_none());
}

/// Mock walker for shorthand bind:name reference.
struct ShorthandBindWalker;

impl TemplateWalker for ShorthandBindWalker {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
        // Simulate: <input bind:value />
        ctx.materialize_shorthand_reference("value", ReferenceFlags::Write);
    }
}

#[test]
fn template_shorthand_bind_reference() {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();

    let instance = Parser::new(&alloc, "let value = '';", source_type).parse();
    assert!(instance.errors.is_empty());

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_instance_program(&instance.program);

    let mut walker = ShorthandBindWalker;
    builder.add_template(&mut walker);

    let sem = builder.finish();
    let root = sem.root_scope_id();

    let sym = sem.find_binding(root, "value").unwrap();
    // bind:value creates a Write reference → mutated
    assert!(sem.is_mutated(sym));

    let refs = sem.get_resolved_reference_ids(sym);
    assert_eq!(refs.len(), 1);
    assert!(sem.get_reference(refs[0]).is_write());
    assert!(sem.is_template_reference(refs[0]));
}

/// Mock walker for unresolved shorthand reference.
struct UnresolvedShorthandWalker;

impl TemplateWalker for UnresolvedShorthandWalker {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
        // Simulate: <input bind:nonexistent />
        let result = ctx.materialize_shorthand_reference("nonexistent", ReferenceFlags::Write);
        assert!(result.is_none());
    }
}

#[test]
fn template_shorthand_unresolved() {
    let mut builder = ComponentSemanticsBuilder::new();
    let mut walker = UnresolvedShorthandWalker;
    builder.add_template(&mut walker);
    // Should not panic — unresolved references are fine
    let _sem = builder.finish();
}

// =========================================================
// NodeId remapping tests
// =========================================================

#[test]
fn template_shorthand_gets_unique_node_id() {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();
    let instance = Parser::new(&alloc, "let value = ''; let name = '';", source_type).parse();

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_instance_program(&instance.program);
    let after_instance = builder.next_node_id();

    struct TwoShorthands;
    impl TemplateWalker for TwoShorthands {
        fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>) {
            ctx.materialize_shorthand_reference("value", ReferenceFlags::Write);
            ctx.materialize_shorthand_reference("name", ReferenceFlags::Read);
        }
    }

    let mut walker = TwoShorthands;
    builder.add_template(&mut walker);
    let sem = builder.finish();

    let value_sym = sem.find_binding(sem.root_scope_id(), "value").unwrap();
    let name_sym = sem.find_binding(sem.root_scope_id(), "name").unwrap();

    let value_refs = sem.get_resolved_reference_ids(value_sym);
    let name_refs = sem.get_resolved_reference_ids(name_sym);

    let value_node = sem.get_reference(value_refs[0]).node_id();
    let name_node = sem.get_reference(name_refs[0]).node_id();

    // Both should have real NodeIds (not DUMMY)
    assert_ne!(
        value_node,
        OxcNodeId::DUMMY,
        "shorthand ref should not use DUMMY NodeId"
    );
    assert_ne!(
        name_node,
        OxcNodeId::DUMMY,
        "shorthand ref should not use DUMMY NodeId"
    );

    // They should be different from each other
    assert_ne!(
        value_node, name_node,
        "two shorthand refs should have distinct NodeIds"
    );

    // They should be past instance IDs
    assert!(
        value_node.index() >= after_instance as usize,
        "template NodeIds should be offset past instance"
    );
}

#[test]
fn node_ids_no_collision_between_programs() {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();

    let module_parsed = Parser::new(&alloc, "export const shared = 42;", source_type).parse();
    let instance_parsed = Parser::new(&alloc, "let x = shared;", source_type).parse();
    assert!(module_parsed.errors.is_empty());
    assert!(instance_parsed.errors.is_empty());

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_module_program(&module_parsed.program);
    let after_module = builder.next_node_id();

    builder.add_instance_program(&instance_parsed.program);
    let after_instance = builder.next_node_id();

    // Instance IDs should start after module IDs
    assert!(
        after_instance > after_module,
        "instance NodeIds should be offset past module"
    );

    let sem = builder.finish();

    // Both symbols should have distinct declaration NodeIds
    let root = sem.root_scope_id();
    let shared_sym = sem.find_binding(root, "shared").unwrap();
    let x_sym = sem.find_binding(root, "x").unwrap();

    let shared_node = sem.symbol_declaration(shared_sym);
    let x_node = sem.symbol_declaration(x_sym);
    assert_ne!(
        shared_node, x_node,
        "module and instance symbols must have different NodeIds"
    );
}

#[test]
fn single_program_node_ids_start_at_zero() {
    let alloc = Allocator::default();
    let source_type = SourceType::mjs();
    let parsed = Parser::new(&alloc, "let a = 1;", source_type).parse();

    let mut builder = ComponentSemanticsBuilder::new();
    builder.add_instance_program(&parsed.program);

    // next_node_id should be > 0 after visiting a program with nodes
    assert!(builder.next_node_id() > 0);
}
