pub fn is_custom_element(element: &hir::Element, store: &hir::HirStore) -> bool {
    return element.name.contains("-")
        || element.attributes.iter().any(|attribute_id| {
            let attr = store.get_attribute(*attribute_id);

            attr.name() == "is"
        });
}

pub fn is_static_element(node: &hir::Node, store: &hir::HirStore) -> bool {
    let dynamic = false;
    let hir::Node::Element(element) = node else {
        return false;
    };

    if dynamic {
        return false;
    }

    // we're setting all attributes on custom elements through properties
    if is_custom_element(element, store) {
        return false;
    }

    // todo
    //

    return true;
}

// /**
//  * @param {AST.SvelteNode} node
//  * @param {ComponentContext["state"]} state
//  */
// function is_static_element(node, state) {
// 	if (node.type !== 'RegularElement') return false;
// 	if (node.fragment.metadata.dynamic) return false;
// 	if (is_custom_element_node(node)) return false; // we're setting all attributes on custom elements through properties

// 	for (const attribute of node.attributes) {
// 		if (attribute.type !== 'Attribute') {
// 			return false;
// 		}

// 		if (is_event_attribute(attribute)) {
// 			return false;
// 		}

// 		if (cannot_be_set_statically(attribute.name)) {
// 			return false;
// 		}

// 		if (attribute.name === 'dir') {
// 			return false;
// 		}

// 		if (
// 			['input', 'textarea'].includes(node.name) &&
// 			['value', 'checked'].includes(attribute.name)
// 		) {
// 			return false;
// 		}

// 		if (node.name === 'option' && attribute.name === 'value') {
// 			return false;
// 		}

// 		// We need to apply src and loading after appending the img to the DOM for lazy loading to work
// 		if (node.name === 'img' && attribute.name === 'loading') {
// 			return false;
// 		}

// 		if (attribute.value !== true && !is_text_attribute(attribute)) {
// 			return false;
// 		}
// 	}

// 	return true;
// }
