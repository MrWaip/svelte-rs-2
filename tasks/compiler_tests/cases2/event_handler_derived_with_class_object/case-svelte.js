import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let counter = $.state(0);
	let active = false;
	function getHandler() {
		return () => $.update(counter);
	}
	var div = root();
	let classes;
	var event_handler = $.derived(getHandler);
	$.template_effect(() => classes = $.set_class(div, 1, $.clsx({ big: $.get(counter) > 10 }), null, classes, { active }));
	$.event("focus", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
