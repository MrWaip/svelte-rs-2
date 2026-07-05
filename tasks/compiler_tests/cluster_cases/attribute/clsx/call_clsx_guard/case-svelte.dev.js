import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	function fn(v) {
		return v;
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(($0) => $.set_class(div, 1, $0), [() => $.clsx(($.deep_read_state(x()), $.untrack(() => fn(x()))))]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
