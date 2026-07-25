App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tags = $.tag($.state($.proxy(["div", "span"])), "tags");
	function bump() {
		$.set(tags, ["p"], true);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.each(node, 17, () => $.get(tags), $.index, ($$anchor, t) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			$.validate_dynamic_element_tag(() => $.get(t));
			$.validate_void_dynamic_element(() => $.get(t));
			$.element(node_1, () => $.get(t), false, ($$element, $$anchor) => {
				var text = $.text("hello");
				$.append($$anchor, text);
			}, void 0, [7, 1]);
		}
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
