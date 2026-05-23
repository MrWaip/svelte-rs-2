import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let h = $.prop($$props, "h", 15, 0);
	var div = root();
	$.bind_element_size(div, "clientHeight", h);
	$.append($$anchor, div);
	$.pop();
}
