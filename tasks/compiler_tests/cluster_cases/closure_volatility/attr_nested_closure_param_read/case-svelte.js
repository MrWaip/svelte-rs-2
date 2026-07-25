import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var div = root();
	$.set_attribute(div, "title", [() => ((y) => y)(1)]);
	$.append($$anchor, div);
	$.pop();
}
