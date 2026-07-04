import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx([a(), b()])));
	$.append($$anchor, div);
	return $.pop($$exports);
}
