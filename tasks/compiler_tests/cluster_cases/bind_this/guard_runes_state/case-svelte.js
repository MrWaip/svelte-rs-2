import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let el = $.state(void 0);
	var div = root();
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, div);
}
