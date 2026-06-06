use oxc_ast::ast::{
    AssignmentTarget, AssignmentTargetMaybeDefault, AssignmentTargetProperty, BindingPattern,
    Expression, IdentifierReference, PropertyKey,
};
use smallvec::SmallVec;

use crate::SymbolId;

pub fn walk_assignment_target_idents<'a, F>(target: &'a AssignmentTarget<'a>, mut visit: F)
where
    F: FnMut(&'a IdentifierReference<'a>),
{
    walk_assignment_targets(target, |v| {
        if let WriteTarget::Identifier(id) = v.target {
            visit(id);
        }
    });
}

pub fn walk_bindings<'a, F>(pat: &'a BindingPattern<'a>, mut visit: F)
where
    F: FnMut(BindingVisit<'a, '_>),
{
    let mut path: SmallVec<[Step<'a>; 4]> = SmallVec::new();
    walk_inner(pat, &mut path, &mut visit);
}

pub struct BindingVisit<'a, 'p> {
    pub symbol: SymbolId,

    pub path: &'p [Step<'a>],

    pub is_rest: bool,

    pub excluded: &'p [&'a PropertyKey<'a>],
}

#[derive(Clone, Copy)]
pub struct Step<'a> {
    pub access: Access<'a>,

    pub default: Option<&'a Expression<'a>>,
}

#[derive(Clone, Copy)]
pub enum Access<'a> {
    Key {
        key: &'a PropertyKey<'a>,
        computed: bool,
    },

    Index {
        index: u32,
        len: u32,
        has_rest: bool,
    },

    Slice {
        from: u32,
    },
}

fn walk_inner<'a, F>(pat: &'a BindingPattern<'a>, path: &mut SmallVec<[Step<'a>; 4]>, visit: &mut F)
where
    F: FnMut(BindingVisit<'a, '_>),
{
    match pat {
        BindingPattern::BindingIdentifier(ident) => {
            let Some(symbol) = ident.symbol_id.get() else {
                return;
            };
            visit(BindingVisit {
                symbol,
                path,
                is_rest: false,
                excluded: &[],
            });
        }
        BindingPattern::AssignmentPattern(assign) => {
            if let Some(last) = path.last_mut() {
                last.default = Some(&assign.right);
            }
            walk_inner(&assign.left, path, visit);
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                path.push(Step {
                    access: Access::Key {
                        key: &prop.key,
                        computed: prop.computed,
                    },
                    default: None,
                });
                walk_inner(&prop.value, path, visit);
                path.pop();
            }
            if let Some(rest) = &obj.rest {
                let BindingPattern::BindingIdentifier(ident) = &rest.argument else {
                    return;
                };
                let Some(symbol) = ident.symbol_id.get() else {
                    return;
                };
                let excluded: SmallVec<[&'a PropertyKey<'a>; 4]> =
                    obj.properties.iter().map(|p| &p.key).collect();
                visit(BindingVisit {
                    symbol,
                    path,
                    is_rest: true,
                    excluded: &excluded,
                });
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            let len = arr.elements.len() as u32;
            let has_rest = arr.rest.is_some();
            for (i, el) in arr.elements.iter().enumerate() {
                let Some(el) = el else { continue };
                path.push(Step {
                    access: Access::Index {
                        index: i as u32,
                        len,
                        has_rest,
                    },
                    default: None,
                });
                walk_inner(el, path, visit);
                path.pop();
            }
            if let Some(rest) = &arr.rest {
                path.push(Step {
                    access: Access::Slice { from: len },
                    default: None,
                });
                walk_inner(&rest.argument, path, visit);
                path.pop();
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum WriteTarget<'a> {
    Identifier(&'a IdentifierReference<'a>),

    Member(&'a AssignmentTarget<'a>),
}

#[derive(Clone, Copy)]
pub enum WriteAccess<'a> {
    Index { index: u32, len: u32, has_rest: bool },

    Slice { from: u32 },

    Key { name: &'a str },

    Computed { key: &'a Expression<'a> },
}

#[derive(Clone, Copy)]
pub struct WriteStep<'a> {
    pub access: WriteAccess<'a>,

    pub default: Option<&'a Expression<'a>>,
}

pub struct AssignmentTargetVisit<'a, 'p> {
    pub target: WriteTarget<'a>,

    pub path: &'p [WriteStep<'a>],

    pub excluded: &'p [&'a str],
}

