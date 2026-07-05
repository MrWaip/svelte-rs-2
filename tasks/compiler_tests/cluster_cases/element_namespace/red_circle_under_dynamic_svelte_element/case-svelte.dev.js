App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<circle r="5"></circle>`), App[$.FILENAME], [[1, 29]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		$.validate_dynamic_element_tag(() => "svg");
		$.validate_void_dynamic_element(() => "svg");
		$.element(node, () => "svg", false, ($$element, $$anchor) => {
			var circle = root();
			$.append($$anchor, circle);
		}, void 0, [1, 0]);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
