import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 12);
	const onClick = (v) => () => {
		value(v);
	};
	var div = root();
	var event_handler = $.derived(() => onClick(1));
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "chip", null, classes, { active: value() === 1 }));
	$.event("click", div, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
