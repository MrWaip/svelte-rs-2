import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	function f() {
		return () => {};
	}
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...$$props.rest,
		onclick: $0
	}), [() => f()]);
	$.append($$anchor, div);
}
