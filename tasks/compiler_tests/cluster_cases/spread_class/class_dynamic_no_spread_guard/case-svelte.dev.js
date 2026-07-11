import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let c = $.prop($$props, "c", 8);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx(c())));
	$.append($$anchor, div);
	return $.pop($$exports);
}
