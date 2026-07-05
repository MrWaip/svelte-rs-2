import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const fn = (node, options) => ({});
	let obj = $.derived(() => ({ inner: fn }));
	var div = root();
	$.transition(3, div, () => $.get(obj).inner);
	$.append($$anchor, div);
}
