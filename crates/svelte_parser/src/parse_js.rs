use std::cell::Cell;

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::Parser as OxcParser;
use oxc_span::SourceType;
use oxc_syntax::node::NodeId;

use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

pub fn parse_expression_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<Expression<'a>, Diagnostic> {
    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let parser = OxcParser::new(alloc, source, src_type);
    let mut expr = parser.parse_expression().map_err(|_| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;
    if typescript {
        strip_ts_expression(&mut expr, alloc);
    }
    Ok(expr)
}

pub fn parse_script_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<oxc_ast::ast::Program<'a>, Vec<Diagnostic>> {
    let source_type = if typescript {
        SourceType::mjs().with_typescript(true)
    } else {
        SourceType::mjs()
    };

    let result = OxcParser::new(alloc, source, source_type).parse();

    if !result.errors.is_empty() {
        return Err(result
            .errors
            .iter()
            .map(|_| {
                Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
            })
            .collect());
    }

    Ok(result.program)
}

pub fn parse_const_declaration_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<oxc_ast::ast::Statement<'a>, Diagnostic> {
    let wrapped_owned = format!("const {};", source);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let src_type = if typescript {
        SourceType::default()
            .with_typescript(true)
            .with_module(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();

    if !result.errors.is_empty() {
        return Err(Diagnostic::invalid_expression(Span::new(
            offset,
            offset + source.len() as u32,
        )));
    }

    let program = result.program;
    let mut stmt = program.body.into_iter().next().ok_or_else(|| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;

    if typescript
        && let oxc_ast::ast::Statement::VariableDeclaration(var_decl) = &mut stmt
        && let Some(declarator) = var_decl.declarations.first_mut()
        && let Some(init) = &mut declarator.init
    {
        strip_ts_expression(init, alloc);
    }

    Ok(stmt)
}

pub(crate) fn parse_each_context_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    typescript: bool,
) -> Option<oxc_ast::ast::Statement<'a>> {
    let trimmed = source.trim();
    let wrapped_owned = format!("let {} = x;", trimmed);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();

    if !result.errors.is_empty() {
        return None;
    }

    result.program.body.into_iter().next()
}

pub(crate) fn parse_each_index_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
) -> Option<oxc_ast::ast::Statement<'a>> {
    let trimmed = source.trim();
    let wrapped_owned = format!("let {};", trimmed);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let result = OxcParser::new(alloc, wrapped_str, SourceType::default()).parse();

    if !result.errors.is_empty() {
        return None;
    }

    result.program.body.into_iter().next()
}

pub(crate) fn parse_snippet_decl_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    typescript: bool,
) -> Option<oxc_ast::ast::Statement<'a>> {
    let trimmed = source.trim();
    let wrapped = if let Some(paren_pos) = trimmed.find('(') {
        let name = &trimmed[..paren_pos];
        let params_with_parens = &trimmed[paren_pos..];
        format!("const {} = {} => {{}}", name, params_with_parens)
    } else {
        format!("const {} = () => {{}}", trimmed)
    };
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped);
    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();
    if !result.errors.is_empty() {
        return None;
    }
    result.program.body.into_iter().next()
}

pub(crate) fn parse_slot_let_decl_with_alloc<'a>(
    alloc: &'a Allocator,
    pattern_source: &'a str,
    slot_prop_name: &str,
    offset: u32,
    typescript: bool,
) -> Result<oxc_ast::ast::Statement<'a>, Diagnostic> {
    let source = format!("{pattern_source} = $$slotProps.{slot_prop_name}");
    let source: &'a str = alloc.alloc_str(&source);
    parse_const_declaration_with_alloc(alloc, source, offset, typescript)
}

fn strip_ts_expression<'a>(expr: &mut Expression<'a>, alloc: &'a Allocator) {
    let dummy = || {
        Expression::NullLiteral(oxc_allocator::Box::new_in(
            oxc_ast::ast::NullLiteral {
                span: oxc_span::SPAN,
                node_id: Cell::new(NodeId::DUMMY),
            },
            alloc,
        ))
    };

    loop {
        let inner = match std::mem::replace(expr, dummy()) {
            Expression::TSAsExpression(ts) => ts.unbox().expression,
            Expression::TSSatisfiesExpression(ts) => ts.unbox().expression,
            Expression::TSNonNullExpression(ts) => ts.unbox().expression,
            Expression::TSTypeAssertion(ts) => ts.unbox().expression,
            Expression::TSInstantiationExpression(ts) => ts.unbox().expression,
            other => {
                *expr = other;
                break;
            }
        };
        *expr = inner;
    }
}
