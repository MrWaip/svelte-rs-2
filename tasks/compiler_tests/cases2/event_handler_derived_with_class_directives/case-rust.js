import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let counter = $.state(0);
	let color = $.state("red");
	function getHandler() {
		return () => {
			$.update(counter);
			$.set(color, "blue");
		};
	}
	var div = root();
	var event_handler = $.derived(getHandler);
	let classes;
	let styles;
	$.template_effect(() => {
		classes = $.set_class(div, 1, "", null, classes, {
			active: $.get(counter) > 5,
			big: $.get(counter) > 10
		});
		styles = $.set_style(div, "", styles, { color: $.get(color) });
	});
	$.event("focus", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
