use std::iter::empty;
use std::mem;

use oxc_ast::ast::Expression;
use svelte_analyze::{
    AttributeSemantics, ClassSemantics, ComponentPropSemantics, ElementBindPropertyKind,
    ElementBindSemantics, NamespaceKind, StyleSemantics, collapse_attribute_whitespace,
    emit_html_attribute_name, is_dom_boolean_attribute,
};
use svelte_ast::{
    Attribute, BindDirective, ClassDirective, ConcatPart, Element, ExprRef, NodeId, StyleDirective,
    StyleDirectiveValue,
};
use svelte_ast_builder::{Arg, ObjProp, TemplatePart};
use svelte_emit_builders::server_refs;

use crate::error::{CodegenError, Result};
use crate::escape::escape_attribute;
use crate::model::ServerCodegen;

pub(crate) enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

const ELEMENT_IS_NAMESPACED: u32 = 1;
const ELEMENT_PRESERVE_ATTRIBUTE_CASE: u32 = 1 << 1;
const ELEMENT_IS_INPUT: u32 = 1 << 2;

enum AttrValue<'a> {
    Static(String),
    Dynamic(Expression<'a>),
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn emit_element_attributes(&mut self, element: &'a Element) -> Result<()> {
        if self.analysis.has_spread(element.id) {
            return self.emit_spread_attributes(element);
        }

        for attr in &element.attributes {
            match self.analysis.attributes.get(attr.id()).clone() {
                AttributeSemantics::Class(class) => {
                    self.emit_class(element, &class)?;
                }
                AttributeSemantics::Style(style) => {
                    self.emit_style(element, &style)?;
                }
                AttributeSemantics::Skip => {}
                AttributeSemantics::ElementBind(sem) => {
                    self.emit_bind_reflection(attr, &sem)?;
                }
                AttributeSemantics::CannotBeStatic(sem) => {
                    if sem.reflects_in_html {
                        self.emit_plain_attribute(element, attr)?;
                    }
                }
                AttributeSemantics::NonSpecial
                | AttributeSemantics::StaticAttr
                | AttributeSemantics::Autofocus
                | AttributeSemantics::HtmlConcat(_) => {
                    self.emit_plain_attribute(element, attr)?;
                }
                AttributeSemantics::Event(_)
                | AttributeSemantics::RuntimeBehavior
                | AttributeSemantics::SpecialValueAttr(_)
                | AttributeSemantics::WindowBind(_)
                | AttributeSemantics::DocumentBind(_)
                | AttributeSemantics::ComponentBind(_)
                | AttributeSemantics::ComponentProp(_)
                | AttributeSemantics::ComponentCssProp(_)
                | AttributeSemantics::SvelteComponentThis(_)
                | AttributeSemantics::ComponentSpread(_)
                | AttributeSemantics::ComponentAttach(_)
                | AttributeSemantics::BoundaryProp(_) => {}
            }
        }

        Ok(())
    }

    fn emit_plain_attribute(&mut self, element: &'a Element, attr: &'a Attribute) -> Result<()> {
        match attr {
            Attribute::StringAttribute(a) => {
                let name = self.attribute_name(element, &a.name);
                let value = a.value(self.component.source.as_str());
                self.push_text(&format!(" {name}=\"{}\"", escape_attribute(value)));
            }
            Attribute::BooleanAttribute(a) => {
                let name = self.attribute_name(element, &a.name);
                self.push_text(&format!(" {name}=\"\""));
            }
            Attribute::ExpressionAttribute(a) => {
                let name = self.attribute_name(element, &a.name);
                let value = self.attr_value_single(a.id, &a.expression)?;
                self.push_named_value(&name, value);
            }
            Attribute::ConcatenationAttribute(a) => {
                let name = self.attribute_name(element, &a.name);
                let value = self.attr_value_concat(&a.parts, false)?;
                self.push_named_value(&name, value);
            }
            _ => {}
        }
        Ok(())
    }

    fn push_named_value(&mut self, name: &str, value: AttrValue<'a>) {
        match value {
            AttrValue::Static(s) => {
                self.push_text(&format!(" {name}=\"{}\"", escape_attribute(&s)));
            }
            AttrValue::Dynamic(expr) => {
                self.push_attr_call(name, expr, is_dom_boolean_attribute(name));
            }
        }
    }

    fn emit_class(&mut self, element: &'a Element, class: &ClassSemantics) -> Result<()> {
        let owner_id = element.id;
        let has_dirs = !class.directives.is_empty();
        let scoped = self.analysis.is_css_scoped(owner_id);
        let hash = self.analysis.css_hash().to_string();

        let value = match class.attr.and_then(|id| self.find_attribute(element, id)) {
            Some(attr) => self.attr_value_of(attr, true)?,
            None => match &class.static_base {
                Some(base) => AttrValue::Static(collapse_attribute_whitespace(base).into_owned()),
                None => AttrValue::Static(String::new()),
            },
        };

        if !has_dirs && let AttrValue::Static(mut s) = value {
            if scoped && !hash.is_empty() {
                s = if s.is_empty() {
                    hash.clone()
                } else {
                    format!("{s} {hash}")
                };
                s = s.trim().to_string();
            }
            if !s.is_empty() {
                self.push_text(&format!(" class=\"{}\"", escape_attribute(&s)));
            }
            return Ok(());
        }

        let mut value_expr = match value {
            AttrValue::Static(s) => self.b.str_expr(&s),
            AttrValue::Dynamic(expr) => expr,
        };
        if class.needs_clsx {
            value_expr = self.b.call_expr("$.clsx", [Arg::Expr(value_expr)]);
        }

        let mut css_hash_arg: Option<Expression<'a>> = None;
        if scoped && !hash.is_empty() {
            if let Expression::StringLiteral(lit) = &value_expr {
                let merged = format!("{} {}", lit.value.as_str(), hash);
                value_expr = self.b.str_expr(merged.trim());
            } else {
                css_hash_arg = Some(self.b.str_expr(&hash));
            }
        }

        let directives = if has_dirs {
            Some(self.build_class_directives_object(element, &class.directives, true)?)
        } else {
            None
        };

        let mut args = vec![Arg::Expr(value_expr)];
        if let Some(directives) = directives {
            let hash_arg = css_hash_arg.unwrap_or_else(|| self.b.void_zero_expr());
            args.push(Arg::Expr(hash_arg));
            args.push(Arg::Expr(directives));
        } else if let Some(hash_arg) = css_hash_arg {
            args.push(Arg::Expr(hash_arg));
        }

        let call = self.b.call_expr("$.attr_class", args);
        self.push_expr(call);
        Ok(())
    }

    fn emit_style(&mut self, element: &'a Element, style: &StyleSemantics) -> Result<()> {
        let has_dirs = !style.directives.is_empty();

        let value = match style.attr.and_then(|id| self.find_attribute(element, id)) {
            Some(attr) => self.attr_value_of(attr, true)?,
            None => match &style.static_base {
                Some(base) => AttrValue::Static(collapse_attribute_whitespace(base).into_owned()),
                None => AttrValue::Static(String::new()),
            },
        };

        if !has_dirs && let AttrValue::Static(s) = value {
            self.push_text(&format!(" style=\"{}\"", escape_attribute(&s)));
            return Ok(());
        }

        let value_expr = match value {
            AttrValue::Static(s) => self.b.str_expr(&s),
            AttrValue::Dynamic(expr) => expr,
        };

        let directives = if has_dirs {
            Some(self.build_style_directives_object(&style.directives)?)
        } else {
            None
        };

        let mut args = vec![Arg::Expr(value_expr)];
        if let Some(directives) = directives {
            args.push(Arg::Expr(directives));
        }
        let call = self.b.call_expr("$.attr_style", args);
        self.push_expr(call);
        Ok(())
    }

    fn build_class_directives_object(
        &mut self,
        element: &'a Element,
        directives: &[svelte_analyze::ClassDirectiveInfo],
        quoted: bool,
    ) -> Result<Expression<'a>> {
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for directive in directives {
            let value = match self.find_class_directive(element, directive.id) {
                Some(cd) => self.take_expression(cd.id, &cd.expression)?,
                None => self.b.rid_expr(&directive.name),
            };
            if quoted {
                props.push(self.b.string_key_prop(&directive.name, value));
            } else {
                let same = expr_is_ident_named(&value, &directive.name);
                props.push(self.b.directive_prop(&directive.name, value, same));
            }
        }
        Ok(self.b.object_expr(props))
    }

    fn build_style_directives_object(
        &mut self,
        directives: &[StyleDirective],
    ) -> Result<Expression<'a>> {
        let mut normal: Vec<ObjProp<'a>> = Vec::new();
        let mut important: Vec<ObjProp<'a>> = Vec::new();

        for directive in directives {
            let value = self.style_directive_value(directive)?;
            let mut name = directive.name.clone();
            if !name.starts_with("--") {
                name = name.to_ascii_lowercase();
            }
            let same = expr_is_ident_named(&value, &name);
            let prop = self.b.directive_prop(&name, value, same);
            if directive.important {
                important.push(prop);
            } else {
                normal.push(prop);
            }
        }

        if important.is_empty() {
            Ok(self.b.object_expr(normal))
        } else {
            let normal = self.b.object_expr(normal);
            let important = self.b.object_expr(important);
            Ok(self.b.array_expr([normal, important]))
        }
    }

    fn style_directive_value(&mut self, directive: &StyleDirective) -> Result<Expression<'a>> {
        match &directive.value {
            StyleDirectiveValue::Expression => {
                self.take_expression(directive.id, &directive.expression)
            }
            StyleDirectiveValue::String(s) => {
                let text = collapse_attribute_whitespace(s.trim());
                Ok(self.b.str_expr(&escape_attribute(&text)))
            }
            StyleDirectiveValue::Concatenation(parts) => {
                match self.attr_value_concat(parts, true)? {
                    AttrValue::Static(s) => Ok(self.b.str_expr(&s)),
                    AttrValue::Dynamic(expr) => Ok(expr),
                }
            }
        }
    }

    fn emit_bind_reflection(
        &mut self,
        attr: &'a Attribute,
        semantics: &ElementBindSemantics,
    ) -> Result<()> {
        let Attribute::BindDirective(directive) = attr else {
            return Ok(());
        };

        match semantics.property {
            ElementBindPropertyKind::Value if semantics.reflects_as_attribute => {
                let expr = self.bind_reflected_expr(directive)?;
                self.push_attr_call("value", expr, false);
            }
            ElementBindPropertyKind::Checked if semantics.reflects_as_attribute => {
                let expr = self.bind_reflected_expr(directive)?;
                self.push_attr_call("checked", expr, true);
            }
            _ => {}
        }
        Ok(())
    }

    fn bind_reflected_expr(&mut self, directive: &'a BindDirective) -> Result<Expression<'a>> {
        let mut expr = self.take_expression(directive.id, &directive.expression)?;
        server_refs::force_store_read(&self.b, self.analysis, &mut expr);
        if let Expression::SequenceExpression(seq) = expr {
            let seq = seq.unbox();
            if let Some(getter) = seq.expressions.into_iter().next() {
                return Ok(self.b.call_expr_callee(getter, empty::<Arg<'a, '_>>()));
            }
            return Ok(self.b.void_zero_expr());
        }
        Ok(expr)
    }

    fn push_attr_call(&mut self, name: &str, expr: Expression<'a>, boolean: bool) {
        let mut args = vec![Arg::Str(name.to_string()), Arg::Expr(expr)];
        if boolean {
            args.push(Arg::Bool(true));
        }
        let call = self.b.call_expr("$.attr", args);
        self.push_expr(call);
    }

    fn emit_spread_attributes(&mut self, element: &'a Element) -> Result<()> {
        let owner_id = element.id;
        let source = self.component.source.as_str();
        let mut props: Vec<ObjProp<'a>> = Vec::new();

        for attr in &element.attributes {
            match attr {
                Attribute::SpreadAttribute(sa) => {
                    let expr = self.take_expression(sa.id, &sa.expression)?;
                    props.push(ObjProp::Spread(expr));
                }
                Attribute::StringAttribute(a) => {
                    let name = self.attribute_name(element, &a.name);
                    let raw = a.value(source);
                    let text = if is_whitespace_insensitive(&name) {
                        collapse_attribute_whitespace(raw).trim().to_string()
                    } else {
                        raw.to_string()
                    };
                    let value = self.b.str_expr(&escape_attribute(&text));
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, value));
                }
                Attribute::BooleanAttribute(a) => {
                    let name = self.attribute_name(element, &a.name);
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, self.b.bool_expr(true)));
                }
                Attribute::ExpressionAttribute(a) => {
                    let name = self.attribute_name(element, &a.name);
                    let expr = self.take_expression(a.id, &a.expression)?;
                    let key = self.b.alloc_str(&name);
                    if self.analysis.elements.flags.is_expression_shorthand(a.id)
                        && expr_is_ident_named(&expr, &name)
                    {
                        props.push(ObjProp::Shorthand(key));
                    } else {
                        props.push(ObjProp::KeyValue(key, expr));
                    }
                }
                Attribute::ConcatenationAttribute(a) => {
                    let name = self.attribute_name(element, &a.name);
                    let value =
                        match self.attr_value_concat(&a.parts, is_whitespace_insensitive(&name))? {
                            AttrValue::Static(s) => self.b.str_expr(&s),
                            AttrValue::Dynamic(expr) => expr,
                        };
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, value));
                }
                Attribute::BindDirective(directive) => {
                    if matches!(
                        self.analysis.attributes.get(directive.id),
                        AttributeSemantics::ElementBind(sem)
                            if matches!(
                                sem.property,
                                ElementBindPropertyKind::This | ElementBindPropertyKind::Group
                            )
                    ) {
                        continue;
                    }
                    let name = self.attribute_name(element, &directive.name);
                    let expr = self.bind_reflected_expr(directive)?;
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, expr));
                }
                _ => {}
            }
        }

        let object = self.b.object_expr(props);

        let class = self.find_class_semantics(element);
        let style = self.find_style_semantics(element);
        let classes = match class.as_ref() {
            Some(class) if !class.directives.is_empty() => {
                Some(self.build_class_directives_object(element, &class.directives, false)?)
            }
            _ => None,
        };
        let styles = match style.as_ref() {
            Some(style) if !style.directives.is_empty() => {
                Some(self.build_style_directives_object(&style.directives)?)
            }
            _ => None,
        };

        let hash = self.analysis.css_hash().to_string();
        let css_hash = if self.analysis.is_css_scoped(owner_id) && !hash.is_empty() {
            Some(self.b.str_expr(&hash))
        } else {
            None
        };

        let flags = self.spread_flags(element);
        let flags_expr = if flags != 0 {
            Some(self.b.num_expr(flags as f64))
        } else {
            None
        };

        let mut optionals: Vec<Option<Expression<'a>>> =
            vec![css_hash, classes, styles, flags_expr];
        while matches!(optionals.last(), Some(None)) {
            optionals.pop();
        }

        let mut args = vec![Arg::Expr(object)];
        for optional in optionals {
            let expr = optional.unwrap_or_else(|| self.b.void_zero_expr());
            args.push(Arg::Expr(expr));
        }

        let call = self.b.call_expr("$.attributes", args);
        self.push_expr(call);
        Ok(())
    }

    fn spread_flags(&self, element: &Element) -> u32 {
        let owner_id = element.id;
        match self.analysis.namespace(owner_id) {
            Some(NamespaceKind::Svg) | Some(NamespaceKind::MathMl) => {
                ELEMENT_IS_NAMESPACED | ELEMENT_PRESERVE_ATTRIBUTE_CASE
            }
            _ if self.analysis.is_custom_element(owner_id) => ELEMENT_PRESERVE_ATTRIBUTE_CASE,
            _ if self.analysis.is_input(owner_id) => ELEMENT_IS_INPUT,
            _ => 0,
        }
    }

    fn attr_value_of(&mut self, attr: &'a Attribute, trim: bool) -> Result<AttrValue<'a>> {
        match attr {
            Attribute::StringAttribute(a) => {
                let raw = a.value(self.component.source.as_str());
                Ok(AttrValue::Static(if trim {
                    collapse_attribute_whitespace(raw).trim().to_string()
                } else {
                    raw.to_string()
                }))
            }
            Attribute::ExpressionAttribute(a) => self.attr_value_single(a.id, &a.expression),
            Attribute::ConcatenationAttribute(a) => self.attr_value_concat(&a.parts, trim),
            _ => Ok(AttrValue::Static(String::new())),
        }
    }

    fn attr_value_single(&mut self, attr_id: NodeId, expr_ref: &ExprRef) -> Result<AttrValue<'a>> {
        if let Some(value) = self.analysis.expression_data(attr_id).and_then(|data| {
            data.references
                .is_empty()
                .then(|| data.evaluation.known_str())
                .flatten()
        }) {
            return Ok(AttrValue::Static(value));
        }
        let expr = self.take_expression(attr_id, expr_ref)?;
        Ok(AttrValue::Dynamic(expr))
    }

    fn attr_value_concat(&mut self, parts: &[ConcatPart], trim: bool) -> Result<AttrValue<'a>> {
        let mut segments: Vec<TemplatePart<'a>> = Vec::new();
        let mut has_dynamic = false;

        for part in parts {
            match part {
                ConcatPart::Static(text) => {
                    let text = if trim {
                        collapse_attribute_whitespace(text).into_owned()
                    } else {
                        text.clone()
                    };
                    push_template_str(&mut segments, &text);
                }
                ConcatPart::Dynamic { id, expr } => {
                    let evaluation = self
                        .analysis
                        .expression_data(*id)
                        .map(|data| data.evaluation.clone());
                    if let Some(known) = evaluation.as_ref().and_then(|e| e.known_str()) {
                        push_template_str(&mut segments, &known);
                        continue;
                    }
                    has_dynamic = true;
                    let is_defined_string =
                        evaluation.as_ref().is_some_and(|e| e.is_defined_string());
                    let value = self.take_expression(*id, expr)?;
                    let value = if is_defined_string {
                        value
                    } else {
                        self.b.call_expr("$.stringify", [Arg::Expr(value)])
                    };
                    segments.push(TemplatePart::Expr(value, true));
                }
            }
        }

        if !has_dynamic {
            let text = match segments.into_iter().next() {
                Some(TemplatePart::Str(s)) => s,
                _ => String::new(),
            };
            return Ok(AttrValue::Static(text));
        }

        Ok(AttrValue::Dynamic(self.b.template_parts_expr(segments)))
    }

    fn attribute_name(&self, element: &Element, raw: &str) -> String {
        let namespaced = matches!(
            self.analysis.namespace(element.id),
            Some(NamespaceKind::Svg) | Some(NamespaceKind::MathMl)
        );
        emit_html_attribute_name(raw, namespaced).into_owned()
    }

    fn find_attribute(&self, element: &'a Element, id: NodeId) -> Option<&'a Attribute> {
        element.attributes.iter().find(|a| a.id() == id)
    }

    fn find_class_semantics(&self, element: &Element) -> Option<ClassSemantics> {
        element
            .attributes
            .iter()
            .find_map(|a| match self.analysis.attributes.get(a.id()) {
                AttributeSemantics::Class(class) => Some(class.clone()),
                _ => None,
            })
    }

    fn find_style_semantics(&self, element: &Element) -> Option<StyleSemantics> {
        element
            .attributes
            .iter()
            .find_map(|a| match self.analysis.attributes.get(a.id()) {
                AttributeSemantics::Style(style) => Some(style.clone()),
                _ => None,
            })
    }

    fn find_class_directive(&self, element: &'a Element, id: NodeId) -> Option<&'a ClassDirective> {
        element.attributes.iter().find_map(|a| match a {
            Attribute::ClassDirective(cd) if cd.id == id => Some(cd),
            _ => None,
        })
    }
}

fn is_whitespace_insensitive(name: &str) -> bool {
    name == "class" || name == "style"
}

fn expr_is_ident_named(expr: &Expression<'_>, name: &str) -> bool {
    expr.get_inner_expression().is_specific_id(name)
}

fn push_template_str<'a>(parts: &mut Vec<TemplatePart<'a>>, text: &str) {
    if let Some(TemplatePart::Str(prev)) = parts.last_mut() {
        prev.push_str(text);
    } else {
        parts.push(TemplatePart::Str(text.to_string()));
    }
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn emit_component_attribute(
        &mut self,
        attr: &'a Attribute,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let attr_id = attr.id();
        match self.analysis.attributes.get(attr_id) {
            AttributeSemantics::ComponentProp(ComponentPropSemantics::Expression(_)) => {
                let Attribute::ExpressionAttribute(ea) = attr else {
                    return Err(CodegenError::Unsupported(attr_id, "component prop"));
                };
                let key: &'a str = self.b.alloc_str(&ea.name);
                let value = self.take_expression(attr_id, &ea.expression)?;
                items.push(PropOrSpread::Prop(prop_kv(key, value)));
                Ok(())
            }
            AttributeSemantics::ComponentSpread(_) => {
                let Attribute::SpreadAttribute(sa) = attr else {
                    return Err(CodegenError::Unsupported(attr_id, "component spread"));
                };
                let value = self.take_expression(attr_id, &sa.expression)?;
                items.push(PropOrSpread::Spread(value));
                Ok(())
            }
            AttributeSemantics::NonSpecial => match attr {
                Attribute::StringAttribute(a) => {
                    let key: &'a str = self.b.alloc_str(&a.name);
                    let value = a.value(self.component.source.as_str());
                    items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                        key,
                        self.b.str_expr(value),
                    )));
                    Ok(())
                }
                Attribute::BooleanAttribute(a) => {
                    let key: &'a str = self.b.alloc_str(&a.name);
                    items.push(PropOrSpread::Prop(ObjProp::KeyValue(
                        key,
                        self.b.bool_expr(true),
                    )));
                    Ok(())
                }
                _ => Err(CodegenError::Unsupported(attr_id, "component attribute")),
            },
            _ => Err(CodegenError::Unsupported(attr_id, "component attribute")),
        }
    }

    pub(crate) fn build_props_expr(&self, items: Vec<PropOrSpread<'a>>) -> Expression<'a> {
        let has_spread = items.iter().any(|i| matches!(i, PropOrSpread::Spread(_)));
        if !has_spread {
            return self
                .b
                .object_expr(items.into_iter().filter_map(|i| match i {
                    PropOrSpread::Prop(p) => Some(p),
                    PropOrSpread::Spread(_) => None,
                }));
        }

        let mut args: Vec<Arg<'a, 'a>> = Vec::new();
        let mut current: Vec<ObjProp<'a>> = Vec::new();
        for item in items {
            match item {
                PropOrSpread::Prop(p) => current.push(p),
                PropOrSpread::Spread(expr) => {
                    if !current.is_empty() {
                        args.push(Arg::Expr(self.b.object_expr(mem::take(&mut current))));
                    }
                    args.push(Arg::Expr(expr));
                }
            }
        }
        if !current.is_empty() {
            args.push(Arg::Expr(self.b.object_expr(current)));
        }
        let array = self.b.array_from_args(args);
        self.b.call_expr("$.spread_props", [Arg::Expr(array)])
    }
}

fn prop_kv<'a>(key: &'a str, value: Expression<'a>) -> ObjProp<'a> {
    if let Expression::Identifier(id) = &value
        && id.name.as_str() == key
    {
        return ObjProp::Shorthand(key);
    }
    ObjProp::KeyValue(key, value)
}
