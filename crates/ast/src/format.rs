use oxc_ast::ast::{Expression, Program};

use crate::{
    Attribute, Comment, ConcatenationPart, Element, IfBlock, Interpolation, Node, ScriptTag, Text,
    VirtualConcatenation,
};

pub trait FormatNode {
    fn format_node(&self) -> String;
}

impl FormatNode for Node<'_> {
    fn format_node(&self) -> String {
        match self {
            Node::Element(it) => it.borrow().format_node(),
            Node::Text(it) => it.borrow().format_node(),
            Node::Interpolation(it) => it.borrow().format_node(),
            Node::IfBlock(it) => it.borrow().format_node(),
            Node::VirtualConcatenation(it) => it.borrow().format_node(),
            Node::ScriptTag(it) => it.borrow().format_node(),
            Node::Comment(it) => it.borrow().format_node(),
        }
    }
}

impl FormatNode for Interpolation<'_> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("{ ");

        let expr_string = print_expression(&self.expression);
        result.push_str(&expr_string);

        result.push_str(" }");

        result
    }
}

impl FormatNode for ScriptTag<'_> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<script");

        if self.is_typescript() {
            result.push_str(" lang=\"ts\"");
        }

        result.push('>');

        result.push_str(&print_program(&self.program));

        result.push_str("</script>");

        result
    }
}

impl FormatNode for VirtualConcatenation<'_> {
    fn format_node(&self) -> String {
        todo!()
    }
}

impl FormatNode for Text<'_> {
    fn format_node(&self) -> String {
        self.value.to_string()
    }
}

impl FormatNode for Element<'_> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push('<');
        result.push_str(&self.name);

        if !self.attributes.is_empty() {
            result.push(' ');
            let mut attributes = vec![];

            for attr in self.attributes.iter() {
                let mut result = String::new();

                match attr {
                    Attribute::ConcatenationAttribute(attr) => {
                        result.push_str(attr.name);
                        result.push_str("=\"");

                        for part in attr.parts.iter() {
                            match part {
                                ConcatenationPart::String(value) => result.push_str(value),
                                ConcatenationPart::Expression(expression) => {
                                    let expr_string = print_expression(expression);
                                    result.push_str(format!("{{{}}}", expr_string).as_str());
                                }
                            }
                        }

                        result.push('"');
                    }
                    Attribute::BooleanAttribute(attr) => {
                        result.push_str(attr.name);
                    }
                    Attribute::StringAttribute(attr) => {
                        result.push_str(attr.name);
                        result.push_str(format!("=\"{}\"", attr.value).as_str());
                    }
                    Attribute::ExpressionAttribute(attr) => {
                        let expr_string = print_expression(&attr.expression);

                        if attr.shorthand {
                            result.push_str(format!("{{{}}}", expr_string).as_str());
                        } else {
                            result.push_str(attr.name);
                            result.push_str(format!("={{{}}}", expr_string).as_str());
                        }
                    }
                    Attribute::ClassDirective(class_directive) => {
                        let expr_string = print_expression(&class_directive.expression);

                        result.push_str("class:");

                        if class_directive.shorthand {
                            result.push_str(class_directive.name);
                        } else {
                            result.push_str(class_directive.name);
                            result.push_str(&format!("={{{}}}", expr_string));
                        }
                    }
                    Attribute::BindDirective(bind_directive) => {
                        let expr_string = print_expression(&bind_directive.expression);

                        result.push_str("bind:");

                        if bind_directive.shorthand {
                            result.push_str(bind_directive.name);
                        } else {
                            result.push_str(bind_directive.name);
                            result.push_str(&format!("={{{}}}", expr_string));
                        }
                    }
                    Attribute::SpreadAttribute(attr) => {
                        let expr_string = print_expression(&attr.expression);

                        result.push_str(format!("{{...{}}}", expr_string).as_str());
                    }
                }

                attributes.push(result);
            }

            result.push_str(attributes.join(" ").as_str());
        }

        if self.self_closing {
            result.push_str("/>");
            return result;
        } else {
            result.push('>');
        }

        for node in self.nodes.iter() {
            let formatted = node.format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push('>');

        result
    }
}

impl FormatNode for Comment<'_> {
    fn format_node(&self) -> String {
        self.value.into()
    }
}

impl FormatNode for IfBlock<'_> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        if self.is_elseif {
            result.push_str(&format!("{{:else if {}}}", &print_expression(&self.test)));
        } else {
            result.push_str(&format!("{{#if {}}}", &print_expression(&self.test)));
        }

        for node in self.consequent.iter() {
            let formatted = &node.format_node();
            result.push_str(formatted);
        }

        if let Some(alternate) = &self.alternate {
            if let Some(node) = alternate.first() {
                if let Node::IfBlock(if_block) = node {
                    if !if_block.borrow().is_elseif {
                        result.push_str("{:else}");
                    }
                } else {
                    result.push_str("{:else}");
                }
            }

            for node in alternate.iter() {
                let formatted = &node.format_node();
                result.push_str(formatted);
            }
        }

        if !self.is_elseif {
            result.push_str("{/if}");
        }

        result
    }
}

fn print_expression(expression: &Expression<'_>) -> String {
    let mut codegen = oxc_codegen::Codegen::default();
    codegen.print_expression(expression);
    codegen.into_source_text()
}

fn print_program(program: &Program<'_>) -> String {
    let codegen = oxc_codegen::Codegen::default();

    codegen.build(program).code
}
