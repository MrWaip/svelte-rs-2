App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>content</div>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let counter = $.tag($.state(0), "counter");
	let active = false;
	function getHandler() {
		return () => $.update(counter);
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	var event_handler = $.derived(getHandler);
	$.template_effect(() => classes = $.set_class(div, 1, $.clsx({ big: $.get(counter) > 10 }), null, classes, { active }));
	$.event("focus", div, function(...$$args) {
		$.apply(() => $.get(event_handler), this, $$args, App, [13, 13], true, true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
