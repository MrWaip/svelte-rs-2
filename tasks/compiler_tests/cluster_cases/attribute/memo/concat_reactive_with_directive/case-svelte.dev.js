import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let status = $.prop($$props, "status", 8, "neutral");
	let disabled = $.prop($$props, "disabled", 8, false);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, `slider ${(status() || "") ?? ""}`, null, classes, { disabled: disabled() }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
