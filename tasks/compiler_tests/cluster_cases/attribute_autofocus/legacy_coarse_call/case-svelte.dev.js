import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let adapter = $.prop($$props, "adapter", 8);
	let day = $.prop($$props, "day", 8);
	let focused = $.tag($.mutable_source(null), "focused");
	function pick() {
		$.set(focused, day());
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	$.autofocus(button, ($.get(focused), $.deep_read_state(adapter()), $.deep_read_state(day()), $.untrack(() => $.strict_equals($.get(focused), null, false) && adapter().isSame(day(), $.get(focused)))));
	$.event("click", button, pick);
	$.append($$anchor, button);
	return $.pop($$exports);
}
