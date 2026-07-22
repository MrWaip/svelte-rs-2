use std::iter::empty;
use std::mem;

use oxc_allocator::CloneIn;
use oxc_ast::ast::Expression;
use oxc_span::SPAN;
use oxc_syntax::operator::BinaryOperator;
use svelte_analyze::{
    AttributeSemantics, ClassSemantics, ComponentBindKind, ComponentBindSemantics,
    ComponentBindTarget, ComponentPropSemantics, ElementBindPropertyKind, ElementBindSemantics,
    ElementSemantics, ElementValueRole, GroupBindValue, GroupReflection, NamespaceKind,
    SpecialValueKind, StyleSemantics, collapse_attribute_whitespace, emit_html_attribute_name,
    is_dom_boolean_attribute,
};
use svelte_ast::{
    Attribute, BindDirective, ClassDirective, ConcatPart, ExprRef, NodeId, StyleDirective,
    StyleDirectiveValue,
};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp, TemplatePart};
use svelte_emit_builders::server_refs;

use crate::error::{CodegenError, Result};
use crate::escape::escape_attribute;
use crate::model::ServerCodegen;

pub(crate) enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

fn attr_is_plain_value(attr: &Attribute) -> bool {
    match attr {
        Attribute::ExpressionAttribute(a) => a.name == "value",
        Attribute::ConcatenationAttribute(a) => a.name == "value",
        Attribute::StringAttribute(a) => a.name == "value",
        _ => false,
    }
}

const ELEMENT_IS_NAMESPACED: u32 = 1;
const ELEMENT_PRESERVE_ATTRIBUTE_CASE: u32 = 1 << 1;
const ELEMENT_IS_INPUT: u32 = 1 << 2;

