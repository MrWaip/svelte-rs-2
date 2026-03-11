use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, ScriptLanguage};
use svelte_diagnostics::Diagnostic;
use svelte_js::{ExpressionInfo, ExpressionKind};

use crate::data::AnalysisData;
use crate::scope::ComponentScoping;

pub fn parse_js(component: &Component, data: &mut AnalysisData, diags: &mut Vec<Diagnostic>) {
    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let typescript = matches!(script.language, ScriptLanguage::TypeScript);
        match svelte_js::analyze_script_with_scoping(source, script.content_span.start, typescript)
        {
            Ok((info, scoping)) => {
                data.exports = info.exports.clone();
                data.script = Some(info);
                data.scoping = ComponentScoping::from_scoping(scoping);
            }
            Err(errs) => diags.extend(errs),
        }
    }

    walk_fragment(&component.fragment, component, data, diags);
}

fn walk_fragment(
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    diags: &mut Vec<Diagnostic>,
) {
    for node in &fragment.nodes {
        walk_node(node, component, data, diags);
    }
}

fn walk_node(
    node: &Node,
    component: &Component,
    data: &mut AnalysisData,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            let source = component.source_text(tag.expression_span);
            let offset = tag.expression_span.start;
            match svelte_js::analyze_expression(source, offset) {
                Ok(info) => {
                    data.expressions.insert(tag.id, info);
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::Element(el) => {
            walk_attr_expressions(el, component, data, diags);
            walk_fragment(&el.fragment, component, data, diags);
        }
        Node::IfBlock(block) => {
            let source = component.source_text(block.test_span);
            let offset = block.test_span.start;
            match svelte_js::analyze_expression(source, offset) {
                Ok(info) => {
                    data.expressions.insert(block.id, info);
                }
                Err(diag) => diags.push(diag),
            }
            walk_fragment(&block.consequent, component, data, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alt, component, data, diags);
            }
        }
        Node::EachBlock(block) => {
            let source = component.source_text(block.expression_span);
            let offset = block.expression_span.start;
            match svelte_js::analyze_expression(source, offset) {
                Ok(info) => {
                    data.expressions.insert(block.id, info);
                }
                Err(diag) => diags.push(diag),
            }
            walk_fragment(&block.body, component, data, diags);
            if let Some(fb) = &block.fallback {
                walk_fragment(fb, component, data, diags);
            }
        }
        Node::SnippetBlock(block) => {
            walk_fragment(&block.body, component, data, diags);
        }
        Node::RenderTag(tag) => {
            let source = component.source_text(tag.expression_span);
            let offset = tag.expression_span.start;
            match svelte_js::analyze_expression(source, offset) {
                Ok(info) => {
                    data.expressions.insert(tag.id, info);
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::Text(_) | Node::Comment(_) => {}
    }
}

/// Parse and store attribute expressions, keyed by (element_id, attr_index).
fn walk_attr_expressions(
    el: &svelte_ast::Element,
    component: &Component,
    data: &mut AnalysisData,
    diags: &mut Vec<Diagnostic>,
) {
    for (attr_idx, attr) in el.attributes.iter().enumerate() {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                let offset = a.expression_span.start;
                match svelte_js::analyze_expression(source, offset) {
                    Ok(info) => { data.attr_expressions.insert((el.id, attr_idx), info); }
                    Err(diag) => diags.push(diag),
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                let mut all_refs = Vec::new();
                for part in &a.parts {
                    if let ConcatPart::Dynamic(span) = part {
                        let source = component.source_text(*span);
                        let offset = span.start;
                        match svelte_js::analyze_expression(source, offset) {
                            Ok(info) => all_refs.extend(info.references),
                            Err(diag) => diags.push(diag),
                        }
                    }
                }
                let merged = ExpressionInfo {
                    kind: ExpressionKind::Other,
                    references: all_refs,
                    has_side_effects: false,
                };
                data.attr_expressions.insert((el.id, attr_idx), merged);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    let offset = span.start;
                    match svelte_js::analyze_expression(source, offset) {
                        Ok(info) => { data.attr_expressions.insert((el.id, attr_idx), info); }
                        Err(diag) => diags.push(diag),
                    }
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    let offset = span.start;
                    match svelte_js::analyze_expression(source, offset) {
                        Ok(info) => { data.attr_expressions.insert((el.id, attr_idx), info); }
                        Err(diag) => diags.push(diag),
                    }
                }
            }
            Attribute::ShorthandOrSpread(a) => {
                // For spread attrs, skip the "..." prefix
                let span = if a.is_spread {
                    svelte_span::Span::new(a.expression_span.start + 3, a.expression_span.end)
                } else {
                    a.expression_span
                };
                let source = component.source_text(span);
                let offset = span.start;
                match svelte_js::analyze_expression(source, offset) {
                    Ok(info) => { data.attr_expressions.insert((el.id, attr_idx), info); }
                    Err(diag) => diags.push(diag),
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }
}
