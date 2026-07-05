use svelte_analyze::AttributeSemantics;
use svelte_ast::Attribute;

use crate::escape::escape_attribute;
use crate::model::ServerCodegen;

impl ServerCodegen<'_> {
    pub(crate) fn print_attribute(&self, attribute: &Attribute, out: &mut String) {
        match self.analysis.attributes.get(attribute.id()) {
            AttributeSemantics::NonSpecial | AttributeSemantics::StaticAttr => {
                self.print_static_attribute(attribute, out);
            }
            AttributeSemantics::ElementBind(_)
            | AttributeSemantics::WindowBind(_)
            | AttributeSemantics::DocumentBind(_)
            | AttributeSemantics::ComponentBind(_)
            | AttributeSemantics::Event(_)
            | AttributeSemantics::ComponentProp(_)
            | AttributeSemantics::SvelteComponentThis(_)
            | AttributeSemantics::ComponentSpread(_)
            | AttributeSemantics::ComponentAttach(_)
            | AttributeSemantics::BoundaryProp(_)
            | AttributeSemantics::HtmlConcat(_)
            | AttributeSemantics::CannotBeStatic(_)
            | AttributeSemantics::SpecialValueAttr(_)
            | AttributeSemantics::StyleDirectives(_)
            | AttributeSemantics::Autofocus
            | AttributeSemantics::RuntimeBehavior => {}
        }
    }

    fn print_static_attribute(&self, attribute: &Attribute, out: &mut String) {
        if let Attribute::StringAttribute(attr) = attribute {
            let value = attr.value(self.component.source.as_str());
            out.push(' ');
            out.push_str(&attr.name);
            out.push_str("=\"");
            out.push_str(&escape_attribute(value));
            out.push('"');
        }
    }
}
