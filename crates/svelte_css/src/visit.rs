use crate::ast::*;

pub trait Visit {
    fn visit_stylesheet(&mut self, node: &StyleSheet) {
        walk_stylesheet(self, node);
    }
    fn visit_rule(&mut self, node: &Rule) {
        walk_rule(self, node);
    }
    fn visit_style_rule(&mut self, node: &StyleRule) {
        walk_style_rule(self, node);
    }
    fn visit_at_rule(&mut self, node: &AtRule) {
        walk_at_rule(self, node);
    }
    fn visit_selector_list(&mut self, node: &SelectorList) {
        walk_selector_list(self, node);
    }
    fn visit_complex_selector(&mut self, node: &ComplexSelector) {
        walk_complex_selector(self, node);
    }
    fn visit_relative_selector(&mut self, node: &RelativeSelector) {
        walk_relative_selector(self, node);
    }
    fn visit_simple_selector(&mut self, _node: &SimpleSelector) {}
    fn visit_block(&mut self, node: &Block) {
        walk_block(self, node);
    }
    fn visit_declaration(&mut self, _node: &Declaration) {}
    fn visit_comment(&mut self, _node: &Comment) {}
}

pub fn walk_stylesheet<V: Visit + ?Sized>(v: &mut V, node: &StyleSheet) {
    for child in &node.children {
        match child {
            StyleSheetChild::Rule(rule) => v.visit_rule(rule),
            StyleSheetChild::Comment(comment) => v.visit_comment(comment),
            StyleSheetChild::Error(_) => {}
        }
    }
}

pub fn walk_rule<V: Visit + ?Sized>(v: &mut V, node: &Rule) {
    match node {
        Rule::Style(r) => v.visit_style_rule(r.as_ref()),
        Rule::AtRule(r) => v.visit_at_rule(r),
    }
}

pub fn walk_style_rule<V: Visit + ?Sized>(v: &mut V, node: &StyleRule) {
    v.visit_selector_list(&node.prelude);
    v.visit_block(&node.block);
}

pub fn walk_at_rule<V: Visit + ?Sized>(v: &mut V, node: &AtRule) {
    if let Some(block) = &node.block {
        v.visit_block(block);
    }
}

pub fn walk_selector_list<V: Visit + ?Sized>(v: &mut V, node: &SelectorList) {
    for child in &node.children {
        v.visit_complex_selector(child);
    }
}

pub fn walk_complex_selector<V: Visit + ?Sized>(v: &mut V, node: &ComplexSelector) {
    for child in &node.children {
        v.visit_relative_selector(child);
    }
}

pub fn walk_relative_selector<V: Visit + ?Sized>(v: &mut V, node: &RelativeSelector) {
    for sel in &node.selectors {
        v.visit_simple_selector(sel);
    }
}

pub fn walk_block<V: Visit + ?Sized>(v: &mut V, node: &Block) {
    for child in &node.children {
        match child {
            BlockChild::Declaration(d) => v.visit_declaration(d),
            BlockChild::Rule(r) => v.visit_rule(r),
            BlockChild::Comment(c) => v.visit_comment(c),
            BlockChild::Error(_) => {}
        }
    }
}

pub trait VisitMut {
    fn visit_stylesheet_mut(&mut self, node: &mut StyleSheet) {
        walk_stylesheet_mut(self, node);
    }
    fn visit_rule_mut(&mut self, node: &mut Rule) {
        walk_rule_mut(self, node);
    }
    fn visit_style_rule_mut(&mut self, node: &mut StyleRule) {
        walk_style_rule_mut(self, node);
    }
    fn visit_at_rule_mut(&mut self, node: &mut AtRule) {
        walk_at_rule_mut(self, node);
    }
    fn visit_selector_list_mut(&mut self, node: &mut SelectorList) {
        walk_selector_list_mut(self, node);
    }
    fn visit_complex_selector_mut(&mut self, node: &mut ComplexSelector) {
        walk_complex_selector_mut(self, node);
    }
    fn visit_relative_selector_mut(&mut self, node: &mut RelativeSelector) {
        walk_relative_selector_mut(self, node);
    }
    fn visit_simple_selector_mut(&mut self, _node: &mut SimpleSelector) {}
    fn visit_block_mut(&mut self, node: &mut Block) {
        walk_block_mut(self, node);
    }
    fn visit_declaration_mut(&mut self, _node: &mut Declaration) {}
    fn visit_comment_mut(&mut self, _node: &mut Comment) {}
}

pub fn walk_stylesheet_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut StyleSheet) {
    for child in &mut node.children {
        match child {
            StyleSheetChild::Rule(rule) => v.visit_rule_mut(rule),
            StyleSheetChild::Comment(comment) => v.visit_comment_mut(comment),
            StyleSheetChild::Error(_) => {}
        }
    }
}

pub fn walk_rule_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut Rule) {
    match node {
        Rule::Style(r) => v.visit_style_rule_mut(r.as_mut()),
        Rule::AtRule(r) => v.visit_at_rule_mut(r),
    }
}

pub fn walk_style_rule_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut StyleRule) {
    v.visit_selector_list_mut(&mut node.prelude);
    v.visit_block_mut(&mut node.block);
}

pub fn walk_at_rule_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut AtRule) {
    if let Some(block) = &mut node.block {
        v.visit_block_mut(block);
    }
}

pub fn walk_selector_list_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut SelectorList) {
    for child in &mut node.children {
        v.visit_complex_selector_mut(child);
    }
}

pub fn walk_complex_selector_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut ComplexSelector) {
    for child in &mut node.children {
        v.visit_relative_selector_mut(child);
    }
}

pub fn walk_relative_selector_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut RelativeSelector) {
    for sel in &mut node.selectors {
        v.visit_simple_selector_mut(sel);
    }
}

pub fn walk_block_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut Block) {
    for child in &mut node.children {
        match child {
            BlockChild::Declaration(d) => v.visit_declaration_mut(d),
            BlockChild::Rule(r) => v.visit_rule_mut(r),
            BlockChild::Comment(c) => v.visit_comment_mut(c),
            BlockChild::Error(_) => {}
        }
    }
}

pub fn walk_simple_selector_args<V: Visit + ?Sized>(v: &mut V, node: &SimpleSelector) {
    let args = match node {
        SimpleSelector::PseudoClass(pc) => pc.args.as_deref(),
        SimpleSelector::Global { args, .. } => args.as_deref(),
        _ => None,
    };
    if let Some(args) = args {
        v.visit_selector_list(args);
    }
}

pub fn walk_simple_selector_args_mut<V: VisitMut + ?Sized>(v: &mut V, node: &mut SimpleSelector) {
    let args = match node {
        SimpleSelector::PseudoClass(pc) => pc.args.as_deref_mut(),
        SimpleSelector::Global { args, .. } => args.as_deref_mut(),
        _ => None,
    };
    if let Some(args) = args {
        v.visit_selector_list_mut(args);
    }
}