pub fn walk_assignment_targets<'a, F>(target: &'a AssignmentTarget<'a>, mut visit: F)
where
    F: FnMut(AssignmentTargetVisit<'a, '_>),
{
    let mut path: SmallVec<[WriteStep<'a>; 4]> = SmallVec::new();
    walk_target_inner(target, &mut path, &[], &mut visit);
}

fn write_maybe_default<'a>(
    el: &'a AssignmentTargetMaybeDefault<'a>,
) -> (&'a AssignmentTarget<'a>, Option<&'a Expression<'a>>) {
    match el {
        AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(with_def) => {
            (&with_def.binding, Some(&with_def.init))
        }
        other => (
            other
                .as_assignment_target()
                .expect("non-default maybe-default is an assignment target"),
            None,
        ),
    }
}

fn write_static_key<'a>(key: &'a PropertyKey<'a>, computed: bool) -> Option<&'a str> {
    if computed {
        return None;
    }
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        PropertyKey::StringLiteral(s) => Some(s.value.as_str()),
        _ => None,
    }
}

fn walk_target_inner<'a, F>(
    target: &'a AssignmentTarget<'a>,
    path: &mut SmallVec<[WriteStep<'a>; 4]>,
    excluded: &[&'a str],
    visit: &mut F,
) where
    F: FnMut(AssignmentTargetVisit<'a, '_>),
{
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(id) => visit(AssignmentTargetVisit {
            target: WriteTarget::Identifier(id),
            path,
            excluded,
        }),
        AssignmentTarget::StaticMemberExpression(_)
        | AssignmentTarget::ComputedMemberExpression(_)
        | AssignmentTarget::PrivateFieldExpression(_) => visit(AssignmentTargetVisit {
            target: WriteTarget::Member(target),
            path,
            excluded,
        }),
        AssignmentTarget::ArrayAssignmentTarget(arr) => {
            let len = arr.elements.len() as u32;
            let has_rest = arr.rest.is_some();
            for (i, el) in arr.elements.iter().enumerate() {
                let Some(el) = el else { continue };
                let (inner, default) = write_maybe_default(el);
                path.push(WriteStep {
                    access: WriteAccess::Index {
                        index: i as u32,
                        len,
                        has_rest,
                    },
                    default,
                });
                walk_target_inner(inner, path, &[], visit);
                path.pop();
            }
            if let Some(rest) = &arr.rest {
                path.push(WriteStep {
                    access: WriteAccess::Slice { from: len },
                    default: None,
                });
                walk_target_inner(&rest.target, path, &[], visit);
                path.pop();
            }
        }
        AssignmentTarget::ObjectAssignmentTarget(obj) => {
            let mut keys: SmallVec<[&'a str; 4]> = SmallVec::new();
            for prop in &obj.properties {
                match prop {
                    AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(sh) => {
                        let name = sh.binding.name.as_str();
                        keys.push(name);
                        path.push(WriteStep {
                            access: WriteAccess::Key { name },
                            default: sh.init.as_ref(),
                        });
                        visit(AssignmentTargetVisit {
                            target: WriteTarget::Identifier(&sh.binding),
                            path,
                            excluded: &[],
                        });
                        path.pop();
                    }
                    AssignmentTargetProperty::AssignmentTargetPropertyProperty(kv) => {
                        let access = match &kv.name {
                            PropertyKey::StaticIdentifier(id) if !kv.computed => {
                                let name = id.name.as_str();
                                keys.push(name);
                                WriteAccess::Key { name }
                            }
                            _ => {
                                if let Some(name) = write_static_key(&kv.name, kv.computed) {
                                    keys.push(name);
                                }
                                match kv.name.as_expression() {
                                    Some(key) => WriteAccess::Computed { key },
                                    None => continue,
                                }
                            }
                        };
                        let (inner, default) = write_maybe_default(&kv.binding);
                        path.push(WriteStep { access, default });
                        walk_target_inner(inner, path, &[], visit);
                        path.pop();
                    }
                }
            }
            if let Some(rest) = &obj.rest {
                walk_target_inner(&rest.target, path, &keys, visit);
            }
        }
        AssignmentTarget::TSAsExpression(_)
        | AssignmentTarget::TSSatisfiesExpression(_)
        | AssignmentTarget::TSNonNullExpression(_)
        | AssignmentTarget::TSTypeAssertion(_) => {
            unreachable!("TypeScript is stripped before assignment-target traversal")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    fn summarize(source: &str) -> Vec<String> {
        let alloc = Allocator::default();
        let ret = Parser::new(&alloc, source, SourceType::mjs()).parse();
        assert!(ret.errors.is_empty(), "parse errors: {:?}", ret.errors);
        let stmt = ret.program.body.first().expect("one statement");
        let Statement::VariableDeclaration(decl) = stmt else {
            panic!("expected var declaration");
        };
        let declarator = decl.declarations.first().expect("one declarator");
        summarize_pat(&declarator.id)
    }

    fn summarize_pat(pat: &BindingPattern<'_>) -> Vec<String> {
        seed_symbol_ids(pat);

        let mut out: Vec<String> = Vec::new();
        walk_bindings(pat, |v| {
            let path = v
                .path
                .iter()
                .map(|s| {
                    let mut label = match s.access {
                        Access::Key {
                            key,
                            computed: false,
                        } => match key {
                            PropertyKey::StaticIdentifier(id) => format!(".{}", id.name),
                            PropertyKey::StringLiteral(s) => format!(".{:?}", s.value),
                            _ => ".?".into(),
                        },
                        Access::Key { computed: true, .. } => "[expr]".into(),
                        Access::Index { index, .. } => format!("[{}]", index),
                        Access::Slice { from } => format!("[slice {}]", from),
                    };
                    if s.default.is_some() {
                        label.push_str("={d}");
                    }
                    label
                })
                .collect::<String>();
            let tag = if v.is_rest {
                let excluded = v
                    .excluded
                    .iter()
                    .map(|k| match k {
                        PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                        _ => "?".into(),
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("rest({}) excl=[{}]", sym_ordinal(v.symbol), excluded)
            } else {
                format!("leaf({})", sym_ordinal(v.symbol))
            };
            out.push(format!("{path} {tag}"));
        });
        out
    }

    fn seed_symbol_ids(pat: &BindingPattern<'_>) {
        use oxc_syntax::symbol::SymbolId;
        use std::cell::Cell;
        thread_local! {
            static COUNTER: Cell<u32> = const { Cell::new(0) };
        }
        fn next_id() -> SymbolId {
            COUNTER.with(|c| {
                let v = c.get();
                c.set(v + 1);
                SymbolId::from_usize(v as usize)
            })
        }
        fn visit(pat: &BindingPattern<'_>) {
            match pat {
                BindingPattern::BindingIdentifier(id) => {
                    id.symbol_id.set(Some(next_id()));
                }
                BindingPattern::AssignmentPattern(a) => visit(&a.left),
                BindingPattern::ObjectPattern(o) => {
                    for p in &o.properties {
                        visit(&p.value);
                    }
                    if let Some(r) = &o.rest {
                        visit(&r.argument);
                    }
                }
                BindingPattern::ArrayPattern(a) => {
                    for e in a.elements.iter().flatten() {
                        visit(e);
                    }
                    if let Some(r) = &a.rest {
                        visit(&r.argument);
                    }
                }
            }
        }
        COUNTER.with(|c| c.set(0));
        visit(pat);
    }

    fn sym_ordinal(sym: SymbolId) -> u32 {
        sym.index() as u32
    }

    #[test]
    fn plain_identifier_root() {
        assert_eq!(summarize("let x = 0;"), vec![" leaf(0)"]);
    }

    #[test]
    fn flat_object_destructure() {
        assert_eq!(
            summarize("let { a, b } = 0;"),
            vec![".a leaf(0)", ".b leaf(1)"]
        );
    }

    #[test]
    fn aliased_property() {
        assert_eq!(summarize("let { a: b } = 0;"), vec![".a leaf(0)"]);
    }

    #[test]
    fn leaf_default() {
        assert_eq!(summarize("let { a = 5 } = 0;"), vec![".a={d} leaf(0)"]);
    }

    #[test]
    fn intermediate_default() {
        assert_eq!(
            summarize("let { a: { b } = {} } = 0;"),
            vec![".a={d}.b leaf(0)"]
        );
    }

    #[test]
    fn both_intermediate_and_leaf_default() {
        assert_eq!(
            summarize("let { a: { b = 3 } = {} } = 0;"),
            vec![".a={d}.b={d} leaf(0)"]
        );
    }

    #[test]
    fn nested_object() {
        assert_eq!(
            summarize("let { a: { b: { c } } } = 0;"),
            vec![".a.b.c leaf(0)"]
        );
    }

    #[test]
    fn flat_array_destructure() {
        assert_eq!(
            summarize("let [a, b] = 0;"),
            vec!["[0] leaf(0)", "[1] leaf(1)"]
        );
    }

    #[test]
    fn array_with_hole() {
        assert_eq!(
            summarize("let [a, , c] = 0;"),
            vec!["[0] leaf(0)", "[2] leaf(1)"]
        );
    }

    #[test]
    fn mixed_object_array() {
        assert_eq!(
            summarize("let { users: [{ name }, second] } = 0;"),
            vec![".users[0].name leaf(0)", ".users[1] leaf(1)"]
        );
    }

    #[test]
    fn object_rest() {
        assert_eq!(
            summarize("let { a, b, ...rest } = 0;"),
            vec![".a leaf(0)", ".b leaf(1)", " rest(2) excl=[a,b]"]
        );
    }

    #[test]
    fn nested_object_rest() {
        assert_eq!(
            summarize("let { a: { b, ...inner }, ...outer } = 0;"),
            vec![".a.b leaf(0)", ".a rest(1) excl=[b]", " rest(2) excl=[a]"]
        );
    }

    #[test]
    fn array_rest() {
        assert_eq!(
            summarize("let [a, ...rest] = 0;"),
            vec!["[0] leaf(0)", "[slice 1] leaf(1)"]
        );
    }

    #[test]
    fn computed_key() {
        assert_eq!(summarize("let { [k]: value } = 0;"), vec!["[expr] leaf(0)"]);
    }

    fn summarize_assign(source: &str) -> Vec<String> {
        let alloc = Allocator::default();
        let ret = Parser::new(&alloc, source, SourceType::mjs()).parse();
        assert!(ret.errors.is_empty(), "parse errors: {:?}", ret.errors);
        let stmt = ret.program.body.first().expect("one statement");
        let Statement::ExpressionStatement(es) = stmt else {
            panic!("expected expression statement");
        };
        let mut expr = &es.expression;
        while let Expression::ParenthesizedExpression(p) = expr {
            expr = &p.expression;
        }
        let Expression::AssignmentExpression(assign) = expr else {
            panic!("expected assignment expression");
        };

        let mut out: Vec<String> = Vec::new();
        walk_assignment_targets(&assign.left, |v| {
            let path = v
                .path
                .iter()
                .map(|s| {
                    let mut label = match s.access {
                        WriteAccess::Index { index, .. } => format!("[{}]", index),
                        WriteAccess::Slice { from } => format!("[slice {}]", from),
                        WriteAccess::Key { name } => format!(".{}", name),
                        WriteAccess::Computed { .. } => "[expr]".to_string(),
                    };
                    if s.default.is_some() {
                        label.push_str("={d}");
                    }
                    label
                })
                .collect::<String>();
            let tgt = match v.target {
                WriteTarget::Identifier(id) if !v.excluded.is_empty() => {
                    format!("rest({}) excl=[{}]", id.name, v.excluded.join(","))
                }
                WriteTarget::Identifier(id) => format!("id({})", id.name),
                WriteTarget::Member(_) => "member".to_string(),
            };
            out.push(format!("{path} {tgt}"));
        });
        out
    }

    #[test]
    fn assign_flat_object() {
        assert_eq!(
            summarize_assign("({ a, b } = o);"),
            vec![".a id(a)", ".b id(b)"]
        );
    }

    #[test]
    fn assign_aliased_property() {
        assert_eq!(summarize_assign("({ a: b } = o);"), vec![".a id(b)"]);
    }

    #[test]
    fn assign_leaf_default() {
        assert_eq!(summarize_assign("({ a = 5 } = o);"), vec![".a={d} id(a)"]);
    }

    #[test]
    fn assign_nested_object() {
        assert_eq!(summarize_assign("({ a: { b } } = o);"), vec![".a.b id(b)"]);
    }

    #[test]
    fn assign_flat_array() {
        assert_eq!(
            summarize_assign("[a, b] = arr;"),
            vec!["[0] id(a)", "[1] id(b)"]
        );
    }

    #[test]
    fn assign_array_hole() {
        assert_eq!(
            summarize_assign("[a, , c] = arr;"),
            vec!["[0] id(a)", "[2] id(c)"]
        );
    }

    #[test]
    fn assign_mixed_object_array() {
        assert_eq!(
            summarize_assign("({ users: [{ name }, second] } = o);"),
            vec![".users[0].name id(name)", ".users[1] id(second)"]
        );
    }

    #[test]
    fn assign_object_rest() {
        assert_eq!(
            summarize_assign("({ a, b, ...rest } = o);"),
            vec![".a id(a)", ".b id(b)", " rest(rest) excl=[a,b]"]
        );
    }

    #[test]
    fn assign_array_rest() {
        assert_eq!(
            summarize_assign("[a, ...rest] = arr;"),
            vec!["[0] id(a)", "[slice 1] id(rest)"]
        );
    }

    #[test]
    fn assign_nested_array_rest() {
        assert_eq!(
            summarize_assign("[x, ...{ z = 26 }] = arr;"),
            vec!["[0] id(x)", "[slice 1].z={d} id(z)"]
        );
    }

    #[test]
    fn assign_computed_key() {
        assert_eq!(summarize_assign("({ [k]: v } = o);"), vec!["[expr] id(v)"]);
    }

    #[test]
    fn assign_member_target() {
        assert_eq!(
            summarize_assign("[obj.x, a] = arr;"),
            vec!["[0] member", "[1] id(a)"]
        );
    }

    #[test]
    fn assign_intermediate_default() {
        assert_eq!(
            summarize_assign("[{ a } = {}] = arr;"),
            vec!["[0]={d}.a id(a)"]
        );
    }

    #[test]
    fn assign_property_default() {
        assert_eq!(summarize_assign("({ a: b = 5 } = o);"), vec![".a={d} id(b)"]);
    }

    fn array_index_steps(source: &str) -> Vec<(u32, u32, bool)> {
        let alloc = Allocator::default();
        let ret = Parser::new(&alloc, source, SourceType::mjs()).parse();
        assert!(ret.errors.is_empty(), "parse errors: {:?}", ret.errors);
        let Statement::ExpressionStatement(es) = ret.program.body.first().expect("one statement")
        else {
            panic!("expected expression statement");
        };
        let mut expr = &es.expression;
        while let Expression::ParenthesizedExpression(p) = expr {
            expr = &p.expression;
        }
        let Expression::AssignmentExpression(assign) = expr else {
            panic!("expected assignment expression");
        };
        let mut out = Vec::new();
        walk_assignment_targets(&assign.left, |v| {
            for step in v.path {
                if let WriteAccess::Index {
                    index,
                    len,
                    has_rest,
                } = step.access
                {
                    out.push((index, len, has_rest));
                }
            }
        });
        out
    }

    #[test]
    fn assign_array_arity_fixed() {
        assert_eq!(
            array_index_steps("[a, b] = arr;"),
            vec![(0, 2, false), (1, 2, false)]
        );
    }

    #[test]
    fn assign_array_arity_with_rest() {
        assert_eq!(array_index_steps("[a, ...b] = arr;"), vec![(0, 1, true)]);
    }
}
