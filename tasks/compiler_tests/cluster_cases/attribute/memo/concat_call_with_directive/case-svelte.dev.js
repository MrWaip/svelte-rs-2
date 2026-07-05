import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let status = $.prop($$props, "status", 8, "neutral");
	let disabled = $.prop($$props, "disabled", 8, false);
	function classify(s) {
		return s + "-x";
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(($0) => classes = $.set_class(div, 1, `slider ${$0 ?? ""}`, null, classes, { disabled: disabled() }), [() => ($.deep_read_state(status()), $.untrack(() => classify(status()) || ""))]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
