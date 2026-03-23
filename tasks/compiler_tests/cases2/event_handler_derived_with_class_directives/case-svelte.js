import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let counter = $.state(0);
	let active = false;
	function getHandler() {
		return () => $.update(counter);
	}
	var div = root();
	var event_handler = $.derived(getHandler);
	let classes;
	$.set_style(div, "", {}, { color: active ? "red" : "blue" });
	$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, {
		active,
		big: $.get(counter) > 10
	}));
	$.event("focus", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
