import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<div>x</div><!>`, 1);
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor, $$props) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var node = $.sibling($.first_child(fragment));
		$.snippet(node, () => $$props.extra);
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
