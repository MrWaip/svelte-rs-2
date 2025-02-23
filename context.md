Context to track:

BindDirective:
expression_type: (SequenceExpression)
is_dev
check ancestry for specific type (IfBlock, EachBlock, AwaitBlock etc)
global unique identifier (binding_group)
is binding for an event (videoHeight - resize )
anchor node 
parent element name
parent_each_blocks
each.metadata.keyed
each.index
contain parent element for specific attr

RegularElement:
is dev
element name
is_custom_element
context.state.metadata.context.template_needs_import_node
context.state.metadata.context.template_contains_script_tag
context.state.metadata.context.template_contains_script_tag
class_directives
style_directives
other_directives
lets
lookup
bindings
has_spread
has_use
attibute name
context.state.metadata.namespace
contains for specific attrubute and his type
contains for specific binding by name
is svg
is mathml
cannot_be_set_statically
has reactive attribute
is_load_error_element
metadata.bound_contenteditable
check for specific attribute value
scope
preserve_whitespace
preserveComments
const use_text_content =
		trimmed.every((node) => node.type === 'Text' || node.type === 'ExpressionTag') &&
		trimmed.every((node) => node.type === 'Text' || !node.metadata.expression.has_state) &&
		trimmed.some((node) => node.type === 'ExpressionTag');
needs_reset
check for parent fragment contains SnippetBlock
node.fragment.metadata.dynamic
is_void
context.state.scope.references
 has_state
 is_dom_property
 has_call

Fragment context:



Each block context:
is_controlled
context.state.scope.parent
node.metadata.keyed
node.index
EACH_INDEX_REACTIVE
key_is_item
uses_store 
has binding store_sub
for (const binding of node.metadata.expression.dependencies) {
		// if the expression doesn't reference any external state, we don't need to
		// create a source for the item. TODO cover more cases (e.g. `x.filter(y)`
		// should also qualify if `y` doesn't reference state, and non-state
		// bindings should also be fine
		if (binding.scope.function_depth >= context.state.scope.function_depth) {
			continue;
		}

		if (!context.state.analysis.runes || !key_is_item || uses_store) {
			flags |= EACH_ITEM_REACTIVE;
			break;
		}
	}

    	if (context.state.analysis.runes && !uses_store) {
		flags |= EACH_ITEM_IMMUTABLE;
	}

EACH_IS_ANIMATED
context.state.scope.declarations
context.state.scope.parent
context.state.scope.root.unique('$$array');
collection_id
indirect_dependencies
transitive_dependencies
contains_group_binding
has_default_value
needs_derived
child_state.transform



Interpolation Context:
