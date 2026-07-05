import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8);
	function fn(v) {
		return v + 1;
	}
	var $$exports = { ...$.legacy_api() };
	var p = root();
	$.template_effect(($0) => $.set_attribute(p, "data-x", $0), [() => ($.deep_read_state(value()), $.untrack(() => fn(value())))]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
