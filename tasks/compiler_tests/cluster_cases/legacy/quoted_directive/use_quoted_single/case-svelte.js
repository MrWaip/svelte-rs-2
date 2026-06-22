import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function foo(node, x) {}
	let bar = 1;
	var div = root();
	$.action(div, ($$node, $$action_arg) => foo?.($$node, $$action_arg), () => bar);
	$.append($$anchor, div);
}
