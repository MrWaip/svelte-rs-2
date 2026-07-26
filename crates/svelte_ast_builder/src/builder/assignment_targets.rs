use super::*;

use oxc_ast::ast::{
    AssignmentTargetMaybeDefault, AssignmentTargetProperty, BindingPattern, BindingProperty,
    BindingRestElement,
};

impl<'a> Builder<'a> {
    pub fn binding_pattern_to_assignment_target(
        &self,
        pattern: BindingPattern<'a>,
    ) -> Option<AssignmentTarget<'a>> {
        match pattern {
            BindingPattern::BindingIdentifier(id) => {
                let name = self.alloc_str(id.name.as_str());
                let reference = self.ast.identifier_reference(SPAN, name);
                Some(AssignmentTarget::AssignmentTargetIdentifier(
                    self.alloc(reference),
                ))
            }
            BindingPattern::ObjectPattern(object) => {
                let object = object.unbox();
                let mut properties = self.ast.vec();
                for property in object.properties {
                    properties.push(self.binding_property_to_target(property)?);
                }
                let rest = self.binding_rest_to_target(object.rest)?;
                let target = self.ast.object_assignment_target(SPAN, properties, rest);
                Some(AssignmentTarget::ObjectAssignmentTarget(self.alloc(target)))
            }
            BindingPattern::ArrayPattern(array) => {
                let array = array.unbox();
                let mut elements = self.ast.vec();
                for element in array.elements {
                    let element = match element {
                        Some(element) => Some(self.binding_pattern_to_maybe_default(element)?),
                        None => None,
                    };
                    elements.push(element);
                }
                let rest = self.binding_rest_to_target(array.rest)?;
                let target = self.ast.array_assignment_target(SPAN, elements, rest);
                Some(AssignmentTarget::ArrayAssignmentTarget(self.alloc(target)))
            }
            BindingPattern::AssignmentPattern(_) => None,
        }
    }

    fn binding_pattern_to_maybe_default(
        &self,
        pattern: BindingPattern<'a>,
    ) -> Option<AssignmentTargetMaybeDefault<'a>> {
        let BindingPattern::AssignmentPattern(assignment) = pattern else {
            let target = self.binding_pattern_to_assignment_target(pattern)?;
            return Some(AssignmentTargetMaybeDefault::from(target));
        };
        let assignment = assignment.unbox();
        let binding = self.binding_pattern_to_assignment_target(assignment.left)?;
        let with_default = self
            .ast
            .assignment_target_with_default(SPAN, binding, assignment.right);
        Some(AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(
            self.alloc(with_default),
        ))
    }

    fn binding_property_to_target(
        &self,
        property: BindingProperty<'a>,
    ) -> Option<AssignmentTargetProperty<'a>> {
        if property.shorthand
            && let BindingPattern::BindingIdentifier(id) = &property.value
        {
            let name = self.alloc_str(id.name.as_str());
            let reference = self.ast.identifier_reference(SPAN, name);
            let shorthand = self
                .ast
                .assignment_target_property_identifier(SPAN, reference, None);
            return Some(
                AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(self.alloc(shorthand)),
            );
        }
        if property.shorthand
            && let BindingPattern::AssignmentPattern(assignment) = property.value
        {
            let assignment = assignment.unbox();
            let BindingPattern::BindingIdentifier(id) = assignment.left else {
                return None;
            };
            let name = self.alloc_str(id.name.as_str());
            let reference = self.ast.identifier_reference(SPAN, name);
            let shorthand = self.ast.assignment_target_property_identifier(
                SPAN,
                reference,
                Some(assignment.right),
            );
            return Some(
                AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(self.alloc(shorthand)),
            );
        }
        let binding = self.binding_pattern_to_maybe_default(property.value)?;
        let keyed = self.ast.assignment_target_property_property(
            SPAN,
            property.key,
            binding,
            property.computed,
        );
        Some(AssignmentTargetProperty::AssignmentTargetPropertyProperty(
            self.alloc(keyed),
        ))
    }

    fn binding_rest_to_target(
        &self,
        rest: Option<Box<'a, BindingRestElement<'a>>>,
    ) -> Option<Option<Box<'a, ast::AssignmentTargetRest<'a>>>> {
        let Some(rest) = rest else {
            return Some(None);
        };
        let target = self.binding_pattern_to_assignment_target(rest.unbox().argument)?;
        let rest = self.ast.assignment_target_rest(SPAN, target);
        Some(Some(self.alloc(rest)))
    }
}
