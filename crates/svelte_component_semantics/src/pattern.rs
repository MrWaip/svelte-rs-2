use oxc_ast::ast::{BindingPattern, Expression, PropertyKey};
use smallvec::SmallVec;

use crate::SymbolId;

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

    Index(u32),
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
            for (i, el) in arr.elements.iter().enumerate() {
                let Some(el) = el else { continue };
                path.push(Step {
                    access: Access::Index(i as u32),
                    default: None,
                });
                walk_inner(el, path, visit);
                path.pop();
            }
            if let Some(rest) = &arr.rest {
                if let BindingPattern::BindingIdentifier(ident) = &rest.argument {
                    let Some(symbol) = ident.symbol_id.get() else {
                        return;
                    };
                    visit(BindingVisit {
                        symbol,
                        path,
                        is_rest: true,
                        excluded: &[],
                    });
                } else {
                    walk_inner(&rest.argument, path, visit);
                }
            }
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
                        Access::Index(i) => format!("[{}]", i),
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
            vec!["[0] leaf(0)", " rest(1) excl=[]"]
        );
    }

    #[test]
    fn computed_key() {
        assert_eq!(summarize("let { [k]: value } = 0;"), vec!["[expr] leaf(0)"]);
    }
}