enum AttrValue<'a> {
    Static(String),
    Dynamic(Expression<'a>),
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn emit_element_attributes(
        &mut self,
        owner_id: NodeId,
        attributes: &'a [Attribute],
    ) -> Result<()> {
        if self.analysis.has_spread(owner_id) {
            return self.emit_spread_attributes(owner_id, attributes);
        }

        let is_textarea_value = self.is_textarea_value_element(owner_id);

        let mut group_value = self.capture_group_value(attributes)?;
        let mut deferred_class: Option<ClassSemantics> = None;
        let mut deferred_style: Option<StyleSemantics> = None;

        for attr in attributes {
            if is_textarea_value && attr_is_plain_value(attr) {
                continue;
            }
            match self.analysis.attributes.get(attr.id()).clone() {
                AttributeSemantics::Class(class) => {
                    if class.attr.is_none() && class.static_attr.is_none() {
                        deferred_class = Some(class);
                    } else {
                        self.emit_class(owner_id, attributes, &class)?;
                    }
                }
                AttributeSemantics::Style(style) => {
                    if style.attr.is_none() && style.static_attr.is_none() {
                        deferred_style = Some(style);
                    } else {
                        self.emit_style(owner_id, attributes, &style)?;
                    }
                }
                AttributeSemantics::Skip(_) => {}
                AttributeSemantics::ElementBind(sem) => {
                    self.emit_bind_reflection(attr, &sem, &mut group_value)?;
                }
                AttributeSemantics::CannotBeStatic(sem) => {
                    if sem.reflects_in_html {
                        self.emit_plain_attribute(owner_id, attr)?;
                    }
                }
                AttributeSemantics::NonSpecial
                | AttributeSemantics::StaticAttr
                | AttributeSemantics::Autofocus
                | AttributeSemantics::HtmlConcat(_) => {
                    self.emit_plain_attribute(owner_id, attr)?;
                }
                AttributeSemantics::SpecialValueAttr(sem)
                    if matches!(sem.kind, SpecialValueKind::InputBindGroup) =>
                {
                    self.emit_group_value_attribute(attr)?;
                }
                AttributeSemantics::SpecialValueAttr(sem)
                    if matches!(sem.kind, SpecialValueKind::InputBindChecked)
                        && matches!(
                            attr,
                            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
                        ) =>
                {
                    self.emit_plain_attribute(owner_id, attr)?;
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

        if let Some(class) = deferred_class {
            self.emit_class(owner_id, attributes, &class)?;
        } else if self.analysis.is_css_scoped(owner_id)
            && self.find_class_semantics(attributes).is_none()
        {
            let hash = self.analysis.css_hash();
            if !hash.is_empty() {
                self.push_text(&format!(" class=\"{}\"", escape_attribute(hash)));
            }
        }
        if let Some(style) = deferred_style {
            self.emit_style(owner_id, attributes, &style)?;
        }

        Ok(())
    }

    fn is_textarea_value_element(&self, owner_id: NodeId) -> bool {
        matches!(
            self.analysis.element_semantics.query(owner_id),
            ElementSemantics::RegularElement(sem)
                if matches!(sem.value_role, ElementValueRole::TextareaValue { .. })
        )
    }

    fn emit_plain_attribute(&mut self, owner_id: NodeId, attr: &'a Attribute) -> Result<()> {
        match attr {
            Attribute::StringAttribute(a) => {
                let name = self.attribute_name(owner_id, &a.name);
                let value = a.value(self.component.source.as_str());
                self.push_text(&format!(" {name}=\"{}\"", escape_attribute(value)));
            }
            Attribute::BooleanAttribute(a) => {
                let name = self.attribute_name(owner_id, &a.name);
                self.push_text(&format!(" {name}=\"\""));
            }
            Attribute::ExpressionAttribute(a) => {
                let name = self.attribute_name(owner_id, &a.name);
                let value = self.attr_value_single(a.id, &a.expression)?;
                self.push_named_value(&name, value);
            }
            Attribute::ConcatenationAttribute(a) => {
                let name = self.attribute_name(owner_id, &a.name);
                let value = if let [ConcatPart::Dynamic { id, expr }] = a.parts.as_slice() {
                    self.attr_value_single(*id, expr)?
                } else {
                    self.attr_value_concat(&a.parts, false)?
                };
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

    fn emit_class(
        &mut self,
        owner_id: NodeId,
        attributes: &'a [Attribute],
        class: &ClassSemantics,
    ) -> Result<()> {
        let has_dirs = !class.directives.is_empty();
        let scoped = self.analysis.is_css_scoped(owner_id);
        let hash = self.analysis.css_hash().to_string();

        let value = match class
            .attr
            .and_then(|id| self.find_attribute(attributes, id))
        {
            Some(attr) => self.attr_value_of(attr, true)?,
            None => match &class.static_base {
                Some(base) => {
                    AttrValue::Static(collapse_attribute_whitespace(base).trim().to_string())
                }
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

        let value_is_expr_string_literal = matches!(
            class.attr.and_then(|id| self.find_attribute(attributes, id)),
            Some(Attribute::ExpressionAttribute(a))
                if matches!(
                    self.js_arena.expr(a.expression.id()),
                    Some(Expression::StringLiteral(_))
                )
        );

        let mut css_hash_arg: Option<Expression<'a>> = None;
        if scoped && !hash.is_empty() && !value_is_expr_string_literal {
            if let Expression::StringLiteral(lit) = &value_expr {
                let merged = format!("{} {}", lit.value.as_str(), hash);
                value_expr = self.b.str_expr(merged.trim());
            } else {
                css_hash_arg = Some(self.b.str_expr(&hash));
            }
        }

        let directives = if has_dirs {
            Some(self.build_class_directives_object(attributes, &class.directives, true)?)
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

    fn emit_style(
        &mut self,
        _owner_id: NodeId,
        attributes: &'a [Attribute],
        style: &StyleSemantics,
    ) -> Result<()> {
        let has_dirs = !style.directives.is_empty();

        let value = match style
            .attr
            .and_then(|id| self.find_attribute(attributes, id))
        {
            Some(attr) => self.attr_value_of(attr, true)?,
            None => match &style.static_base {
                Some(base) => {
                    AttrValue::Static(collapse_attribute_whitespace(base).trim().to_string())
                }
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
        attributes: &'a [Attribute],
        directives: &[svelte_analyze::ClassDirectiveInfo],
        quoted: bool,
    ) -> Result<Expression<'a>> {
        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for directive in directives {
            let value = match self.find_class_directive(attributes, directive.id) {
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

    fn capture_group_value(
        &mut self,
        attributes: &'a [Attribute],
    ) -> Result<Option<Expression<'a>>> {
        for attr in attributes {
            let Attribute::BindDirective(directive) = attr else {
                continue;
            };
            let AttributeSemantics::ElementBind(sem) =
                self.analysis.attributes.get(directive.id).clone()
            else {
                continue;
            };
            if matches!(sem.property, ElementBindPropertyKind::Group) && sem.reflects_as_attribute {
                return self.group_bind_value_expr(sem.group_value, directive.id, attributes);
            }
        }
        Ok(None)
    }

    fn emit_bind_reflection(
        &mut self,
        attr: &'a Attribute,
        semantics: &ElementBindSemantics,
        group_value: &mut Option<Expression<'a>>,
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
            ElementBindPropertyKind::Open if semantics.reflects_as_attribute => {
                let expr = self.bind_reflected_expr(directive)?;
                self.push_attr_call("open", expr, true);
            }
            ElementBindPropertyKind::Focused if semantics.reflects_as_attribute => {
                let expr = self.bind_reflected_expr(directive)?;
                self.push_attr_call("focused", expr, false);
            }
            ElementBindPropertyKind::Group if semantics.reflects_as_attribute => {
                let value = group_value.take();
                let reflection = semantics
                    .group_reflection
                    .unwrap_or(GroupReflection::Includes);
                if let Some(checked) = self.group_checked_expr(directive, reflection, value)? {
                    self.push_attr_call("checked", checked, true);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn group_bind_checked_expr(
        &mut self,
        directive: &'a BindDirective,
        group_value: Option<GroupBindValue>,
        reflection: GroupReflection,
        attributes: &'a [Attribute],
    ) -> Result<Option<Expression<'a>>> {
        let value = self.group_bind_value_expr(group_value, directive.id, attributes)?;
        self.group_checked_expr(directive, reflection, value)
    }

    fn group_checked_expr(
        &mut self,
        directive: &'a BindDirective,
        reflection: GroupReflection,
        value: Option<Expression<'a>>,
    ) -> Result<Option<Expression<'a>>> {
        let Some(value) = value else {
            return Ok(None);
        };
        let group = self.bind_reflected_expr(directive)?;
        let checked = match reflection {
            GroupReflection::Equality => {
                self.b
                    .ast
                    .expression_binary(SPAN, group, BinaryOperator::StrictEquality, value)
            }
            GroupReflection::Includes => self.b.call_expr_callee(
                self.b.static_member_expr(group, "includes"),
                [Arg::Expr(value)],
            ),
        };
        Ok(Some(checked))
    }

    fn emit_component_bind(
        &mut self,
        directive: &'a BindDirective,
        sem: &ComponentBindSemantics,
        items: &mut Vec<PropOrSpread<'a>>,
    ) -> Result<()> {
        let key: &'a str = self.b.alloc_str(&directive.name);
        let expr = self.take_expression(directive.id, &directive.expression)?;
        let (get_expr, set_stmts) = match &sem.kind {
            ComponentBindKind::FunctionPair => {
                let Expression::SequenceExpression(seq) = expr else {
                    return Err(CodegenError::Unsupported(
                        directive.id,
                        "component bind pair",
                    ));
                };
                let mut parts = seq.unbox().expressions.into_iter();
                let (Some(get_fn), Some(set_fn)) = (parts.next(), parts.next()) else {
                    return Err(CodegenError::Unsupported(
                        directive.id,
                        "component bind pair",
                    ));
                };
                let (get_name, set_name) = match self.bind_pair_names.get(&directive.id) {
                    Some((g, s)) => (self.b.alloc_str(g), self.b.alloc_str(s)),
                    None => (
                        self.b.alloc_str(&self.ident_gen.generate("bind_get")),
                        self.b.alloc_str(&self.ident_gen.generate("bind_set")),
                    ),
                };
                self.hoist_stmt(self.b.var_stmt(get_name, get_fn));
                self.hoist_stmt(self.b.var_stmt(set_name, set_fn));
                let get_call = self.b.call_expr(get_name, empty::<Arg<'a, '_>>());
                let set_call = self.b.call_expr(set_name, [Arg::Ident("$$value")]);
                (get_call, vec![self.b.expr_stmt(set_call)])
            }
            ComponentBindKind::StoreSubscribed { base_symbol } => {
                let mut get_expr = expr;
                server_refs::force_store_read(&self.b, self.analysis, &mut get_expr);
                let base_name: &'a str = self
                    .b
                    .alloc_str(self.analysis.scoping.symbol_name(*base_symbol));
                let base = self.b.rid_expr(base_name);
                let store_set =
                    server_refs::server_store_set(&self.b, base, self.b.rid_expr("$$value"));
                let settled = self.b.assign_stmt(
                    AssignLeft::Ident("$$settled".to_string()),
                    self.b.bool_expr(false),
                );
                (get_expr, vec![self.b.expr_stmt(store_set), settled])
            }
            ComponentBindKind::StoreMemberMutation { store_symbol } => {
                let get_expr = self.b.clone_expr(&expr);
                let target = self.b.expr_to_assignment_target(expr);
                let assign = self.b.assign_expr_raw(target, self.b.rid_expr("$$value"));
                let dollar_name = self.analysis.scoping.symbol_name(*store_symbol).to_string();
                let base_sym = server_refs::store_base_symbol(self.analysis, *store_symbol)
                    .unwrap_or(*store_symbol);
                let base_name: &str = self
                    .b
                    .alloc_str(self.analysis.scoping.symbol_name(base_sym));
                let base = self.b.rid_expr(base_name);
                let mutation =
                    server_refs::server_store_mutate(&self.b, &dollar_name, base, assign);
                let settled = self.b.assign_stmt(
                    AssignLeft::Ident("$$settled".to_string()),
                    self.b.bool_expr(false),
                );
                (get_expr, vec![self.b.expr_stmt(mutation), settled])
            }
            ComponentBindKind::This { .. } => {
                return Err(CodegenError::Unsupported(
                    directive.id,
                    "component bind this",
                ));
            }
            ComponentBindKind::Identifier {
                symbol,
                target: ComponentBindTarget::RuneDerived,
            } => {
                let name: &'a str = self.b.alloc_str(self.analysis.scoping.symbol_name(*symbol));
                let get_expr = self.b.call_expr(name, empty::<Arg<'a, '_>>());
                let set_call = self.b.call_expr(name, [Arg::Ident("$$value")]);
                let settled = self.b.assign_stmt(
                    AssignLeft::Ident("$$settled".to_string()),
                    self.b.bool_expr(false),
                );
                (get_expr, vec![self.b.expr_stmt(set_call), settled])
            }
            ComponentBindKind::Expression | ComponentBindKind::Identifier { .. } => {
                let get_expr = self.b.clone_expr(&expr);
                let target = self.b.expr_to_assignment_target(expr);
                let assign = self.b.assign_expr_raw(target, self.b.rid_expr("$$value"));
                let settled = self.b.assign_stmt(
                    AssignLeft::Ident("$$settled".to_string()),
                    self.b.bool_expr(false),
                );
                (get_expr, vec![self.b.expr_stmt(assign), settled])
            }
        };
        items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_expr)));
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key, "$$value", None, set_stmts,
        )));
        Ok(())
    }

    fn emit_group_value_attribute(&mut self, attr: &'a Attribute) -> Result<()> {
        match attr {
            Attribute::StringAttribute(a) => {
                let value = a.value(self.component.source.as_str());
                self.push_text(&format!(" value=\"{}\"", escape_attribute(value)));
            }
            Attribute::BooleanAttribute(_) => {
                self.push_text(" value=\"\"");
            }
            Attribute::ExpressionAttribute(a) => {
                let value = self.attr_value_single(a.id, &a.expression)?;
                self.push_named_value("value", value);
            }
            Attribute::ConcatenationAttribute(a) => {
                let value = if let [ConcatPart::Dynamic { id, expr }] = a.parts.as_slice() {
                    self.attr_value_single(*id, expr)?
                } else {
                    self.attr_value_concat(&a.parts, false)?
                };
                self.push_named_value("value", value);
            }
            _ => {}
        }
        Ok(())
    }

    fn group_bind_value_expr(
        &mut self,
        group_value: Option<GroupBindValue>,
        owner: NodeId,
        attributes: &'a [Attribute],
    ) -> Result<Option<Expression<'a>>> {
        let alloc = self.b.ast.allocator;
        match group_value {
            Some(GroupBindValue::Expression { expression, .. }) => {
                let expr = self
                    .js_arena
                    .expr(expression)
                    .ok_or(CodegenError::MissingExpression(owner))?;
                Ok(Some(expr.clone_in(alloc)))
            }
            Some(GroupBindValue::Static { node }) => {
                Ok(self.group_static_value_text(node, attributes).map(|text| {
                    let text: &str = &text;
                    self.b.str_expr(text)
                }))
            }
            None => Ok(None),
        }
    }

    fn group_static_value_text(&self, node: NodeId, attributes: &'a [Attribute]) -> Option<String> {
        let source = self.component.source.as_str();
        attributes.iter().find_map(|attr| match attr {
            Attribute::StringAttribute(a) if a.id == node => Some(a.value(source).to_string()),
            Attribute::ConcatenationAttribute(a) if a.id == node => match a.parts.as_slice() {
                [ConcatPart::Static(text)] => Some(text.clone()),
                _ => None,
            },
            _ => None,
        })
    }

    fn bind_reflected_expr(&mut self, directive: &'a BindDirective) -> Result<Expression<'a>> {
        let mut expr = self.take_expression(directive.id, &directive.expression)?;
        server_refs::force_derived_read(&self.b, self.analysis, &mut expr);
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

    fn emit_spread_attributes(
        &mut self,
        owner_id: NodeId,
        attributes: &'a [Attribute],
    ) -> Result<()> {
        let (object, optionals) = self.build_element_attribute_object(owner_id, attributes)?;
        let mut args = vec![Arg::Expr(object)];
        args.extend(self.optional_trailing_args(optionals.into_iter().collect()));
        let call = self.b.call_expr("$.attributes", args);
        self.push_expr(call);
        Ok(())
    }

    pub(crate) fn optional_trailing_args(
        &self,
        mut optionals: Vec<Option<Expression<'a>>>,
    ) -> Vec<Arg<'a, 'a>> {
        while matches!(optionals.last(), Some(None)) {
            optionals.pop();
        }
        optionals
            .into_iter()
            .map(|optional| Arg::Expr(optional.unwrap_or_else(|| self.b.void_zero_expr())))
            .collect()
    }

    pub(crate) fn build_element_attribute_object(
        &mut self,
        owner_id: NodeId,
        attributes: &'a [Attribute],
    ) -> Result<(Expression<'a>, [Option<Expression<'a>>; 4])> {
        let source = self.component.source.as_str();
        let mut props: Vec<ObjProp<'a>> = Vec::new();

        let is_textarea_value = self.is_textarea_value_element(owner_id);

        let is_select_or_option = matches!(
            self.analysis.element_semantics.query(owner_id),
            ElementSemantics::RegularElement(sem)
                if matches!(
                    sem.value_role,
                    ElementValueRole::Select { .. } | ElementValueRole::Option { .. }
                )
        );

        let (class_attr_id, class_needs_clsx) = match self.find_class_semantics(attributes) {
            Some(class) => (class.attr, class.needs_clsx),
            None => (None, false),
        };

        for attr in attributes {
            if matches!(
                self.analysis.attributes.get(attr.id()),
                AttributeSemantics::Skip(_) | AttributeSemantics::Event(_)
            ) {
                continue;
            }
            if let AttributeSemantics::CannotBeStatic(sem) = self.analysis.attributes.get(attr.id())
                && !sem.reflects_in_html
            {
                continue;
            }
            if is_textarea_value
                && (attr_is_plain_value(attr)
                    || matches!(attr, Attribute::BindDirective(d) if d.name == "value"))
            {
                continue;
            }
            match attr {
                Attribute::SpreadAttribute(sa) => {
                    let expr = self.take_expression(sa.id, &sa.expression)?;
                    props.push(ObjProp::Spread(expr));
                }
                Attribute::StringAttribute(a) => {
                    let name = self.attribute_name(owner_id, &a.name);
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
                    let name = self.attribute_name(owner_id, &a.name);
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, self.b.bool_expr(true)));
                }
                Attribute::ExpressionAttribute(a) => {
                    let name = self.attribute_name(owner_id, &a.name);
                    let mut expr = self.take_expression(a.id, &a.expression)?;
                    let key = self.b.alloc_str(&name);
                    if class_needs_clsx && class_attr_id == Some(a.id) {
                        expr = self.b.call_expr("$.clsx", [Arg::Expr(expr)]);
                        props.push(ObjProp::KeyValue(key, expr));
                    } else if self.analysis.elements.flags.is_expression_shorthand(a.id)
                        && expr_is_ident_named(&expr, &name)
                    {
                        props.push(ObjProp::Shorthand(key));
                    } else {
                        props.push(ObjProp::KeyValue(key, expr));
                    }
                }
                Attribute::ConcatenationAttribute(a) => {
                    let name = self.attribute_name(owner_id, &a.name);
                    let value =
                        match self.attr_value_concat(&a.parts, is_whitespace_insensitive(&name))? {
                            AttrValue::Static(s) => self.b.str_expr(&s),
                            AttrValue::Dynamic(expr) => expr,
                        };
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, value));
                }
                Attribute::BindDirective(directive) => {
                    let (property, group_value, group_reflection) =
                        match self.analysis.attributes.get(directive.id) {
                            AttributeSemantics::ElementBind(sem) => {
                                (Some(sem.property), sem.group_value, sem.group_reflection)
                            }
                            _ => (None, None, None),
                        };
                    match property {
                        Some(ElementBindPropertyKind::This) => {
                            if is_select_or_option {
                                let expr =
                                    self.take_expression(directive.id, &directive.expression)?;
                                props.push(ObjProp::KeyValue("this", expr));
                            }
                            continue;
                        }
                        Some(ElementBindPropertyKind::Group) => {
                            let reflection = group_reflection.unwrap_or(GroupReflection::Includes);
                            if let Some(checked) = self.group_bind_checked_expr(
                                directive,
                                group_value,
                                reflection,
                                attributes,
                            )? {
                                props.push(ObjProp::KeyValue("checked", checked));
                            }
                            continue;
                        }
                        _ => {}
                    }
                    if let Some(property) = property
                        && !property.reflects_in_html()
                    {
                        continue;
                    }
                    let name = self.attribute_name(owner_id, &directive.name);
                    let expr = self.bind_reflected_expr(directive)?;
                    let key = self.b.alloc_str(&name);
                    props.push(ObjProp::KeyValue(key, expr));
                }
                _ => {}
            }
        }

        let hash_str = self.analysis.css_hash().to_string();
        let scoped = self.analysis.is_css_scoped(owner_id) && !hash_str.is_empty();
        let has_class_attr = self
            .find_class_semantics(attributes)
            .is_some_and(|class| class.attr.is_some() || class.static_attr.is_some());
        if is_select_or_option && scoped && !has_class_attr {
            props.push(ObjProp::KeyValue("class", self.b.str_expr("")));
        }

        let object = self.b.object_expr(props);

        let class = self.find_class_semantics(attributes);
        let style = self.find_style_semantics(attributes);
        let classes = match class.as_ref() {
            Some(class) if !class.directives.is_empty() => {
                Some(self.build_class_directives_object(attributes, &class.directives, false)?)
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

        let flags = self.spread_flags(owner_id);
        let flags_expr = if flags != 0 {
            Some(self.b.num_expr(flags as f64))
        } else {
            None
        };

        Ok((object, [css_hash, classes, styles, flags_expr]))
    }

    fn spread_flags(&self, owner_id: NodeId) -> u32 {
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
        let is_string_literal = self.js_arena.expr(expr_ref.id()).is_some_and(|expr| {
            matches!(expr.get_inner_expression(), Expression::StringLiteral(_))
        });
        if is_string_literal
            && let Some(value) = self.analysis.expression_data(attr_id).and_then(|data| {
                (data.references.is_empty() && data.declared_evaluation.is_defined_string())
                    .then(|| data.declared_evaluation.known_str())
                    .flatten()
            })
        {
            return Ok(AttrValue::Static(value));
        }
        let expr = self.take_expression(attr_id, expr_ref)?;
        let expr = self.maybe_hoist_async_expr(attr_id, expr);
        Ok(AttrValue::Dynamic(expr))
    }

    pub(crate) fn concat_value_expr(&mut self, parts: &[ConcatPart]) -> Result<Expression<'a>> {
        Ok(match self.attr_value_concat(parts, false)? {
            AttrValue::Static(s) => self.b.str_expr(&s),
            AttrValue::Dynamic(expr) => expr,
        })
    }

    fn attr_value_concat(&mut self, parts: &[ConcatPart], trim: bool) -> Result<AttrValue<'a>> {
        if let [single] = parts {
            return match single {
                ConcatPart::Static(text) => Ok(AttrValue::Static(if trim {
                    collapse_attribute_whitespace(text).trim().to_string()
                } else {
                    text.clone()
                })),
                ConcatPart::Dynamic { id, expr } => self.attr_value_single(*id, expr),
            };
        }

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
                        .map(|data| data.declared_evaluation.clone());
                    if let Some(known) = evaluation.as_ref().and_then(|e| e.known_str()) {
                        push_template_str(&mut segments, &known);
                        continue;
                    }
                    has_dynamic = true;
                    let is_defined_string =
                        evaluation.as_ref().is_some_and(|e| e.is_defined_string());
                    let value = self.take_expression(*id, expr)?;
                    segments.push(TemplatePart::Expr(value, is_defined_string));
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

        if segments.len() == 1 {
            return Ok(match segments.into_iter().next() {
                Some(TemplatePart::Expr(expr, _)) => AttrValue::Dynamic(expr),
                Some(TemplatePart::Str(text)) => AttrValue::Static(text),
                None => AttrValue::Static(String::new()),
            });
        }

        let mut template_parts: Vec<TemplatePart<'a>> = Vec::with_capacity(segments.len());
        for segment in segments {
            match segment {
                TemplatePart::Str(text) => template_parts.push(TemplatePart::Str(text)),
                TemplatePart::Expr(expr, true) => {
                    template_parts.push(TemplatePart::Expr(expr, true))
                }
                TemplatePart::Expr(expr, false) => {
                    let value = self.b.call_expr("$.stringify", [Arg::Expr(expr)]);
                    template_parts.push(TemplatePart::Expr(value, true));
                }
            }
        }

        Ok(AttrValue::Dynamic(
            self.b.template_parts_expr(template_parts),
        ))
    }

    fn attribute_name(&self, owner_id: NodeId, raw: &str) -> String {
        let namespaced = matches!(
            self.analysis.namespace(owner_id),
            Some(NamespaceKind::Svg) | Some(NamespaceKind::MathMl)
        );
        emit_html_attribute_name(raw, namespaced).into_owned()
    }

    fn find_attribute(&self, attributes: &'a [Attribute], id: NodeId) -> Option<&'a Attribute> {
        attributes.iter().find(|a| a.id() == id)
    }

    fn find_class_semantics(&self, attributes: &[Attribute]) -> Option<ClassSemantics> {
        attributes
            .iter()
            .find_map(|a| match self.analysis.attributes.get(a.id()) {
                AttributeSemantics::Class(class) => Some(class.clone()),
                _ => None,
            })
    }

    fn find_style_semantics(&self, attributes: &[Attribute]) -> Option<StyleSemantics> {
        attributes
            .iter()
            .find_map(|a| match self.analysis.attributes.get(a.id()) {
                AttributeSemantics::Style(style) => Some(style.clone()),
                _ => None,
            })
    }

    fn find_class_directive(
        &self,
        attributes: &'a [Attribute],
        id: NodeId,
    ) -> Option<&'a ClassDirective> {
        attributes.iter().find_map(|a| match a {
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
            AttributeSemantics::ComponentProp(ComponentPropSemantics::Concat(_)) => {
                let Attribute::ConcatenationAttribute(ca) = attr else {
                    return Err(CodegenError::Unsupported(attr_id, "component prop concat"));
                };
                let key: &'a str = self.b.alloc_str(&ca.name);
                let value = match self.attr_value_concat(&ca.parts, false)? {
                    AttrValue::Static(s) => self.b.str_expr(&s),
                    AttrValue::Dynamic(expr) => expr,
                };
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
            AttributeSemantics::ComponentBind(sem) => {
                let Attribute::BindDirective(directive) = attr else {
                    return Err(CodegenError::Unsupported(attr_id, "component bind"));
                };
                self.emit_component_bind(directive, sem, items)
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
            AttributeSemantics::Event(_) => Ok(()),
            AttributeSemantics::Skip(_) => Ok(()),
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
