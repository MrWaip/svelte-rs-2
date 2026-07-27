App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(1), "n");
	function bump() {
		$.update(n);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		$.validate_dynamic_element_tag(() => "h");
		$.validate_void_dynamic_element(() => "h");
		$.element(node, () => "h", false, ($$element, $$anchor) => {
			var text = $.text("hello");
			$.append($$anchor, text);
		}, void 0, [6, 0]);
	}
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
