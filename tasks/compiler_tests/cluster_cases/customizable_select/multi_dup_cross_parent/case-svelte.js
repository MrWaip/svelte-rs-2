import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<!>`, 1);
var root = $.from_html(`<select><!></select> <select><optgroup label="g"><!></optgroup></select> <select><option><!></option></select>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root();
	var select = $.first_child(fragment);
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment_1 = select_content();
		var node = $.first_child(fragment_1);
		$.snippet(node, () => $$props.r);
		$.append(anchor, fragment_1);
	});
	var select_1 = $.sibling(select, 2);
	var optgroup = $.child(select_1);
	$.customizable_select(optgroup, () => {
		var anchor_1 = $.child(optgroup);
		var fragment_2 = select_content();
		var node_1 = $.first_child(fragment_2);
		$.snippet(node_1, () => $$props.r);
		$.append(anchor_1, fragment_2);
	});
	$.reset(select_1);
	var select_2 = $.sibling(select_1, 2);
	var option = $.child(select_2);
	$.customizable_select(option, () => {
		var anchor_2 = $.child(option);
		var fragment_3 = select_content();
		var node_2 = $.first_child(fragment_3);
		$.snippet(node_2, () => $$props.r);
		$.append(anchor_2, fragment_3);
	});
	$.reset(select_2);
	$.append($$anchor, fragment);
}
