use std::collections::{HashMap, HashSet};

use crate::{
    AnimationDirective, AnyAttribute, Attribute, BindDirective, ClassDirective, Directives,
    LetDirective, OnDirective, StyleDirective, TransitionDirective, UseDirective,
};

#[derive(Debug, Default)]
pub struct AttributeStore<'hir> {
    has_spread: bool,
    attributes: Vec<Attribute<'hir>>,
    style_directives: Vec<&'hir StyleDirective<'hir>>,
    class_directives: Vec<&'hir ClassDirective<'hir>>,
    use_directives: Vec<&'hir UseDirective<'hir>>,
    animation_directives: Vec<&'hir AnimationDirective<'hir>>,
    let_directives: Vec<&'hir LetDirective<'hir>>,
    transition_directives: Vec<&'hir TransitionDirective<'hir>>,
    on_directives: Vec<&'hir OnDirective<'hir>>,
    bind_directives: HashMap<&'hir str, &'hir BindDirective<'hir>>,
    attribute_names: HashSet<&'hir str>,
}

impl<'hir> AttributeStore<'hir> {
    pub fn has_spread(&self) -> bool {
        return self.has_spread;
    }

    pub fn has_use(&self) -> bool {
        return !self.use_directives.is_empty();
    }

    pub fn has_by_name(&self, name: &str) -> bool {
        return self.attribute_names.contains(name);
    }

    pub fn push_class_directive(&mut self, directive: &'hir ClassDirective<'hir>) {
        self.attribute_names.insert("class");
        self.class_directives.push(directive);
    }

    pub fn push_let_directive(&mut self, directive: &'hir LetDirective<'hir>) {
        self.let_directives.push(directive);
    }

    pub fn push_animation_directive(&mut self, directive: &'hir AnimationDirective<'hir>) {
        self.animation_directives.push(directive);
    }

    pub fn push_transition_directive(&mut self, directive: &'hir TransitionDirective<'hir>) {
        self.transition_directives.push(directive);
    }

    pub fn push_on_directive(&mut self, directive: &'hir OnDirective<'hir>) {
        self.on_directives.push(directive);
    }

    pub fn push_style_directive(&mut self, directive: &'hir StyleDirective<'hir>) {
        self.attribute_names.insert("style");
        self.style_directives.push(directive);
    }

    pub fn push_bind_directive(&mut self, directive: &'hir BindDirective<'hir>) {
        self.attribute_names.insert(directive.name);
        self.bind_directives.insert(directive.name, directive);
    }

    pub fn push_attr(&mut self, attr: Attribute<'hir>) {
        if let Some(name) = attr.name() {
            self.attribute_names.insert(name);
        }

        self.has_spread = attr.is_spread();
        self.attributes.push(attr);
    }

    pub fn iter_attrs(&self) -> impl Iterator<Item = &Attribute<'hir>> {
        return self.attributes.iter();
    }

    pub fn iter_all(&self) -> impl Iterator<Item = AnyAttribute<'hir>> {
        let attrs = self
            .attributes
            .iter()
            .map(|attr| AnyAttribute::from_attr(attr));

        let classes = self
            .class_directives
            .iter()
            .map(|directive| AnyAttribute::Class(directive));

        let styles = self
            .style_directives
            .iter()
            .map(|directive| AnyAttribute::Style(directive));

        let binds = self
            .bind_directives
            .iter()
            .map(|directive| AnyAttribute::Bind(directive.1));

        let uses = self
            .use_directives
            .iter()
            .map(|directive| AnyAttribute::Use(directive));

        let animations = self
            .animation_directives
            .iter()
            .map(|directive| AnyAttribute::Animation(directive));

        let lets = self
            .let_directives
            .iter()
            .map(|directive| AnyAttribute::Let(directive));

        let ons = self
            .on_directives
            .iter()
            .map(|directive| AnyAttribute::On(directive));

        let transitions = self
            .transition_directives
            .iter()
            .map(|directive| AnyAttribute::Transition(directive));

        return attrs
            .chain(classes)
            .chain(styles)
            .chain(binds)
            .chain(uses)
            .chain(lets)
            .chain(ons)
            .chain(transitions)
            .chain(animations);
    }
}
