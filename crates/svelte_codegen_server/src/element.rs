use std::iter::empty;

use oxc_ast::ast::Statement;
use svelte_ast::{Element, Node, is_void};
use svelte_ast_builder::Arg;

use crate::error::Result;
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn element(
        &mut self,
        element: &'a Element,
        preserve_whitespace: bool,
    ) -> Result<()> {
        let open_tag = self.open_tag(element);

        if let Some(raw_content) = self.raw_text_content(element) {
            let close_tag = format!("</{}>", element.name);
            let raw_statements = self.child_statements(|codegen| {
                codegen.push_text(&format!("{open_tag}{raw_content}{close_tag}"));
                Ok(())
            })?;
            for stmt in raw_statements {
                self.push_stmt(stmt);
            }
            return Ok(());
        }

        self.push_text(&open_tag);

        if self.dev {
            let push_element = self.push_element_stmt(element);
            self.push_stmt(push_element);
        }

        let child_preserve_whitespace =
            preserve_whitespace || element.name == "pre" || element.name == "textarea";
        self.fragment(
            element.fragment,
            FragmentParent::Element(element),
            child_preserve_whitespace,
        )?;

        if !is_void(&element.name) {
            self.push_text(&format!("</{}>", element.name));
        }

        if self.dev {
            let pop_element = self
                .b
                .expr_stmt(self.b.call_expr("$.pop_element", empty::<Arg<'_, '_>>()));
            self.push_stmt(pop_element);
        }
        Ok(())
    }

    fn open_tag(&self, element: &Element) -> String {
        let mut open_tag = format!("<{}", element.name);
        for attribute in &element.attributes {
            self.print_attribute(attribute, &mut open_tag);
        }
        if is_void(&element.name) {
            open_tag.push_str("/>");
        } else {
            open_tag.push('>');
        }
        open_tag
    }

    fn raw_text_content(&self, element: &Element) -> Option<String> {
        if element.name != "script" && element.name != "style" {
            return None;
        }
        let nodes = self.component.store.fragment_nodes(element.fragment);
        let [only] = nodes else {
            return None;
        };
        let Node::Text(text) = self.component.store.get(*only) else {
            return None;
        };
        Some(text.value(self.component.source.as_str()).to_string())
    }

    fn push_element_stmt(&self, element: &Element) -> Statement<'a> {
        let (line, col) = self.line_index.line_col(element.span.start);
        let name: &str = self.b.alloc_str(&element.name);
        self.b.expr_stmt(self.b.call_expr(
            "$.push_element",
            [
                Arg::Ident("$$renderer"),
                Arg::StrRef(name),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        ))
    }
}
