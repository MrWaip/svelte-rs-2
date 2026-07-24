use oxc_allocator::Allocator;
use oxc_ast::AstBuilder;
use oxc_ast::ast::{Expression, Program, Statement, VariableDeclarationKind};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan, SourceType, Span as OxcSpan};

use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

use crate::js_postprocess::{
    process_binding_pattern, process_expression, process_formal_parameters, process_program,
    process_statement, shift_comments, wrapper_delta,
};

fn parse_expression_as_program<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    src_type: SourceType,
    typescript: bool,
) -> Option<(Expression<'a>, Vec<oxc_ast::Comment>)> {
    let result = OxcParser::new(alloc, source, src_type).parse();
    if !result.diagnostics.is_empty() || result.program.body.len() != 1 {
        return None;
    }
    let delta = wrapper_delta(offset, 0, 0);
    let mut comments: Vec<oxc_ast::Comment> = result.program.comments.to_vec();
    shift_comments(&mut comments, delta);
    let Statement::ExpressionStatement(expr_stmt) = result.program.body.into_iter().next()? else {
        return None;
    };
    let mut expr = expr_stmt.unbox().expression;
    process_expression(alloc, &mut expr, delta, typescript);
    Some((expr, comments))
}

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
    process_expression(alloc, &mut expr, wrapper_delta(offset, 0, 0), typescript);
    Ok(expr)
}

pub(crate) enum ExpressionTagBody<'a> {
    Expression(Expression<'a>),
    Declaration(Statement<'a>),
    Invalid(Option<Diagnostic>),
}

pub(crate) fn parse_expression_tag_body<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
    has_comment: bool,
    comments_sink: &mut Vec<oxc_ast::Comment>,
) -> ExpressionTagBody<'a> {
    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    if has_comment
        && let Some((expr, comments)) =
            parse_expression_as_program(alloc, source, offset, src_type, typescript)
    {
        comments_sink.extend(comments);
        return ExpressionTagBody::Expression(expr);
    }

    let parsed = OxcParser::new(alloc, source, src_type).parse_expression();
    let consumed_full = matches!(&parsed, Ok(expr) if expr.span().end as usize == source.len());

    if !consumed_full
        && let Some(stmt) = parse_body_declaration(alloc, source, offset, src_type, typescript)
    {
        return ExpressionTagBody::Declaration(stmt);
    }

    match parsed {
        Ok(mut expr) => {
            process_expression(alloc, &mut expr, wrapper_delta(offset, 0, 0), typescript);
            ExpressionTagBody::Expression(expr)
        }
        Err(errs) => {
            let diag = errs.first().and_then(|e| {
                let acorn_message = match e.message.as_ref() {
                    "Cannot assign to this expression" => "Assigning to rvalue",
                    _ => return None,
                };
                let label_offset = e
                    .labels
                    .as_slice()
                    .first()
                    .map_or(0u32, |label| label.offset());
                let pos = offset + label_offset;
                Some(Diagnostic::js_parse_error(
                    Span::new(pos, pos),
                    acorn_message.to_string(),
                ))
            });
            ExpressionTagBody::Invalid(diag)
        }
    }
}

pub(crate) fn placeholder_expression<'a>(alloc: &'a Allocator, offset: u32) -> Expression<'a> {
    AstBuilder::new(alloc).expression_null_literal(OxcSpan::new(offset, offset))
}

pub(crate) fn placeholder_statement<'a>(alloc: &'a Allocator, offset: u32) -> Statement<'a> {
    AstBuilder::new(alloc).statement_empty(OxcSpan::new(offset, offset))
}

fn parse_body_declaration<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    src_type: SourceType,
    typescript: bool,
) -> Option<Statement<'a>> {
    let result = OxcParser::new(alloc, source, src_type).parse();
    let mut stmt = result.program.body.into_iter().next()?;
    if !stmt.is_declaration() {
        return None;
    }
    process_statement(alloc, &mut stmt, wrapper_delta(offset, 0, 0), typescript);
    Some(stmt)
}

pub fn parse_script_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
    is_module: bool,
) -> Result<Program<'a>, Vec<Diagnostic>> {
    let source_type = if typescript {
        SourceType::mjs().with_typescript(true)
    } else {
        SourceType::mjs()
    };

    let result = OxcParser::new(alloc, source, source_type).parse();

    if !result.diagnostics.is_empty() {
        return Err(result
            .diagnostics
            .iter()
            .map(|_| {
                Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
            })
            .collect());
    }

    let mut program = result.program;
    process_program(
        alloc,
        &mut program,
        wrapper_delta(offset, 0, 0),
        typescript,
        is_module,
    );
    Ok(program)
}

pub enum DeclarationTagBody<'a> {
    Declaration(Statement<'a>),
    InvalidType(Span),
    ParseError(Diagnostic),
}

fn is_identifier_continue_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$' || byte >= 0x80
}

fn unsupported_declaration_keyword_len(source: &str) -> Option<u32> {
    let bytes = source.as_bytes();
    let keyword = match bytes.first()? {
        b'v' => "var",
        b'i' => "interface",
        b'e' => "enum",
        _ => return None,
    };
    if !bytes.starts_with(keyword.as_bytes()) {
        return None;
    }
    if bytes
        .get(keyword.len())
        .is_some_and(|&byte| is_identifier_continue_byte(byte))
    {
        return None;
    }
    Some(keyword.len() as u32)
}

