use std::cell::RefMut;

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Program};
use oxc_span::Language;
use rccell::RcCell;
use span::{GetSpan, Span};

use diagnostics::Diagnostic;

pub struct Ast<'a> {
    pub template: Vec<RcCell<Node<'a>>>,
    pub script: Option<ScriptTag<'a>>,
}

pub trait FormatNode {
    fn format_node(&self) -> String;
}

pub trait AsNode<'a> {
    fn as_node(self) -> Node<'a>;
}

#[derive(Debug)]
pub enum Node<'a> {
    Element(Element<'a>),
    Text(Text<'a>),
    Interpolation(Interpolation<'a>),
    IfBlock(IfBlock<'a>),
    /** Напоминание для себя. Сейчас во время трансформации шаблона последовательность Text + Interpolation схлопывается в эту Node */
    VirtualConcatenation(VirtualConcatenation<'a>),
    ScriptTag(ScriptTag<'a>),
}

impl<'a> Node<'a> {
    pub fn as_rc_cell(self) -> RcCell<Node<'a>> {
        return RcCell::new(self);
    }

    pub fn is_if_block(&self) -> bool {
        return matches!(self, Node::IfBlock(_));
    }

    pub fn is_compressible(&self) -> bool {
        return matches!(self, Node::Text(_) | Node::Interpolation(_));
    }

    pub fn is_text(&self) -> bool {
        return matches!(self, Node::Text(_));
    }

    pub fn is_element(&self) -> bool {
        return matches!(self, Node::Element(_));
    }

    pub fn is_interpolation(&self) -> bool {
        return matches!(self, Node::Interpolation(_));
    }

    pub fn from_option_mut<'local, 'long>(
        option: Option<&'local mut RcCell<Node<'long>>>,
    ) -> Result<RefMut<'local, Node<'long>>, Diagnostic> {
        if let Some(cell) = option {
            let borrow = cell.try_borrow_mut().map_err(|_| unimplemented!())?;

            return Ok(borrow);
        } else {
            unimplemented!()
        }
    }
}

impl<'a> Into<ScriptTag<'a>> for Node<'a> {
    fn into(self) -> ScriptTag<'a> {
        return match self {
            Node::ScriptTag(script_tag) => script_tag,
            _ => panic!("node is not ScriptTag"),
        };
    }
}

impl<'short, 'long> TryInto<&'short mut IfBlock<'long>> for &'short mut Node<'long> {
    type Error = Diagnostic;

    fn try_into(self) -> Result<&'short mut IfBlock<'long>, Self::Error> {
        if let Node::IfBlock(if_block) = self {
            return Ok(if_block);
        } else {
            unimplemented!()
        }
    }
}

impl<'a> FormatNode for Node<'a> {
    fn format_node(&self) -> String {
        return match self {
            Node::Element(element) => element.format_node(),
            Node::Text(text) => text.format_node(),
            Node::Interpolation(interpolation) => interpolation.format_node(),
            Node::IfBlock(if_block) => if_block.format_node(),
            Node::VirtualConcatenation(virtual_concatenation) => {
                virtual_concatenation.format_node()
            }
            Node::ScriptTag(script_tag) => script_tag.format_node(),
        };
    }
}

impl<'a> GetSpan for Node<'a> {
    fn span(&self) -> Span {
        match self {
            Node::Element(element) => element.span,
            Node::Text(text) => text.span,
            Node::Interpolation(interpolation) => interpolation.span,
            Node::IfBlock(if_block) => if_block.span,
            Node::VirtualConcatenation(virtual_concatenation) => virtual_concatenation.span,
            Node::ScriptTag(script_tag) => script_tag.span,
        }
    }
}

#[derive(Debug)]
pub struct Interpolation<'a> {
    pub expression: Expression<'a>,
    pub span: Span,
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

impl<'a> AsNode<'a> for Interpolation<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Interpolation(self);
    }
}

#[derive(Debug)]
pub struct IfBlock<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub is_elseif: bool,
    pub consequent: Vec<RcCell<Node<'a>>>,
    pub alternate: Option<Vec<RcCell<Node<'a>>>>,
}
impl<'a> IfBlock<'a> {
    pub fn push(&mut self, node: RcCell<Node<'a>>) {
        if let Some(alternate) = self.alternate.as_mut() {
            alternate.push(node);
        } else {
            self.consequent.push(node);
        }
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
            let formatted = &node.borrow().format_node();
            result.push_str(formatted);
        }

