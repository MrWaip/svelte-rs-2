import * as $ from "svelte/internal/client";
var root = $.from_html(`<set-property-before-mounted></set-property-before-mounted> <set-property-before-mounted></set-property-before-mounted>`, 3);
export default function App($$anchor) {
	let value = "";
	var fragment = root();
	var set_property_before_mounted = $.first_child(fragment);
	$.set_custom_element_data(set_property_before_mounted, "property", value);
	var set_property_before_mounted_1 = $.sibling(set_property_before_mounted, 2);
	$.set_custom_element_data(set_property_before_mounted_1, "property", value);
	$.append($$anchor, fragment);
}
