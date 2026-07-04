import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 12);
	const onClick = (v) => () => {
		value(v);
	};
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var event_handler = $.derived(() => onClick(1));
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "chip", null, classes, { active: $.strict_equals(value(), 1) }));
	$.event("click", div, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [14, 14], true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
