use std::collections::HashMap;

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement, VariableDeclarator};
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{SemanticBuilder, SymbolTable};
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};

use svelte_ast::ScriptLanguage;

use crate::builder::{Arg, Builder};
use crate::context::Ctx;

/// Parse and transform the script block.
///
/// Returns `(imports, body)` — imports are extracted separately so they can
/// be hoisted to the top of the generated module.
pub fn gen_script<'a>(ctx: &mut Ctx<'a>) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let Some(script) = &ctx.component.script else {
        return (vec![], vec![]);
    };

    let is_ts = script.language == ScriptLanguage::TypeScript;
    let allocator = ctx.b.ast.allocator;
    let script_text = ctx.component.source_text(script.content_span);

    let rune_names = &ctx.analysis.rune_names;
    let mutated_runes = &ctx.analysis.mutated_runes;

    transform_script_text(allocator, script_text, is_ts, rune_names, mutated_runes)
}

/// Parse the script source and apply rune transformations, returning (imports, body).
fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    rune_names: &std::collections::HashSet<String>,
    mutated_runes: &std::collections::HashSet<String>,
) -> (Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let src_type = if is_ts {
        SourceType::default().with_typescript(true).with_module(true)
    } else {
        SourceType::mjs()
    };
    let result = OxcParser::new(allocator, source, src_type).parse();

    let b = Builder::new(allocator);
    let mut program = result.program;

    // Run semantic analysis — annotates AST nodes with symbol_id / reference_id
    let sem = SemanticBuilder::new().build(&program);
    let (symbols, _scopes) = sem.semantic.into_symbol_table_and_scope_tree();

    // Build rune_table: OXC SymbolId → RuneInfo
    let mut rune_table: HashMap<oxc_semantic::SymbolId, RuneInfo> = HashMap::new();

    for sym_id in symbols.symbol_ids() {
        let name = symbols.get_name(sym_id);
        if rune_names.contains(name) {
            let mutated = mutated_runes.contains(name);
            rune_table.insert(sym_id, RuneInfo { mutated });
        }
    }

    let mut transformer = ScriptTransformer {
        b: &b,
        rune_table,
        symbols,
    };

    // Pass empty tables to traverse_mut — real IDs are already on AST nodes
    let empty_symbols = SymbolTable::default();
    let empty_scopes = oxc_semantic::ScopeTree::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_symbols, empty_scopes);

    let mut imports = vec![];
    let mut body = vec![];

    for stmt in program.body {
        if matches!(
            stmt,
            Statement::TSTypeAliasDeclaration(_)
                | Statement::TSInterfaceDeclaration(_)
                | Statement::TSEnumDeclaration(_)
        ) {
            continue;
        }
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            imports.push(stmt);
        } else {
            body.push(stmt);
        }
    }

    (imports, body)
}

struct RuneInfo {
    mutated: bool,
}

struct ScriptTransformer<'b, 'a> {
    b: &'b Builder<'a>,
    rune_table: HashMap<oxc_semantic::SymbolId, RuneInfo>,
    /// Kept for reference_id → symbol_id resolution
    symbols: SymbolTable,
}

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    /// Look up rune info via OXC SymbolId on a BindingIdentifier.
    fn rune_info_for_binding(
        &self,
        id: &oxc_ast::ast::BindingIdentifier<'a>,
    ) -> Option<&RuneInfo> {
        let sym_id = id.symbol_id.get()?;
        self.rune_table.get(&sym_id)
    }

    /// Look up rune info via reference_id on an IdentifierReference.
    fn rune_info_for_ref(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<&RuneInfo> {
        let ref_id = id.reference_id.get()?;
        let sym_id = self.symbols.get_reference(ref_id).symbol_id()?;
        self.rune_table.get(&sym_id)
    }

    fn should_proxy(e: &Expression) -> bool {
        if e.is_literal() {
            return false;
        }
        if matches!(
            e,
            Expression::TemplateLiteral(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::UnaryExpression(_)
                | Expression::BinaryExpression(_)
        ) {
            return false;
        }
        if let Expression::Identifier(id) = e {
            if id.name == "undefined" {
                return false;
            }
        }
        true
    }
}

impl<'a> Traverse<'a> for ScriptTransformer<'_, 'a> {
    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a>,
    ) {
        let rune_info = match &node.id.kind {
            oxc_ast::ast::BindingPatternKind::BindingIdentifier(id) => {
                self.rune_info_for_binding(id)
            }
            _ => return,
        };

        let Some(info) = rune_info else {
            return;
        };
        let mutated = info.mutated;

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if let Expression::CallExpression(mut call) = init_expr {
            if mutated {
                call.callee = self.b.rid_expr("$.state");

                if call.arguments.is_empty() {
                    let void_zero = self.b.ast.expression_unary(
                        oxc_span::SPAN,
                        oxc_ast::ast::UnaryOperator::Void,
                        self.b.num_expr(0.0),
                    );
                    call.arguments.push(void_zero.into());
                }

                node.init = Some(Expression::CallExpression(call));
            } else {
                let value = if call.arguments.is_empty() {
                    self.b.ast.expression_unary(
                        oxc_span::SPAN,
                        oxc_ast::ast::UnaryOperator::Void,
                        self.b.num_expr(0.0),
                    )
                } else {
                    let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                    std::mem::swap(&mut call.arguments[0], &mut dummy);
                    dummy.into_expression()
                };
                let value = if Self::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                node.init = Some(value);
            }
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        match node {
            Expression::AssignmentExpression(_) => {
                self.transform_assignment(node, ctx);
            }
            Expression::UpdateExpression(_) => {
                self.transform_update(node, ctx);
            }
            Expression::Identifier(id) => {
                let Some(info) = self.rune_info_for_ref(id) else {
                    return;
                };
                if info.mutated {
                    let name = id.name.as_str().to_string();
                    *node = crate::rune_transform::transform_rune_get(self.b, &name);
                }
            }
            _ => {}
        }
    }
}

impl<'a> ScriptTransformer<'_, 'a> {
    fn transform_assignment(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };

        let ident_name =
            if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
                if let Some(info) = self.rune_info_for_ref(id) {
                    if info.mutated {
                        Some(id.name.as_str().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

        if let Some(name) = ident_name {
            let right = self.b.move_expr(&mut assign.right);
            let needs_proxy = Self::should_proxy(&right);
            *node = crate::rune_transform::transform_rune_set(self.b, &name, right, needs_proxy);
        }
    }

    fn transform_update(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        let ident_name = if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(
            id,
        ) = &upd.argument
        {
            if let Some(info) = self.rune_info_for_ref(id) {
                if info.mutated {
                    Some(id.name.as_str().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = ident_name {
            let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
            *node = crate::rune_transform::transform_rune_update(
                self.b, &name, upd.prefix, is_increment,
            );
        }
    }
}
