use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{ExprSite, NamespaceKind};
use svelte_ast::{Attribute, Element, NodeId};
use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};

use super::super::data_structures::{MemoAttr, MemoAttrUpdate};
use super::super::{Codegen, CodegenError, Result};

pub(super) enum RegularAttrUpdate {
    Call {
        setter_fn: &'static str,
        attr_name: Option<String>,
    },
    Assignment {
        property: String,
    },
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn is_html_attr_namespace(&self, el_id: NodeId) -> bool {
        !matches!(
            self.ctx.query.view.namespace(el_id),
            Some(NamespaceKind::Svg)
                | Some(NamespaceKind::MathMl)
                | Some(NamespaceKind::AnnotationXml)
        )
    }

    pub(super) fn attr_blockers(&self, attr_id: NodeId) -> Vec<u32> {
        self.ctx
            .expr_deps(ExprSite::Attr(attr_id))
            .map(|deps| deps.blockers.into_iter().collect())
            .unwrap_or_default()
    }

    pub(super) fn regular_attr_update(
        &self,
        el_id: NodeId,
        tag_name: &str,
        attr_name: &str,
    ) -> RegularAttrUpdate {
        let el = self.ctx.element(el_id);

        if attr_name == "value" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_value",
                attr_name: None,
            };
        }

        if attr_name == "checked" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_checked",
                attr_name: None,
            };
        }

        if attr_name == "selected" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_selected",
                attr_name: None,
            };
        }

        if attr_name == "defaultValue"
            && (has_static_text_attribute(el, "value")
                || (tag_name == "textarea" && !el.fragment.nodes.is_empty()))
        {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_default_value",
                attr_name: None,
            };
        }

        if attr_name == "defaultChecked" && has_static_true_boolean_attribute(el, "checked") {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_default_checked",
                attr_name: None,
            };
        }

        if attr_name == "style" {
            return RegularAttrUpdate::Call {
                setter_fn: "$.set_style",
                attr_name: None,
            };
        }

        if svelte_analyze::is_regular_dom_property(attr_name) {
            return RegularAttrUpdate::Assignment {
                property: attr_name.to_string(),
            };
        }

        RegularAttrUpdate::Call {
            setter_fn: if attr_name.starts_with("xlink") {
                "$.set_xlink_attribute"
            } else {
                "$.set_attribute"
            },
            attr_name: Some(attr_name.to_string()),
        }
    }

    pub(super) fn push_regular_attr_update(
        &self,
        target: &mut Vec<Statement<'a>>,
        el_name: &str,
        update: RegularAttrUpdate,
        val: Expression<'a>,
    ) {
        let b = &self.ctx.b;
        match update {
            RegularAttrUpdate::Call {
                setter_fn,
                attr_name,
            } => {
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(el_name)];
                if let Some(name) = attr_name {
                    args.push(Arg::Str(name));
                }
                args.push(Arg::Expr(val));
                target.push(b.call_stmt(setter_fn, args));
            }
            RegularAttrUpdate::Assignment { property } => {
                target.push(b.assign_stmt(
                    AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), &property)),
                    val,
                ));
            }
        }
    }

    pub(super) fn memoize_regular_attr_update(
        &self,
        memo_attrs: &mut Vec<MemoAttr<'a>>,
        attr_id: NodeId,
        el_name: &str,
        update: RegularAttrUpdate,
        expr: Expression<'a>,
    ) {
        let update = match update {
            RegularAttrUpdate::Call {
                setter_fn,
                attr_name,
            } => MemoAttrUpdate::Call {
                setter_fn,
                attr_name,
            },
            RegularAttrUpdate::Assignment { property } => MemoAttrUpdate::Assignment { property },
        };
        memo_attrs.push(MemoAttr {
            attr_id,
            el_name: el_name.to_string(),
            update,
            expr,
            is_node_site: false,
        });
    }

    pub(super) fn wrap_run_after_blockers(
        &self,
        stmt: Statement<'a>,
        blockers: &[u32],
    ) -> Statement<'a> {
        if blockers.is_empty() {
            return stmt;
        }
        let b = &self.ctx.b;
        let blockers_arr = b.promises_array(blockers);
        let thunk = b.thunk_block(vec![stmt]);
        b.call_stmt(
            "$.run_after_blockers",
            [Arg::Expr(blockers_arr), Arg::Expr(thunk)],
        )
    }

    pub(in super::super) fn build_concat_expr_collapse_single(
        &mut self,
        attr_id: NodeId,
        parts: &[svelte_ast::ConcatPart],
    ) -> Result<Expression<'a>> {
        let mut tpl_parts = self.concat_to_tpl_parts(attr_id, parts, true)?;

        if tpl_parts.len() == 1 {
            if let TemplatePart::Str(s) = &tpl_parts[0] {
                let expr = self.ctx.b.str_expr(s);
                tpl_parts.clear();
                return Ok(expr);
            }
        }

        Ok(self.ctx.b.template_parts_expr(tpl_parts))
    }

    pub(super) fn build_concat_expr_template(
        &mut self,
        attr_id: NodeId,
        parts: &[svelte_ast::ConcatPart],
    ) -> Result<Expression<'a>> {
        let tpl_parts = self.concat_to_tpl_parts(attr_id, parts, false)?;
        Ok(self.ctx.b.template_parts_expr(tpl_parts))
    }

    fn concat_to_tpl_parts(
        &mut self,
        attr_id: NodeId,
        parts: &[svelte_ast::ConcatPart],
        fold_literals: bool,
    ) -> Result<Vec<TemplatePart<'a>>> {
        let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
        for part in parts {
            match part {
                svelte_ast::ConcatPart::Static(s) => push_tpl_str(&mut tpl_parts, s.clone()),
                svelte_ast::ConcatPart::Dynamic { expr: expr_ref, .. } => {
                    let Some(expr) = self.take_expr_by_ref(expr_ref) else {
                        return CodegenError::missing_expression(attr_id);
                    };
                    if fold_literals {
                        if let Some(lit) = literal_value(&expr) {
                            push_tpl_str(&mut tpl_parts, lit);
                            continue;
                        }
                    }
                    tpl_parts.push(TemplatePart::Expr(expr, false));
                }
            }
        }
        Ok(tpl_parts)
    }
}

fn has_static_text_attribute(el: &Element, name: &str) -> bool {
    el.attributes
        .iter()
        .any(|attr| matches!(attr, Attribute::StringAttribute(sa) if sa.name == name))
}

fn has_static_true_boolean_attribute(el: &Element, name: &str) -> bool {
    el.attributes
        .iter()
        .any(|attr| matches!(attr, Attribute::BooleanAttribute(ba) if ba.name == name))
}

fn push_tpl_str<'a>(tpl_parts: &mut Vec<TemplatePart<'a>>, value: String) {
    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
        prev.push_str(&value);
    } else {
        tpl_parts.push(TemplatePart::Str(value));
    }
}

fn literal_value(expr: &Expression<'_>) -> Option<String> {
    match expr {
        Expression::StringLiteral(lit) => Some(lit.value.as_str().to_string()),
        Expression::NumericLiteral(lit) => Some(lit.value.to_string()),
        Expression::BooleanLiteral(lit) => Some(lit.value.to_string()),
        Expression::NullLiteral(_) => Some(String::new()),
        _ => None,
    }
}
