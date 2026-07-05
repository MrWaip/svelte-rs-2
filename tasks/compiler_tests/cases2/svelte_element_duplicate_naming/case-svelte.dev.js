App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tag = $.prop($$props, "tag", 3, "div");
	let title = "hello";
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(tag);
		$.validate_void_dynamic_element(tag);
		$.element(node, tag, false, ($$element, $$anchor) => {
			$.set_class($$element, 0, "first");
			var text = $.text();
			text.nodeValue = "First: hello";
			$.append($$anchor, text);
		}, void 0, [6, 0]);
	}
	var node_1 = $.sibling(node, 2);
	{
		$.validate_dynamic_element_tag(tag);
		$.validate_void_dynamic_element(tag);
		$.element(node_1, tag, false, ($$element_1, $$anchor) => {
			$.set_class($$element_1, 0, "second");
			var text_1 = $.text();
			text_1.nodeValue = "Second: hello";
			$.append($$anchor, text_1);
		}, void 0, [10, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