pub fn parse_declaration_body<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> DeclarationTagBody<'a> {
    if let Some(len) = unsupported_declaration_keyword_len(source) {
        return DeclarationTagBody::InvalidType(Span::new(offset, offset + len));
    }

    let src_type = if typescript {
        SourceType::default()
            .with_typescript(true)
            .with_module(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, source, src_type).parse();

    if let Some(error) = result.diagnostics.first() {
        let label_offset = error
            .labels
            .as_slice()
            .first()
            .map_or(0usize, |label| label.offset() as usize);
        let bytes = source.as_bytes();
        let mut position = label_offset;
        while position < bytes.len() && bytes[position].is_ascii_whitespace() {
            position += 1;
        }
        let point = offset + position as u32;
        return DeclarationTagBody::ParseError(Diagnostic::js_parse_error(
            Span::new(point, point),
            error.message.to_string(),
        ));
    }

    let Some(mut stmt) = result.program.body.into_iter().next() else {
        return DeclarationTagBody::ParseError(Diagnostic::js_parse_error(
            Span::new(offset, offset),
            "Unexpected token".to_string(),
        ));
    };

    let supported = match &stmt {
        Statement::VariableDeclaration(decl) => matches!(
            decl.kind,
            VariableDeclarationKind::Const | VariableDeclarationKind::Let
        ),
        _ => false,
    };
    if !supported {
        let raw = stmt.span();
        return DeclarationTagBody::InvalidType(Span::new(offset + raw.start, offset + raw.end));
    }

    process_statement(alloc, &mut stmt, wrapper_delta(offset, 0, 0), typescript);
    DeclarationTagBody::Declaration(stmt)
}

pub fn parse_const_declaration_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<Statement<'a>, Diagnostic> {
    const PREFIX: &str = "const ";
    let wrapped_owned = format!("{PREFIX}{source};");
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let src_type = if typescript {
        SourceType::default()
            .with_typescript(true)
            .with_module(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();

    if !result.diagnostics.is_empty() {
        return Err(Diagnostic::invalid_expression(Span::new(
            offset,
            offset + source.len() as u32,
        )));
    }

    let program = result.program;
    let mut stmt = program.body.into_iter().next().ok_or_else(|| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;

    process_statement(
        alloc,
        &mut stmt,
        wrapper_delta(offset, 0, PREFIX.len() as i64),
        typescript,
    );
    Ok(stmt)
}

pub(crate) fn parse_each_context_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Option<Statement<'a>> {
    const PREFIX: &str = "let ";
    let leading_ws = leading_whitespace_len(source);
    let trimmed = source.trim();
    let wrapped_owned = format!("{PREFIX}{trimmed} = x;");
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();

    if !result.diagnostics.is_empty() {
        return None;
    }

    let mut stmt = result.program.body.into_iter().next()?;
    process_statement(
        alloc,
        &mut stmt,
        wrapper_delta(offset, leading_ws, PREFIX.len() as i64),
        typescript,
    );
    Some(stmt)
}

pub(crate) fn parse_each_index_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
) -> Option<Statement<'a>> {
    const PREFIX: &str = "let ";
    let leading_ws = leading_whitespace_len(source);
    let trimmed = source.trim();
    let wrapped_owned = format!("{PREFIX}{trimmed};");
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let result = OxcParser::new(alloc, wrapped_str, SourceType::default()).parse();

    if !result.diagnostics.is_empty() {
        return None;
    }

    let mut stmt = result.program.body.into_iter().next()?;
    process_statement(
        alloc,
        &mut stmt,
        wrapper_delta(offset, leading_ws, PREFIX.len() as i64),
        false,
    );
    Some(stmt)
}

pub(crate) fn parse_snippet_decl_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Option<Statement<'a>> {
    const PREFIX: &str = "const ";
    const ASSIGN: &str = " = ";
    let leading_ws = leading_whitespace_len(source);
    let trimmed = source.trim();
    let paren_pos = trimmed.find('(');
    let ident_end = trimmed.find(['<', '(']).unwrap_or(trimmed.len());
    let ident = trimmed[..ident_end].trim_end();
    let wrapped = if let Some(p) = paren_pos {
        let params_with_parens = &trimmed[p..];
        format!("{PREFIX}{ident}{ASSIGN}{params_with_parens} => {{}}")
    } else {
        format!("{PREFIX}{ident}{ASSIGN}() => {{}}")
    };
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped);
    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();
    if !result.diagnostics.is_empty() && (!typescript || result.panicked) {
        return None;
    }
    let mut stmt = result.program.body.into_iter().next()?;

    let name_prefix = PREFIX.len() as i64;
    if let Statement::VariableDeclaration(var_decl) = &mut stmt
        && let Some(declarator) = var_decl.declarations.first_mut()
    {
        process_binding_pattern(
            alloc,
            &mut declarator.id,
            wrapper_delta(offset, leading_ws, name_prefix),
            typescript,
        );
        if let Some(p) = paren_pos
            && let Some(Expression::ArrowFunctionExpression(arrow)) = &mut declarator.init
        {
            let params_prefix =
                (PREFIX.len() + ASSIGN.len()) as i64 + ident.len() as i64 - p as i64;
            process_formal_parameters(
                alloc,
                &mut arrow.params,
                wrapper_delta(offset, leading_ws, params_prefix),
                typescript,
            );
        }
    }

    Some(stmt)
}

fn leading_whitespace_len(s: &str) -> usize {
    s.len() - s.trim_start().len()
}

pub(crate) fn parse_slot_let_decl_with_alloc<'a>(
    alloc: &'a Allocator,
    pattern_source: &'a str,
    slot_prop_name: &str,
    offset: u32,
    typescript: bool,
) -> Result<Statement<'a>, Diagnostic> {
    let source = format!("{pattern_source} = $$slotProps.{slot_prop_name}");
    let source: &'a str = alloc.alloc_str(&source);
    parse_const_declaration_with_alloc(alloc, source, offset, typescript)
}
