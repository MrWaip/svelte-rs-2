import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	$.template_effect(() => $.set_attribute(div, "title", $$props.title));
	$.append($$anchor, div);
}
