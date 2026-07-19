import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let opacity = $.prop($$props, "opacity", 3, .5);
	var div = root();
	$.attach(div, () => (node) => node.style.opacity = opacity());
	$.append($$anchor, div);
}
