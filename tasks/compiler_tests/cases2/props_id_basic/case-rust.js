import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	const id = $.props_id();
	var div = root();
	$.template_effect(() => $.set_attribute(div, "id", id));
	$.append($$anchor, div);
}
