import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div></div>`);
export default function App($$anchor) {
	const fn = (node, options) => ({});
	let a = { b: { "c-d": fn } };
	let directive = $.derived(() => a);
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	$.each(node_1, 24, () => [], (i) => i, ($$anchor, i) => {
		var div = root_1();
		$.animation(div, () => $.get(directive).b["c-d"], null);
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
