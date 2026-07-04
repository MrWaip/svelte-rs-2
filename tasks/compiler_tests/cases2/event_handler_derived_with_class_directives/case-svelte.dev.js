App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>content</div>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let counter = $.tag($.state(0), "counter");
	let color = $.tag($.state("red"), "color");
	function getHandler() {
		return () => {
			$.update(counter);
			$.set(color, "blue");
		};
	}
	var $$exports = { ...$.legacy_api() };
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
		$.apply(() => $.get(event_handler), this, $$args, App, [14, 13], true, true);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
