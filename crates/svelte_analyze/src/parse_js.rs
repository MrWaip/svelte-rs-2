use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, ScriptLanguage};
use svelte_diagnostics::Diagnostic;

use crate::data::AnalysisData;

pub fn parse_js(component: &Component, data: &mut AnalysisData, diags: &mut Vec<Diagnostic>) {
    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let typescript = matches!(script.language, ScriptLanguage::TypeScript);
        match svelte_js::analyze_script(source, script.content_span.start as u32, typescript) {
            Ok(info) => data.script = Some(info),
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
            let offset = tag.expression_span.start as u32;
            match svelte_js::analyze_expression(source, offset) {
                Ok(info) => {
                    data.expressions.insert(tag.id, info);
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::Element(el) => {
            walk_attr_expressions(el, component, diags);
            walk_fragment(&el.fragment, component, data, diags);
        }
        Node::IfBlock(block) => {
            let source = component.source_text(block.test_span);
            let offset = block.test_span.start as u32;
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
            let offset = block.expression_span.start as u32;
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
        Node::Text(_) | Node::Comment(_) => {}
    }
}

/// Validate attribute expressions early so errors are collected.
/// Attribute expressions are re-analyzed in the reactivity pass (no NodeId key for attrs).
fn walk_attr_expressions(
    el: &svelte_ast::Element,
    component: &Component,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in &el.attributes {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                let offset = a.expression_span.start as u32;
                if let Err(diag) = svelte_js::analyze_expression(source, offset) {
                    diags.push(diag);
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                for part in &a.parts {
                    if let ConcatPart::Dynamic(span) = part {
                        let source = component.source_text(*span);
                        let offset = span.start as u32;
                        if let Err(diag) = svelte_js::analyze_expression(source, offset) {
                            diags.push(diag);
                        }
                    }
                }
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    let offset = span.start as u32;
                    if let Err(diag) = svelte_js::analyze_expression(source, offset) {
                        diags.push(diag);
                    }
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    let offset = span.start as u32;
                    if let Err(diag) = svelte_js::analyze_expression(source, offset) {
                        diags.push(diag);
                    }
                }
            }
            Attribute::ShorthandOrSpread(a) => {
                let source = component.source_text(a.expression_span);
                let offset = a.expression_span.start as u32;
                if let Err(diag) = svelte_js::analyze_expression(source, offset) {
                    diags.push(diag);
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
        }
    }
}
