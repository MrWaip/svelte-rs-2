use oxc_ast::ast::{Expression, Program};

use crate::{
    Attribute, AttributeValue, ConcatenationPart, Element, IfBlock, Interpolation, Node, ScriptTag,
    Text, VirtualConcatenation,
};

pub trait FormatNode {
    fn format_node(&self) -> String;
}

impl<'a> FormatNode for Node<'a> {
    fn format_node(&self) -> String {
        return match self {
            Node::Element(it) => it.borrow().format_node(),
            Node::Text(it) => it.borrow().format_node(),
            Node::Interpolation(it) => it.borrow().format_node(),
            Node::IfBlock(it) => it.borrow().format_node(),
            Node::VirtualConcatenation(it) => it.borrow().format_node(),
            Node::ScriptTag(it) => it.borrow().format_node(),
        };
    }
}

impl<'a> FormatNode for Interpolation<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("{ ");

        let expr_string = print_expression(&self.expression);
        result.push_str(&expr_string);

        result.push_str(" }");

        return result;
    }
}

impl<'a> FormatNode for ScriptTag<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<script");

        if self.is_typescript() {
            result.push_str(" lang=\"ts\"");
        }

        result.push_str(">");

        result.push_str(&print_program(&self.program));

        result.push_str("</script>");

        return result;
    }
}

impl<'a> FormatNode for VirtualConcatenation<'a> {
    fn format_node(&self) -> String {
        todo!()
    }
}

impl<'a> FormatNode for Text<'a> {
    fn format_node(&self) -> String {
        return self.value.to_string();
    }
}

impl<'a> FormatNode for Element<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<");
        result.push_str(&self.name);

        if !self.attributes.is_empty() {
            result.push_str(" ");
            let mut attributes = vec![];

            for attr in self.attributes.iter() {
                let mut result = String::new();

                match attr {
                    Attribute::HTMLAttribute(attr) => {
                        result.push_str(attr.name);

                        match &attr.value {
                            AttributeValue::String(value) => {
                                result.push_str(format!("=\"{}\"", value).as_str());
                            }
                            AttributeValue::Boolean => (),
                            AttributeValue::Expression(expression) => {
                                let expr_string = print_expression(&expression.expression);
                                result.push_str(format!("={{{}}}", expr_string).as_str());
                            }
                            AttributeValue::Concatenation(concatenation) => {
                                result.push_str("=\"");

                                for part in concatenation.parts.iter() {
                                    match part {
                                        ConcatenationPart::String(value) => result.push_str(value),
                                        ConcatenationPart::Expression(expression) => {
                                            let expr_string = print_expression(expression);
                                            result
                                                .push_str(format!("{{{}}}", expr_string).as_str());
                                        }
                                    }
                                }

                                result.push_str("\"");
                            }
                        }
                    }
                    Attribute::Expression(expression) => {
                        let expr_string = print_expression(&expression.expression);
                        result.push_str(format!("{{{}}}", expr_string).as_str());
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
                }

                attributes.push(result);
            }

            result.push_str(attributes.join(" ").as_str());
        }

        if self.self_closing {
            result.push_str("/>");
            return result;
        } else {
            result.push_str(">");
        }

        for node in self.nodes.iter() {
            let formatted = node.format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push_str(">");

        return result;
    }
}

impl<'a> FormatNode for IfBlock<'a> {
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

        return result;
    }
}

fn print_expression<'a>(expression: &Expression<'a>) -> String {
    let mut codegen = oxc_codegen::Codegen::default();
    codegen.print_expression(expression);
    return codegen.into_source_text();
}

fn print_program<'a>(program: &Program<'a>) -> String {
    let codegen = oxc_codegen::Codegen::default();

    return codegen.build(program).code;
}
