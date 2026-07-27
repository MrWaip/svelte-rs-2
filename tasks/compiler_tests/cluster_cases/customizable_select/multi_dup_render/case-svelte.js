import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<!>`, 1);
var root = $.from_html(`<select><!></select> <select><!></select>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root();
	var select = $.first_child(fragment);
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment_1 = select_content();
		var node = $.first_child(fragment_1);
		$.snippet(node, () => $$props.opt);
		$.append(anchor, fragment_1);
	});
	var select_1 = $.sibling(select, 2);
	$.customizable_select(select_1, () => {
		var anchor_1 = $.child(select_1);
		var fragment_2 = select_content();
		var node_1 = $.first_child(fragment_2);
		$.snippet(node_1, () => $$props.opt);
		$.append(anchor_1, fragment_2);
	});
	$.append($$anchor, fragment);
}
