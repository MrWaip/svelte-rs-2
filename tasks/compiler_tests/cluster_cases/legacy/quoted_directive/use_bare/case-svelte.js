import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function foo(node) {}
	var div = root();
	$.action(div, ($$node) => foo?.($$node));
	$.append($$anchor, div);
}
