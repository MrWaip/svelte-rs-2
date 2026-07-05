import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { createBubbler } from "svelte/legacy";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let rest = $.prop($$props, "rest", 24, () => ({}));
	const bubbler = createBubbler();
	var $$exports = { ...$.legacy_api() };
	$.init();
	var div = root();
	var event_handler = $.derived(() => bubbler("click"));
	$.attribute_effect(div, () => ({ ...rest() }));
	$.event("click", div, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [6, 25], true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
