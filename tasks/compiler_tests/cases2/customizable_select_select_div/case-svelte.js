import * as $ from "svelte/internal/client";
var select_content = $.from_html(`<div>Rich</div>`, 1);
var root = $.from_html(`<select><!></select>`);
export default function App($$anchor) {
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
}
