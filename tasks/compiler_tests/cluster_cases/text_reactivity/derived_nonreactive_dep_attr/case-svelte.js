import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function load() {
		return { foo: 1 };
	}
	const c = load();
	const x = $.derived(() => c.foo);
	var div = root();
	$.template_effect(() => $.set_attribute(div, "title", $.get(x)));
	$.append($$anchor, div);
}
