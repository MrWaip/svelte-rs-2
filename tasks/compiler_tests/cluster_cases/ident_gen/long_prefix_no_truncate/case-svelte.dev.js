App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<set-property-before-mounted></set-property-before-mounted> <set-property-before-mounted></set-property-before-mounted>`, 3), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = "";
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var set_property_before_mounted = $.first_child(fragment);
	$.set_custom_element_data(set_property_before_mounted, "property", value);
	var set_property_before_mounted_1 = $.sibling(set_property_before_mounted, 2);
	$.set_custom_element_data(set_property_before_mounted_1, "property", value);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
