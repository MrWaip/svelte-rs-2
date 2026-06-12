import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let count = $.state(0);
	function make() {
		return () => $.update(count);
	}
	var div = root();
	var event_handler = $.derived(make);
	$.template_effect(() => $.set_attribute(div, "title", $.get(count)));
	$.delegated("click", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
$.delegate(["click"]);