        if let Some(alternate) = &self.alternate {
            if let Some(cell) = alternate.first() {
                let borrow = &*cell.borrow();

                if let Node::IfBlock(if_block) = borrow {
                    if !if_block.is_elseif {
                        result.push_str("{:else}");
                    }
                } else {
                    result.push_str("{:else}");
                }
            }

            for node in alternate.iter() {
                let formatted = &node.borrow().format_node();
                result.push_str(formatted);
            }
        }

        if !self.is_elseif {
            result.push_str("{/if}");
        }

        return result;
    }
}

impl<'a> AsNode<'a> for IfBlock<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::IfBlock(self);
    }
}

#[derive(Debug)]
pub struct Element<'a> {
    pub name: String,
    pub span: Span,
    pub self_closing: bool,
    pub has_complex_nodes: bool,
    pub nodes: Vec<RcCell<Node<'a>>>,
    pub attributes: Vec<Attribute<'a>>,
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
                                let expr_string = print_expression(expression);
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
                        let expr_string = print_expression(expression);
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
            let formatted = node.borrow().format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push_str(">");

        return result;
    }
}

impl<'a> AsNode<'a> for Element<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Element(self);
    }
}

impl<'a> Element<'a> {
    pub fn push(&mut self, node: RcCell<Node<'a>>) {
        if !self.has_complex_nodes {
            self.has_complex_nodes = !node.borrow().is_text();
        }

        self.nodes.push(node);
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub value: &'a str,
    pub span: Span,
}

impl<'a> FormatNode for Text<'a> {
    fn format_node(&self) -> String {
        return self.value.to_string();
    }
}

impl<'a> Text<'a> {
    pub fn is_removable(&self) -> bool {
        return self.value.chars().all(|char| char.is_whitespace());
    }

    pub fn trim_start(&mut self) -> bool {
        let new = self.value.trim_ascii_start();
        let trimmed = new.len() != self.value.len();
        self.value = new;
        return trimmed;
    }

    pub fn trim_end(&mut self) -> bool {
        let new = self.value.trim_ascii_end();
        let trimmed = new.len() != self.value.len();
        self.value = new;
        return trimmed;
    }

    pub fn trim_start_one_whitespace(&mut self, allocator: &'a Allocator) {
        if !self.trim_start() {
            return;
        }

        let mut new = String::from(" ");
        new.push_str(&self.value);

        self.value = allocator.alloc_str(new.as_str());
    }

    pub fn trim_end_one_whitespace(&mut self, allocator: &'a Allocator) {
        if !self.trim_end() {
            return;
        }

        let mut new = String::from(self.value);
        new.push(' ');

        self.value = allocator.alloc_str(new.as_str());
    }
}

impl<'a> AsNode<'a> for Text<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Text(self);
    }
}

#[derive(Debug)]
pub enum Attribute<'a> {
    HTMLAttribute(HTMLAttribute<'a>),
    Expression(Expression<'a>),
    ClassDirective(ClassDirective<'a>),
}

#[derive(Debug)]
pub struct ClassDirective<'a> {
    pub shorthand: bool,
    pub name: &'a str,
    pub expression: Expression<'a>,
}

#[derive(Debug)]
pub struct HTMLAttribute<'a> {
    pub name: &'a str,
    pub value: AttributeValue<'a>,
}

#[derive(Debug)]
pub enum AttributeValue<'a> {
    String(&'a str),
    Expression(Expression<'a>),
    Boolean,
    Concatenation(Concatenation<'a>),
}

#[derive(Debug)]
pub struct VirtualConcatenation<'a> {
    pub parts: Vec<ConcatenationPart<'a>>,
    pub span: Span,
}

impl<'a> FormatNode for VirtualConcatenation<'a> {
    fn format_node(&self) -> String {
        todo!()
    }
}

impl<'a> AsNode<'a> for VirtualConcatenation<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::VirtualConcatenation(self);
    }
}

#[derive(Debug)]
pub struct Concatenation<'a> {
    pub parts: Vec<ConcatenationPart<'a>>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ConcatenationPart<'a> {
    String(&'a str),
    Expression(Expression<'a>),
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

#[derive(Debug)]
pub struct ScriptTag<'a> {
    pub program: Program<'a>,
    pub span: Span,
    pub language: Language,
}

impl<'a> ScriptTag<'a> {
    pub fn is_typescript(&self) -> bool {
        return self.language == Language::TypeScript;
    }
}

impl<'a> AsNode<'a> for ScriptTag<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::ScriptTag(self);
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
